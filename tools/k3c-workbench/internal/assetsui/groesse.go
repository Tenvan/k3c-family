package assetsui

import (
	"encoding/json"
	"errors"
	"fmt"
	"maps"
	"os"
	"path"
	"path/filepath"
	"regexp"
	"slices"
	"strings"
)

// Größenvergleich (B-733): Spiel-Pixel je Einheit eines Assets und die Hintergrund-Ebenen der Biome. Die Regeln folgen dem
// Client (`src/scenes/render/sprites.ts` › targetHeight, `paperAssets.ts` › paperScale): eine Figur mit Spielrolle wird
// so skaliert, dass ihr Rahmen über dem Fuß `height · scale` Pixel hoch ist; Gebäude, Objekte und Hintergründe zeichnet
// das Spiel in Originalgröße (1 px je Einheit).

// Groesse ist der Maßstab eines Assets im Spiel.
type Groesse struct {
	// PxProEinheit: Spiel-Pixel je Einheit der Spec (Rahmen `ausgabe.rahmen`).
	PxProEinheit float64 `json:"pxProEinheit"`
	// Quelle sagt, woher der Maßstab kommt (Text der Oberfläche).
	Quelle string `json:"quelle"`
}

// figurGruppen sind Gruppen, deren Assets wie Figuren skaliert werden; ohne Spielrolle gilt der Maßstab des Monarchen.
var figurGruppen = []string{"figuren", "gegner", "truppen", "bosse", "reittiere"}

const monarchRef = "figuren/monarch"

// AssetsGroesse liefert den Spiel-Maßstab eines Assets.
func (d Dienst) AssetsGroesse(id string) (Groesse, error) {
	ctx, cancel := kontext()
	defer cancel()
	p, err := d.pfadZu(ctx, id, "")
	if err != nil {
		return Groesse{}, err
	}
	ref := strings.Split(p, "/")[2] + "/" + idVon(p)
	rollen, err := d.rollenHoehen()
	if err != nil {
		return Groesse{}, err
	}
	if r, ok := rollen[ref]; ok {
		return d.figurGroesse(p, r.hoehe, fmt.Sprintf("Spielrolle %s (data/sprites.json)", r.rolle))
	}
	if !slices.Contains(figurGruppen, strings.Split(ref, "/")[0]) {
		return Groesse{PxProEinheit: 1, Quelle: "1 px je Einheit wie im Spiel (Gebäude, Objekte, Hintergründe)"}, nil
	}
	m, ok := rollen[monarchRef]
	if !ok {
		return Groesse{}, errors.New("Spielrolle des Monarchen fehlt in data/paper.json") //nolint:staticcheck // Text der Oberfläche
	}
	mp, err := d.pfadZu(ctx, idVon(monarchRef), "")
	if err != nil {
		return Groesse{}, err
	}
	return d.figurGroesse(mp, m.hoehe, "Maßstab des Monarchen (keine Spielrolle)")
}

// figurGroesse: Zielhöhe in Spiel-Pixeln geteilt durch die Höhe des Rahmens über dem Fuß.
func (d Dienst) figurGroesse(pfad string, hoehe float64, quelle string) (Groesse, error) {
	var s struct {
		Ausgabe struct{ Rahmen []float64 } `json:"ausgabe"`
	}
	raw, err := os.ReadFile(filepath.Join(d.Root, filepath.FromSlash(pfad)))
	if err == nil {
		err = json.Unmarshal(raw, &s)
	}
	if err != nil {
		return Groesse{}, err
	}
	if len(s.Ausgabe.Rahmen) != 4 || s.Ausgabe.Rahmen[1] >= 0 {
		return Groesse{}, fmt.Errorf("%s: ausgabe.rahmen fehlt oder liegt nicht über dem Fuß", pfad)
	}
	return Groesse{PxProEinheit: hoehe / -s.Ausgabe.Rahmen[1], Quelle: quelle}, nil
}

type rolle struct {
	rolle string
	hoehe float64
}

