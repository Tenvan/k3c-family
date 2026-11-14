package planning

import (
	"fmt"
	"io"
	"os"
	"slices"
	"sort"
	"strings"
)

// Relationen zwischen Tickets (B-696 Teil A): „Duplikat von“ (genau eine ID) und „Ersetzt“ (Liste) stehen nur im
// neueren Ticket. Die Gegenrichtung („Ersetzt durch“, „Duplikate“) berechnet Get beim Lesen; die Datei des
// Gegenstücks bleibt unverändert. Fehlt ein Feld (Altdatei), gilt es als `–`.

const (
	fieldDup     = "Duplikat von"
	fieldReplace = "Ersetzt"
	headLimit    = 8 << 10 // der Kopf eines Tickets steht in den ersten 8 KB
)

// relInfo ist der Teil eines Ticket-Kopfs, den die Relationen brauchen.
type relInfo struct {
	status, dup string
	replaces    []string
}

// relIDs liest die IDs eines Relationsfelds; leer oder `–` heißt keine.
func relIDs(v string) []string {
	v = strings.TrimSpace(v)
	if v == "" || strings.HasPrefix(v, "–") {
		return nil
	}
	var out []string
	for _, p := range strings.Split(v, ",") {
		out = append(out, strings.TrimSpace(p))
	}
	return out
}

// checkRelFormat: `–`, bei „Duplikat von“ genau eine ID, bei „Ersetzt“ eine Liste `B-001, B-002` ohne Dopplung (das
// Format prüft auch tests/planningDocs.ts › relationOk).
func checkRelFormat(name, value string) error {
	ids := relIDs(value)
	if value != "–" && len(ids) == 0 {
		return fmt.Errorf("%s: `–` oder Ticket-IDs wie B-123", name)
	}
	if name == fieldDup && len(ids) > 1 {
		return fmt.Errorf("%s: genau eine ID, nicht %d", name, len(ids))
	}
	if value != "–" && value != strings.Join(ids, ", ") {
		return fmt.Errorf("%s: IDs mit „, “ trennen, z. B. B-001, B-002", name)
	}
	for i, id := range ids {
		if !reTicketID.MatchString(id) {
			return fmt.Errorf("%s: %q ist keine Ticket-ID (B-123)", name, id)
		}
		if slices.Contains(ids[:i], id) {
			return fmt.Errorf("%s: %s doppelt", name, id)
		}
	}
	return nil
}

// readHead liest den Kopf einer Datei (bis zur ersten `## `-Überschrift, höchstens headLimit Bytes).
func readHead(p string) string {
	f, err := os.Open(p)
	if err != nil {
		return ""
	}
	defer func() { _ = f.Close() }()
	b, _ := io.ReadAll(io.LimitReader(f, headLimit))
	s := strings.ReplaceAll(string(b), "\r\n", "\n")
	if i := strings.Index(s, "\n## "); i >= 0 {
		s = s[:i]
	}
	return s
}

// relations liest Status und Relationen aller Tickets in Backlog und Archiv.
func relations(root string) map[string]relInfo {
	out := map[string]relInfo{}
	for _, folder := range []string{"backlog", "backlog/archiv"} {
		entries, _ := os.ReadDir(docsPath(root, folder))
		for _, e := range entries {
			if e.IsDir() || !ticket.MatchString(e.Name()) {
				continue
			}
			f := headFields(readHead(docsPath(root, folder+"/"+e.Name())))
			info := relInfo{status: f["Status"], replaces: relIDs(f[fieldReplace])}
			if d := relIDs(f[fieldDup]); len(d) > 0 {
				info.dup = d[0]
			}
			// Doppelt vergebene Nummer (B-199, B-696): das Ticket im Backlog gilt, es wird zuerst gelesen.
			if _, dup := out[e.Name()[:5]]; !dup {
				out[e.Name()[:5]] = info
			}
		}
	}
	return out
}

// pick ist der neue Wert, sonst der alte aus dem Kopf.
func pick(values, head map[string]string, name string) string {
	if v, ok := values[name]; ok {
		return v
	}
	return head[name]
}

// checkRelations prüft gesetzte Relationen gegen alle Tickets: jede ID gibt es, kein Selbstverweis, kein Zyklus, ein
// Duplikat ist nicht erledigt. Ohne Relationsfeld in values passiert nichts. Die Notizen sind Hinweise (Wurzel der Kette).
func checkRelations(root, id string, head, values map[string]string) ([]string, error) {
	_, dupSet := values[fieldDup]
	_, repSet := values[fieldReplace]
	if !dupSet && !repSet {
		return nil, nil
	}
	for _, name := range []string{fieldDup, fieldReplace} {
		if v, ok := values[name]; ok {
			if err := checkRelFormat(name, v); err != nil {
				return nil, err
			}
		}
	}
	all := relations(root)
	dup, rep := relIDs(pick(values, head, fieldDup)), relIDs(pick(values, head, fieldReplace))
	if err := checkTargets(all, id, append(slices.Clone(dup), rep...)); err != nil {
		return nil, err
	}
	if len(dup) > 0 && (head["Status"] == "erledigt" || values["Status"] == "erledigt") {
		return nil, fmt.Errorf("%s ist erledigt und damit kein Duplikat (🧑 entscheidet), nichts geändert", id)
	}
	self := all[id]
	self.dup, self.replaces = "", rep
	if len(dup) > 0 {
		self.dup = dup[0]
	}
	all[id] = self
	if path := cyclePath(all, id); path != nil {
		return nil, fmt.Errorf("%s ergäbe einen Zyklus %s, nichts geändert", id, strings.Join(path, " → "))
	}
	if len(dup) == 0 {
		return nil, nil
	}
	if r := chainRoot(all, dup[0]); r != dup[0] {
		return []string{fmt.Sprintf("Hinweis: %s ist selbst verworfen oder ersetzt; Wurzel der Kette ist %s", dup[0], r)}, nil
	}
	return nil, nil
}

