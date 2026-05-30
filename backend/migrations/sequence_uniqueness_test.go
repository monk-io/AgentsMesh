package migrations

import (
	"regexp"
	"sort"
	"testing"
)

// migrationName matches golang-migrate's convention: <version>_<title>.(up|down).sql,
// where <version> is the zero-padded sequence number the migrator keys on.
var migrationName = regexp.MustCompile(`^(\d+)_.+\.(up|down)\.sql$`)

// findDuplicateMigrationSequences groups filenames by "<version>.<direction>"
// (e.g. "000157.up") and returns the groups owning more than one file — i.e. a
// sequence number two migrations both claim. Names that break the convention
// come back in `malformed`. Pure so the detection itself is unit-tested below.
func findDuplicateMigrationSequences(names []string) (dups map[string][]string, malformed []string) {
	byKey := map[string][]string{}
	for _, name := range names {
		m := migrationName.FindStringSubmatch(name)
		if m == nil {
			malformed = append(malformed, name)
			continue
		}
		byKey[m[1]+"."+m[2]] = append(byKey[m[1]+"."+m[2]], name)
	}
	dups = map[string][]string{}
	for key, files := range byKey {
		if len(files) > 1 {
			sort.Strings(files)
			dups[key] = files
		}
	}
	return dups, malformed
}

// TestNoDuplicateMigrationSequence fails when two migrations claim the same
// sequence number — the collision that happens when two branches each add the
// "next" migration (e.g. both add 000157_*) and both land on main. golang-migrate
// keys migrations by that version, so a duplicate silently applies the wrong file
// or wedges `migrate up` on a dirty version at deploy time. Static audit over the
// embedded corpus (no DB), run by CI via `bazel test //backend/...`.
func TestNoDuplicateMigrationSequence(t *testing.T) {
	entries, err := FS.ReadDir(".")
	if err != nil {
		t.Fatalf("read embedded migrations: %v", err)
	}
	names := make([]string, 0, len(entries))
	for _, e := range entries {
		if !e.IsDir() {
			names = append(names, e.Name())
		}
	}

	dups, malformed := findDuplicateMigrationSequences(names)
	for _, name := range malformed {
		t.Errorf("migration %q breaks the <version>_<title>.(up|down).sql convention", name)
	}
	keys := make([]string, 0, len(dups))
	for k := range dups {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	for _, k := range keys {
		t.Errorf("duplicate migration sequence number: %d files share %q: %v — renumber one to the next free sequence", len(dups[k]), k, dups[k])
	}
}

// TestFindDuplicateMigrationSequences proves the guard actually fires — a check
// that only ever sees clean input is no check at all.
func TestFindDuplicateMigrationSequences(t *testing.T) {
	if dups, mal := findDuplicateMigrationSequences([]string{
		"000001_a.up.sql", "000001_a.down.sql",
		"000002_b.up.sql", "000002_b.down.sql",
	}); len(dups) != 0 || len(mal) != 0 {
		t.Errorf("clean set flagged: dups=%v malformed=%v", dups, mal)
	}

	dups, _ := findDuplicateMigrationSequences([]string{
		"000157_foo.up.sql", "000157_foo.down.sql",
		"000157_bar.up.sql", "000157_bar.down.sql",
	})
	if got := dups["000157.up"]; len(got) != 2 {
		t.Errorf("expected the colliding 000157 up files caught, got %v", got)
	}
	if got := dups["000157.down"]; len(got) != 2 {
		t.Errorf("expected the colliding 000157 down files caught, got %v", got)
	}

	if _, mal := findDuplicateMigrationSequences([]string{"not_a_migration.sql"}); len(mal) != 1 {
		t.Errorf("expected the malformed name flagged, got %v", mal)
	}
}
