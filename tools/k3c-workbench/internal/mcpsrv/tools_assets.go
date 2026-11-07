package mcpsrv

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	pa "k3c/tools/k3c-workbench/internal/paperassets"
)

// Asset-Tools auf Spec-Basis (Vertrag docs/standards/paper-mcp.md › MCP-Tools, MC1). Sie rufen die Render-CLI über
// internal/paperassets; bis RD1 zeigt K3C_PAPER_CLI auf die Attrappe tools/paper/fixtures/cli-stub.ts.

// assetGueltig merkt sich validate je Spec, solange Spec und Stil-Datei gleich bleiben (assets_list).
var assetGueltig = pa.NewGueltigkeit()

type assetsGuideIn struct {
	Rig string `json:"rig,omitempty" jsonschema:"Rig-Typ: figur, vierbeiner, gebaeude, requisite, hintergrund; leer = alle"`
}

type assetsListIn struct {
	Rig       string `json:"rig,omitempty" jsonschema:"nur dieser Rig-Typ"`
	Gruppe    string `json:"gruppe,omitempty" jsonschema:"nur diese Gruppe (Ordner unter public/grafik, z. B. figuren)"`
	Geaendert bool   `json:"geaendert,omitempty" jsonschema:"nur Assets mit Änderung gegen HEAD"`
}

type assetsGetIn struct {
	ID     string   `json:"id" jsonschema:"Asset-ID (Dateiname ohne .paper.json), z. B. monarch"`
	Felder []string `json:"felder,omitempty" jsonschema:"nur diese obersten Schlüssel, z. B. [umhang, ausgabe]"`
}

type assetsCreateIn struct {
	ID           string `json:"id" jsonschema:"neue Asset-ID: a-z, 0-9, - (eindeutig über alle Gruppen)"`
	Rig          string `json:"rig" jsonschema:"figur, vierbeiner, gebaeude, requisite oder hintergrund"`
	Beschreibung string `json:"beschreibung" jsonschema:"was das Asset zeigt"`
	Gruppe       string `json:"gruppe" jsonschema:"Ordner unter public/grafik, z. B. requisiten"`
}

// assetsIDIn ist die Eingabe der Recherche-Tools (Slot des eingefrorenen Registers data/assets.json).
type assetsIDIn struct {
	ID string `json:"id" jsonschema:"Asset-Slot, z. B. farm, mount:horse, sfx:coin"`
}

// registerAssets registriert die Asset-Tools auf Spec-Basis (die Recherche-Tools: tools_assets_research.go).
func registerAssets(s *Server) {
	add(s, &mcp.Tool{
		Name: "assets_guide",
		Description: "Einstieg jeder Asset-Arbeit: Stil-Regeln, Papier-Materialien und Einzeltöne der Stil-Datei, Pflicht-Teile und Pivots " +
			"des Rig-Typs, Proportion, Lagen-Arten, Vorlage, Ablauf. Nutze es bei: bevor du ein Asset anlegst oder änderst. " +
			"Statt: paper-spec.md und Stil-Datei lesen.",
		Annotations: readOnly(),
	}, func(ctx context.Context, in assetsGuideIn) (string, error) {
		return assetsGuide(s.ws(ctx).root, in.Rig)
	})
	add(s, &mcp.Tool{
		Name: "assets_list",
		Description: "Alle Assets (Paper-Specs unter public/grafik) je Zeile: id · rig · pfad · gültig|ungültig[ · geändert]. " +
			"Nutze es bei: Überblick, ungültige oder geänderte Assets finden. Statt: public/grafik durchsuchen.",
		Annotations: readOnly(),
	}, func(ctx context.Context, in assetsListIn) (string, error) {
		return assetsListText(ctx, s.ws(ctx).root, in)
	})
	add(s, &mcp.Tool{
		Name: "assets_get",
		Description: "Pfad und Paper-Spec eines Assets als JSON; felder kürzt auf oberste Schlüssel. " +
			"Nutze es bei: vor einem assets_set. Statt: die .paper.json-Datei lesen.",
		Annotations: readOnly(),
	}, func(ctx context.Context, in assetsGetIn) (string, error) { return assetsGetText(s.ws(ctx).root, in) })
	notDestructive, closed := false, false
	add(s, &mcp.Tool{
		Name: "assets_create",
		Description: "Legt public/grafik/<gruppe>/<id>.paper.json aus der Rig-Vorlage an (Pflicht-Teile mit leeren Lagen), prüft und rendert. " +
			"Nutze es bei: neues Asset. Statt: Spec-Datei von Hand anlegen.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &notDestructive, OpenWorldHint: &closed},
	}, func(ctx context.Context, in assetsCreateIn) (string, error) {
		return assetsCreateText(ctx, s.ws(ctx).root, in)
	})
	registerAssetsSet(s)
	registerAssetsRender(s)
	registerAssetsPreview(s)
	registerAssetsReview(s)
	registerAssetPrompts(s)
}

