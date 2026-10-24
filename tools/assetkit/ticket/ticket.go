// Package ticket legt Tickets nach docs/vorlagen/ticket.md an (B-595): Nummer, Vorlage, Index-Zeile. Der Ticket-Teil
// von plan_create (k3c-workbench) nutzt dieselben Bausteine, das Studio ruft Create direkt (offline, ohne Workbench).
package ticket

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"slices"
	"strings"
	"time"
)

// Auswahllisten der Eingabe; Typ und Prio sind die Teilmenge der Vorlage, die ein Mensch im Dialog wählt.
var (
	Types   = []string{"Idee", "Problem", "Frage"}
	Prios   = []string{"hoch", "mittel", "niedrig"}
	Domains = []string{"REG", "SIM", "SRV", "CLI", "PLAT", "INF", "DEV"}
)

var reProject = regexp.MustCompile(`^[A-Z]{2,4}$`)

// Context ist die Seite, von der das Ticket kommt; leere Felder fehlen im Ticket.
type Context struct {
	App     string `json:"app"`
	Seite   string `json:"seite"`
	Auswahl string `json:"auswahl,omitempty"`
	Version string `json:"version,omitempty"`
	Zeit    string `json:"zeit"`
}

// Input ist die Eingabe des Ticket-Dialogs. Projekt ist optional (Kürzel aus docs/projekte/, sonst `–`).
type Input struct {
	Titel        string  `json:"titel"`
	Beschreibung string  `json:"beschreibung"`
	Typ          string  `json:"typ"`
	Prio         string  `json:"prio"`
	Domaene      string  `json:"domaene"`
	Projekt      string  `json:"projekt,omitempty"`
	Kontext      Context `json:"kontext"`
}

// Created ist ein angelegtes Ticket; Pfad relativ zur Repo-Wurzel mit `/`.
type Created struct {
	ID    string `json:"id"`
	Pfad  string `json:"pfad"`
	Titel string `json:"titel"`
}

// Create legt das Ticket mit der nächsten freien Nummer (Backlog und Archiv) an und trägt es in den Index ein.
func Create(root string, in Input) (Created, error) {
	in.Titel = strings.TrimSpace(in.Titel)
	if err := validate(root, in); err != nil {
		return Created{}, err
	}
	top, err := LocalHighest(root)
	if err != nil {
		return Created{}, fmt.Errorf("backlog nicht lesbar: %w", err)
	}
	id := fmt.Sprintf("B-%03d", top+1)
	tpl, tplCRLF, err := readText(filepath.Join(root, "docs", "vorlagen", "ticket.md"))
	if err != nil {
		return Created{}, fmt.Errorf("vorlage fehlt: %w", err)
	}
	text, err := fill(tpl, id, in)
	if err != nil {
		return Created{}, err
	}
	indexPath := filepath.Join(root, "docs", "backlog", "README.md")
	index, indexCRLF, err := readText(indexPath)
	if err != nil {
		return Created{}, fmt.Errorf("index nicht lesbar: %w", err)
	}
	file := id + "-" + Slug(in.Titel) + ".md"
	index, err = AddToIndex(index, Row{Nr: id, Link: file, Domain: in.Domaene, Typ: in.Typ, Prio: in.Prio,
		Status: "offen", Sprint: "–", Title: in.Titel})
	if err != nil {
		return Created{}, err
	}
	return Created{ID: id, Pfad: "docs/backlog/" + file, Titel: in.Titel}, store(root, file, text, tplCRLF, index, indexCRLF)
}

// fill füllt die Vorlage: Kopf wie plan_create, Felder der Eingabe, Ausgangslage mit Beschreibung und Kontext.
func fill(tpl, id string, in Input) (string, error) {
	values := Defaults(time.Now())
	values["Domäne"], values["Typ"], values["Prio"] = in.Domaene, in.Typ, in.Prio
	if in.Projekt != "" {
		values["Projekt"] = in.Projekt
	}
	text, err := Draft(tpl, id, in.Titel, values)
	if err != nil {
		return "", err
	}
	return setSection(text, "Ausgangslage", situation(in))
}

// store schreibt erst das Ticket, dann den Index; scheitert der Index, verschwindet das Ticket wieder.
func store(root, file, text string, textCRLF bool, index string, indexCRLF bool) error {
	dir := filepath.Join(root, "docs", "backlog")
	if _, err := os.Stat(filepath.Join(dir, file)); err == nil {
		return fmt.Errorf("%s gibt es schon, bitte erneut senden", file)
	}
	if err := writeAtomic(filepath.Join(dir, file), text, textCRLF); err != nil {
		return fmt.Errorf("backlog nicht beschreibbar: %w", err)
	}
	if err := writeAtomic(filepath.Join(dir, "README.md"), index, indexCRLF); err != nil {
		return errors.Join(fmt.Errorf("index nicht beschreibbar: %w", err), os.Remove(filepath.Join(dir, file)))
	}
	return nil
}

func validate(root string, in Input) error {
	if in.Titel == "" || strings.ContainsAny(in.Titel, "|\r\n") {
		return fmt.Errorf("titel fehlt oder enthält `|` bzw. einen Zeilenumbruch")
	}
	for _, c := range []struct {
		name, value string
		list        []string
	}{{"Typ", in.Typ, Types}, {"Prio", in.Prio, Prios}, {"Domäne", in.Domaene, Domains}} {
		if !slices.Contains(c.list, c.value) {
			return fmt.Errorf("%s %q ungültig (%s)", c.name, c.value, strings.Join(c.list, ", "))
		}
	}
	if in.Projekt == "" {
		return nil
	}
	if found, _ := filepath.Glob(filepath.Join(root, "docs", "projekte", in.Projekt+"-*.md")); !reProject.MatchString(in.Projekt) || len(found) == 0 {
		return fmt.Errorf("unbekanntes Projekt %q", in.Projekt)
	}
	return nil
}
