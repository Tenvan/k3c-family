package planning

import (
	"bytes"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// TestGolden rechnet jeden Fall unter testdata/planung/<fall>/: vorher/ aufbauen, den Aufruf aus aufruf.json ausführen,
// Antwort gegen antwort.txt und Dateibaum gegen nachher/ vergleichen. UPDATE_GOLDEN=1 schreibt antwort.txt und nachher/
// neu (nur bewusst, Begründung in den Commit). Format: testdata/planung/README.md.
func TestGolden(t *testing.T) {
	t.Parallel()
	for _, fall := range goldenFaelle(t) {
		t.Run(fall, func(t *testing.T) {
			t.Parallel()
			runGoldenFall(t, fall)
		})
	}
}

func runGoldenFall(t *testing.T, fall string) {
	t.Helper()
	call, err := ladeFall(filepath.Join(goldenDir, fall))
	if err != nil {
		t.Fatal(err)
	}
	root := baueVorher(t, fall)
	out, err := fuehreAus(root, call)
	antwort := maskiere(out, root)
	if err != nil {
		antwort = errPrefix + maskiere(err.Error(), root)
	}
	if os.Getenv(goldenUpdate) == "1" {
		schreibeNachher(t, fall, root, antwort)
		return
	}
	want, rerr := os.ReadFile(filepath.Join(goldenDir, fall, "antwort.txt"))
	if rerr != nil {
		t.Fatal(rerr)
	}
	if w := strings.ReplaceAll(string(want), "\r\n", "\n"); w != antwort {
		t.Errorf("Antwort weicht ab, %s", ersteAbweichung(w, antwort))
	}
	vergleiche(t, root, filepath.Join(goldenDir, fall, "nachher"))
}

// decodeArgs liest die Argumente eines Aufrufs streng (unbekannte Felder sind ein Fehler im Fall, nicht im Programm).
func decodeArgs[T any](raw json.RawMessage) (T, error) {
	var v T
	if len(bytes.TrimSpace(raw)) == 0 {
		return v, nil
	}
	dec := json.NewDecoder(bytes.NewReader(raw))
	dec.DisallowUnknownFields()
	if err := dec.Decode(&v); err != nil {
		return v, fmt.Errorf("aufruf.json args: %w", err)
	}
	return v, nil
}

// Argumente der Werkzeuge wie im MCP-Tool (tools_planning.go); lesende mit Filter-Typen des Pakets.
type (
	gCreate struct {
		Kind   string            `json:"kind"`
		ID     string            `json:"id"`
		Slug   string            `json:"slug"`
		Title  string            `json:"title"`
		Fields map[string]string `json:"fields"`
	}
	gSet struct {
		ID     string            `json:"id"`
		Fields map[string]string `json:"fields"`
	}
	gSection struct {
		ID      string `json:"id"`
		Section string `json:"section"`
		Text    string `json:"text"`
	}
	gSections struct {
		ID       string            `json:"id"`
		Sections map[string]string `json:"sections"`
	}
	gID struct {
		ID string `json:"id"`
	}
	gIndex struct {
		Dry *bool `json:"dry"`
	}
)

// golden ist ein Werkzeug des Aufrufs: Wurzel und rohe Argumente rein, Text oder Fehler raus.
type golden func(root string, raw json.RawMessage) (string, error)

// goldenWerkzeuge ordnet die Namen der MCP-Tools den Funktionen der Planung zu.
var goldenWerkzeuge = map[string]golden{
	"plan_create": func(root string, raw json.RawMessage) (string, error) {
		a, err := decodeArgs[gCreate](raw)
		if err != nil {
			return "", err
		}
		return Create(root, NewDoc{Kind: a.Kind, ID: a.ID, Slug: a.Slug, Title: a.Title, Fields: a.Fields})
	},
	"plan_set": func(root string, raw json.RawMessage) (string, error) {
		a, err := decodeArgs[gSet](raw)
		if err != nil {
			return "", err
		}
		return Set(root, a.ID, a.Fields)
	},
	"plan_section": func(root string, raw json.RawMessage) (string, error) {
		a, err := decodeArgs[gSection](raw)
		if err != nil {
			return "", err
		}
		return Section(root, a.ID, a.Section, a.Text)
	},
	"plan_sections": func(root string, raw json.RawMessage) (string, error) {
		a, err := decodeArgs[gSections](raw)
		if err != nil {
			return "", err
		}
		return Sections(root, a.ID, a.Sections)
	},
	"plan_delete": func(root string, raw json.RawMessage) (string, error) {
		a, err := decodeArgs[gID](raw)
		if err != nil {
			return "", err
		}
		return Delete(root, a.ID)
	},
	"plan_get": func(root string, raw json.RawMessage) (string, error) {
		a, err := decodeArgs[gID](raw)
		if err != nil {
			return "", err
		}
		return Get(root, a.ID)
	},
	"plan_check": func(root string, raw json.RawMessage) (string, error) {
		a, err := decodeArgs[CheckIn](raw)
		if err != nil {
			return "", err
		}
		return Check(root, a)
	},
	"plan_index": func(root string, raw json.RawMessage) (string, error) {
		a, err := decodeArgs[gIndex](raw)
		if err != nil {
			return "", err
		}
		return RebuildIndex(root, a.Dry == nil || *a.Dry) // dry ist wie im Tool standardmäßig an
	},
	"plan_list": func(root string, raw json.RawMessage) (string, error) {
		a, err := decodeArgs[Filter](raw)
		if err != nil {
			return "", err
		}
		return List(root, a)
	},
	"plan_next": func(root string, raw json.RawMessage) (string, error) {
		a, err := decodeArgs[NextFilter](raw)
		if err != nil {
			return "", err
		}
		return Next(root, a)
	},
	"plan_waves": func(root string, raw json.RawMessage) (string, error) {
		a, err := decodeArgs[WavesFilter](raw)
		if err != nil {
			return "", err
		}
		return Waves(root, a)
	},
	"plan_queue": func(root string, raw json.RawMessage) (string, error) {
		a, err := decodeArgs[QueueFilter](raw)
		if err != nil {
			return "", err
		}
		return Queue(root, a)
	},
}

// fuehreAus ruft das Werkzeug eines Falls auf der Wurzel auf.
func fuehreAus(root string, call goldenAufruf) (string, error) {
	f, ok := goldenWerkzeuge[call.Werkzeug]
	if !ok {
		return "", fmt.Errorf("aufruf.json: unbekanntes werkzeug %q", call.Werkzeug)
	}
	return f(root, call.Args)
}
