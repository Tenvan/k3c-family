package planning

import (
	"fmt"
	"slices"
	"strings"
)

// plan_check (B-784): nennt je Ticket oder Sprint die Mängel der Planungsangaben für Arbeitswellen (docs/arbeitsweise.md
// › Planungsangaben): Bereich leer, Umgebung ?, Größe L, unbekannte ID in „Abhängig von“, Bereich überlappt mit einem
// anderen offenen Ticket, dazu fehlende Zeilen. Archivierte Tickets werden nicht geprüft; der Bestand bleibt gültig.

// CheckIn ist die Eingabe von plan_check.
type CheckIn struct {
	ID string `json:"id" jsonschema:"Ticket B-123 oder Sprint M8"`
}

// Check liefert die Mängel eines Tickets oder Sprints als Text.
func Check(root string, in CheckIn) (string, error) {
	id := strings.TrimSpace(in.ID)
	r, err := resolve(root, id)
	if err != nil {
		return "", err
	}
	if r.kind != "ticket" && r.kind != "sprint" {
		return "", fmt.Errorf("plan_check prüft Tickets und Sprints, nicht %s %s", r.kind, id)
	}
	if r.kind == "ticket" && r.state == "archiv" {
		return id + ": archiviert, nicht geprüft", nil
	}
	text, _, err := readText(root, r.rel)
	if err != nil {
		return "", err
	}
	w, err := loadWaveData(root)
	if err != nil {
		return "", err
	}
	found := w.defects(r.kind, id, text)
	if len(found) == 0 {
		return id + ": keine Mängel in den Planungsangaben", nil
	}
	return fmt.Sprintf("%s: %d Mängel\n- %s", id, len(found), strings.Join(found, "\n- ")), nil
}

// defects sind die Mängel eines Tickets oder einer Sprint-README.
func (w *waveData) defects(kind, id, text string) []string {
	spec := parsePlanSpec(text)
	var out []string
	if !spec.HasLines {
		out = append(out, "Planungsangaben fehlen im Abschnitt „Regeln und Einschränkungen“")
	}
	for _, name := range planLines {
		if spec.HasLines && !spec.Seen[name] && (name != "Größe" || kind == "ticket") {
			out = append(out, "Zeile „"+name+"“ fehlt")
		}
	}
	if spec.Seen["Bereich"] && len(spec.Area) == 0 {
		out = append(out, "Bereich leer")
	}
	out = append(out, w.envDefects(kind, id, text)...)
	if strings.EqualFold(spec.Size, "L") {
		out = append(out, "Größe: L (vor der Freigabe in Tickets je Teil zerlegen)")
	}
	for _, dep := range spec.Deps {
		if _, _, _, unknown := w.depTarget(dep); unknown {
			out = append(out, "unbekannte ID "+dep+" in „Abhängig von“")
		}
	}
	for _, bad := range spec.BadDeps {
		out = append(out, "keine ID in „Abhängig von“: "+bad)
	}
	return append(out, w.overlaps(kind, id, spec.Area)...)
}

// envDefects: `Umgebung: ?` beim Ticket, bei einem Sprint in jeder offenen Session.
func (w *waveData) envDefects(kind, id, text string) []string {
	if kind == "ticket" {
		if strings.HasPrefix(headFields(strings.ReplaceAll(text, "\r\n", "\n"))["Umgebung"], "?") {
			return []string{"Umgebung: ?"}
		}
		return nil
	}
	var out []string
	for _, x := range w.sprints[id].Sessions {
		if strings.HasPrefix(x.Env, "?") && x.Status != "fertig" && x.Status != "verworfen" {
			out = append(out, "Umgebung: ? in "+x.Nr)
		}
	}
	return out
}

// overlaps: Bereich überlappt mit anderen offenen Tickets (beim Sprint ohne seine eigenen Tickets).
func (w *waveData) overlaps(kind, id string, area []string) []string {
	own := []string{id}
	if kind == "sprint" {
		own = append(own, w.sprints[id].Tickets...)
	}
	var out []string
	for _, t := range w.d.Tickets {
		if t.Status != "offen" || slices.Contains(own, t.Nr) {
			continue
		}
		if pair, ok := areaOverlap(area, w.specs[t.Nr].Area); ok {
			out = append(out, "Bereich überlappt mit "+t.Nr+" ("+pair+")")
		}
	}
	return out
}
