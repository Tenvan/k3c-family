package world

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io/fs"
	"maps"
	"slices"
	"strconv"
	"strings"
)

// Geschichte einer Welt schreiben und lesen (B-975, docs/rules/welten.md § 2.2): `story` der Welt und je Insel. Format
// und Regeln prüft die Engine vor dem Schreiben (sim.ParseWorld › checkStory über commit); hier steht keine eigene
// Prüfregel. Geschrieben wird format-stabil: Nur `story` bekommt neue Bytes, alle übrigen Felder bleiben byte-gleich.

// Story ist `story` der Welt wie in der Datei: Wortschatz und Zeilen für Start, Etappen, Ende und Flaschenpost.
type Story struct {
	Lexicon []string            `json:"lexicon,omitempty"`
	Start   []string            `json:"start,omitempty"`
	Etappen map[string][]string `json:"etappen,omitempty"`
	End     []string            `json:"end,omitempty"`
	Bottles [][]string          `json:"bottles,omitempty"`
}

// IslandStory ist `story` einer Insel: Ankunft, Endboss ausgelöst, Abschlusszeile.
type IslandStory struct {
	Arrival []string `json:"arrival,omitempty"`
	Boss    []string `json:"boss,omitempty"`
	Done    []string `json:"done,omitempty"`
}

func (s Story) empty() bool {
	return len(s.Lexicon)+len(s.Start)+len(s.Etappen)+len(s.End)+len(s.Bottles) == 0
}

func (s IslandStory) empty() bool { return len(s.Arrival)+len(s.Boss)+len(s.Done) == 0 }

// StoryPatch ist eine Änderung der Geschichte: Ein Feld nil bleibt, wie es ist; ein leeres Feld (`[]`) entfernt den
// Eintrag, sonst ersetzt der Wert ihn. `Etappen` und `Islands` gelten je Schlüssel, `Bottles` ersetzt die ganze Liste.
type StoryPatch struct {
	Story
	Islands map[string]IslandStory
}

func (p StoryPatch) empty() bool {
	return p.Lexicon == nil && p.Start == nil && len(p.Etappen) == 0 && p.End == nil && p.Bottles == nil && len(p.Islands) == 0
}

// StoryResult nennt, was SetStory gesetzt und entfernt hat, die noch fehlenden Einträge und die Warnungen der Prüfung.
type StoryResult struct {
	Set, Removed, Missing, Warnings []string
}

// note trägt den Eintrag label als gesetzt (n > 0) oder entfernt ein.
func (r *StoryResult) note(label string, n int) {
	if n > 0 {
		r.Set = append(r.Set, label)
		return
	}
	r.Removed = append(r.Removed, label)
}

// SetStory ändert die Geschichte der Welt id nach p und schreibt nur, wenn die Prüfung der Engine besteht; sonst bleibt
// die Datei, und der Fehler nennt Feld und Zeile. Eine unbekannte Welt oder Insel wird mit den bekannten IDs abgelehnt.
func (e Editor) SetStory(ctx context.Context, id string, p StoryPatch) (StoryResult, error) {
	if p.empty() {
		return StoryResult{}, errors.New("kein Eintrag genannt (lexicon, start, etappen, end, bottles, islands)")
	}
	writeMu.Lock()
	defer writeMu.Unlock()
	raw, err := e.read(id)
	if errors.Is(err, fs.ErrNotExist) {
		return StoryResult{}, fmt.Errorf("die Welt %q gibt es nicht (bekannt: %s)", id, strings.Join(e.ids(), ", "))
	}
	if err != nil {
		return StoryResult{}, err
	}
	out, res, err := applyStory(raw, p)
	if err != nil {
		return StoryResult{}, err
	}
	if res.Warnings, err = e.commitChange(ctx, id, raw, out); err != nil {
		return StoryResult{}, err
	}
	res.Missing = storyGaps(out)
	return res, nil
}

// ids sind die IDs aller Welten des Checkouts.
func (e Editor) ids() []string {
	var out []string
	for _, w := range e.List() {
		if w.Missing {
			continue
		}
		out = append(out, w.ID)
	}
	return out
}

// storyFile ist der Teil der Welt-Datei, den die Geschichte braucht.
type storyFile struct {
	Story   *Story `json:"story"`
	Islands []struct {
		ID    string       `json:"id"`
		Story *IslandStory `json:"story"`
	} `json:"islands"`
}