// stilKurz sind die Teile der Stil-Datei, die guide und style_get zeigen (Feldnamen ohne Tag: json ignoriert Groß-/Kleinschreibung).
type stilKurz struct {
	Papier struct {
		Lage struct {
			Dx, Dy, Unschaerfe, Deckkraft float64
			Farbe                         string
		}
		Auflage  struct{ Dx, Dy, Unschaerfe, Deckkraft float64 }
		Struktur struct{ Frequenz, Oktaven, Staerke float64 }
		Falz     float64 `json:"falz_deckkraft"`
	}
	Licht        string
	Proportionen struct {
		Figur struct {
			KopfAnteil float64 `json:"kopf_anteil"`
			Toleranz   float64
		}
		Vierbeiner struct {
			Kopflaengen float64 `json:"rumpfhoehe_kopflaengen"`
			Toleranz    float64
		}
	}
}

func zahl(f float64) string { return strconv.FormatFloat(f, 'f', -1, 64) }

// leseStil liest die Stil-Datei roh und verdichtet.
func leseStil(root string) ([]byte, stilKurz, error) {
	var k stilKurz
	b, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(pa.StilPfad)))
	if err != nil {
		return nil, k, fmt.Errorf("Stil-Datei %s fehlt", pa.StilPfad) //nolint:staticcheck // Satz für die Antwort
	}
	if err := json.Unmarshal(b, &k); err != nil {
		return nil, k, fmt.Errorf("Stil-Datei: %v", err) //nolint:staticcheck // Satz für die Antwort
	}
	return b, k, nil
}

func assetsGuide(root, rig string) (string, error) {
	if rig != "" {
		if _, err := pa.FindeRig(rig); err != nil {
			return "", err
		}
	}
	roh, k, err := leseStil(root)
	if err != nil {
		return "", err
	}
	p := k.Papier
	var b strings.Builder
	titel := "alle Rigs"
	if rig != "" {
		titel = "Rig " + rig
	}
	fmt.Fprintf(&b, "Papierschnitt · %s · Standard docs/standards/paper-spec.md\n", titel)
	fmt.Fprintf(&b, "Stil: keine Konturen; jede Fläche ein Papierstück mit Schatten papier.lage (%s/%s, Unschärfe %s, %s %%); "+
		"Schattierungen als aufgeklebte Lage (auflage: true, %s/%s/%s/%s %%); Innenlinien nur als Falz (Deckkraft %s); Licht von %s; "+
		"keine Farben in der Spec, nur Papier-Materialien.\n", zahl(p.Lage.Dx), zahl(p.Lage.Dy), zahl(p.Lage.Unschaerfe),
		zahl(p.Lage.Deckkraft*100), zahl(p.Auflage.Dx), zahl(p.Auflage.Dy), zahl(p.Auflage.Unschaerfe), zahl(p.Auflage.Deckkraft*100),
		zahl(p.Falz), strings.ReplaceAll(k.Licht, "-", " "))
	fmt.Fprintf(&b, "Papier-Materialien (licht/basis/schatten): %s\n", strings.Join(pa.Schluessel(roh, "materialien"), ", "))
	fmt.Fprintf(&b, "Einzeltöne (ohne Ton): %s\n", strings.Join(pa.Schluessel(roh, "einzeltoene"), ", "))
	rigs := pa.RigNamen()
	if rig != "" {
		rigs = []string{rig}
	}
	for _, name := range rigs {
		guideRig(&b, name, rig != "", k)
	}
	b.WriteString("Lagen: flaeche {form, material, ton?, auflage?} · falz {form, breite, material, ton?} · punkt {mitte, radius, material, ton?}\n")
	if rig != "" {
		vorlage, _ := pa.Vorlage(rig, "beispiel", "…")
		var kompakt bytes.Buffer
		_ = json.Compact(&kompakt, vorlage)
		fmt.Fprintf(&b, "Vorlage %s: %s\n", rig, kompakt.String())
	}
	b.WriteString("Beispiele (vollständig): tools/paper/fixtures/specs/monarch.paper.json (figur), pferd.paper.json (vierbeiner), mauer.paper.json (gebaeude)\n")
	b.WriteString("Ablauf: assets_get → assets_set (Merge-Patch) → assets_preview → assets_review")
	return b.String(), nil
}

func guideRig(b *strings.Builder, name string, einzeln bool, k stilKurz) {
	r := pa.Rigs[name]
	pflicht := strings.Join(r.Pflicht, ", ")
	if pflicht == "" {
		pflicht = "mindestens ein Teil (Vorlage: " + r.Platzhalter + ")"
	}
	fmt.Fprintf(b, "Pflicht-Teile %s: %s\n", name, pflicht)
	if einzeln {
		fmt.Fprintf(b, "Pivots: %s\n", r.PivotText)
	} else {
		fmt.Fprintf(b, "Pivots %s: %s\n", name, r.PivotText)
	}
	switch name {
	case "figur":
		fmt.Fprintf(b, "Proportion: Kopfhöhe / Rahmenhöhe = %s ± %s\n", zahl(k.Proportionen.Figur.KopfAnteil), zahl(k.Proportionen.Figur.Toleranz))
	case "vierbeiner":
		fmt.Fprintf(b, "Proportion: Rumpfhöhe = %s Kopflängen ± %s\n", zahl(k.Proportionen.Vierbeiner.Kopflaengen), zahl(k.Proportionen.Vierbeiner.Toleranz))
	}
}

