package planning

import (
	"context"
	"hash/fnv"
	"io/fs"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"time"
)

// maxChangedIDs: Ab so vielen geänderten Dateien nennt das Ereignis keine IDs mehr (Oberfläche lädt alles neu).
const maxChangedIDs = 50

// Changed ist die Nutzlast des Ereignisses `planning:changed`: die IDs der geänderten Dokumente (B-123, WZ61, WZ61.1,
// `plan`, `fragen`, `glossar`), sortiert und ohne Doppelte. Fehlt ids, ist die Lage unklar oder die Änderung groß:
// die Oberfläche lädt dann die ganze Liste neu.
type Changed struct {
	IDs []string `json:"ids,omitempty"`
}

// Snapshot ist der Stempel je Planungsdatei (Pfad relativ zur Wurzel mit `/` → Größe und Änderungszeit); Verzeichnisse
// der erledigten Sprints zählen nur über ihren Namen: ihre Änderungszeit ist unter Windows unzuverlässig (sie zieht
// verspätet nach), und nur die Zahl der erledigten Sprints ändert die Anzeige. Gelesen werden nur Metadaten.
func Snapshot(root string) map[string]string {
	snap := map[string]string{}
	docs := filepath.Join(root, "docs")
	rel := func(path string) string { r, _ := filepath.Rel(root, path); return filepath.ToSlash(r) }
	for _, dir := range []string{"sprints/aktiv", "sprints/geplant", "sprints/erledigt", "backlog", "projekte"} {
		top := filepath.Join(docs, filepath.FromSlash(dir))
		_ = filepath.WalkDir(top, func(path string, d fs.DirEntry, err error) error {
			if err != nil {
				return nil
			}
			sub := d.IsDir() && path != top
			if sub && (dir == "backlog" || dir == "projekte") {
				return filepath.SkipDir // archiv/ ändert die Anzeige nicht
			}
			if sub && strings.HasSuffix(dir, "erledigt") {
				snap[rel(path)+"/"] = ""
				return filepath.SkipDir
			}
			if info, err := d.Info(); err == nil && strings.HasSuffix(path, ".md") {
				snap[rel(path)] = strconv.FormatInt(info.Size(), 10) + "|" + strconv.FormatInt(info.ModTime().UnixNano(), 10)
			}
			return nil
		})
	}
	for _, file := range Docs {
		if info, err := os.Stat(filepath.Join(docs, file)); err == nil {
			snap["docs/"+file] = strconv.FormatInt(info.Size(), 10) + "|" + strconv.FormatInt(info.ModTime().UnixNano(), 10)
		}
	}
	return snap
}

// Stamp ist ein Fingerabdruck aller Planungsdateien (Snapshot): ändert sich irgendetwas daran, ändert sich der Wert;
// Dateien außerhalb der Planung zählen nicht.
func Stamp(root string) string { return stampOf(Snapshot(root)) }

func stampOf(snap map[string]string) string {
	keys := make([]string, 0, len(snap))
	for k := range snap {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	h := fnv.New64a()
	for _, k := range keys {
		_, _ = h.Write([]byte(k + "|" + snap[k] + "\n"))
	}
	return strconv.FormatUint(h.Sum64(), 16)
}

var (
	reChangedTicket  = regexp.MustCompile(`^(B-\d+)-.*\.md$`)
	reChangedProject = regexp.MustCompile(`^([A-Z][A-Z0-9]*)-.*\.md$`)
	reChangedSession = regexp.MustCompile(`^([A-Za-z]+\d+\.\d+[a-z]?)-.*\.md$`)
)

// ChangedIDs vergleicht zwei Snapshots und nennt die IDs der Dokumente, die neu, weg oder anders sind. Mehr als
// maxChangedIDs Dateien oder eine Datei ohne erkennbare ID ergeben nil (unklare Lage: alles neu laden).
func ChangedIDs(old, cur map[string]string) []string {
	files := map[string]bool{}
	for k, v := range cur {
		if o, ok := old[k]; !ok || o != v {
			files[k] = true
		}
	}
	for k := range old {
		if _, ok := cur[k]; !ok {
			files[k] = true
		}
	}
	if len(files) == 0 || len(files) > maxChangedIDs {
		return nil
	}
	set := map[string]bool{}
	for f := range files {
		if isIndexFile(f) {
			continue // Index-README: ändert keine Liste, kommt mit jeder Ticket-Änderung mit
		}
		id := idOfPath(f)
		if id == "" {
			return nil
		}
		set[id] = true
	}
	ids := make([]string, 0, len(set))
	for id := range set {
		ids = append(ids, id)
	}
	sort.Strings(ids)
	return ids
}

// isIndexFile erkennt die Index-READMEs von Backlog und Sprint-Status-Ordnern.
func isIndexFile(p string) bool {
	parts := strings.Split(p, "/")
	return (len(parts) == 3 && (parts[1] == "backlog" || parts[1] == "projekte") && parts[2] == "README.md") ||
		(len(parts) == 4 && parts[1] == "sprints" && parts[3] == "README.md")
}

// projectIDOf: `WZG-werkzeuge.md` → `WZG`; "" bei anderen Namen.
func projectIDOf(name string) string {
	if m := reChangedProject.FindStringSubmatch(name); m != nil {
		return m[1]
	}
	return ""
}

// idOfPath bildet einen Planungspfad (docs/…) auf die ID ab; "" bei unbekannter Form.
func idOfPath(p string) string {
	for name, file := range Docs {
		if p == "docs/"+file {
			return name
		}
	}
	parts := strings.Split(strings.TrimSuffix(p, "/"), "/")
	switch {
	case len(parts) == 3 && parts[1] == "backlog":
		if m := reChangedTicket.FindStringSubmatch(parts[2]); m != nil {
			return m[1]
		}
	case len(parts) == 3 && parts[1] == "projekte":
		return projectIDOf(parts[2])
	case len(parts) >= 4 && parts[1] == "sprints":
		sprint := strings.SplitN(parts[3], "-", 2)[0]
		if len(parts) == 4 {
			if strings.HasSuffix(p, "/") {
				return sprint // Ordner eines erledigten Sprints
			}
			return ""
		}
		if len(parts) == 5 {
			if m := reChangedSession.FindStringSubmatch(parts[4]); m != nil && !strings.EqualFold(parts[4], "README.md") {
				return m[1]
			}
			return sprint
		}
	}
	return ""
}

// WatchIDs prüft im Takt every, ob sich der Snapshot ändert, und ruft dann onChange mit den geänderten IDs (nil bei
// unklarer Lage, siehe ChangedIDs). Der Aufruf direkt nach dem Start gilt als Ausgangsstand und löst nichts aus. Endet
// ctx, endet die Goroutine.
//
// ponytail: Polling statt Dateisystem-Ereignissen; ein Stat über ein paar hundert Dateien alle 2 s kostet nichts, und
// fsnotify wäre eine neue Abhängigkeit. Bei sehr großem Backlog den Takt erhöhen.
func WatchIDs(ctx context.Context, root string, every time.Duration, onChange func(ids []string)) {
	last := Snapshot(root)
	lastStamp := stampOf(last)
	go func() {
		t := time.NewTicker(every)
		defer t.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-t.C:
				cur := Snapshot(root)
				if s := stampOf(cur); s != lastStamp {
					ids := ChangedIDs(last, cur)
					last, lastStamp = cur, s
					onChange(ids)
				}
			}
		}
	}()
}

// Watch ist WatchIDs ohne IDs.
func Watch(ctx context.Context, root string, every time.Duration, onChange func()) {
	WatchIDs(ctx, root, every, func([]string) { onChange() })
}
