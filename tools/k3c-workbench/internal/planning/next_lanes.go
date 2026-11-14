package planning

import (
	"fmt"
	"regexp"
	"strings"
)

// Kollision wie bei den Arbeitswellen (B-783, docs/arbeitsweise.md › Parallelität und Wellen): gleiche Sperre des
// Sprints oder überlappende Erlaubte Dateien. Übergang für Sessions ohne Dateimenge: Sperre je Domäne und Lane
// (PV1.1/PV1.2): ohne Lane sperrt eine Session in Arbeit ihre ganze Domäne, mit Lane nur Domäne und Lane. Neben der
// Auswahl schlägt plan_next höchstens maxParallel weniger die Sessions in Arbeit vor (vier Agenten, B-839).

// maxParallel begrenzt „Parallel möglich“: Sessions in Arbeit, Auswahl und Vorschläge zusammen höchstens vier Agenten.
const maxParallel = 3

var (
	// models sind die Werte des Session-Felds `Modell`; ohne Feld oder `–` gilt sonnet.
	models = []string{"–", "haiku", "sonnet", "opus", "fable"}
	reLane = regexp.MustCompile(`^(–|[a-z0-9-]+)$`)
)

// laneOf ist das Feld `Lane` einer Session; leer, wenn es fehlt oder `–` ist.
func laneOf(x Session) string {
	if l := sessionHead(x)["Lane"]; l != "" && !strings.HasPrefix(l, "–") {
		return l
	}
	return ""
}

// modelOf ist das Feld `Modell` einer Session; ohne Wert gilt sonnet.
func modelOf(x Session) string {
	if m := sessionHead(x)["Modell"]; m != "" && !strings.HasPrefix(m, "–") {
		return m
	}
	return "sonnet (Standard)"
}

// lockKey ist `Domäne` oder `Domäne/Lane`.
func lockKey(x Session) string {
	if l := laneOf(x); l != "" {
		return x.Domain + "/" + l
	}
	return x.Domain
}

// scoped ist eine Session mit ihrem Sprint, ihrer Dateimenge (Erlaubte Dateien) und den Sperren ihres Sprints (B-783).
type scoped struct {
	session     Session
	sprint      string
	area, locks []string
}

// busy nennt, warum x neben den Sessions in Arbeit nicht laufen darf; leer = frei.
func busy(running []scoped, x scoped) string {
	for _, r := range running {
		if why := clash(r, x); why != "" {
			return why + " gesperrt (" + r.session.Nr + " in Arbeit)"
		}
	}
	return ""
}

// clash ist die Kollisionsprüfung der Arbeitswellen (wie plan_waves): derselbe Sprint, gleiche Sperre oder
// überlappende Dateimengen. Sessions desselben Sprints laufen nie parallel (B-838: ein Sprint hat einen Branch, zwei
// Worktrees checken ihn nicht zugleich aus). Übergang: Fehlt einer der beiden die Dateimenge, gilt die alte Regel nach
// Domäne und Lane.
func clash(a, b scoped) string {
	if a.sprint != "" && a.sprint == b.sprint {
		return "Sprint " + a.sprint
	}
	if l := sharedLock(a.locks, b.locks); l != "" {
		return "Sperre " + l
	}
	if len(a.area) > 0 && len(b.area) > 0 {
		if pair, ok := areaOverlap(a.area, b.area); ok {
			return "Bereich " + pair
		}
		return ""
	}
	if !conflict(a.session, b.session) {
		return ""
	}
	if l := laneOf(b.session); l != "" && l == laneOf(a.session) {
		return "Domäne " + b.session.Domain + ", Lane " + l
	}
	return "Domäne " + b.session.Domain
}

// conflict: zwei Sessions ohne Dateimenge dürfen nicht gleichzeitig laufen (gleiche Domäne und gleiche oder fehlende Lane).
func conflict(a, b Session) bool {
	la, lb := laneOf(a), laneOf(b)
	return a.Domain == b.Domain && (la == "" || lb == "" || la == lb)
}

// parallel sind weitere freie Sessions, die weder mit pick noch untereinander kollidieren: höchstens maxParallel
// abzüglich der Sessions in Arbeit (B-839), damit Auswahl, Vorschläge und laufende zusammen vier Agenten nicht übersteigen.
func parallel(cands []candidate, pick candidate, running int) []candidate {
	limit := max(maxParallel-running, 0)
	chosen := []scoped{pick.scope}
	var out []candidate
	for _, c := range cands {
		if c.why != "" || len(out) >= limit {
			continue
		}
		free := true
		for _, s := range chosen {
			free = free && clash(s, c.scope) == ""
		}
		if free {
			chosen = append(chosen, c.scope)
			out = append(out, c)
		}
	}
	return out
}

func parallelLines(cs []candidate) []string {
	if len(cs) == 0 {
		return []string{"keine (sonst ist keine Session frei, die nicht kollidiert)"}
	}
	out := make([]string, 0, len(cs))
	for _, c := range cs {
		out = append(out, fmt.Sprintf("- %s · %s · %s (Sprint %s, %s) · Modell: %s", lockKey(c.session), c.session.Nr,
			c.session.Titel, c.sprint.ID, firstOf(c.session.Env, "?"), modelOf(c.session)))
	}
	return out
}

// domainLine ist die Kopfzeile mit Domäne, Lane (nur wenn gesetzt), Umgebung und Modell.
func domainLine(x Session) string {
	out := "Domäne: " + x.Domain
	if l := laneOf(x); l != "" {
		out += " · Lane: " + l
	}
	return out + " · Umgebung: " + firstOf(x.Env, "?") + " · Modell: " + modelOf(x)
}

// checkLane prüft das Session-Feld `Lane`: `–` oder Kurzname aus a-z, 0-9 und `-`.
func checkLane(value string) error {
	if !reLane.MatchString(value) {
		return fmt.Errorf("feld Lane = %q ungültig (– oder Kurzname aus a-z, 0-9, -)", value)
	}
	return nil
}

// optionalAfter: Felder, die in älteren Dateien fehlen dürfen, und die Felder, hinter die sie beim Setzen kommen.
var optionalAfter = map[string]map[string][]string{
	"session": {"Lane": {"Domäne"}, "Modell": {"Lane", "Domäne"}},
	"ticket":  {fieldDup: {"Freigabe", "Status"}, fieldReplace: {fieldDup, "Freigabe", "Status"}},
}

// insertField fügt ein fehlendes optionales Feld hinter dem ersten vorhandenen Vorgänger ein.
func insertField(kind, text, name, value string) (string, bool) {
	after, ok := optionalAfter[kind][name]
	if !ok {
		return text, false
	}
	lines := splitLines(text)
	for _, prev := range after {
		for i := range lines[:headEnd(lines)] {
			if f := field.FindStringSubmatch(lines[i]); f != nil && f[1] == prev {
				lines = append(lines[:i+1], append([]string{"- **" + name + ":** " + value}, lines[i+1:]...)...)
				return joinLines(lines), true
			}
		}
	}
	return text, false
}
