package mcpsrv

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/tools/k3c-studio/internal/exeengine"
)

// B-923: Roundtrip der Tools hub_get, hub_check und hub_save (Engine: Ersatz, fakehub_test.go) auf einer Kopie von data/hub.json und der Regeltabelle
// (t.TempDir); die Dateien des Checkouts werden nur gelesen.

const hubRepoRoot = "../../../.."

func hubRepo(t *testing.T) string {
	t.Helper()
	exeengine.Require(t) // Welten- und Hub-Tools rechnen über die Spiel-EXE
	root := t.TempDir()
	for _, rel := range []string{"data/hub.json", "docs/rules/materialien-gebaeude.md"} {
		raw, err := os.ReadFile(filepath.Join(hubRepoRoot, filepath.FromSlash(rel)))
		if err != nil {
			t.Fatal(err)
		}
		dst := filepath.Join(root, filepath.FromSlash(rel))
		if err := os.MkdirAll(filepath.Dir(dst), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(dst, raw, 0o644); err != nil {
			t.Fatal(err)
		}
	}
	return root
}

func TestHubGetNenntMatrixPruefungUndStand(t *testing.T) {
	t.Parallel()
	cs := connect(t, New(Config{Version: "test", Root: hubRepo(t)}))
	text, isErr := callText(t, cs, "hub_get", map[string]any{})
	if isErr {
		t.Fatalf("hub_get: %s", text)
	}
	for _, want := range []string{"data/hub.json · ", "Hub-Stufen", "Prüfung: ok", "\nStand: hub=", "armory@40 · sites#", "zone1 · sitePlacement#0 · ab 1 · ±44", "Regeltabelle: stimmt"} {
		if !strings.Contains(text, want) {
			t.Errorf("%q fehlt:\n%s", want, text)
		}
	}
}

func TestHubCheckSchreibtNichts(t *testing.T) {
	t.Parallel()
	root := hubRepo(t)
	before, _ := os.ReadFile(filepath.Join(root, "data/hub.json"))
	cs := connect(t, New(Config{Version: "test", Root: root}))
	text, isErr := callText(t, cs, "hub_check", map[string]any{"patch": map[string]any{"merchant": map[string]any{"buyOffsetUnits": 18}}})
	if isErr || !strings.Contains(text, "(Kandidat, nichts geschrieben)") || !strings.Contains(text, "merchantBuy@18 ·") || !strings.Contains(text, "Regeltabelle: ") {
		t.Fatalf("hub_check:\n%s", text)
	}
	if after, _ := os.ReadFile(filepath.Join(root, "data/hub.json")); string(after) != string(before) {
		t.Error("hub_check hat data/hub.json geändert")
	}
	text, isErr = callText(t, cs, "hub_check", map[string]any{"patch": map[string]any{"a": 1}, "candidate": "{}"})
	if !isErr || !strings.Contains(text, "nicht beides") {
		t.Errorf("patch und candidate: %q", text)
	}
}

func TestHubSaveSchreibtEineZeileUndLehntAltenStandAb(t *testing.T) {
	t.Parallel()
	root := hubRepo(t)
	cs := connect(t, New(Config{Version: "test", Root: root}))
	hub := filepath.Join(root, "data/hub.json")
	before, _ := os.ReadFile(hub)
	text, isErr := callText(t, cs, "hub_save", map[string]any{"patch": map[string]any{"homeRadiusUnits": 13}, "stand": "000000000000"})
	if !isErr || !strings.Contains(text, "seit hub_get geändert") {
		t.Fatalf("alter Stand: %q", text)
	}
	text, isErr = callText(t, cs, "hub_save", map[string]any{"patch": map[string]any{"homeRadiusUnits": 13}})
	if !isErr || !strings.Contains(text, "stand fehlt") {
		t.Fatalf("ohne stand: %q", text)
	}
	stand := hubEditor(root).Plan(context.Background()).Stand.Hub
	text, isErr = callText(t, cs, "hub_save", map[string]any{"patch": map[string]any{"homeRadiusUnits": 13}, "stand": stand})
	if isErr || !strings.HasPrefix(text, "Gespeichert: data/hub.json und Regeltabelle. REG-Änderung") || !strings.Contains(text, "\nCheckout: ") {
		t.Fatalf("hub_save:\n%s", text)
	}
	after, _ := os.ReadFile(hub)
	if strings.Count(string(after), "\n") != strings.Count(string(before), "\n") || !strings.Contains(string(after), `"homeRadiusUnits": 13`) {
		t.Errorf("hub.json nicht format-stabil geschrieben:\n%s", after)
	}
	stand = hubEditor(root).Plan(context.Background()).Stand.Hub
	text, _ = callText(t, cs, "hub_save", map[string]any{"patch": map[string]any{"homeRadiusUnits": 13}, "stand": stand})
	if !strings.HasPrefix(text, "Unverändert") {
		t.Errorf("zweites Speichern: %q", text)
	}
	text, isErr = callText(t, cs, "hub_save", map[string]any{})
	if !isErr || !strings.Contains(text, "patch oder candidate fehlt") {
		t.Errorf("ohne Angabe: %q", text)
	}
}

func TestHubSaveMitBefundSchreibtNichts(t *testing.T) {
	t.Parallel()
	root := hubRepo(t)
	cs := connect(t, New(Config{Version: "test", Root: root}))
	hub := filepath.Join(root, "data/hub.json")
	before, _ := os.ReadFile(hub)
	text, isErr := callText(t, cs, "hub_save", map[string]any{"patch": map[string]any{"sites": []any{map[string]any{"kind": "mill", "offsetUnits": 1, "hubLevel": 1}}}, "stand": hubEditor(root).Plan(context.Background()).Stand.Hub})
	if !isErr || !strings.Contains(text, "Befunde der Prüfung") || !strings.Contains(text, "\nBefund: ") {
		t.Errorf("Befund:\n%s", text)
	}
	if after, _ := os.ReadFile(hub); string(after) != string(before) {
		t.Error("hub.json trotz Befund geändert")
	}
}
