package client

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"
	"time"
)

const defaultRESTTimeout = 30 * time.Second

// REST is a minimal client over backend's /api/v1 surface. Only the endpoints
// e2e fixtures actually need are exposed; we deliberately do not import the
// backend's REST request/response structs to keep this layer black-box and
// avoid cross-package coupling that would defeat the e2e contract.
type REST struct {
	baseURL string
	token   string
	hc      *http.Client
}

func NewREST(baseURL string) *REST {
	return &REST{
		baseURL: baseURL,
		hc:      &http.Client{Timeout: defaultRESTTimeout},
	}
}

// SetToken installs the bearer token returned by Login on every subsequent
// request. Tests typically call this once after a process-wide login.
func (r *REST) SetToken(t string) { r.token = t }

type LoginResponse struct {
	Token        string `json:"token"`
	RefreshToken string `json:"refresh_token"`
	ExpiresIn    int64  `json:"expires_in"`
}

func (r *REST) Login(ctx context.Context, email, password string) (*LoginResponse, error) {
	// REST /auth/login was deleted by R5 in favour of the Connect-RPC
	// AuthService.Login procedure. The MCP e2e client owns one REST→Connect
	// shim here (rather than rewriting every test against a typed Connect
	// client) because Login is auth bootstrap — every other call still
	// passes through the existing REST surface (`/orgs/:slug/...`).
	//
	// Connect-RPC binary procedure URL: /proto.auth.v1.AuthService/Login.
	// Wire format: JSON request, JSON response (Connect's
	// `application/json` profile — same as REST but with Connect-Protocol-Version
	// + the procedure-prefixed URL).
	//
	// Important: r.baseURL ends with `/api/v1` (set by fixture/env.go) which
	// is the REST namespace. Connect handlers mount at the bare `/proto.*`
	// prefix in backend/cmd/server/connect_init.go — strip the `/api/v1`
	// suffix here before forming the Connect URL.
	connectBase := strings.TrimSuffix(r.baseURL, "/api/v1")
	var out struct {
		Token        string `json:"token"`
		RefreshToken string `json:"refreshToken"`
		ExpiresIn    int64  `json:"expiresIn"`
	}
	body := map[string]string{"email": email, "password": password}
	buf, err := json.Marshal(body)
	if err != nil {
		return nil, err
	}
	req, err := http.NewRequestWithContext(ctx, http.MethodPost,
		connectBase+"/proto.auth.v1.AuthService/Login",
		bytes.NewReader(buf))
	if err != nil {
		return nil, err
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Connect-Protocol-Version", "1")
	resp, err := r.hc.Do(req)
	if err != nil {
		return nil, fmt.Errorf("connect login: %w", err)
	}
	defer resp.Body.Close()
	raw, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}
	if resp.StatusCode/100 != 2 {
		return nil, fmt.Errorf("connect login -> %d: %s", resp.StatusCode, string(raw))
	}
	if err := json.Unmarshal(raw, &out); err != nil {
		return nil, fmt.Errorf("decode login: %w (body=%s)", err, string(raw))
	}
	return &LoginResponse{
		Token:        out.Token,
		RefreshToken: out.RefreshToken,
		ExpiresIn:    out.ExpiresIn,
	}, nil
}

type Runner struct {
	ID     int64  `json:"id"`
	NodeID string `json:"node_id"`
	Status string `json:"status"`
}

type listRunnersResponse struct {
	Runners []Runner `json:"runners"`
	// Some endpoints return the slice directly; we also tolerate that.
}

// ListRunners returns the runners attached to an org. Backend wraps the slice
// in {runners: [...]} on this route; we tolerate either shape so future schema
// tweaks don't break this client.
func (r *REST) ListRunners(ctx context.Context, orgSlug string) ([]Runner, error) {
	path := fmt.Sprintf("/orgs/%s/runners", orgSlug)
	raw, err := r.doRaw(ctx, http.MethodGet, path, nil)
	if err != nil {
		return nil, err
	}
	var wrapped listRunnersResponse
	if err := json.Unmarshal(raw, &wrapped); err == nil && len(wrapped.Runners) > 0 {
		return wrapped.Runners, nil
	}
	var direct []Runner
	if err := json.Unmarshal(raw, &direct); err == nil {
		return direct, nil
	}
	return wrapped.Runners, nil
}

type CreatePodRequest struct {
	AgentSlug      string  `json:"agent_slug"`
	RunnerID       int64   `json:"runner_id,omitempty"`
	Alias          *string `json:"alias,omitempty"`
	AgentfileLayer *string `json:"agentfile_layer,omitempty"`
	Cols           int32   `json:"cols"`
	Rows           int32   `json:"rows"`
}

type Pod struct {
	ID      int64  `json:"id"`
	PodKey  string `json:"pod_key"`
	Status  string `json:"status"`
	Agent   string `json:"agent_slug"`
	OrgSlug string `json:"organization_slug,omitempty"`
}

type createPodResponse struct {
	Pod     Pod    `json:"pod"`
	Warning string `json:"warning,omitempty"`
}

func (r *REST) CreatePod(ctx context.Context, orgSlug string, req CreatePodRequest) (*Pod, error) {
	path := fmt.Sprintf("/orgs/%s/pods", orgSlug)
	var resp createPodResponse
	if err := r.do(ctx, http.MethodPost, path, req, &resp); err != nil {
		return nil, err
	}
	return &resp.Pod, nil
}

