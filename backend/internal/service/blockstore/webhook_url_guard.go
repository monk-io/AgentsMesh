package blockstoreservice

import (
	"fmt"
	"net"
	"net/url"
	"os"
	"strings"

	"github.com/anthropics/agentsmesh/backend/internal/domain/blockstore"
)

// allowedHostsEnv holds hostnames that bypass the private-IP check. Intended
// for dev/CI where the backend inside Docker needs to hit a listener on the
// host via host.docker.internal (which resolves to RFC1918 inside the
// container). Production leaves this empty so every URL goes through the
// full private-address policy.
//
// Format: comma-separated hostnames, e.g. "host.docker.internal,host.lan".
// Wildcards are not supported to keep the policy obvious.
const allowedHostsEnv = "BLOCKSTORE_WEBHOOK_ALLOW_HOSTS"

// ValidateWebhookURL is the exported entry point used by both the REST MCP
// handler (create-time rejection) and the trigger engine (fire-time defense).
func ValidateWebhookURL(raw string) error {
	return validateWebhookURL(raw)
}

// validateWebhookURL blocks SSRF-prone targets before the trigger engine
// POSTs an outbound request. An Agent (or a compromised one) could otherwise
// set action.url to internal metadata endpoints (169.254.169.254), database
// ports on localhost, or RFC1918 hosts and use the backend as a proxy.
//
// Policy:
//   1. scheme must be http or https
//   2. host must parse; bare IP literals go through IP checks directly
//   3. host names that resolve to any private/loopback/link-local/unspecified
//      address are rejected. We resolve once at validation time; the actual
//      fire-time request will re-resolve but the policy is the same.
//   4. unresolvable host → reject (don't defer to HTTP client)
//
// Kept as a package-level function (not on Service) so the validator is
// reusable in both trigger.define (create-time reject) and fireWebhook
// (defense in depth if an older row was stored before this check landed).
func validateWebhookURL(raw string) error {
	u, err := url.Parse(raw)
	if err != nil {
		return fmt.Errorf("invalid url: %w", err)
	}
	scheme := strings.ToLower(u.Scheme)
	if scheme != "http" && scheme != "https" {
		return fmt.Errorf("unsupported scheme %q: only http/https allowed", u.Scheme)
	}
	host := u.Hostname()
	if host == "" {
		return fmt.Errorf("url missing host")
	}
	// Env-configured allowlist — dev/CI bypass for host.docker.internal and
	// friends. Checked before any IP resolution so the operator intent is
	// honored regardless of what DNS returns.
	if hostInAllowlist(host) {
		return nil
	}
	// Direct IP literal shortcut — skip DNS.
	if ip := net.ParseIP(host); ip != nil {
		if isPrivateIP(ip) {
			return fmt.Errorf("url host %q is a private or reserved address", host)
		}
		return nil
	}
	// Common localhost aliases that DNS may not surface as 127.0.0.1.
	switch strings.ToLower(host) {
	case "localhost", "localhost.localdomain", "ip6-localhost":
		return fmt.Errorf("url host %q is blocked", host)
	}
	addrs, err := net.LookupIP(host)
	if err != nil {
		return fmt.Errorf("host %q did not resolve: %w", host, err)
	}
	if len(addrs) == 0 {
		return fmt.Errorf("host %q resolved to no addresses", host)
	}
	for _, ip := range addrs {
		if isPrivateIP(ip) {
			return fmt.Errorf("url host %q resolves to private/reserved address %s", host, ip)
		}
	}
	return nil
}

// isPrivateIP returns true for addresses we refuse to contact from the
// trigger webhook path: loopback, RFC1918, CGNAT (100.64/10), link-local
// (incl. 169.254.0.0/16 where AWS/GCP metadata services live), multicast,
// broadcast, unspecified, and IPv6 ULA (fc00::/7).
func isPrivateIP(ip net.IP) bool {
	if ip == nil {
		return true
	}
	if ip.IsLoopback() || ip.IsLinkLocalUnicast() || ip.IsLinkLocalMulticast() ||
		ip.IsInterfaceLocalMulticast() || ip.IsMulticast() || ip.IsUnspecified() {
		return true
	}
	if ip4 := ip.To4(); ip4 != nil {
		// RFC1918 + CGNAT (100.64/10) + broadcast.
		switch {
		case ip4[0] == 10:
			return true
		case ip4[0] == 172 && ip4[1] >= 16 && ip4[1] <= 31:
			return true
		case ip4[0] == 192 && ip4[1] == 168:
			return true
		case ip4[0] == 100 && ip4[1] >= 64 && ip4[1] <= 127:
			return true
		case ip4[0] == 255 && ip4[1] == 255 && ip4[2] == 255 && ip4[3] == 255:
			return true
		}
		return false
	}
	// IPv6 ULA (fc00::/7).
	if len(ip) == net.IPv6len && (ip[0]&0xfe) == 0xfc {
		return true
	}
	return false
}

// hostInAllowlist checks BLOCKSTORE_WEBHOOK_ALLOW_HOSTS for an exact hostname
// match (case-insensitive). Empty env → no bypass.
func hostInAllowlist(host string) bool {
	raw := os.Getenv(allowedHostsEnv)
	if raw == "" {
		return false
	}
	host = strings.ToLower(host)
	for _, entry := range strings.Split(raw, ",") {
		if strings.ToLower(strings.TrimSpace(entry)) == host {
			return true
		}
	}
	return false
}

// validateTriggerDefData is the service-layer gate for trigger_def block
// writes. When action.kind == "webhook" it runs the SSRF guard on action.url.
// Called from applyCreateBlock / applyUpdateBlock so both gRPC MCP and REST
// writers of trigger_def blocks go through the same check — previously this
// lived in the REST MCP dispatcher and a direct /blocks/ops writer would
// have bypassed it.
//
// The predicate is kept tolerant of missing fields: an action block with no
// URL (e.g. action.kind == "agent") is simply not checked; the shape-level
// "action required" rule is handled by ValidateRecord upstream if the spec
// declares it.
func validateTriggerDefData(data blockstore.JSONMap) error {
	action, ok := data["action"].(map[string]any)
	if !ok {
		return nil
	}
	kind, _ := action["kind"].(string)
	if kind != "webhook" {
		return nil
	}
	rawURL, _ := action["url"].(string)
	if rawURL == "" {
		return nil
	}
	if err := validateWebhookURL(rawURL); err != nil {
		return fmt.Errorf("%w: action.url %s", blockstore.ErrColumnValueInvalid, err)
	}
	return nil
}
