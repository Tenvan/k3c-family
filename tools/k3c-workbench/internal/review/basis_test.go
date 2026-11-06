package review

import (
	"context"
	"testing"
)

const basisCommit = "32eb7a7" // HEAD~1 im Review-Test-Repo (paper-mcp.md › Attrappen)

// TestBasisAufloesen: leer und HEAD = HEAD, HEAD~1 und SHA (kurz, lang) lösen auf, Stand aktuell (AC-02).
func TestBasisAufloesen(t *testing.T) {
	t.Parallel()
	repo := testRepo(t)
	head := gitIn(t, repo, "rev-parse", "--short=7", "HEAD")
	voll := gitIn(t, repo, "rev-parse", "HEAD~1")
	faelle := []struct{ basis, name, commit string }{
		{"", "HEAD", head},
		{"HEAD", "HEAD", head},
		{"HEAD~1", "HEAD~1", basisCommit},
		{voll, voll, basisCommit},
		{basisCommit, basisCommit, basisCommit},
	}
	for _, f := range faelle {
		s, err := Aufloesen(context.Background(), repo, f.basis)
		if err != nil {
			t.Fatalf("%q: %v", f.basis, err)
		}
		if s.Basis != f.name || s.Commit != f.commit || s.BasisStand != "aktuell" {
			t.Errorf("%q: %+v, will Basis %s, Commit %s, aktuell", f.basis, s, f.name, f.commit)
		}
	}
}

// TestBasisOriginDevelopLokal: origin/develop ohne erreichbares Remote → lokaler Stand.
func TestBasisOriginDevelopLokal(t *testing.T) {
	t.Parallel()
	repo := testRepo(t)
	gitIn(t, repo, "update-ref", "refs/remotes/origin/develop", "HEAD~1")
	s, err := Aufloesen(context.Background(), repo, "origin/develop")
	if err != nil {
		t.Fatal(err)
	}
	if s.Commit != basisCommit || s.BasisStand != "lokal" {
		t.Fatalf("%+v, will %s lokal", s, basisCommit)
	}
}

// TestBasisUngueltig: Fehlertext laut Vertrag, auch für einen gültig geformten, aber fehlenden Commit (AC-02).
func TestBasisUngueltig(t *testing.T) {
	t.Parallel()
	repo := testRepo(t)
	for _, b := range []string{"main", "HEAD~2", "xyz", "deadbeef", "origin/develop"} {
		_, err := Aufloesen(context.Background(), repo, b)
		want := "Basis „" + b + "“ ungültig: HEAD, HEAD~1, Commit-SHA oder origin/develop"
		if err == nil || err.Error() != want {
			t.Errorf("%q: Fehler %v, will %s", b, err, want)
		}
	}
}

// TestBasisFetchNurEinmal: origin/develop holt höchstens einmal je Minute (Review-Befund RV1.2/5).
func TestBasisFetchNurEinmal(t *testing.T) {
	repo := testRepo(t)
	gitIn(t, repo, "update-ref", "refs/remotes/origin/develop", "HEAD~1")
	vorher := fetchAufrufe.Load()
	for range 3 {
		if _, err := Aufloesen(context.Background(), repo, "origin/develop"); err != nil {
			t.Fatal(err)
		}
	}
	if n := fetchAufrufe.Load() - vorher; n != 1 {
		t.Fatalf("%d Fetches, will 1", n)
	}
}
