package simrun

import (
	"fmt"
	"regexp"
	"slices"
	"strconv"
	"strings"
	"unicode/utf8"

	"k3c/tools/k3c-workbench/internal/balance"
)

// Spec ist ein Offline-Lauf der Balancing-Seite; jedes Feld wird ein Flag der Balance-CLI (internal/balance/cmd).
// Null bzw. leer heißt: Flag weglassen, die CLI nimmt alle Ziele aus data/balance-targets.json.
type Spec struct {
	Seeds   int      `json:"seeds,omitempty"`
	Depth   *int     `json:"depth,omitempty"` // 0 ist Wald, daher Zeiger: nil = Tiefe des Ziels
	Players int      `json:"players,omitempty"`
	Bot     string   `json:"bot,omitempty"`
	Grade   string   `json:"grade,omitempty"`
	Days    int      `json:"days,omitempty"`
	Vary    []string `json:"vary,omitempty"` // datei.json:pfad.zum.wert=wert
	Label   string   `json:"label,omitempty"`
}

const maxVary = 20

var (
	gradeName = regexp.MustCompile(`^[a-z0-9-]{1,20}$`)
	// varyArg: Datei ohne Ordner, Pfad aus Namen und Indizes, Wert ohne Leerraum.
	varyArg = regexp.MustCompile(`^[A-Za-z0-9_-]+\.json:[A-Za-z0-9_.\[\]-]+=[^\s=]{1,64}$`)
)

// Check prüft die Spec über Grenzen und Positivlisten; jeder Fehler ist eine Zeile mit Grund.
func (s Spec) Check() error {
	checks := []struct {
		bad bool
		msg string
	}{
		{s.Seeds < 0 || s.Seeds > 200, "seeds 1 bis 200"},
		{s.Depth != nil && (*s.Depth < 0 || *s.Depth > 20), "depth 0 bis 20"},
		{s.Players < 0 || s.Players > 4, "players 1 bis 4"},
		{s.Days < 0 || s.Days > 100, "days 1 bis 100"},
		{s.Bot != "" && !slices.Contains(balance.BotNames(), s.Bot), "bot: " + strings.Join(balance.BotNames(), ", ")},
		{s.Grade != "" && !gradeName.MatchString(s.Grade), "grade: Kleinbuchstaben, Ziffern, Bindestrich"},
		{len(s.Vary) > maxVary, fmt.Sprintf("höchstens %d Varianten", maxVary)},
		{utf8.RuneCountInString(s.Label) > 60 || strings.ContainsAny(s.Label, "\r\n"), "label höchstens 60 Zeichen, eine Zeile"},
	}
	for _, c := range checks {
		if c.bad {
			return fmt.Errorf("lauf abgelehnt: %s", c.msg)
		}
	}
	for _, v := range s.Vary {
		if !varyArg.MatchString(v) || strings.Contains(v, "..") {
			return fmt.Errorf("lauf abgelehnt: vary %q, Form datei.json:pfad.zum.wert=wert", v)
		}
	}
	return nil
}

// Title ist die Bezeichnung des Laufs: Label oder eine Kurzform der gesetzten Felder.
func (s Spec) Title() string {
	if s.Label != "" {
		return s.Label
	}
	parts := []string{"offline·balance"}
	for _, f := range []struct {
		name string
		n    int
	}{{"seeds", s.Seeds}, {"players", s.Players}, {"days", s.Days}} {
		if f.n > 0 {
			parts = append(parts, f.name+" "+strconv.Itoa(f.n))
		}
	}
	if s.Depth != nil {
		parts = append(parts, "depth "+strconv.Itoa(*s.Depth))
	}
	for _, f := range []string{s.Bot, s.Grade} {
		if f != "" {
			parts = append(parts, f)
		}
	}
	if len(s.Vary) > 0 {
		parts = append(parts, fmt.Sprintf("%d Varianten", len(s.Vary)))
	}
	return strings.Join(parts, " · ")
}

// args baut die Argumente der Balance-CLI; dir ist der Berichtsordner relativ zum Checkout.
func (s Spec) args(exe, dir string) []string {
	args := []string{exe, "--targets", "--report-dir", dir, "--progress"}
	for _, f := range []struct {
		flag string
		n    int
	}{{"--seeds", s.Seeds}, {"--players", s.Players}, {"--days", s.Days}} {
		if f.n > 0 {
			args = append(args, f.flag, strconv.Itoa(f.n))
		}
	}
	if s.Depth != nil {
		args = append(args, "--depth", strconv.Itoa(*s.Depth))
	}
	if s.Bot != "" {
		args = append(args, "--bot", s.Bot)
	}
	if s.Grade != "" {
		args = append(args, "--grade", s.Grade)
	}
	for _, v := range s.Vary {
		args = append(args, "--vary", v)
	}
	return args
}
