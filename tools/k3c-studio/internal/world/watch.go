package world

import (
	"context"
	"io/fs"
	"os"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
	"time"
)

// Changed ist die Nutzlast des Ereignisses `world:changed` (B-1216): die IDs der Welten, deren Datei sich geändert
// hat. Fehlt ids, hat sich eine Basis-Datei unter data/ geändert (Werte aller Welten) oder die Lage ist unklar: die
// Oberfläche lädt dann jede Welt neu, auch die gewählte.
type Changed struct {
	IDs []string `json:"ids,omitempty"`
}

// base ist der Schlüssel eines Snapshots für Dateien außerhalb von data/worlds/ (Basis-Werte, Biome).
const base = "*"

// Snapshot ist der Stempel je Datei (Größe und Änderungszeit): `<id>` für data/worlds/<id>.json, sonst `*<pfad>`
// für die Basis-Dateien data/*.json und data/biomes/*.json, aus denen die Werte der Welten kommen. Gelesen werden nur
// Metadaten.
func Snapshot(root string) map[string]string {
	snap := map[string]string{}
	stamp := func(info fs.FileInfo) string {
		return strconv.FormatInt(info.Size(), 10) + "|" + strconv.FormatInt(info.ModTime().UnixNano(), 10)
	}
	walk := func(dir string, key func(name string) string) {
		entries, _ := os.ReadDir(filepath.Join(root, dir))
		for _, d := range entries {
			if d.IsDir() || !strings.HasSuffix(d.Name(), ".json") {
				continue
			}
			if info, err := d.Info(); err == nil {
				snap[key(d.Name())] = stamp(info)
			}
		}
	}
	walk(worldsDir, func(n string) string { return strings.TrimSuffix(n, ".json") })
	walk("data", func(n string) string { return base + "data/" + n })
	walk("data/biomes", func(n string) string { return base + "data/biomes/" + n })
	return snap
}

// ChangedIDs nennt die Welt-IDs, deren Stempel sich zwischen `old` und `cur` geändert hat (neu, gelöscht, anders), sortiert.
// Hat sich eine Basis-Datei geändert, ist die Antwort leer und `all` wahr.
func ChangedIDs(old, cur map[string]string) (ids []string, all bool) {
	seen := map[string]bool{}
	mark := func(a, b map[string]string) {
		for k, v := range a {
			if b[k] == v {
				continue
			}
			if strings.HasPrefix(k, base) {
				all = true
			} else {
				seen[k] = true
			}
		}
	}
	mark(cur, old)
	mark(old, cur)
	for id := range seen {
		ids = append(ids, id)
	}
	sort.Strings(ids)
	if all {
		return nil, true
	}
	return ids, false
}

// Watch fragt alle `every` die Dateien der Welten ab und ruft `onChange` mit den geänderten IDs (leer: alle).
// Polling statt Dateisystem-Ereignissen wie beim Planungs-Wächter (keine neue Abhängigkeit, ein paar Stat-Aufrufe).
func Watch(ctx context.Context, root string, every time.Duration, onChange func(ids []string)) {
	last := Snapshot(root)
	go func() {
		t := time.NewTicker(every)
		defer t.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-t.C:
				cur := Snapshot(root)
				if ids, all := ChangedIDs(last, cur); all || len(ids) > 0 {
					last = cur
					if all {
						ids = nil
					}
					onChange(ids)
				}
			}
		}
	}()
}
