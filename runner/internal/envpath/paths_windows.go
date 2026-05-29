//go:build windows

package envpath

import (
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

// UserBinaryDirs returns common directories where user-installed binaries
// live on Windows. Order: system prefixes first (canonical installs win),
// per-tool home subdirs last (safety net for installers that bypass
// standard prefixes).
//
//   - %USERPROFILE%\.local\bin
//   - %LOCALAPPDATA%\Programs
//   - %ProgramFiles%
//   - %USERPROFILE%\bin
//   - %USERPROFILE%\.opencode\bin
//   - %USERPROFILE%\.cursor\bin
//
// If os.UserHomeDir() fails (e.g. LOCAL SERVICE / SYSTEM account with no
// resolvable USERPROFILE), home-rooted entries are omitted — never returned
// as drive-root-relative paths (which would let any C:\ writer hijack a
// lookup).
func UserBinaryDirs() []string {
	dirs := []string{}

	home, err := os.UserHomeDir()
	homeOK := err == nil && home != ""

	if homeOK {
		dirs = append(dirs, filepath.Join(home, ".local", "bin"))
	}

	if localAppData := os.Getenv("LOCALAPPDATA"); localAppData != "" {
		dirs = append(dirs, filepath.Join(localAppData, "Programs"))
	}

	if programFiles := os.Getenv("ProgramFiles"); programFiles != "" {
		dirs = append(dirs, programFiles)
	}

	if homeOK {
		dirs = append(dirs,
			filepath.Join(home, "bin"),
			filepath.Join(home, ".opencode", "bin"),
			filepath.Join(home, ".cursor", "bin"),
		)
	}

	return dirs
}

// exeSuffix returns the executable file extension for Windows.
func exeSuffix() string {
	return ".exe"
}

// DefaultSystemPath returns a minimal system PATH for Windows.
func DefaultSystemPath() string {
	systemRoot := os.Getenv("SystemRoot")
	if systemRoot == "" {
		systemRoot = `C:\Windows`
	}
	return filepath.Join(systemRoot, "System32") + ";" + systemRoot
}

// defaultWindowsExts is the fallback list when PATHEXT is empty.
var defaultWindowsExts = []string{".COM", ".EXE", ".BAT", ".CMD"}

// ValidateExecutable checks that path has a Windows-executable extension (from PATHEXT).
// If not (e.g. an extensionless npm Unix shell script), it probes path+ext for each
// PATHEXT extension and returns the first match. Returns "" if nothing is executable.
func ValidateExecutable(path string) string {
	exts := getPathExts()

	// Already has a valid extension — return as-is.
	ext := strings.ToUpper(filepath.Ext(path))
	for _, e := range exts {
		if ext == e {
			return path
		}
	}

	// Try appending each PATHEXT extension.
	for _, e := range exts {
		candidate := path + strings.ToLower(e)
		if info, err := os.Stat(candidate); err == nil && !info.IsDir() {
			return candidate
		}
	}
	return ""
}

// SafeLookPath searches PATH for a command, respecting PATHEXT on Windows.
// Unlike exec.LookPath, it never returns extensionless Unix shell scripts.
func SafeLookPath(command string) (string, error) {
	exts := getPathExts()
	pathEnv := os.Getenv("PATH")

	for _, dir := range filepath.SplitList(pathEnv) {
		if dir == "" {
			dir = "."
		}
		for _, e := range exts {
			candidate := filepath.Join(dir, command+strings.ToLower(e))
			if info, err := os.Stat(candidate); err == nil && !info.IsDir() {
				return candidate, nil
			}
		}
	}
	return "", &exec.Error{Name: command, Err: exec.ErrNotFound}
}

// getPathExts returns executable extensions from PATHEXT, or defaults.
func getPathExts() []string {
	pathext := os.Getenv("PATHEXT")
	if pathext == "" {
		return defaultWindowsExts
	}
	parts := strings.Split(strings.ToUpper(pathext), ";")
	var exts []string
	for _, p := range parts {
		p = strings.TrimSpace(p)
		if p != "" {
			exts = append(exts, p)
		}
	}
	if len(exts) == 0 {
		return defaultWindowsExts
	}
	return exts
}