// applyStory legt p über die Geschichte in raw: erst `story` der Welt, dann je genannter Insel (nach ID sortiert).
func applyStory(raw []byte, p StoryPatch) ([]byte, StoryResult, error) {
	var cur storyFile
	if err := json.Unmarshal(raw, &cur); err != nil {
		return nil, StoryResult{}, fmt.Errorf("Welt-Datei: %w", err)
	}
	var res StoryResult
	s := Story{}
	if cur.Story != nil {
		s = *cur.Story
	}
	mergeWorldStory(&s, p.Story, &res)
	out, err := raw, error(nil)
	if len(res.Set)+len(res.Removed) > 0 { // nur Inseln genannt: `story` der Welt bleibt byte-gleich
		if out, err = setWorldStory(raw, s); err != nil {
			return nil, StoryResult{}, err
		}
	}
	known := map[string]IslandStory{}
	var ids []string
	for _, isl := range cur.Islands {
		ids = append(ids, isl.ID)
		known[isl.ID] = IslandStory{}
		if isl.Story != nil {
			known[isl.ID] = *isl.Story
		}
	}
	for _, id := range slices.Sorted(maps.Keys(p.Islands)) {
		is, ok := known[id]
		if !ok {
			return nil, StoryResult{}, fmt.Errorf("die Insel %q gibt es nicht (bekannt: %s)", id, strings.Join(ids, ", "))
		}
		mergeIslandStory(&is, p.Islands[id], id, &res)
		if out, err = setIslandStory(out, id, is); err != nil {
			return nil, StoryResult{}, err
		}
	}
	return out, res, nil
}

// mergeWorldStory legt die genannten Felder von p über s und trägt sie in res ein.
func mergeWorldStory(s *Story, p Story, res *StoryResult) {
	if p.Lexicon != nil {
		s.Lexicon = p.Lexicon
		res.note(fmt.Sprintf("lexicon (%d Wörter)", len(p.Lexicon)), len(p.Lexicon))
	}
	mergeEntry(&s.Start, p.Start, "start", res)
	for _, k := range numericKeys(p.Etappen) {
		if s.Etappen == nil {
			s.Etappen = map[string][]string{}
		}
		if len(p.Etappen[k]) > 0 {
			s.Etappen[k] = p.Etappen[k]
		} else {
			delete(s.Etappen, k)
		}
		res.note("etappen."+k, len(p.Etappen[k]))
	}
	mergeEntry(&s.End, p.End, "end", res)
	if p.Bottles != nil {
		s.Bottles = p.Bottles
		res.note(fmt.Sprintf("bottles (%d Einträge)", len(p.Bottles)), len(p.Bottles))
	}
}

func mergeIslandStory(s *IslandStory, p IslandStory, id string, res *StoryResult) {
	mergeEntry(&s.Arrival, p.Arrival, id+".arrival", res)
	mergeEntry(&s.Boss, p.Boss, id+".boss", res)
	mergeEntry(&s.Done, p.Done, id+".done", res)
}

// mergeEntry setzt den Eintrag dst auf lines (nil lässt ihn, leer entfernt ihn).
func mergeEntry(dst *[]string, lines []string, label string, res *StoryResult) {
	if lines == nil {
		return
	}
	*dst = lines
	res.note(label, len(lines))
}

// numericKeys sind die Schlüssel von m, nach Zahl sortiert (Nicht-Zahlen nach Text).
func numericKeys[V any](m map[string]V) []string {
	keys := slices.Collect(maps.Keys(m))
	slices.SortFunc(keys, func(x, y string) int {
		a, ea := strconv.Atoi(x)
		b, eb := strconv.Atoi(y)
		if ea != nil || eb != nil {
			return strings.Compare(x, y)
		}
		return a - b
	})
	return keys
}

