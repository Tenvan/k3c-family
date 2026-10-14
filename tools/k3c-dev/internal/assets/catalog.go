package assets

import (
	"encoding/json"
	"image"
	"io/fs"
	"os"
	"path"
	"path/filepath"
	"sort"
	"strings"
)

// Asset ist eine Datei des Asset-Katalogs mit Pack, Gruppe (Tag), Maßen und Credit.
type Asset struct {
	File    string `json:"datei"` // relativ zu public/
	Kind    string `json:"art"`
	Pack    string `json:"pack"`
	Group   string `json:"gruppe,omitempty"`
	W       int    `json:"w,omitempty"`
	H       int    `json:"h,omitempty"`
	Title   string `json:"titel,omitempty"`
	Artist  string `json:"urheber,omitempty"`
	License string `json:"lizenz"`
	Source  string `json:"quelle,omitempty"`
	Flag    string `json:"kennzeichen,omitempty"`
}

// catalogDirs sind die Ordner unter public/ mit ihrer Credit-Datei.
var catalogDirs = []struct{ dir, credits string }{
	{"grafik", "grafik/CREDITS.md"},
	{"sprites", "sprites/CREDITS.md"},
	{"audio", "audio/README.md"},
	{"audio-test", "audio-test/README.md"},
}

// selfMade gilt für Sounds ohne Credit-Zeile: die Atlas- und Probe-Töne sind selbst erzeugt (public/audio/README.md).
// ponytail: Ordner-Annahme; ein fremder Sound braucht eine Zeile in der Credit-Tabelle (Import M10.5).
const selfMade = "selbst erzeugt (kein Credit nötig)"

// Catalog liest alle Bilder und Sounds unter public/ mit Pack, Gruppe, Maßen und Credit.
func Catalog(root string) ([]Asset, error) {
	pub := filepath.Join(root, "public")
	groups := grafikGroups(pub)
	var list []Asset
	for _, d := range catalogDirs {
		credits := readCredits(filepath.Join(pub, filepath.FromSlash(d.credits)))
		err := filepath.WalkDir(filepath.Join(pub, d.dir), func(p string, e fs.DirEntry, err error) error {
			if err != nil || e.IsDir() {
				return nil //nolint:nilerr // fehlender Ordner ist kein Fehler des Katalogs
			}
			rel, _ := filepath.Rel(pub, p)
			a, ok := asset(filepath.ToSlash(rel), p, credits)
			if ok {
				a.Group = groups[a.File]
				list = append(list, a)
			}
			return nil
		})
		if err != nil {
			return nil, err
		}
	}
	sort.Slice(list, func(i, j int) bool { return list[i].File < list[j].File })
	return list, nil
}

func asset(rel, full string, credits map[string]credit) (Asset, bool) {
	ext := strings.ToLower(path.Ext(rel))
	parts := strings.Split(rel, "/")
	a := Asset{File: rel, Kind: KindGraphic}
	switch {
	case soundExt[ext]:
		a.Kind, a.Pack = KindSound, parts[0]
	case ext == ".png" && len(parts) > 2:
		a.Pack = parts[1]
		a.W, a.H = pngSize(full)
	default:
		return a, false
	}
	c, ok := credits[strings.Join(parts[1:], "/")] // Import: Zeile je Datei (kandidaten/<datei>)
	if !ok {
		c, ok = credits[a.Pack]
	}
	switch {
	case ok:
		a.Title, a.Artist, a.License, a.Source = c.title, c.artist, c.license, c.source
	case a.Kind == KindSound:
		a.License = selfMade
	default:
		a.License = "unbekannt"
	}
	a.Flag = LicenseFlag(a.License)
	return a, true
}

func pngSize(full string) (int, int) {
	f, err := os.Open(full)
	if err != nil {
		return 0, 0
	}
	defer func() { _ = f.Close() }()
	cfg, _, err := image.DecodeConfig(f)
	if err != nil {
		return 0, 0
	}
	return cfg.Width, cfg.Height
}

// grafikGroups ordnet jeder Datei unter grafik/ ihre Gruppe aus public/grafik/index.json zu (Tag der Suche).
func grafikGroups(pub string) map[string]string {
	out := map[string]string{}
	b, err := os.ReadFile(filepath.Join(pub, "grafik", "index.json"))
	if err != nil {
		return out
	}
	var items []struct{ Pack, Group, File string }
	if json.Unmarshal(b, &items) != nil {
		return out
	}
	for _, it := range items {
		out["grafik/"+it.Pack+"/"+it.File] = it.Group
	}
	return out
}

type credit struct{ title, artist, license, source string }

// readCredits liest die Tabellen „| Ordner | Pack | Urheber | Lizenz | Quelle |“ einer Credit-Datei;
// eine Zelle Ordner darf mehrere Ordner durch Komma nennen.
func readCredits(file string) map[string]credit {
	out := map[string]credit{}
	b, err := os.ReadFile(file)
	if err != nil {
		return out
	}
	inTable := false
	for _, line := range strings.Split(strings.ReplaceAll(string(b), "\r\n", "\n"), "\n") {
		switch {
		case !strings.HasPrefix(line, "|"):
			inTable = false
		case strings.HasPrefix(line, "| Ordner |"):
			inTable = true
		case inTable && !strings.HasPrefix(line, "|---"):
			cells := strings.Split(strings.Trim(line, "|"), "|")
			if len(cells) < 5 {
				continue
			}
			c := credit{strings.TrimSpace(cells[1]), strings.TrimSpace(cells[2]), strings.TrimSpace(cells[3]), strings.TrimSpace(cells[4])}
			for _, dir := range strings.Split(cells[0], ",") {
				out[strings.Trim(strings.TrimSpace(dir), "`")] = c
			}
		}
	}
	return out
}

// LicenseFlag kennzeichnet eine Lizenz nach Q70: ND (nicht übernehmbar), NC (nur markierter Lückenfüller),
// SA (Ausschnitt bleibt unter CC-BY-SA), leer = übernehmbar oder unbekannt.
func LicenseFlag(license string) string {
	// Elemente einzeln (CC BY-NC-SA 4.0), damit Wörter wie „Sample“ nicht zählen.
	found := map[string]bool{}
	for _, part := range strings.FieldsFunc(strings.ToUpper(license), func(r rune) bool { return r == ' ' || r == '-' || r == '/' }) {
		found[part] = true
	}
	for _, f := range []string{"ND", "NC", "SA"} {
		if found[f] {
			return f
		}
	}
	return ""
}

// Search filtert den Katalog: jedes Wort muss in Datei, Pack, Gruppe, Titel oder Lizenz vorkommen (ohne Groß/Klein).
// kind leer = beide Arten; limit ≤ 0 = alle.
func Search(list []Asset, query, kind string, limit int) []Asset {
	words := strings.Fields(strings.ToLower(query))
	var out []Asset
	for _, a := range list {
		if kind != "" && a.Kind != kind {
			continue
		}
		hay := strings.ToLower(strings.Join([]string{a.File, a.Pack, a.Group, a.Title, a.License}, " "))
		if containsAll(hay, words) {
			out = append(out, a)
			if limit > 0 && len(out) == limit {
				break
			}
		}
	}
	return out
}

func containsAll(hay string, words []string) bool {
	for _, w := range words {
		if !strings.Contains(hay, w) {
			return false
		}
	}
	return true
}
