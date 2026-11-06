package review

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path"
	"path/filepath"
	"regexp"
	"slices"
	"strconv"
	"strings"
)

// reName schützt die Dateinamen im Cache: Animation (auch `idle@spieler2`, `stand.s2`) und Variante sind Schlüssel
// der Spec, nie Pfade.
var reName = regexp.MustCompile(`^[a-z0-9][a-z0-9.@_-]*$`)

func gueltigerName(s string) bool { return reName.MatchString(s) && !strings.Contains(s, "..") }

// SpecUnlesbar: die Spec ist kein lesbares JSON; im Review ein ungültiger Eintrag (Regel `schema`).
type SpecUnlesbar struct {
	Pfad string
	Err  error
}

func (e *SpecUnlesbar) Error() string { return e.Pfad + ": JSON unlesbar: " + e.Err.Error() }

// Quelle ist ein Stand eines Assets: Spec und Stil-Datei als Text, ihre Hashes und ob er aus der Basis stammt.
type Quelle struct {
	Pfad     string // relativ zum Checkout, mit /
	Spec     []byte
	Stil     []byte
	SpecHash string
	StilHash string
	Basis    bool
	// Beilagen sind Specs, die die CLI neben dieser braucht (Reittiere, Regel reittier-unbekannt): Pfad → Inhalt.
	Beilagen     map[string][]byte
	beilagenHash string
}

// Gerendert ist ein Frame im Cache mit den Teil-Boxen.
type Gerendert struct {
	Datei string
	Teile map[string][]int
}

// leser liefert eine Datei eines Stands; ok false, wenn sie fehlt.
type leser func(pfad string) ([]byte, bool, error)

func neueQuelle(pfad string, spec, stil []byte, basis bool, specs []string, lies leser) (Quelle, error) {
	sh, err := SpecHash(spec)
	if err != nil {
		return Quelle{}, &SpecUnlesbar{Pfad: pfad, Err: err}
	}
	th, err := SpecHash(stil)
	if err != nil {
		return Quelle{}, fmt.Errorf("%s: %w", stilPfad, err)
	}
	q := Quelle{Pfad: pfad, Spec: spec, Stil: stil, SpecHash: sh, StilHash: th, Basis: basis}
	q.Beilagen, q.beilagenHash, err = beilagen(spec, specs, lies)
	return q, err
}

// reittiere liefert die Reittier-IDs aus animationen[*].reittier.
func reittiere(spec []byte) []string {
	var s struct {
		Animationen map[string]struct{ Reittier string } `json:"animationen"`
	}
	_ = json.Unmarshal(spec, &s)
	var ids []string
	for _, a := range s.Animationen {
		if a.Reittier != "" && !slices.Contains(ids, a.Reittier) {
			ids = append(ids, a.Reittier)
		}
	}
	slices.Sort(ids)
	return ids
}

// beilagen sucht je Reittier <id>.paper.json unter den Specs des Stands; ein fehlendes bleibt fehlend (die CLI meldet es).
func beilagen(spec []byte, specs []string, lies leser) (map[string][]byte, string, error) {
	ids := reittiere(spec)
	if len(ids) == 0 || lies == nil {
		return nil, "", nil
	}
	out := map[string][]byte{}
	var schluessel strings.Builder
	for _, id := range ids {
		for _, p := range specs {
			if path.Base(p) != id+endung {
				continue
			}
			raw, ok, err := lies(p)
			if err != nil {
				return nil, "", err
			}
			if ok {
				out[p] = raw
				schluessel.WriteString(p + ":" + sha(string(raw)) + ";")
			}
			break
		}
	}
	return out, sha(schluessel.String()), nil
}

