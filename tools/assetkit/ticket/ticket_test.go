package ticket

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// tempRepo legt docs/ mit der echten Vorlage und dem echten Index an, dazu Tickets in Backlog und Archiv.
func tempRepo(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	copyDoc(t, root, "vorlagen/ticket.md")
	copyDoc(t, root, "backlog/README.md")
	write(t, root, "backlog/B-910-alt.md", "# B-910 · Altes Ticket\n")
	write(t, root, "backlog/B-912-neuer.md", "# B-912 · Neueres Ticket\n")
	write(t, root, "backlog/archiv/B-931-erledigt.md", "# B-931 · Erledigt\n")
	write(t, root, "projekte/WZG-werkzeuge.md", "# WZG · Werkzeuge\n")
	return root
}

func copyDoc(t *testing.T, root, rel string) {
	t.Helper()
	b, err := os.ReadFile(filepath.Join("..", "..", "..", "docs", filepath.FromSlash(rel)))
	if err != nil {
		t.Fatal(err)
	}
	write(t, root, rel, string(b))
}

func write(t *testing.T, root, rel, text string) {
	t.Helper()
	p := filepath.Join(root, "docs", filepath.FromSlash(rel))
	if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(p, []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
}

func read(t *testing.T, root, rel string) string {
	t.Helper()
	b, err := os.ReadFile(filepath.Join(root, "docs", filepath.FromSlash(rel)))
	if err != nil {
		t.Fatal(err)
	}
	return string(b)
}

func input() Input {
	return Input{Titel: "Slot shardling sieht aus wie ein Pilz", Beschreibung: "Erste Zeile\r\n## keine Überschrift",
		Typ: "Problem", Prio: "mittel", Domaene: "DEV",
		Kontext: Context{App: "k3c-studio", Seite: "grafik", Auswahl: "Slot shardling", Zeit: "2026-10-24T14:30:00+02:00"}}
}

func TestCreateLegtTicketMitKontextAn(t *testing.T) {
	root := tempRepo(t)
	got, err := Create(root, input())
	if err != nil {
		t.Fatal(err)
	}
	want := Created{ID: "B-932", Pfad: "docs/backlog/B-932-slot-shardling-sieht-aus-wie-ein-pilz.md", Titel: input().Titel}
	if got != want {
		t.Fatalf("Ergebnis %+v, erwartet %+v", got, want)
	}
	raw := read(t, root, strings.TrimPrefix(got.Pfad, "docs/"))
	if strings.Contains(read(t, root, "vorlagen/ticket.md"), "\r\n") != strings.Contains(raw, "\r\n") {
		t.Error("Zeilenenden weichen von der Vorlage ab")
	}
	text := strings.ReplaceAll(raw, "\r\n", "\n")
	for _, s := range []string{"# B-932 · Slot shardling sieht aus wie ein Pilz\n", "- **Domäne:** DEV\n", "- **Typ:** Problem\n",
		"- **Prio:** mittel\n", "- **Status:** offen\n", "- **Projekt:** –\n", "- **Spec:** Entwurf\n",
		"## Ausgangslage\n\nErste Zeile\n\\## keine Überschrift\n\n- **App:** k3c-studio\n- **Seite:** grafik\n" +
			"- **Auswahl:** Slot shardling\n- **Zeit:** 2026-10-24T14:30:00+02:00\n\n## Ziel\n"} {
		if !strings.Contains(text, s) {
			t.Errorf("fehlt im Ticket: %q", s)
		}
	}
	if strings.Contains(text, "**Version:**") {
		t.Error("leere Version steht im Ticket")
	}
}

func TestCreateSortiertIndexZeileEin(t *testing.T) {
	root := tempRepo(t)
	before := read(t, root, "backlog/README.md")
	if _, err := Create(root, input()); err != nil {
		t.Fatal(err)
	}
	after := read(t, root, "backlog/README.md")
	nl := "\n"
	if strings.Contains(before, "\r\n") {
		nl = "\r\n"
	}
	row := "| [B-932](B-932-slot-shardling-sieht-aus-wie-ein-pilz.md) | DEV | Problem | mittel | offen | – | Slot shardling sieht aus wie ein Pilz |"
	lines := strings.Split(after, nl)
	at := -1
	for i, l := range lines {
		if l == row {
			at = i
		}
	}
	if at < 0 || !strings.HasPrefix(lines[at-1], "| [B-") || strings.HasPrefix(lines[at+1], "|") {
		t.Fatalf("Zeile nicht am Ende der Tabelle unter ## Offen (Platz %d)", at)
	}
	for i, l := range lines {
		if strings.HasPrefix(l, "## Archiv") && at > i {
			t.Fatal("Zeile steht im Archiv")
		}
	}
	if len(lines) != len(strings.Split(before, nl))+1 {
		t.Error("Index hat mehr als eine neue Zeile")
	}
}

func TestCreateLehntUngueltigeEingabeAb(t *testing.T) {
	root := tempRepo(t)
	for name, mod := range map[string]func(*Input){
		"Titel leer":        func(in *Input) { in.Titel = "  " },
		"Titel mit |":       func(in *Input) { in.Titel = "a | b" },
		"Typ":               func(in *Input) { in.Typ = "Schuld" },
		"Prio":              func(in *Input) { in.Prio = "?" },
		"Domäne":            func(in *Input) { in.Domaene = "XYZ" },
		"Projekt unbekannt": func(in *Input) { in.Projekt = "NIX" },
	} {
		in := input()
		mod(&in)
		if _, err := Create(root, in); err == nil {
			t.Errorf("%s: kein Fehler", name)
		}
	}
	if top, _ := LocalHighest(root); top != 931 {
		t.Errorf("abgelehnte Eingabe hat geschrieben, höchste Nummer %d", top)
	}
	in := input()
	in.Projekt = "WZG"
	if _, err := Create(root, in); err != nil {
		t.Errorf("bekanntes Projekt abgelehnt: %v", err)
	}
}

func TestCreateOhneBeschreibbarenBacklogFehler(t *testing.T) {
	root := tempRepo(t)
	if err := os.RemoveAll(filepath.Join(root, "docs", "backlog")); err != nil {
		t.Fatal(err)
	}
	write(t, root, "backlog", "kein Ordner") // docs/backlog ist eine Datei
	if _, err := Create(root, input()); err == nil || !strings.Contains(err.Error(), "backlog") {
		t.Fatalf("erwartet Fehler zum Backlog, bekommen %v", err)
	}
}

func TestStoreRaeumtTicketAufWennIndexScheitert(t *testing.T) {
	root := tempRepo(t)
	dir := filepath.Join(root, "docs", "backlog")
	if err := os.Remove(filepath.Join(dir, "README.md")); err != nil {
		t.Fatal(err)
	}
	if err := os.Mkdir(filepath.Join(dir, "README.md"), 0o755); err != nil { // Index als Ordner: Umbenennen scheitert
		t.Fatal(err)
	}
	if err := store(root, "B-099-x.md", "text", false, "index", false); err == nil {
		t.Fatal("kein Fehler")
	}
	if _, err := os.Stat(filepath.Join(dir, "B-099-x.md")); !os.IsNotExist(err) {
		t.Error("Ticket blieb liegen")
	}
}

func TestRecentNeuesteZuerst(t *testing.T) {
	root := tempRepo(t)
	got, err := Recent(root, 2)
	if err != nil {
		t.Fatal(err)
	}
	if len(got) != 2 || got[0].ID != "B-912" || got[1].ID != "B-910" || got[0].Titel != "Neueres Ticket" ||
		got[0].Pfad != "docs/backlog/B-912-neuer.md" {
		t.Fatalf("Recent: %+v", got)
	}
	if all, _ := Recent(root, 10); len(all) != 2 {
		t.Errorf("Archiv gezählt: %+v", all)
	}
}

func TestSlug(t *testing.T) {
	for in, want := range map[string]string{
		"Größe über Maß!":           "groesse-ueber-mass",
		"  ---  ":                   "ticket",
		"A/B  Test (2)":             "a-b-test-2",
		strings.Repeat("wort ", 20): "wort-wort-wort-wort-wort-wort-wort-wort-wort-wort",
	} {
		if got := Slug(in); got != want {
			t.Errorf("Slug(%q) = %q, erwartet %q", in, got, want)
		}
	}
}
