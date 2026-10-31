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

// Wartezeit auf die Backlog-Sperre und Alter, ab dem eine Sperrdatei als verwaist gilt.
const (
	lockWait  = 10 * time.Second
	lockStale = 30 * time.Second
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
	unlock, err := lockBacklog(root)
	if err != nil {
		return Created{}, err
	}
	defer unlock()
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

// lockBacklog sperrt das Anlegen über Prozesse hinweg (B-615): exklusives Anlegen einer Sperrdatei im Backlog, solange
// Nummer vergeben und Ticket samt Index geschrieben werden. Eine Sperre älter als lockStale gilt als verwaist.
func lockBacklog(root string) (func(), error) {
	p := filepath.Join(root, "docs", "backlog", ".ticket.lock")
	token := fmt.Sprintf("%d-%d", os.Getpid(), time.Now().UnixNano())
	deadline := time.Now().Add(lockWait)
	for {
		f, err := os.OpenFile(p, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0o644)
		if err == nil {
			_, _ = f.WriteString(token)
			_ = f.Close()
			return func() { releaseLock(p, token) }, nil
		}
		// Windows meldet eine gerade zum Löschen markierte Datei als ErrPermission: wie belegt behandeln.
		if !errors.Is(err, os.ErrExist) && !errors.Is(err, os.ErrPermission) {
			return nil, fmt.Errorf("backlog nicht beschreibbar: %w", err)
		}
		if st, serr := os.Stat(p); serr == nil && time.Since(st.ModTime()) > lockStale {
			// Übernahme per Umbenennen: nur ein Prozess gewinnt, eine frische Sperre eines anderen bleibt unberührt.
			grave := p + "." + token
			if os.Rename(p, grave) == nil {
				_ = os.Remove(grave)
			}
			continue
		}
		if time.Now().After(deadline) {
			return nil, errors.New("backlog gesperrt, bitte erneut senden")
		}
		time.Sleep(5 * time.Millisecond)
	}
}

// releaseLock löscht die Sperre nur, wenn sie noch die eigene ist.
func releaseLock(p, token string) {
	if b, err := os.ReadFile(p); err == nil && string(b) == token {
		_ = os.Remove(p)
	}
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

// Validate prüft die Eingabe wie Create, ohne zu schreiben (Vorabprüfung der Chat-Aktion ticket, B-679).
func Validate(root string, in Input) error { return validate(root, in) }

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
	if !reProject.MatchString(in.Projekt) {
		return fmt.Errorf("unbekanntes Projekt %q", in.Projekt)
	}
	if found, _ := filepath.Glob(filepath.Join(root, "docs", "projekte", in.Projekt+"-*.md")); len(found) == 0 {
		return fmt.Errorf("unbekanntes Projekt %q", in.Projekt)
	}
	return nil
}
