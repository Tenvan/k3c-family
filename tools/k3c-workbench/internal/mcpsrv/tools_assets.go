package mcpsrv

import (
	"context"
	"fmt"
	"strings"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	assets "k3c/tools/assetkit"
)

type assetsListIn struct {
	Art       string `json:"art,omitempty" jsonschema:"grafik oder sound; leer = beide"`
	Kategorie string `json:"kategorie,omitempty" jsonschema:"z. B. gebaeude, figuren, sfx, musik"`
	Status    string `json:"status,omitempty" jsonschema:"zugeordnet, luecke oder fehlt"`
}

type assetsIDIn struct {
	ID string `json:"id" jsonschema:"Asset-Slot, z. B. farm, mount:horse, sfx:coin"`
}

type assetsSetIn struct {
	ID   string          `json:"id" jsonschema:"Asset-Slot, z. B. farm"`
	Wahl []assets.Choice `json:"wahl" jsonschema:"Zuordnungen nach Präferenz: datei (relativ zu public/), optional sheet, ausschnitt {x,y,w,h}, raster {w,h}, sprite"`
}

type assetsCatalogIn struct {
	Query string `json:"query,omitempty" jsonschema:"Wörter, die alle in Datei, Pack, Gruppe, Titel oder Lizenz vorkommen"`
	Art   string `json:"art,omitempty" jsonschema:"grafik oder sound; leer = beide"`
	Limit int    `json:"limit,omitempty" jsonschema:"höchstens so viele Treffer (Vorgabe 50)"`
}

// registerAssets sind die Tools des Ressourcen-Managers (B-298): Slot-Register data/assets.json und Asset-Katalog.
func registerAssets(s *Server) {
	closed, destructive := false, false
	add(s, &mcp.Tool{
		Name: "assets_list",
		Description: "Asset-Slots des Slot-Registers (data/assets.json) als je eine Zeile: ID, Kategorie, Status, aktive Zuordnung. " +
			"Nutze es bei: welche Grafik- oder Sound-Slots gibt es, wo fehlt etwas. Statt: data/assets.json oder docs/assets lesen.",
		Annotations: readOnly(),
	}, func(ctx context.Context, in assetsListIn) (string, error) { return assetsList(s.ws(ctx).root, in) })
	add(s, &mcp.Tool{
		Name: "assets_get",
		Description: "Ein Asset-Slot mit allen Zuordnungen nach Präferenz, Ausschnitt und Status. " +
			"Nutze es bei: Zuordnung eines Slots prüfen. Statt: data/assets.json durchsuchen.",
		Annotations: readOnly(),
	}, func(ctx context.Context, in assetsIDIn) (string, error) { return assetsGet(s.ws(ctx).root, in.ID) })
	add(s, &mcp.Tool{
		Name: "assets_set",
		Description: "Ersetzt die Zuordnungen eines Asset-Slots (Reihenfolge = Präferenz, nur die erste ist im Spiel aktiv); " +
			"prüft Datei unter public/, Art und Ausschnitt. Nutze es bei: Zuordnung setzen oder umordnen. Statt: data/assets.json von Hand ändern.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &destructive, OpenWorldHint: &closed},
	}, func(ctx context.Context, in assetsSetIn) (string, error) {
		root := s.ws(ctx).root
		slot, err := assets.SetChoices(root, in.ID, in.Wahl)
		if err != nil {
			return "", err
		}
		return fmt.Sprintf("%s gespeichert: %d Zuordnung(en)\n%s", slot.ID, len(slot.Choices), slotText(root, *slot)), nil
	})
	add(s, &mcp.Tool{
		Name: "assets_catalog",
		Description: "Sucht im Asset-Katalog (public/grafik, public/sprites, public/audio): Datei, Pack, Gruppe, Maße, Lizenz mit Kennzeichen NC/SA/ND (Q70). " +
			"Nutze es bei: passende Datei für einen Slot finden. Statt: public/ durchsuchen und CREDITS lesen.",
		Annotations: readOnly(),
	}, func(ctx context.Context, in assetsCatalogIn) (string, error) {
		return assetsCatalog(s.ws(ctx).root, in)
	})
}

func assetsList(root string, in assetsListIn) (string, error) {
	reg, err := assets.Load(root)
	if err != nil {
		return "", err
	}
	var b strings.Builder
	n := 0
	for _, sl := range reg.Slots {
		st := assets.Status(root, sl)
		if (in.Art != "" && sl.Kind != in.Art) || (in.Kategorie != "" && sl.Category != in.Kategorie) || (in.Status != "" && st != in.Status) {
			continue
		}
		n++
		active := "–"
		if len(sl.Choices) > 0 {
			active = choiceText(sl.Choices[0])
			if more := len(sl.Choices) - 1; more > 0 {
				active += fmt.Sprintf(" (+%d)", more)
			}
		}
		fmt.Fprintf(&b, "%s %s %s · %s → %s\n", sl.ID, sl.Category, st, sl.Description, active)
	}
	if n == 0 {
		return "keine Asset-Slots für diesen Filter", nil
	}
	return fmt.Sprintf("%d Asset-Slots\n%s", n, b.String()), nil
}

func assetsGet(root, id string) (string, error) {
	reg, err := assets.Load(root)
	if err != nil {
		return "", err
	}
	sl := reg.Find(id)
	if sl == nil {
		return "", fmt.Errorf("unbekannter Asset-Slot %q", id)
	}
	return slotText(root, *sl), nil
}

func slotText(root string, sl assets.Slot) string {
	var b strings.Builder
	fmt.Fprintf(&b, "%s · %s/%s · %s · %s", sl.ID, sl.Kind, sl.Category, sl.Description, assets.Status(root, sl))
	if sl.Bus != "" {
		fmt.Fprintf(&b, " · Bus %s", sl.Bus)
	}
	for i, c := range sl.Choices {
		fmt.Fprintf(&b, "\n%d. %s", i+1, choiceText(c))
	}
	return b.String()
}

func choiceText(c assets.Choice) string {
	t := c.File
	if c.Sprite != "" {
		t += "#" + c.Sprite
	}
	if c.Sheet != "" {
		t += " sheet " + c.Sheet
	}
	if r := c.Cut; r != nil {
		t += fmt.Sprintf(" [%d,%d,%d,%d]", r.X, r.Y, r.W, r.H)
	}
	if g := c.Grid; g != nil {
		t += fmt.Sprintf(" raster %d×%d", g.W, g.H)
	}
	return t
}

func assetsCatalog(root string, in assetsCatalogIn) (string, error) {
	list, err := assets.Catalog(root)
	if err != nil {
		return "", err
	}
	limit := in.Limit
	if limit <= 0 {
		limit = 50
	}
	hits := assets.Search(list, in.Query, in.Art, limit)
	if len(hits) == 0 {
		return "keine Treffer im Asset-Katalog", nil
	}
	var b strings.Builder
	fmt.Fprintf(&b, "%d Treffer (höchstens %d)\n", len(hits), limit)
	for _, a := range hits {
		size := ""
		if a.W > 0 {
			size = fmt.Sprintf(" · %d×%d", a.W, a.H)
		}
		flag := ""
		if a.Flag != "" {
			flag = " [" + a.Flag + "]"
		}
		fmt.Fprintf(&b, "%s · %s %s%s · %s%s\n", a.File, a.Pack, a.Group, size, a.License, flag)
	}
	return b.String(), nil
}