// rollenHoehen ordnet jedem Asset (`gruppe/id`) mit Spielrolle die Figurhöhe in Spiel-Pixeln zu (data/paper.json ›
// spieler, truppen, gegner; data/sprites.json › sheets).
func (d Dienst) rollenHoehen() (map[string]rolle, error) {
	var paper struct {
		Spieler []string          `json:"spieler"`
		Truppen map[string]string `json:"truppen"`
		Gegner  map[string]string `json:"gegner"`
	}
	type figur struct {
		Sheet string   `json:"sheet"`
		Scale *float64 `json:"scale"`
	}
	var sprites struct {
		Sheets  map[string]struct{ Height, Scale float64 } `json:"sheets"`
		Players []figur                                    `json:"players"`
		Troops  map[string]figur                           `json:"troops"`
		Enemies map[string]figur                           `json:"enemies"`
	}
	if err := d.leseJSON("data/paper.json", &paper); err != nil {
		return nil, err
	}
	if err := d.leseJSON("data/sprites.json", &sprites); err != nil {
		return nil, err
	}
	out := map[string]rolle{}
	setze := func(ref, name string, f figur) {
		ref = strings.Split(ref, "@")[0]
		sh, ok := sprites.Sheets[f.Sheet]
		if _, schon := out[ref]; schon || !ok {
			return
		}
		h := sh.Height * sh.Scale
		if f.Scale != nil {
			h *= *f.Scale
		}
		out[ref] = rolle{rolle: name, hoehe: h}
	}
	for i, ref := range paper.Spieler {
		if i < len(sprites.Players) {
			setze(ref, "Spieler", sprites.Players[i])
		}
	}
	// sortiert: zeigen zwei Rollen auf dasselbe Asset, gewinnt immer dieselbe
	for _, name := range slices.Sorted(maps.Keys(paper.Truppen)) {
		setze(paper.Truppen[name], name, sprites.Troops[name])
	}
	for _, name := range slices.Sorted(maps.Keys(paper.Gegner)) {
		setze(paper.Gegner[name], name, sprites.Enemies[name])
	}
	return out, nil
}

func (d Dienst) leseJSON(datei string, ziel any) error {
	raw, err := os.ReadFile(filepath.Join(d.Root, filepath.FromSlash(datei)))
	if err != nil {
		return err
	}
	return json.Unmarshal(raw, ziel)
}

// Hintergrund sind die drei Ebenen eines Biom-Hintergrunds (Variante 1) als Data-URLs, hinten nach vorn.
type Hintergrund struct {
	Fern  string `json:"fern"`
	Mitte string `json:"mitte"`
	Nah   string `json:"nah"`
}

const hintergrundOrdner = "public/grafik/hintergruende"

var hgName = regexp.MustCompile(`^hg-([a-z]+)-nah-v1$`)

// AssetsHintergruende listet die Biome mit Hintergrund (Namensteil `hg-<teil>-…`, z. B. wald, hoehle).
func (d Dienst) AssetsHintergruende() ([]string, error) {
	eintraege, err := os.ReadDir(filepath.Join(d.Root, filepath.FromSlash(hintergrundOrdner)))
	if err != nil {
		return nil, err
	}
	teile := []string{}
	for _, e := range eintraege {
		if m := hgName.FindStringSubmatch(e.Name()); m != nil && e.IsDir() {
			teile = append(teile, m[1])
		}
	}
	return teile, nil
}

// AssetsHintergrund liefert die Ebenen des Biom-Hintergrunds `teil` (aus AssetsHintergruende).
func (d Dienst) AssetsHintergrund(teil string) (Hintergrund, error) {
	teile, err := d.AssetsHintergruende()
	if err != nil {
		return Hintergrund{}, err
	}
	if !slices.Contains(teile, teil) {
		return Hintergrund{}, fmt.Errorf("Hintergrund „%s“ fehlt", teil) //nolint:staticcheck // Text der Oberfläche
	}
	var h Hintergrund
	for _, e := range []struct {
		ebene string
		ziel  *string
	}{{"fern", &h.Fern}, {"mitte", &h.Mitte}, {"nah", &h.Nah}} {
		datei := path.Join(hintergrundOrdner, "hg-"+teil+"-"+e.ebene+"-v1", "stand.png")
		if *e.ziel, err = dataURL(filepath.Join(d.Root, filepath.FromSlash(datei))); err != nil {
			return Hintergrund{}, err
		}
	}
	return h, nil
}
