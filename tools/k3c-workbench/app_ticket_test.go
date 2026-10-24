package main

import (
	"os"
	"path/filepath"
	"testing"

	"k3c/tools/assetkit/ticket"
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
