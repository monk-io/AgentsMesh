package envpath

import (
	"os"
	"path/filepath"
	"runtime"
	"slices"
	"strings"
	"testing"
)

func TestPrependToPath_ExactMatch_NoSubstringFalsePositive(t *testing.T) {
	// "/usr/local/bin" is a substring of "/usr/local/bin/extra" but they are
	// different PATH elements. PrependToPath must NOT skip "/usr/local/bin".
	sep := string(os.PathListSeparator)
	current := "/usr/local/bin/extra" + sep + "/usr/bin"

	result := PrependToPath(current, "/usr/local/bin")
	if !strings.HasPrefix(result, "/usr/local/bin"+sep) {
		t.Errorf("expected /usr/local/bin to be prepended, got: %s", result)
	}
}

func TestPrependToPath_SkipsDuplicateExactElement(t *testing.T) {
	sep := string(os.PathListSeparator)
	current := "/usr/local/bin" + sep + "/usr/bin"

	result := PrependToPath(current, "/usr/local/bin")
	if result != current {
		t.Errorf("expected no change when dir already exists exactly, got: %s", result)
	}
}

func TestPrependToPath_EmptyDirsSkipped(t *testing.T) {
	current := "/usr/bin"

	result := PrependToPath(current, "", "")
	if result != current {
		t.Errorf("expected no change for empty dirs, got: %s", result)
	}
}

func TestPrependToPath_MultipleNewDirs(t *testing.T) {
	sep := string(os.PathListSeparator)
	current := "/usr/bin"

	result := PrependToPath(current, "/opt/a", "/opt/b")

	// Dirs are prepended in order: /opt/a should come before /opt/b
	expected := "/opt/a" + sep + "/opt/b" + sep + current
	if result != expected {
		t.Errorf("expected %q, got %q", expected, result)
	}
}

func TestPrependToPath_MixedNewAndExisting(t *testing.T) {
	sep := string(os.PathListSeparator)
	current := "/usr/bin" + sep + "/opt/existing"

	result := PrependToPath(current, "/opt/new", "/opt/existing")

	// /opt/existing already present → only /opt/new prepended
	expected := "/opt/new" + sep + current
	if result != expected {
		t.Errorf("expected %q, got %q", expected, result)
	}
}

func TestPrependToPath_EmptyCurrent(t *testing.T) {
	sep := string(os.PathListSeparator)
	result := PrependToPath("", "/usr/bin")

	expected := "/usr/bin" + sep
	if result != expected {
		t.Errorf("expected %q, got %q", expected, result)
	}
}

func TestPrependToPath_WindowsSemicolonSeparator(t *testing.T) {
	if runtime.GOOS != "windows" {
		t.Skip("Windows-specific test")
	}

	current := `C:\Windows\System32;C:\Windows`
	result := PrependToPath(current, `C:\Go\bin`)

	if !strings.HasPrefix(result, `C:\Go\bin;`) {
		t.Errorf("expected C:\\Go\\bin to be prepended with semicolon, got: %s", result)
	}

	// Already existing → no change
	result2 := PrependToPath(current, `C:\Windows\System32`)
	if result2 != current {
		t.Errorf("expected no change for existing dir, got: %s", result2)
	}
}

// TestUserBinaryDirs_IncludesAgentSubdirs ensures agent installers that drop
// binaries under per-tool home subdirs (OpenCode → ~/.opencode/bin, some
// Cursor wrappers → ~/.cursor/bin) are reachable via the fallback search.
func TestUserBinaryDirs_IncludesAgentSubdirs(t *testing.T) {
	// Force a known HOME so the test exercises the happy path even in Bazel
	// sandboxes that may clear or unset $HOME (which would cause UserHomeDir
	// to return an error, taking us to the home-empty branch covered by the
	// dedicated test below).
	home := t.TempDir()
	t.Setenv("HOME", home)
	t.Setenv("USERPROFILE", home) // Windows

	dirs := UserBinaryDirs()

	for _, want := range []string{
		filepath.Join(home, ".local", "bin"),
		filepath.Join(home, "bin"),
		filepath.Join(home, ".opencode", "bin"),
		filepath.Join(home, ".cursor", "bin"),
	} {
		if !slices.Contains(dirs, want) {
			t.Errorf("UserBinaryDirs() missing %q\nfull list: %v", want, dirs)
		}
	}
}