func (r *REST) TerminatePod(ctx context.Context, orgSlug, podKey string) error {
	path := fmt.Sprintf("/orgs/%s/pods/%s/terminate", orgSlug, podKey)
	return r.do(ctx, http.MethodPost, path, nil, nil)
}

func (r *REST) GetPod(ctx context.Context, orgSlug, podKey string) (*Pod, error) {
	path := fmt.Sprintf("/orgs/%s/pods/%s", orgSlug, podKey)
	var resp struct {
		Pod Pod `json:"pod"`
	}
	if err := r.do(ctx, http.MethodGet, path, nil, &resp); err != nil {
		return nil, err
	}
	return &resp.Pod, nil
}

type Org struct {
	ID   int64  `json:"id"`
	Name string `json:"name"`
	Slug string `json:"slug"`
}

// CreateOrg creates a new organization owned by the authenticated user. Used
// by the cross-org isolation spec to construct an org that the secondary
// user has no access to. Returns the created org's slug + id.
func (r *REST) CreateOrg(ctx context.Context, name, slug string) (*Org, error) {
	body := map[string]any{"name": name, "slug": slug}
	var resp struct {
		Organization Org `json:"organization"`
	}
	if err := r.do(ctx, http.MethodPost, "/orgs", body, &resp); err != nil {
		return nil, err
	}
	if resp.Organization.Slug == "" {
		// Some routes return the org as the top-level body.
		return nil, fmt.Errorf("create_org returned empty slug; raw=%+v", resp)
	}
	return &resp.Organization, nil
}

// DeleteOrg removes an org by slug. Used in t.Cleanup to keep the dev DB
// from accumulating per-test orgs.
func (r *REST) DeleteOrg(ctx context.Context, slug string) error {
	return r.do(ctx, http.MethodDelete, "/orgs/"+slug, nil, nil)
}

type CreateTicketRequest struct {
	Title    string `json:"title"`
	Content  string `json:"content,omitempty"`
	Priority string `json:"priority,omitempty"`
}

type Ticket struct {
	ID    int64  `json:"id"`
	Slug  string `json:"slug"`
	Title string `json:"title"`
}

// CreateTicket POSTs a ticket to a specific org. Used by the cross-org spec
// to seed a ticket in the foreign org from the human REST surface (avoiding
// the MCP route's content/BlockNote handling, which is exercised separately).
func (r *REST) CreateTicket(ctx context.Context, orgSlug string, req CreateTicketRequest) (*Ticket, error) {
	path := fmt.Sprintf("/orgs/%s/tickets", orgSlug)
	var resp struct {
		Ticket Ticket `json:"ticket"`
	}
	if err := r.do(ctx, http.MethodPost, path, req, &resp); err != nil {
		return nil, err
	}
	return &resp.Ticket, nil
}

type CreateLoopRequest struct {
	Name           string `json:"name"`
	AgentSlug      string `json:"agent_slug"`
	PromptTemplate string `json:"prompt_template"`
	RunnerID       *int64 `json:"runner_id,omitempty"`
}

type Loop struct {
	ID   int64  `json:"id"`
	Slug string `json:"slug"`
	Name string `json:"name"`
}

// CreateLoop seeds a loop directly via REST so loop e2e specs have a target
// to trigger. Backend's CreateLoop validation requires `prompt_template`
// non-empty; the rest of the request struct (autopilot config, retention,
// etc.) defaults to backend-side sensible values.
func (r *REST) CreateLoop(ctx context.Context, orgSlug string, req CreateLoopRequest) (*Loop, error) {
	path := fmt.Sprintf("/orgs/%s/loops", orgSlug)
	var resp struct {
		Loop Loop `json:"loop"`
	}
	if err := r.do(ctx, http.MethodPost, path, req, &resp); err != nil {
		return nil, err
	}
	return &resp.Loop, nil
}

// EnableLoop flips a loop from disabled (the default after creation) to
// enabled so trigger_loop will dispatch instead of returning a sentinel.
func (r *REST) EnableLoop(ctx context.Context, orgSlug, loopSlug string) error {
	path := fmt.Sprintf("/orgs/%s/loops/%s/enable", orgSlug, loopSlug)
	return r.do(ctx, http.MethodPost, path, nil, nil)
}

func (r *REST) do(ctx context.Context, method, path string, body, out any) error {
	raw, err := r.doRaw(ctx, method, path, body)
	if err != nil {
		return err
	}
	if out == nil || len(raw) == 0 {
		return nil
	}
	if err := json.Unmarshal(raw, out); err != nil {
		return fmt.Errorf("decode %s %s: %w (body=%s)", method, path, err, string(raw))
	}
	return nil
}

func (r *REST) doRaw(ctx context.Context, method, path string, body any) ([]byte, error) {
	var rdr io.Reader
	if body != nil {
		buf, err := json.Marshal(body)
		if err != nil {
			return nil, err
		}
		rdr = bytes.NewReader(buf)
	}
	req, err := http.NewRequestWithContext(ctx, method, r.baseURL+path, rdr)
	if err != nil {
		return nil, err
	}
	if body != nil {
		req.Header.Set("Content-Type", "application/json")
	}
	if r.token != "" {
		req.Header.Set("Authorization", "Bearer "+r.token)
	}
	resp, err := r.hc.Do(req)
	if err != nil {
		return nil, fmt.Errorf("rest http: %w", err)
	}
	defer resp.Body.Close()
	raw, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}
	if resp.StatusCode/100 != 2 {
		return nil, fmt.Errorf("rest %s %s -> %d: %s", method, path, resp.StatusCode, string(raw))
	}
	return raw, nil
}
