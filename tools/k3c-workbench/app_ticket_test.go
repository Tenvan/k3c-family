package main

import (
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/tools/assetkit/ticket"
	"k3c/tools/k3c-workbench/internal/planning"
)

func ticketRepo(t *testing.T) *App {
	t.Helper()
	root := t.TempDir()
	for _, f := range []string{"docs/vorlagen/ticket.md", "docs/backlog/README.md"} {
		b, err := os.ReadFile(filepath.Join("..", "..", filepath.FromSlash(f)))
		if err != nil {
			t.Fatal(err)
		}
		dst := filepath.Join(root, filepath.FromSlash(f))
		if err := os.MkdirAll(filepath.Dir(dst), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(dst, b, 0o644); err != nil {
			t.Fatal(err)
		}
	}
	seed := filepath.Join(root, "docs", "backlog", "B-912-neuer.md")
	if err := os.WriteFile(seed, []byte("# B-912 · Neueres Ticket\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	return &App{root: root}
}

func TestTicketCreateUndRecent(t *testing.T) {
	a := ticketRepo(t)
	in := ticket.Input{Titel: "Probe aus dem Test", Beschreibung: "Text", Typ: "Idee", Prio: "niedrig", Domaene: "DEV",
		Kontext: ticket.Context{App: "k3c-workbench", Seite: "grafik", Zeit: "2026-10-24 12:00"}}
	got, err := a.TicketCreate(in)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := os.Stat(filepath.Join(a.root, filepath.FromSlash(got.Pfad))); err != nil {
		t.Errorf("Ticketdatei fehlt: %v", err)
	}
	rec, err := a.TicketRecent(0)
	if err != nil || len(rec) != 2 || rec[0].ID != got.ID {
		t.Errorf("Recent = %v, %v; erwartet %s", rec, err, got.ID)
	}
	if _, err := a.TicketCreate(ticket.Input{}); err == nil {
		t.Error("leere Eingabe angenommen")
	}
}

// TestTicketButtonUndArchivIndex (B-696/AC-09): ein Ticket vom Ticket-Button steht im Haupt-Index; erledigt wandert
// seine Zeile in docs/backlog/archiv/README.md und zurück.
func TestTicketButtonUndArchivIndex(t *testing.T) {
	a := ticketRepo(t)
	got, err := a.TicketCreate(ticket.Input{Titel: "Knopf-Ticket", Typ: "Idee", Prio: "niedrig", Domaene: "DEV"})
	if err != nil {
		t.Fatal(err)
	}
	read := func(rel string) string {
		b, _ := os.ReadFile(filepath.Join(a.root, "docs", filepath.FromSlash(rel)))
		return string(b)
	}
	if !strings.Contains(read("backlog/README.md"), "| ["+got.ID+"](") {
		t.Fatal("nicht im Haupt-Index")
	}
	if _, err := planning.Set(a.root, got.ID, map[string]string{"Status": "erledigt"}); err != nil {
		t.Fatal(err)
	}
	if strings.Contains(read("backlog/README.md"), "| ["+got.ID+"](") || !strings.Contains(read("backlog/archiv/README.md"), "| ["+got.ID+"](") {
		t.Error("Zeile nicht im Archiv-Index")
	}
	if _, err := planning.Set(a.root, got.ID, map[string]string{"Status": "offen"}); err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(read("backlog/README.md"), "| ["+got.ID+"](") || strings.Contains(read("backlog/archiv/README.md"), "| ["+got.ID+"](") {
		t.Error("Zeile nicht zurück im Haupt-Index")
	}
}
