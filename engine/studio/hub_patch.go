package studio

import (
	"bytes"
	"encoding/json"
	"errors"
	"io"
	"reflect"
	"slices"
	"strings"
)

// Format-stabiles Schreiben der hub.json (B-462/AC-06, WZ9.3): Der Planer liefert den Kandidaten als JSON beliebiger
// Form; geschrieben wird die alte Datei, in der nur die Werte ersetzt sind, die sich im Kandidaten geändert haben.
// Schlüsselreihenfolge, Einrückung, Zahlenformat und Zeilenenden bleiben, eine geänderte Zahl ist ein Ein-Zeilen-Diff.
// Hinzugefügte und entfernte Listen-Einträge (Plätze) folgen dem Layout der Liste (eine Zeile je Eintrag).

// jnode ist ein JSON-Wert mit seiner Lage im Text: kind '{', '[' oder 0 (Zahl, Text, true, false, null).
type jnode struct {
	start, end int
	kind       byte
	keys       []string // Objekt: Schlüssel in Dateireihenfolge, parallel zu kids
	kids       []*jnode
}

// scanJSON liest raw als einen JSON-Wert mit den Lagen aller Teilwerte.
func scanJSON(raw []byte) (*jnode, error) {
	dec := json.NewDecoder(bytes.NewReader(raw))
	n, err := scanValue(dec, raw)
	if err != nil {
		return nil, err
	}
	if _, err := dec.Token(); !errors.Is(err, io.EOF) {
		return nil, errors.New("weitere Daten nach dem JSON-Wert")
	}
	return n, nil
}

func scanValue(dec *json.Decoder, raw []byte) (*jnode, error) {
	from := skipSep(raw, int(dec.InputOffset()))
	tok, err := dec.Token()
	if err != nil {
		return nil, err
	}
	d, ok := tok.(json.Delim)
	if !ok {
		return &jnode{start: from, end: int(dec.InputOffset())}, nil
	}
	n := &jnode{start: from, kind: byte(d)}
	for dec.More() {
		if d == '{' {
			k, err := dec.Token()
			if err != nil {
				return nil, err
			}
			key, _ := k.(string)
			n.keys = append(n.keys, key)
		}
		kid, err := scanValue(dec, raw)
		if err != nil {
			return nil, err
		}
		n.kids = append(n.kids, kid)
	}
	if _, err := dec.Token(); err != nil {
		return nil, err
	}
	n.end = int(dec.InputOffset())
	return n, nil
}

// skipSep überspringt Leerraum und die Trenner , und : vor einem Wert.
func skipSep(raw []byte, at int) int {
	for at < len(raw) && strings.IndexByte(" \t\r\n,:", raw[at]) >= 0 {
		at++
	}
	return at
}

// sameJSON: a und b bedeuten denselben Wert (Zahlenformat, Leerraum und Schlüsselreihenfolge egal).
func sameJSON(a, b []byte) bool {
	var x, y any
	return json.Unmarshal(a, &x) == nil && json.Unmarshal(b, &y) == nil && reflect.DeepEqual(x, y)
}

// patchJSON schreibt cand in das Format von old: Unveränderte Werte bleiben byte-gleich, geänderte werden ersetzt.
// Das Ergebnis bedeutet garantiert dasselbe wie cand, sonst ist es ein Fehler.
func patchJSON(old, cand []byte) ([]byte, error) {
	o, err := scanJSON(old)
	if err != nil {
		return nil, err
	}
	c, err := scanJSON(cand)
	if err != nil {
		return nil, err
	}
	nl := "\n"
	if bytes.Contains(old, []byte("\r\n")) {
		nl = "\r\n"
	}
	p := patcher{old: old, cand: cand, nl: nl}
	out := slices.Concat(old[:o.start], []byte(p.value(o, c)), old[o.end:])
	if !sameJSON(out, cand) {
		return nil, errors.New("kandidat lässt sich nicht format-stabil schreiben")
	}
	return out, nil
}

type patcher struct {
	old, cand []byte
	nl        string
}

func (p *patcher) oldText(n *jnode) []byte  { return p.old[n.start:n.end] }
func (p *patcher) candText(n *jnode) []byte { return p.cand[n.start:n.end] }

// value liefert den Text für c an der Stelle von o.
func (p *patcher) value(o, c *jnode) string {
	if sameJSON(p.oldText(o), p.candText(c)) {
		return string(p.oldText(o))
	}
	switch {
	case o.kind == '{' && c.kind == '{' && sameKeys(o.keys, c.keys):
		kids := make([]*jnode, len(o.keys))
		for j, key := range o.keys {
			kids[j] = c.kids[slices.Index(c.keys, key)]
		}
		return p.splice(o, kids)
	case o.kind == '{' && c.kind == '{':
		return p.object(o, c)
	case o.kind == '[' && c.kind == '[':
		return p.array(o, c)
	}
	return p.inline(c)
}

// sameKeys: dieselben Schlüssel, Reihenfolge egal.
func sameKeys(a, b []string) bool {
	return len(a) == len(b) && !slices.ContainsFunc(a, func(k string) bool { return !slices.Contains(b, k) })
}