// TestUserBinaryDirs_AllAbsolute documents the invariant that every returned
// path is absolute — never a CWD-relative confused-deputy hazard. The home-
// empty branch (UserHomeDir error / unset $HOME on Unix) must omit home-rooted
// entries entirely rather than emit relative ones like `.opencode/bin`.
func TestUserBinaryDirs_AllAbsolute(t *testing.T) {
	// Setting HOME="" simulates a UserHomeDir failure on Unix (Go falls back
	// to $HOME first). On Windows the equivalent is USERPROFILE="".
	t.Setenv("HOME", "")
	t.Setenv("USERPROFILE", "")

	dirs := UserBinaryDirs()
	for _, d := range dirs {
		if !filepath.IsAbs(d) {
			t.Errorf("UserBinaryDirs() returned non-absolute path %q with empty home; full list: %v", d, dirs)
		}
	}
}

func TestUserBinaryDirs_PlatformSpecificDirs(t *testing.T) {
	home := t.TempDir()
	t.Setenv("HOME", home)
	t.Setenv("USERPROFILE", home)

	dirs := UserBinaryDirs()

	switch runtime.GOOS {
	case "darwin":
		for _, want := range []string{"/opt/homebrew/bin", "/usr/local/bin"} {
			if !slices.Contains(dirs, want) {
				t.Errorf("darwin UserBinaryDirs() missing %q\nfull list: %v", want, dirs)
			}
		}
	case "linux":
		for _, want := range []string{"/usr/local/bin", "/snap/bin"} {
			if !slices.Contains(dirs, want) {
				t.Errorf("linux UserBinaryDirs() missing %q\nfull list: %v", want, dirs)
			}
		}
	case "windows":
		// Windows-specific dirs depend on env (LOCALAPPDATA/ProgramFiles);
		// the home-relative subdirs are covered by IncludesAgentSubdirs.
	default:
		// Other Unix (freebsd/openbsd/netbsd) fall through paths_unix.go's
		// `else` branch and currently get /snap/bin (Linux-specific). Skip
		// rather than assert, but make the gap visible.
		t.Skipf("UserBinaryDirs platform assertions not defined for GOOS=%s (returns: %v)", runtime.GOOS, dirs)
	}
}

// TestUserBinaryDirs_SystemDirsBeforeAgentSubdirs locks the precedence
// invariant: canonical system prefixes (/opt/homebrew/bin, /usr/local/bin)
// must be searched BEFORE per-tool home subdirs (~/.opencode/bin, etc),
// otherwise a stale ~/bin/claude shadows the brew-installed version.
func TestUserBinaryDirs_SystemDirsBeforeAgentSubdirs(t *testing.T) {
	if runtime.GOOS == "windows" {
		t.Skip("precedence test targets Unix system prefixes")
	}
	home := t.TempDir()
	t.Setenv("HOME", home)

	dirs := UserBinaryDirs()

	systemDir := "/usr/local/bin"
	agentSubdir := filepath.Join(home, ".opencode", "bin")
	sysIdx := slices.Index(dirs, systemDir)
	agentIdx := slices.Index(dirs, agentSubdir)

	if sysIdx < 0 || agentIdx < 0 {
		t.Fatalf("expected both %q and %q in list; got %v", systemDir, agentSubdir, dirs)
	}
	if sysIdx >= agentIdx {
		t.Errorf("system dir %q (idx=%d) must precede agent subdir %q (idx=%d); full list: %v",
			systemDir, sysIdx, agentSubdir, agentIdx, dirs)
	}
}
