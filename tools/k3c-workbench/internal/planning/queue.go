package planning

import (
	"fmt"
	"regexp"
	"slices"
	"strings"
)

// plan_queue (B-783): die Ralf-Queue gebündelt (docs/arbeitsweise.md › Ralf-Queue): Freigaben (Specs im Entwurf mit
// `Braucht 🧑: Freigabe`), Entscheidungen und Abnahmen je Gerät (offene Mensch-Sessions aktiver Sprints, dazu Tickets
// und Sprints mit `Braucht 🧑: Abnahme` oder `Gerät`). Das Gerät steht in einer Zeile `Gerät:` im Abschnitt Kontext,
// sonst gilt „unbekannt“.

// QueueFilter ist die Eingabe von plan_queue.
type QueueFilter struct {
	Device string `json:"device,omitempty" jsonschema:"nur Abnahmen an diesem Gerät: PC, Xbox/TV, Pi, Mini-PC, Workshop, Veröffentlichung oder unbekannt"`
}

// devices ist die Reihenfolge der Geräte; andere folgen nach Name, „unbekannt“ zuletzt.
var devices = []string{"PC", "Xbox/TV", "Pi", "Mini-PC", "Workshop", "Veröffentlichung"}

const unknownDevice = "unbekannt"

var (
	reDevice = regexp.MustCompile(`^\s*(?:- )?(?:\*\*)?Gerät:(?:\*\*)?\s*(.+?)\s*$`)
	rePi     = regexp.MustCompile(`(?i)\bpi\b`)
	rePC     = regexp.MustCompile(`(?i)\bpc\b`)
)

// queueItem ist ein Eintrag der Ralf-Queue.
type queueItem struct{ device, line string }

// Queue liefert die Ralf-Queue aus den lokalen Dateien.
func Queue(root string, f QueueFilter) (string, error) {
	w, err := loadWaveData(root)
	if err != nil {
		return "", err
	}
	var release, decide []string
	var accept []queueItem
	w.queueDocs(&release, &decide, &accept)
	for _, sp := range w.d.Sprints {
		if sp.Status == "aktiv" {
			accept = append(accept, humanItems(sp)...)
		}
	}
	want := normDevice(f.Device)
	if f.Device != "" {
		accept = slices.DeleteFunc(accept, func(q queueItem) bool { return !strings.EqualFold(q.device, want) })
		release, decide = nil, nil
	}
	return queueText(release, decide, accept, f.Device != ""), nil
}

// humanItems sind die offenen oder laufenden Sessions eines Sprints, die nicht autonom sind, mit ihrem Gerät
// (plan_queue und Fahrplan „Offen am Gerät“, B-842).
func humanItems(sp Sprint) []queueItem {
	var out []queueItem
	for _, x := range sp.Sessions {
		if (x.Status == "offen" || x.Status == "in Arbeit") && x.Agent != "autonom" {
			out = append(out, queueItem{deviceOf(x.Text), fmt.Sprintf("- %s · %s (Sprint %s, %s, Agent %s)",
				x.Nr, x.Titel, sp.ID, x.Status, firstOf(x.Agent, "?"))})
		}
	}
	return out
}

// queueDocs sammelt Tickets und Sprints mit `Braucht 🧑` ≠ `–`.
func (w *waveData) queueDocs(release, decide *[]string, accept *[]queueItem) {
	add := func(id, kind, title, domain, spec, needs, status string) {
		if !needsHuman(needs) || status == "erledigt" || status == "verworfen" {
			return
		}
		line := fmt.Sprintf("- %s · %s (%s, %s, Spec %s, Braucht 🧑: %s)", id, title, kind, firstOf(domain, "?"), firstOf(spec, "?"), needs)
		low := strings.ToLower(needs)
		if strings.Contains(low, "freigabe") && spec != "freigegeben" {
			*release = append(*release, line)
		}
		if strings.Contains(low, "entscheidung") {
			*decide = append(*decide, line)
		}
		if strings.Contains(low, "abnahme") || strings.Contains(low, "gerät") {
			*accept = append(*accept, queueItem{deviceIn(needs), line})
		}
	}
	for _, t := range w.d.Tickets {
		add(t.Nr, "Ticket", t.Title, t.Domain, t.Spec, w.specs[t.Nr].Needs, t.Status)
	}
	for _, sp := range w.d.Sprints {
		add(sp.ID, "Sprint", sp.Title, sp.Domain, sp.Spec, w.specs[sp.ID].Needs, sp.Status)
	}
}