// QuelleBasis liest Spec und Stil-Datei aus dem Commit; fehlt die Spec dort, ist ok false.
func QuelleBasis(ctx context.Context, root, commit, pfad string) (Quelle, bool, error) {
	if !istSpec(pfad) {
		return Quelle{}, false, fmt.Errorf("%s ist keine Spec", pfad)
	}
	spec, ok, err := Show(ctx, root, commit, pfad)
	if err != nil || !ok {
		return Quelle{}, false, err
	}
	stil, ok, err := Show(ctx, root, commit, stilPfad)
	if err != nil {
		return Quelle{}, false, err
	}
	if !ok {
		return Quelle{}, false, fmt.Errorf("%s fehlt in %s", stilPfad, commit)
	}
	var specs []string
	if len(reittiere(spec)) > 0 {
		if specs, err = SpecsInCommit(ctx, root, commit); err != nil {
			return Quelle{}, false, err
		}
	}
	q, err := neueQuelle(pfad, spec, stil, true, specs, func(p string) ([]byte, bool, error) { return Show(ctx, root, commit, p) })
	return q, err == nil, err
}

// QuelleArbeitsstand liest Spec und Stil-Datei aus dem Checkout; fehlt die Spec, ist ok false.
func QuelleArbeitsstand(root, pfad string) (Quelle, bool, error) {
	if !istSpec(pfad) {
		return Quelle{}, false, fmt.Errorf("%s ist keine Spec", pfad)
	}
	spec, ok, err := leseArbeitsstand(root, pfad)
	if err != nil || !ok {
		return Quelle{}, false, err
	}
	stil, ok, err := leseArbeitsstand(root, stilPfad)
	if err != nil {
		return Quelle{}, false, err
	}
	if !ok {
		return Quelle{}, false, fmt.Errorf("%s fehlt", stilPfad)
	}
	var specs []string
	if len(reittiere(spec)) > 0 {
		if specs, err = SpecsImArbeitsstand(root); err != nil {
			return Quelle{}, false, err
		}
	}
	q, err := neueQuelle(pfad, spec, stil, false, specs, func(p string) ([]byte, bool, error) { return leseArbeitsstand(root, p) })
	return q, err == nil, err
}

// kennung ist die Renderer-Version im Cache-Schlüssel; die Attrappe hat eine eigene, damit ihre Bilder nach dem
// Wechsel auf die echte CLI (RD1) nicht weiter aus dem Cache kommen.
func kennung(r Renderer) string {
	if k, ok := r.(interface{ Kennung() string }); ok {
		return k.Kennung()
	}
	return RendererVersion
}

// Kennung ist die Renderer-Version, unter der der Cache Ergebnisse dieses Renderers ablegt.
func Kennung(r Renderer) string { return kennung(r) }

// ordner ist .cache/paper/<renderKey>/ (renderKey mit der Kennung des Renderers).
func (q Quelle) ordner(root string, r Renderer) string {
	spec := q.SpecHash
	if q.beilagenHash != "" {
		spec = sha(spec + "+" + q.beilagenHash) // ein geändertes Reittier ändert den Render des Reiters
	}
	return filepath.Join(root, ".cache", "paper", renderKeyMit(spec, q.StilHash, kennung(r)))
}

// schnappschuss legt Spec und Stil-Datei als Mini-Baum in den Cache (<ordner>/basis|stand/public/grafik/…): die CLI
// rendert so genau die gehashten Bytes, und Spec und Stil-Datei des Stands liegen zusammen.
func (q Quelle) schnappschuss(root string, r Renderer) (spec, stil string, err error) {
	art := "stand"
	if q.Basis {
		art = "basis"
	}
	baum := filepath.Join(q.ordner(root, r), art)
	spec = filepath.Join(baum, filepath.FromSlash(q.Pfad))
	stil = filepath.Join(baum, filepath.FromSlash(stilPfad))
	if err := schreibeWennNeu(spec, q.Spec); err != nil {
		return "", "", err
	}
	for p, raw := range q.Beilagen {
		if err := schreibeWennNeu(filepath.Join(baum, filepath.FromSlash(p)), raw); err != nil {
			return "", "", err
		}
	}
	return spec, stil, schreibeWennNeu(stil, q.Stil)
}

// SpecDatei liefert die Spec des Stands als Datei für die CLI (Schnappschuss im Cache).
func (q Quelle) SpecDatei(root string, r Renderer) (string, error) {
	spec, _, err := q.schnappschuss(root, r)
	return spec, err
}

