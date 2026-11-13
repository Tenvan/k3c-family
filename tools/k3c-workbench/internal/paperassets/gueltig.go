package paperassets

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"sync"
)

// Gueltigkeit merkt sich das Ergebnis von validate je Spec, solange Spec, Stil-Datei und die Specs ihrer Reittiere
// unverändert sind (B-706).
type Gueltigkeit struct {
	mu    sync.Mutex
	cache map[string]gueltigEintrag
}

type gueltigEintrag struct {
	stand  string
	fehler []Fehler
}

// NewGueltigkeit liefert einen leeren Cache.
func NewGueltigkeit() *Gueltigkeit { return &Gueltigkeit{cache: map[string]gueltigEintrag{}} }

func dateiStand(p string) string {
	fi, err := os.Stat(p)
	if err != nil {
		return "–"
	}
	return fmt.Sprintf("%d/%d", fi.Size(), fi.ModTime().UnixNano())
}

// Pruefe liefert je Asset die Fehler von validate (leer = gültig); höchstens 8 Prüfungen gleichzeitig.
func (g *Gueltigkeit) Pruefe(ctx context.Context, cli CLI, assets []Asset) map[string][]Fehler {
	stil := dateiStand(filepath.Join(cli.Root, filepath.FromSlash(StilPfad)))
	nachID := map[string]Asset{}
	if alle, err := Assets(cli.Root); err == nil {
		for _, a := range alle {
			nachID[a.ID] = a
		}
	}
	out := make(map[string][]Fehler, len(assets))
	var mu sync.Mutex
	var wg sync.WaitGroup
	sem := make(chan struct{}, 8)
	for _, a := range assets {
		pfad := a.Abs(cli.Root)
		stand := dateiStand(pfad) + "|" + stil + reittierStand(cli.Root, pfad, nachID)
		g.mu.Lock()
		e, ok := g.cache[pfad]
		g.mu.Unlock()
		if ok && e.stand == stand {
			mu.Lock()
			out[a.ID] = e.fehler
			mu.Unlock()
			continue
		}
		wg.Go(func() {
			sem <- struct{}{}
			defer func() { <-sem }()
			fehler, sicher := validiereEins(ctx, cli, pfad)
			if sicher {
				g.mu.Lock()
				g.cache[pfad] = gueltigEintrag{stand: stand, fehler: fehler}
				g.mu.Unlock()
			}
			mu.Lock()
			out[a.ID] = fehler
			mu.Unlock()
		})
	}
	wg.Wait()
	return out
}

// validiereEins prüft eine Spec; sicher = false bei Aufruf- oder Abbruchfehlern (nicht cachen).
func validiereEins(ctx context.Context, cli CLI, pfad string) ([]Fehler, bool) {
	err := cli.Validate(ctx, pfad)
	if err == nil {
		return []Fehler{}, true
	}
	if ab, ok := AlsAbgelehnt(err); ok {
		return ab.Fehler, true
	}
	return []Fehler{{Feld: "", Regel: "cli", Text: err.Error()}}, false
}