func mehrzahl(n int, eins, viele string) string {
	if n == 1 {
		return eins
	}
	return viele
}

func assetsListText(ctx context.Context, root string, in assetsListIn) (string, error) {
	alle, err := pa.Assets(root)
	if err != nil {
		return "", err
	}
	geaendert, err := pa.Geaendert(ctx, root)
	if err != nil {
		return "", err
	}
	var liste []pa.Asset
	for _, a := range alle {
		if (in.Rig != "" && a.Rig != in.Rig) || (in.Gruppe != "" && a.Gruppe != in.Gruppe) || (in.Geaendert && !geaendert[a.ID]) {
			continue
		}
		liste = append(liste, a)
	}
	if len(liste) == 0 {
		return "keine Assets für diesen Filter", nil
	}
	fehler := assetGueltig.Pruefe(ctx, pa.NewCLI(root), liste)
	var b strings.Builder
	ungueltig, nGeaendert := 0, 0
	for _, a := range liste {
		zustand := "gültig"
		if len(fehler[a.ID]) > 0 {
			zustand = "ungültig"
			ungueltig++
		}
		if geaendert[a.ID] {
			zustand += " · geändert"
			nGeaendert++
		}
		fmt.Fprintf(&b, "%s · %s · %s · %s\n", a.ID, a.Rig, a.Pfad, zustand)
	}
	return fmt.Sprintf("%d %s · %d ungültig · %d geändert gegen HEAD\n%s", len(liste), mehrzahl(len(liste), "Asset", "Assets"),
		ungueltig, nGeaendert, strings.TrimSuffix(b.String(), "\n")), nil
}

func assetsGetText(root string, in assetsGetIn) (string, error) {
	a, err := pa.Finde(root, in.ID)
	if err != nil {
		return "", err
	}
	roh, err := os.ReadFile(a.Abs(root))
	if err != nil {
		return "", err
	}
	auswahl, err := pa.Felder(roh, in.Felder)
	if err != nil {
		return "", err
	}
	text, err := pa.Format(auswahl)
	if err != nil {
		return "", err
	}
	kopf := a.Pfad
	if len(in.Felder) > 0 {
		kopf += " · Felder " + strings.Join(in.Felder, ", ")
	}
	return kopf + "\n" + strings.TrimSuffix(string(text), "\n"), nil
}

func assetsCreateText(ctx context.Context, root string, in assetsCreateIn) (string, error) {
	r, err := pa.Anlegen(ctx, pa.NewCLI(root), in.ID, in.Rig, in.Beschreibung, in.Gruppe)
	if err != nil {
		return "", err
	}
	var b strings.Builder
	fmt.Fprintf(&b, "%s angelegt: %s (Rig %s, Vorlage mit %s %s)\n", r.Asset.ID, r.Asset.Pfad, r.Asset.Rig,
		mehrzahl(len(r.Teile), "Teil", "Teilen"), strings.Join(r.Teile, ", "))
	if len(r.Fehler) == 0 {
		b.WriteString("validate: ok\n")
		b.WriteString(renderZeile(r.Render) + "\n")
	} else {
		fmt.Fprintf(&b, "validate: %d %s\n%s\nRender: übersprungen (ungültig)\n", len(r.Fehler), mehrzahl(len(r.Fehler), "Fehler", "Fehler"), fehlerZeilen(r.Fehler))
	}
	fmt.Fprintf(&b, "Nächster Schritt: assets_set {id: %q, patch: …}, dann assets_preview", r.Asset.ID)
	return b.String(), nil
}

// renderZeile fasst das Render-Ergebnis eines Assets zusammen.
func renderZeile(e pa.AssetErgebnis) string {
	switch {
	case e.Fehler != "":
		return "Render: Fehler · " + e.Fehler
	case e.Uebersprungen != "":
		return "Render: übersprungen (ungültig: " + e.Uebersprungen + ")"
	}
	t := fmt.Sprintf("Render: %d %s", e.PNGs, mehrzahl(e.PNGs, "PNG", "PNGs"))
	switch {
	case e.Manifest != "":
		t += " · " + e.Manifest
	case len(e.Dateien) > 0:
		t += " · " + e.Dateien[0]
	}
	return t
}

// fehlerZeilen schreibt je Fehler `feld · regel · text`.
func fehlerZeilen(fehler []pa.Fehler) string {
	z := make([]string, len(fehler))
	for i, f := range fehler {
		z[i] = f.Feld + " · " + f.Regel + " · " + f.Text
	}
	return strings.Join(z, "\n")
}
