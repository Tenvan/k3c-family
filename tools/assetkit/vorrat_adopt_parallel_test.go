package assets

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"testing"
)

// B-678 AC-01: Parallele Übernahmen desselben Eintrags legen genau eine Datei und eine Zuordnung an.
func TestVorratAdoptParallelEineKopie(t *testing.T) {
	root, client, _ := vorratRepo(t)
	if _, err := VorratDownload(context.Background(), root, suggestion(t, root, "v1"), client, day, nil); err != nil {
		t.Fatal(err)
	}
	const n = 8
	rels := make([]string, n)
	var wg sync.WaitGroup
	for i := 0; i < n; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			rels[i], _ = VorratAdopt(root, "v1", day)
		}()
	}
	wg.Wait()
	for _, r := range rels {
		if r == "" || r != rels[0] {
			t.Fatalf("unterschiedliche Ergebnisse: %v", rels)
		}
	}
	files, _ := filepath.Glob(filepath.Join(root, "public", "audio", "kandidaten", "*"))
	if len(files) != 1 {
		t.Errorf("dateien: %v", files)
	}
	reg, _ := Load(root)
	if c := reg.Find("sfx:coin").Choices; len(c) != 1 {
		t.Errorf("zuordnungen: %v", c)
	}
	credits, _ := os.ReadFile(filepath.Join(root, "public", "audio", "README.md"))
	if strings.Count(string(credits), "kandidaten") != 1 {
		t.Errorf("credit-zeilen:\n%s", credits)
	}
}