// Bild rendert einen Frame in den Cache; liegt er schon dort, rendert es nicht neu.
func Bild(ctx context.Context, r Renderer, root string, q Quelle, animation string, frame int, variante string) (Gerendert, error) {
	if !gueltigerName(animation) || (variante != "" && !gueltigerName(variante)) || frame < 0 {
		return Gerendert{}, fmt.Errorf("Animation „%s“, Frame %d oder Variante „%s“ ungültig", animation, frame, variante) //nolint:staticcheck // Text der Oberfläche
	}
	name := animation + "-" + strconv.Itoa(frame)
	if variante != "" {
		name += "@" + variante
	}
	ordner := q.ordner(root, r)
	png, meta := filepath.Join(ordner, name+".png"), filepath.Join(ordner, name+".json")
	if g, ok := ausCache(png, meta); ok {
		return g, nil
	}
	spec, err := q.SpecDatei(root, r)
	if err != nil {
		return Gerendert{}, err
	}
	tmp, err := tempName(ordner, name+".png")
	if err != nil {
		return Gerendert{}, err
	}
	a, err := r.Frame(ctx, spec, animation, frame, variante, tmp)
	if err == nil {
		err = uebernehme(tmp, png)
	}
	if err != nil {
		_ = os.Remove(tmp)
		return Gerendert{}, err
	}
	raw, err := json.Marshal(a.Teile)
	if err != nil {
		return Gerendert{}, err
	}
	return Gerendert{Datei: png, Teile: a.Teile}, schreibeAtomar(meta, raw)
}

func ausCache(png, meta string) (Gerendert, bool) {
	if _, err := os.Stat(png); err != nil {
		return Gerendert{}, false
	}
	raw, err := os.ReadFile(meta)
	if err != nil {
		return Gerendert{}, false
	}
	var teile map[string][]int
	if json.Unmarshal(raw, &teile) != nil {
		return Gerendert{}, false
	}
	return Gerendert{Datei: png, Teile: teile}, true
}

// tempName reserviert einen eindeutigen Temp-Namen im Ordner (je Aufruf, auch zwischen Goroutinen); die CLI schreibt
// dorthin, danach wird umbenannt.
func tempName(ordner, name string) (string, error) {
	if err := os.MkdirAll(ordner, 0o755); err != nil {
		return "", err
	}
	f, err := os.CreateTemp(ordner, name+".tmp-*"+filepath.Ext(name))
	if err != nil {
		return "", err
	}
	return f.Name(), f.Close()
}

// schreibeWennNeu schreibt atomar, falls die Datei fehlt (Inhalt ist durch den Ordnernamen festgelegt).
func schreibeWennNeu(pfad string, raw []byte) error {
	if _, err := os.Stat(pfad); err == nil {
		return nil
	}
	return schreibeAtomar(pfad, raw)
}

// schreibeAtomar schreibt per Temp-Datei und Umbenennen; ein Abbruch hinterlässt keinen halben Eintrag.
func schreibeAtomar(pfad string, raw []byte) error {
	if err := os.MkdirAll(filepath.Dir(pfad), 0o755); err != nil {
		return err
	}
	f, err := os.CreateTemp(filepath.Dir(pfad), filepath.Base(pfad)+".tmp-*")
	if err != nil {
		return err
	}
	if _, err := f.Write(raw); err != nil {
		_ = f.Close()
		_ = os.Remove(f.Name())
		return err
	}
	if err := f.Close(); err != nil {
		_ = os.Remove(f.Name())
		return err
	}
	return uebernehme(f.Name(), pfad)
}

// uebernehme benennt tmp in ziel um. Unter Windows scheitert das, solange ein paralleler Aufruf ziel offen hält; da
// der Inhalt im Cache durch den Schlüssel festgelegt ist, genügt dann die vorhandene Datei.
func uebernehme(tmp, ziel string) error {
	err := os.Rename(tmp, ziel)
	if err != nil {
		if _, statErr := os.Stat(ziel); statErr == nil {
			_ = os.Remove(tmp)
			return nil
		}
	}
	return err
}
