package world

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
)

// Endboss je Insel und Skins der Welt (WE6.4, B-981, B-984): beides geht über den Patch von Set, damit gelten die
// gleichen Prüfungen wie bei world_set (Engine-Check, Format, Atomarität). Die Endboss-Liste steht in data/bosses.json.

// EndBossOption ist ein wählbarer Endboss (ID und Anzeigename).
type EndBossOption struct {
	ID   string `json:"id"`
	Name string `json:"name"`
}

// endBossOptions liest data/bosses.json › endBosses; fehlt die Datei, ist die Liste leer.
func (e Editor) endBossOptions() []EndBossOption {
	raw, err := os.ReadFile(filepath.Join(e.Root, "data", "bosses.json"))
	if err != nil {
		return []EndBossOption{}
	}
	var doc struct {
		EndBosses []EndBossOption `json:"endBosses"`
	}
	if json.Unmarshal(raw, &doc) != nil || doc.EndBosses == nil {
		return []EndBossOption{}
	}
	return doc.EndBosses
}

// SetIslandEndBoss setzt den Endboss der Insel island; leer entfernt das Feld (Endboss der tiefsten Stufe).
func (e Editor) SetIslandEndBoss(ctx context.Context, id, island, boss string) CreateView {
	entry := map[string]any{"id": island, "endBoss": nil}
	if boss != "" {
		entry["endBoss"] = boss
	}
	return e.setPatch(ctx, id, map[string]any{"islands": []any{entry}})
}

// SetSkins ersetzt die Skins der Welt; eine leere Auswahl entfernt das Feld.
func (e Editor) SetSkins(ctx context.Context, id string, skins map[string]string) CreateView {
	var value any
	if len(skins) > 0 {
		value = skins
	}
	return e.setPatch(ctx, id, map[string]any{"skins": value})
}

// setPatch schickt einen Patch an Set und fasst den Fehler in der Antwort zusammen.
func (e Editor) setPatch(ctx context.Context, id string, patch map[string]any) CreateView {
	raw, err := json.Marshal(patch)
	if err != nil {
		return CreateView{ID: id, Error: err.Error()}
	}
	warnings, err := e.Set(ctx, id, raw)
	if err != nil {
		return CreateView{ID: id, Error: err.Error()}
	}
	return CreateView{ID: id, Warnings: warnings}
}