// checkTargets: jedes Ziel gibt es und keins ist das Ticket selbst.
func checkTargets(all map[string]relInfo, id string, targets []string) error {
	for _, t := range targets {
		if t == id {
			return fmt.Errorf("%s verweist auf sich selbst, nichts geändert", id)
		}
		if _, ok := all[t]; !ok {
			return fmt.Errorf("%s gibt es nicht (Backlog und Archiv)%s, nichts geändert", t, nearIDs(all, t))
		}
	}
	return nil
}

// ticketRelations prüft die Relationen für Set und setzt bei „Duplikat von“ den Status verworfen; values bleibt
// unverändert, die Rückgabe ist eine Kopie.
func ticketRelations(c *changeSet, r ref, text string, values map[string]string) (map[string]string, error) {
	notes, err := checkRelations(c.root, r.id, headFields(text), values)
	if err != nil {
		return nil, err
	}
	c.notes = append(c.notes, notes...)
	out := make(map[string]string, len(values)+1)
	for k, v := range values {
		out[k] = v
	}
	if d := relIDs(values[fieldDup]); len(d) > 0 && out["Status"] != "verworfen" {
		out["Status"] = "verworfen"
		c.notes = append(c.notes, r.id+" ist Duplikat von "+d[0]+": Status verworfen")
	}
	return out, nil
}

// edges sind die Verweise eines Tickets (Duplikat und Ersatz).
func (r relInfo) edges() []string {
	if r.dup == "" {
		return r.replaces
	}
	return append([]string{r.dup}, r.replaces...)
}

// cyclePath sucht einen Weg von id über Verweise zurück zu id; nil ohne Zyklus.
func cyclePath(all map[string]relInfo, id string) []string {
	seen := map[string]bool{}
	var walk func(cur string, path []string) []string
	walk = func(cur string, path []string) []string {
		for _, next := range all[cur].edges() {
			if next == id {
				return append(path, next)
			}
			if !seen[next] {
				seen[next] = true
				if p := walk(next, append(path, next)); p != nil {
					return p
				}
			}
		}
		return nil
	}
	return walk(id, []string{id})
}

// replacedBy ist das erste Ticket (nach Nummer), das id ersetzt.
func replacedBy(all map[string]relInfo, id string) string {
	var hits []string
	for k, r := range all {
		if slices.Contains(r.replaces, id) {
			hits = append(hits, k)
		}
	}
	sort.Strings(hits)
	if len(hits) == 0 {
		return ""
	}
	return hits[0]
}

// chainRoot folgt Duplikat und Ersatz bis zu einem Ticket, das weder Duplikat noch ersetzt ist.
func chainRoot(all map[string]relInfo, id string) string {
	seen := map[string]bool{id: true}
	for {
		next := all[id].dup
		if next == "" {
			next = replacedBy(all, id)
		}
		if next == "" || seen[next] {
			return id
		}
		seen[next], id = true, next
	}
}

// nearIDs nennt vorhandene IDs, die sich in einer Ziffer unterscheiden (Tippfehler), höchstens fünf.
func nearIDs(all map[string]relInfo, id string) string {
	var hits []string
	for k := range all {
		diff := 0
		for i := 0; i < len(k) && len(k) == len(id); i++ {
			if k[i] != id[i] {
				diff++
			}
		}
		if len(k) == len(id) && diff == 1 {
			hits = append(hits, k)
		}
	}
	if len(hits) == 0 {
		return ""
	}
	sort.Strings(hits)
	return "; ähnlich: " + strings.Join(hits[:min(5, len(hits))], ", ")
}

// reverseLines ist die berechnete Gegenrichtung für Get: wer id ersetzt und wessen Duplikat id ist.
func reverseLines(root, id string) []string {
	var by, dups []string
	for k, r := range relations(root) {
		if slices.Contains(r.replaces, id) {
			by = append(by, k)
		}
		if r.dup == id {
			dups = append(dups, k)
		}
	}
	sort.Strings(by)
	sort.Strings(dups)
	var out []string
	if len(by) > 0 {
		out = append(out, "> **Ersetzt durch:** "+strings.Join(by, ", ")+" (berechnet)")
	}
	if len(dups) > 0 {
		out = append(out, "> **Duplikate:** "+strings.Join(dups, ", ")+" (berechnet)")
	}
	return out
}

// withReverse setzt die Gegenrichtung als Zitatzeilen hinter den Kopf eines Tickets.
func withReverse(text string, rev []string) string {
	if len(rev) == 0 {
		return text
	}
	lines := splitLines(text)
	at := headEnd(lines)
	ins := append(slices.Clone(rev), "")
	return joinLines(append(lines[:at:at], append(ins, lines[at:]...)...))
}
