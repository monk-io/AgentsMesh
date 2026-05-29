//go:build !windows

package envpath

import (
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
)

// UserBinaryDirs returns common directories where user-installed binaries live.
//
// Order matters — entries are searched and prepended to PATH in order:
// system prefixes come first so canonical (e.g. Homebrew) installs win over
// stale dotfile copies; per-tool home subdirs come last as a safety net for
// installers that bypass standard prefixes.
//
//   - darwin: ~/.local/bin, /opt/homebrew/bin, /opt/homebrew/sbin,
//     /usr/local/bin, ~/bin, ~/.opencode/bin, ~/.cursor/bin
//   - linux:  ~/.local/bin, /usr/local/bin, /snap/bin,
//     ~/bin, ~/.opencode/bin, ~/.cursor/bin
//
// If os.UserHomeDir() fails (e.g. systemd unit with no HOME and no passwd
// entry), home-rooted entries are omitted entirely — never returned as
// relative paths. LookPathFallback consumers therefore only ever see absolute
// paths, eliminating CWD-relative confused-deputy risk.
func UserBinaryDirs() []string {
	dirs := []string{}

	home, err := os.UserHomeDir()
	homeOK := err == nil && home != ""

	if homeOK {
		// ~/.local/bin is the XDG-standard user prefix — keep it FIRST so it
		// continues to behave as the canonical user install location.
		dirs = append(dirs, filepath.Join(home, ".local", "bin"))
	}

	if runtime.GOOS == "darwin" {
		dirs = append(dirs,
			"/opt/homebrew/bin",
			"/opt/homebrew/sbin",
			"/usr/local/bin",
		)
	} else {
		// linux and other unix
		dirs = append(dirs,
			"/usr/local/bin",
			"/snap/bin",
		)
	}

	// Per-tool home subdirs as a safety net for installers that bypass
	// standard prefixes (OpenCode's install.sh → ~/.opencode/bin when
	// $OPENCODE_INSTALL_DIR and $XDG_BIN_DIR are unset; some Cursor wrappers
	// → ~/.cursor/bin). Placed AFTER system dirs so brew/apt stays canonical.
	if homeOK {
		dirs = append(dirs,
			filepath.Join(home, "bin"),
			filepath.Join(home, ".opencode", "bin"),
			filepath.Join(home, ".cursor", "bin"),
		)
	}

	return dirs
}

// exeSuffix returns the executable file extension for the current OS.
func exeSuffix() string {
	return ""
}

// DefaultSystemPath returns a minimal system PATH for the current OS.
func DefaultSystemPath() string {
	return "/usr/bin:/bin:/usr/sbin:/sbin"
}

// ValidateExecutable on Unix returns path as-is (no extension validation needed).
func ValidateExecutable(path string) string {
	return path
}

// SafeLookPath on Unix delegates directly to exec.LookPath.
func SafeLookPath(command string) (string, error) {
	return exec.LookPath(command)
}
