package paperassets

import (
	"bytes"
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"reflect"
	"sort"
	"strings"

	"github.com/google/jsonschema-go/jsonschema"

	"k3c/tools/k3c-workbench/internal/assetsui"
)

// stilSchema ist das JSON-Schema der Stil-Datei relativ zum Checkout (PV1.3).
const stilSchema = "tools/paper/schema/stil.schema.json"

// Wert zeigt einen Feldwert kurz: Text ohne Anführungszeichen, fehlend als –, sonst kompaktes JSON.
func Wert(v any) string {
	switch x := v.(type) {
	case nil:
		return "–"
	case string:
		return x
	}
	b, _ := json.Marshal(v)
	return string(b)
}

// kanonisch ist die kanonische JSON-Form wie hash.ts: Schlüssel sortiert, ohne Leerraum, CRLF in Texten als LF.
func kanonisch(v any) string {
	switch x := v.(type) {
	case map[string]any:
		keys := make([]string, 0, len(x))
		for k := range x {
			keys = append(keys, k)
		}
		sort.Strings(keys)
		teile := make([]string, len(keys))
		for i, k := range keys {
			teile[i] = jsonText(k) + ":" + kanonisch(x[k])
		}
		return "{" + strings.Join(teile, ",") + "}"
	case []any:
		teile := make([]string, len(x))
		for i, e := range x {
			teile[i] = kanonisch(e)
		}
		return "[" + strings.Join(teile, ",") + "]"
	case string:
		return jsonText(strings.ReplaceAll(x, "\r\n", "\n"))
	}
	return jsonText(v)
}

func jsonText(v any) string {
	var b bytes.Buffer
	enc := json.NewEncoder(&b)
	enc.SetEscapeHTML(false)
	_ = enc.Encode(v)
	return strings.TrimSuffix(b.String(), "\n")
}

// StilHash ist der stilHash laut paper-spec.md › Hash (SHA-256 der kanonischen Form ohne $schema).
func StilHash(b []byte) (string, error) {
	var v any
	if err := json.Unmarshal(b, &v); err != nil {
		return "", err
	}
	if m, ok := v.(map[string]any); ok {
		delete(m, "$schema")
	}
	sum := sha256.Sum256([]byte(kanonisch(v)))
	return hex.EncodeToString(sum[:]), nil
}

// FeldDiff vergleicht zwei JSON-Dokumente je Schlüssel (sortiert); Listen und Werte als Ganzes.
func FeldDiff(alt, neu []byte) ([]assetsui.FeldAenderung, error) {
	var a, n any
	if err := json.Unmarshal(alt, &a); err != nil {
		return nil, err
	}
	if err := json.Unmarshal(neu, &n); err != nil {
		return nil, err
	}
	var out []assetsui.FeldAenderung
	feldDiff(a, n, "", &out)
	return out, nil
}

func feldDiff(a, n any, pfad string, out *[]assetsui.FeldAenderung) {
	ma, okA := a.(map[string]any)
	mn, okN := n.(map[string]any)
	if okA && okN {
		keys := map[string]bool{}
		for k := range ma {
			keys[k] = true
		}
		for k := range mn {
			keys[k] = true
		}
		sorted := make([]string, 0, len(keys))
		for k := range keys {
			if k != "$schema" {
				sorted = append(sorted, k)
			}
		}
		sort.Strings(sorted)
		for _, k := range sorted {
			p := k
			if pfad != "" {
				p = pfad + "." + k
			}
			feldDiff(ma[k], mn[k], p, out)
		}
		return
	}
	if !reflect.DeepEqual(a, n) {
		*out = append(*out, assetsui.FeldAenderung{Pfad: pfad, Alt: a, Neu: n})
	}
}

// Materialien sind alle Materialien und Einzeltöne, die eine Spec nennt (Lagen, Umhang, tausch).
func Materialien(spec []byte) map[string]bool {
	var v any
	_ = json.Unmarshal(spec, &v)
	out := map[string]bool{}
	sammle(v, out)
	return out
}

func sammle(v any, out map[string]bool) {
	switch x := v.(type) {
	case map[string]any:
		for k, e := range x {
			sammleFeld(k, e, out)
			sammle(e, out)
		}
	case []any:
		for _, e := range x {
			sammle(e, out)
		}
	}
}

// sammleFeld nimmt "material": "<name>" und die Paare einer "tausch"-Tabelle auf.
func sammleFeld(k string, e any, out map[string]bool) {
	if s, ok := e.(string); ok && k == "material" {
		out[s] = true
	}
	t, ok := e.(map[string]any)
	if !ok || k != "tausch" {
		return
	}
	for von, nach := range t {
		out[von] = true
		if s, ok := nach.(string); ok {
			out[s] = true
		}
	}
}

// StilErgebnis sind die geänderten Felder der Stil-Datei und die Assets, die sie nutzen.
type StilErgebnis struct {
	Felder    []assetsui.FeldAenderung
	Betroffen []Asset
}

