package planning

import (
	"fmt"
	"regexp"
	"strings"
)

// Sperre je Domäne und Lane (docs/arbeitsweise.md › Domänen, PV1.1/PV1.2): Eine Session `in Arbeit` ohne Lane sperrt
// ihre ganze Domäne, eine mit Lane nur Domäne und Lane. Eine Session ohne Lane ist gesperrt, sobald irgendeine Session
// derselben Domäne in Arbeit ist. Neben der Auswahl schlägt plan_next höchstens maxParallel weitere vor (vier Agenten).

// maxParallel begrenzt „Parallel möglich“: Auswahl plus drei = höchstens vier Agenten gleichzeitig.
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

// locks sind die Sessions in Arbeit aktiver Sprints: je Sperrschlüssel und je Domäne die erste.
type locks struct {
	keys   map[string]string // Domäne oder Domäne/Lane → Session
	domain map[string]string // Domäne → erste Session in Arbeit, gleich welche Lane
}

func newLocks() locks { return locks{keys: map[string]string{}, domain: map[string]string{}} }

func (l locks) add(x Session) {
	if _, ok := l.keys[lockKey(x)]; !ok {
		l.keys[lockKey(x)] = x.Nr
	}
	if _, ok := l.domain[x.Domain]; !ok {
		l.domain[x.Domain] = x.Nr
	}
}

// why nennt die Sperre einer offenen Session; leer = frei.
func (l locks) why(x Session) string {
	if by, ok := l.keys[x.Domain]; ok {
		return "Domäne " + x.Domain + " gesperrt (" + by + " in Arbeit)"
	}
	lane := laneOf(x)
	if by, ok := l.domain[x.Domain]; ok && lane == "" {
		return "Domäne " + x.Domain + " gesperrt (" + by + " in Arbeit)"
	}
	if by, ok := l.keys[x.Domain+"/"+lane]; ok && lane != "" {
		return "Domäne " + x.Domain + ", Lane " + lane + " gesperrt (" + by + " in Arbeit)"
	}
	return ""
}

// conflict: zwei Sessions dürfen nicht gleichzeitig laufen (gleiche Domäne und gleiche oder fehlende Lane).
func conflict(a, b Session) bool {
	la, lb := laneOf(a), laneOf(b)
	return a.Domain == b.Domain && (la == "" || lb == "" || la == lb)
}

// parallel sind weitere freie Sessions, die weder mit pick noch untereinander kollidieren, höchstens maxParallel.
func parallel(cands []candidate, pick candidate) []candidate {
	chosen := []Session{pick.session}
	var out []candidate
	for _, c := range cands {
		if c.why != "" || len(out) == maxParallel {
			continue
		}
		free := true
		for _, s := range chosen {
			free = free && !conflict(s, c.session)
		}
		if free {
			chosen = append(chosen, c.session)
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
var optionalAfter = map[string]map[string][]string{"session": {"Lane": {"Domäne"}, "Modell": {"Lane", "Domäne"}}}

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