// setWorldStory setzt `story` der Welt auf s (leer entfernt das Feld): ein Feld je Zeile, ein neues `story` steht vor
// `islands`. Inhaltlich gleich bleibt die Datei byte-gleich.
func setWorldStory(raw []byte, s Story) ([]byte, error) {
	whole := span{0, len(raw)}
	at, found, err := valueSpan(raw, whole, "story")
	if err != nil {
		return nil, err
	}
	if s.empty() {
		if !found {
			return raw, nil
		}
		return removeKey(raw, at, "story"), nil
	}
	text, err := worldStoryText(s)
	if err != nil {
		return nil, err
	}
	if found {
		if sameJSON(raw[at.from:at.to], []byte(text)) {
			return raw, nil
		}
		return slices.Concat(raw[:at.from], eol(raw, text), raw[at.to:]), nil
	}
	isl, ok, err := valueSpan(raw, whole, "islands")
	if err != nil || !ok {
		return nil, errors.New("islands fehlt in der Welt-Datei")
	}
	k := strings.LastIndex(string(raw[:isl.from]), `"islands"`)
	return slices.Concat(raw[:k], eol(raw, `"story": `+text+",\n  "), raw[k:]), nil
}

// worldStoryText ist `story` der Welt im Stil der Datei: je Feld eine Zeile, Etappen nach Nummer sortiert.
func worldStoryText(s Story) (string, error) {
	var lines []string
	add := func(key string, v any, has bool) error {
		if !has {
			return nil
		}
		raw, err := json.Marshal(v)
		if err != nil {
			return err
		}
		text, err := inline(raw)
		lines = append(lines, "    "+quote(key)+": "+text)
		return err
	}
	var etappen []string
	for _, k := range numericKeys(s.Etappen) {
		raw, err := json.Marshal(s.Etappen[k])
		if err != nil {
			return "", err
		}
		etappen = append(etappen, quote(k)+": "+string(raw))
	}
	err := errors.Join(
		add("lexicon", s.Lexicon, len(s.Lexicon) > 0),
		add("start", s.Start, len(s.Start) > 0),
		add("etappen", json.RawMessage("{"+strings.Join(etappen, ",")+"}"), len(etappen) > 0),
		add("end", s.End, len(s.End) > 0),
		add("bottles", s.Bottles, len(s.Bottles) > 0),
	)
	return "{\n" + strings.Join(lines, ",\n") + "\n  }", err
}

// setIslandStory setzt `story` der Insel id auf s (leer entfernt das Feld), in der Zeile der Insel am Ende.
func setIslandStory(raw []byte, id string, s IslandStory) ([]byte, error) {
	obj, _, err := islandSpan(raw, id)
	if err != nil {
		return nil, err
	}
	var v json.RawMessage
	if !s.empty() {
		if v, err = json.Marshal(s); err != nil {
			return nil, err
		}
	}
	return setField(raw, obj, "story", v, false)
}

// storyGaps nennt die fehlenden Einträge der Geschichte in raw (siehe Detail.StoryGaps).
func storyGaps(raw []byte) []string {
	var f storyFile
	if json.Unmarshal(raw, &f) != nil {
		return nil
	}
	d := Detail{Story: f.Story}
	for _, isl := range f.Islands {
		d.Islands = append(d.Islands, Island{ID: isl.ID, Story: isl.Story})
	}
	return d.StoryGaps()
}

// StoryGaps nennt die fehlenden Einträge der Geschichte (B-975/R3): Welt ohne `story`, ohne `start`, `end` oder
// `bottles`, Insel ohne `story`, ohne `arrival`, `boss` oder `done`. Es sind Hinweise, keine Fehler.
func (d Detail) StoryGaps() []string {
	gaps := []string{}
	add := func(prefix string, missing bool, field string) {
		if missing {
			gaps = append(gaps, prefix+field)
		}
	}
	if d.Story == nil || d.Story.empty() {
		gaps = append(gaps, "story")
	} else {
		add("story.", len(d.Story.Start) == 0, "start")
		add("story.", len(d.Story.End) == 0, "end")
		add("story.", len(d.Story.Bottles) == 0, "bottles")
	}
	for _, isl := range d.Islands {
		prefix := "Insel " + isl.ID + ": story"
		if isl.Story == nil || isl.Story.empty() {
			gaps = append(gaps, prefix)
			continue
		}
		add(prefix+".", len(isl.Story.Arrival) == 0, "arrival")
		add(prefix+".", len(isl.Story.Boss) == 0, "boss")
		add(prefix+".", len(isl.Story.Done) == 0, "done")
	}
	return gaps
}