// deviceOf liest das Gerät aus einer Zeile `Gerät:` im Abschnitt Kontext (oder im Kopf); sonst „unbekannt“.
func deviceOf(text string) string {
	all := splitLines(strings.ReplaceAll(text, "\r\n", "\n"))
	for _, l := range append(sectionLines(text, "## Kontext"), all[:headEnd(all)]...) {
		if m := reDevice.FindStringSubmatch(l); m != nil {
			return normDevice(m[1])
		}
	}
	return unknownDevice
}

// deviceIn findet in einem Wert wie „Abnahme (Xbox)“ das Gerät; sonst „unbekannt“.
func deviceIn(v string) string {
	if d := normDevice(v); slices.Contains(devices, d) {
		return d
	}
	return unknownDevice
}

// normDevice ordnet Schreibweisen den Geräten zu; Unbekanntes bleibt wie geschrieben.
func normDevice(v string) string {
	v = strings.TrimSpace(strings.Trim(v, "`*"))
	low := strings.ToLower(v)
	switch {
	case v == "":
		return ""
	case strings.Contains(low, "xbox") || strings.Contains(low, "tv"):
		return "Xbox/TV"
	case strings.Contains(low, "mini"):
		return "Mini-PC"
	case strings.Contains(low, "workshop"):
		return "Workshop"
	case strings.Contains(low, "veröffentl"):
		return "Veröffentlichung"
	case rePi.MatchString(v):
		return "Pi"
	case rePC.MatchString(v):
		return "PC"
	case strings.EqualFold(v, unknownDevice):
		return unknownDevice
	}
	return v
}

// queueText: Kopf mit Zahlen, Freigaben, Entscheidungen, Abnahmen gruppiert nach Gerät.
func queueText(release, decide []string, accept []queueItem, onlyAccept bool) string {
	groups := map[string][]string{}
	for _, q := range accept {
		groups[q.device] = append(groups[q.device], q.line)
	}
	order := deviceOrder(groups)
	out := []string{fmt.Sprintf("Ralf-Queue: %d Freigaben · %d Entscheidungen · %d Abnahmen an %d Geräten",
		len(release), len(decide), len(accept), len(order))}
	if !onlyAccept {
		out = append(out, "── Freigaben ──")
		out = append(out, firstLines(release, "keine")...)
		out = append(out, "── Entscheidungen ──")
		out = append(out, firstLines(decide, "keine")...)
	}
	out = append(out, "── Abnahmen je Gerät ──")
	if len(order) == 0 {
		out = append(out, "keine")
	}
	for _, d := range order {
		out = append(out, fmt.Sprintf("%s (%d):", d, len(groups[d])))
		out = append(out, groups[d]...)
	}
	return strings.Join(out, "\n")
}

// deviceOrder: bekannte Geräte in fester Reihenfolge, andere nach Name, „unbekannt“ zuletzt.
func deviceOrder(groups map[string][]string) []string {
	var known, other []string
	for _, d := range devices {
		if len(groups[d]) > 0 {
			known = append(known, d)
		}
	}
	for d := range groups {
		if !slices.Contains(devices, d) && d != unknownDevice {
			other = append(other, d)
		}
	}
	slices.Sort(other)
	out := append(known, other...)
	if len(groups[unknownDevice]) > 0 {
		out = append(out, unknownDevice)
	}
	return out
}