// StilSet wendet einen Merge-Patch auf die Stil-Datei an: Schema, material-doppelt und material-unbekannt (Specs, die
// ein entferntes Material nutzen) prüfen, dann per Temp-Datei und Umbenennen ersetzen. Abgelehnt ändert nichts.
func StilSet(ctx context.Context, cli CLI, patch json.RawMessage) (StilErgebnis, error) {
	defer sperre(cli.Root, StilPfad)()
	pfad := filepath.Join(cli.Root, filepath.FromSlash(StilPfad))
	alt, err := os.ReadFile(pfad)
	if err != nil {
		return StilErgebnis{}, err
	}
	neu, err := MergePatch(alt, patch)
	if err != nil {
		return StilErgebnis{}, err
	}
	text, err := Format(neu)
	if err != nil {
		return StilErgebnis{}, err
	}
	assets, err := Assets(cli.Root)
	if err != nil {
		return StilErgebnis{}, err
	}
	if fehler := pruefeStil(cli.Root, text, assets); len(fehler) > 0 {
		return StilErgebnis{}, &Abgelehnt{Fehler: fehler}
	}
	felder, err := FeldDiff(alt, text)
	if err != nil {
		return StilErgebnis{}, err
	}
	r := StilErgebnis{Felder: felder, Betroffen: betroffen(cli.Root, felder, assets)}
	tmp := pfad + ".tmp"
	if err := os.WriteFile(tmp, text, 0o644); err != nil {
		return r, err
	}
	if err := os.Rename(tmp, pfad); err != nil {
		_ = os.Remove(tmp)
		return r, err
	}
	return r, nil // geschrieben: ein Abbruch danach ändert nichts mehr am Ergebnis
}

// stilTeile liest Materialien und Einzeltöne der Stil-Datei.
type stilTeile struct {
	Materialien map[string]json.RawMessage `json:"materialien"`
	Einzeltoene map[string]json.RawMessage `json:"einzeltoene"`
}

func pruefeStil(root string, text []byte, assets []Asset) []Fehler {
	var inst any
	if err := json.Unmarshal(text, &inst); err != nil {
		return []Fehler{{Feld: "stil", Regel: "json", Text: err.Error()}}
	}
	if err := validiereSchema(root, inst); err != nil {
		return []Fehler{{Feld: "stil", Regel: "schema", Text: err.Error()}}
	}
	var st stilTeile
	_ = json.Unmarshal(text, &st)
	var fehler []Fehler
	for name := range st.Einzeltoene {
		if _, ok := st.Materialien[name]; ok {
			fehler = append(fehler, Fehler{Feld: "einzeltoene." + name, Regel: "material-doppelt",
				Text: fmt.Sprintf("„%s“ ist zugleich Material und Einzelton", name)})
		}
	}
	for _, a := range assets {
		b, err := os.ReadFile(a.Abs(root))
		if err != nil {
			continue
		}
		for _, m := range sortiert(Materialien(b)) {
			_, ist := st.Materialien[m]
			_, ton := st.Einzeltoene[m]
			if !ist && !ton {
				fehler = append(fehler, Fehler{Feld: a.ID + endung, Regel: "material-unbekannt",
					Text: fmt.Sprintf("Material „%s“ fehlt in der Stil-Datei", m)})
			}
		}
	}
	sort.SliceStable(fehler, func(i, j int) bool {
		return fehler[i].Regel == "material-doppelt" && fehler[j].Regel != "material-doppelt"
	})
	return fehler
}

func validiereSchema(root string, inst any) error {
	b, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(stilSchema)))
	if err != nil {
		return fmt.Errorf("Schema %s fehlt: %w", stilSchema, err) //nolint:staticcheck // Satz für die Antwort
	}
	var s jsonschema.Schema
	if err := json.Unmarshal(b, &s); err != nil {
		return err
	}
	rs, err := s.Resolve(nil)
	if err != nil {
		return err
	}
	return rs.Validate(inst)
}

func sortiert(m map[string]bool) []string {
	out := make([]string, 0, len(m))
	for k := range m {
		out = append(out, k)
	}
	sort.Strings(out)
	return out
}

// betroffen: Änderungen nur an Materialien oder Einzeltönen betreffen die Assets, die sie nutzen; alles andere alle.
func betroffen(root string, felder []assetsui.FeldAenderung, assets []Asset) []Asset {
	namen := map[string]bool{}
	for _, f := range felder {
		teile := strings.SplitN(f.Pfad, ".", 3)
		if len(teile) < 2 || (teile[0] != "materialien" && teile[0] != "einzeltoene") {
			return assets
		}
		namen[teile[1]] = true
	}
	var out []Asset
	for _, a := range assets {
		b, err := os.ReadFile(a.Abs(root))
		if err != nil {
			continue
		}
		for m := range Materialien(b) {
			if namen[m] {
				out = append(out, a)
				break
			}
		}
	}
	return out
}
