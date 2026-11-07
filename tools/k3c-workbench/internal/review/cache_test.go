package review

import (
	"context"
	"os"
	"strings"
	"testing"
)

// TestVorherCache: der zweite Aufruf für dieselbe Basis rendert nicht neu; die Datei liegt unter .cache/paper/<renderKey>/.
func TestVorherCache(t *testing.T) {
	t.Parallel()
	repo := testRepo(t)
	r := &fakeFrames{CLI: cli()}
	ctx := context.Background()
	q, ok, err := QuelleBasis(ctx, repo, basisCommit, "public/grafik/figuren/monarch.paper.json")
	if err != nil || !ok {
		t.Fatalf("Quelle: %v %v", ok, err)
	}
	erst, err := Bild(ctx, r, repo, q, "reiten", 0, "")
	if err != nil {
		t.Fatal(err)
	}
	zweit, err := Bild(ctx, r, repo, q, "reiten", 0, "")
	if err != nil {
		t.Fatal(err)
	}
	if n := r.aufrufe.Load(); n != 1 {
		t.Fatalf("Frame %d-mal gerendert, will 1", n)
	}
	ordner := q.ordner(repo, r)
	if !strings.HasPrefix(erst.Datei, ordner) || erst.Datei != zweit.Datei {
		t.Fatalf("Datei %s nicht unter %s", erst.Datei, ordner)
	}
	if _, err := os.Stat(erst.Datei); err != nil {
		t.Fatal(err)
	}
	if len(zweit.Teile) == 0 || zweit.Teile["kopf"] == nil {
		t.Fatalf("Teile aus dem Cache fehlen: %v", zweit.Teile)
	}
}