// splice ersetzt die Teilwerte von o paarweise durch die von kids; der Text dazwischen bleibt.
func (p *patcher) splice(o *jnode, kids []*jnode) string {
	var b strings.Builder
	at := o.start
	for i, k := range o.kids {
		b.Write(p.old[at:k.start])
		b.WriteString(p.value(k, kids[i]))
		at = k.end
	}
	b.Write(p.old[at:o.end])
	return b.String()
}

// object: Schlüssel hinzugekommen oder entfallen; bleibende Schlüssel in der alten Reihenfolge, neue dahinter in der
// des Kandidaten, Layout von o.
func (p *patcher) object(o, c *jnode) string {
	parts := []string{}
	for j, key := range o.keys {
		if i := slices.Index(c.keys, key); i >= 0 {
			parts = append(parts, quote(key)+": "+p.value(o.kids[j], c.kids[i]))
		}
	}
	for i, key := range c.keys {
		if !slices.Contains(o.keys, key) {
			parts = append(parts, quote(key)+": "+p.inline(c.kids[i]))
		}
	}
	return p.layout(o, "{", "}", parts)
}

// array: Einträge paarweise, solange die Anzahl gleich ist; sonst neu gelegt, unveränderte Einträge byte-gleich.
func (p *patcher) array(o, c *jnode) string {
	if len(o.kids) == len(c.kids) {
		return p.splice(o, c.kids)
	}
	parts := make([]string, len(c.kids))
	j := 0
	for i, k := range c.kids {
		switch at := p.find(o.kids[j:], k, p.oldText, p.candText); {
		case at >= 0:
			parts[i], j = string(p.oldText(o.kids[j+at])), j+at+1
		case j < len(o.kids) && p.find(c.kids[i:], o.kids[j], p.candText, p.oldText) < 0:
			parts[i], j = p.value(o.kids[j], k), j+1 // geänderter Eintrag, der alte kommt nicht mehr vor
		default:
			parts[i] = p.fresh(o, k)
		}
	}
	return p.layout(o, "[", "]", parts)
}

// fresh schreibt einen neuen Eintrag der Liste o wie ihren ersten: ein mehrzeiliges Objekt mehrzeilig, sonst einzeilig.
func (p *patcher) fresh(o, k *jnode) string {
	if len(o.kids) == 0 || k.kind != '{' || len(k.kids) == 0 {
		return p.inline(k)
	}
	sib := o.kids[0]
	if sib.kind != '{' || len(sib.kids) == 0 || !bytes.Contains(p.oldText(sib), []byte("\n")) {
		return p.inline(k)
	}
	inner, outer := lineIndent(p.old, sib.kids[0].start), lineIndent(p.old, sib.end-1)
	parts := make([]string, len(k.kids))
	for i, v := range k.kids {
		parts[i] = quote(k.keys[i]) + ": " + p.inline(v)
	}
	return "{" + p.nl + inner + strings.Join(parts, ","+p.nl+inner) + p.nl + outer + "}"
}

// find sucht in list (Text über of) den ersten Eintrag mit demselben Wert wie k (Text über kOf).
func (p *patcher) find(list []*jnode, k *jnode, of, kOf func(*jnode) []byte) int {
	return slices.IndexFunc(list, func(n *jnode) bool { return sameJSON(of(n), kOf(k)) })
}

// layout legt parts wie o: einzeilig „{ a, b }“ bzw. „[a, b]“, mehrzeilig ein Teil je Zeile mit der Einrückung von o.
func (p *patcher) layout(o *jnode, open, closing string, parts []string) string {
	text := string(p.oldText(o))
	if !strings.Contains(text, "\n") || len(parts) == 0 {
		if open == "{" && len(parts) > 0 {
			return "{ " + strings.Join(parts, ", ") + " }"
		}
		return open + strings.Join(parts, ", ") + closing
	}
	outer := lineIndent(p.old, o.end-1)
	inner := outer + "  "
	if len(o.kids) > 0 {
		inner = lineIndent(p.old, o.kids[0].start)
	}
	return open + p.nl + inner + strings.Join(parts, ","+p.nl+inner) + p.nl + outer + closing
}

// lineIndent ist der führende Leerraum der Zeile, in der at steht.
func lineIndent(raw []byte, at int) string {
	from := bytes.LastIndexByte(raw[:at], '\n') + 1
	to := from
	for to < len(raw) && (raw[to] == ' ' || raw[to] == '\t') {
		to++
	}
	return string(raw[from:to])
}

// inline schreibt einen neuen Wert des Kandidaten einzeilig im Stil der Datei: „{ "a": 1 }“, „[1, 2]“.
func (p *patcher) inline(c *jnode) string {
	switch c.kind {
	case '{':
		parts := make([]string, len(c.kids))
		for i, k := range c.kids {
			parts[i] = quote(c.keys[i]) + ": " + p.inline(k)
		}
		if len(parts) == 0 {
			return "{}"
		}
		return "{ " + strings.Join(parts, ", ") + " }"
	case '[':
		parts := make([]string, len(c.kids))
		for i, k := range c.kids {
			parts[i] = p.inline(k)
		}
		return "[" + strings.Join(parts, ", ") + "]"
	}
	return string(p.candText(c))
}

// quote schreibt einen Schlüssel als JSON-Text, Umlaute im Klartext.
func quote(s string) string {
	var b bytes.Buffer
	enc := json.NewEncoder(&b)
	enc.SetEscapeHTML(false)
	_ = enc.Encode(s)
	return strings.TrimRight(b.String(), "\n")
}
