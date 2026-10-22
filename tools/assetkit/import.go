package assets

import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"os"
	"path"
	"path/filepath"
	"sort"
	"strings"
	"time"
)

// LicenseConfirmed setzt nur 🧑 in der Oberfläche (B-298/AC-05), nie ein Agent.
const LicenseConfirmed = "bestaetigt"

// maxImportBytes begrenzt Import und Download.
const maxImportBytes = 20 << 20

// importSection ist der Abschnitt der Credit-Datei für Importe; je Datei eine Zeile (Ordner = kandidaten/<datei>).
const importSection = "## Importe (k3c-workbench)"

// importDir und creditFile je Art: Ablage unter public/ und die Credit-Datei (B-165).
var (
	importDir  = map[string]string{KindGraphic: "grafik/kandidaten", KindSound: "audio/kandidaten"}
	creditFile = map[string]string{KindGraphic: "grafik/CREDITS.md", KindSound: "audio/README.md"}
	importExt  = map[string][]string{KindGraphic: {".png"}, KindSound: {".ogg", ".mp3", ".wav"}}
)

// ImportFile legt eine (manuell geladene) Datei zu einem Recherche-Treffer ab: Ablage, Credit-Zeile, Zuordnung als letzte
// Präferenz. Nicht übernehmbare Lizenzen (Q70) werden abgelehnt; die Lizenz bleibt „prüfen“, bis 🧑 bestätigt.
func ImportFile(root, slotID, hitID, src string, now time.Time) (string, error) {
	slot, hit, err := importTarget(root, slotID, hitID)
	if err != nil {
		return "", err
	}
	data, err := readImport(src, slot.Kind)
	if err != nil {
		return "", err
	}
	rel, err := storeImport(root, slot.Kind, hit.Title, strings.ToLower(filepath.Ext(src)), data)
	if err != nil {
		return "", err
	}
	if err := addCredit(root, slot.Kind, rel, hit, now); err != nil {
		return "", err
	}
	if err := markImported(root, slotID, hitID, rel); err != nil {
		return "", err
	}
	reg, err := Load(root)
	if err != nil {
		return "", err
	}
	_, err = SetChoices(root, slotID, append(reg.Find(slotID).Choices, Choice{File: rel}))
	return rel, err
}

func importTarget(root, slotID, hitID string) (Slot, Hit, error) {
	reg, err := Load(root)
	if err != nil {
		return Slot{}, Hit{}, err
	}
	slot := reg.Find(slotID)
	if slot == nil {
		return Slot{}, Hit{}, fmt.Errorf("unbekannter Asset-Slot %q", slotID)
	}
	hits, err := HitsOf(root, slotID)
	if err != nil {
		return Slot{}, Hit{}, err
	}
	for _, h := range hits {
		if h.ID != hitID {
			continue
		}
		if h.LicenseStatus == LicenseExcluded {
			return Slot{}, Hit{}, fmt.Errorf("treffer %s ist nicht übernehmbar (Q70: %s)", hitID, h.License)
		}
		return *slot, h, nil
	}
	return Slot{}, Hit{}, fmt.Errorf("treffer %q zu %s nicht gefunden", hitID, slotID)
}

// readImport liest die Quelldatei: erlaubte Endung je Art, höchstens 20 MB, Signatur passt zur Endung.
func readImport(src, kind string) ([]byte, error) {
	ext := strings.ToLower(filepath.Ext(src))
	if !contains(importExt[kind], ext) {
		return nil, fmt.Errorf("%s: erlaubt sind %s", filepath.Base(src), strings.Join(importExt[kind], ", "))
	}
	f, err := os.Open(src)
	if err != nil {
		return nil, err
	}
	defer func() { _ = f.Close() }()
	data, err := io.ReadAll(io.LimitReader(f, maxImportBytes+1))
	if err != nil {
		return nil, err
	}
	return data, checkSignature(data, ext)
}

func checkSignature(data []byte, ext string) error {
	if len(data) > maxImportBytes {
		return fmt.Errorf("datei größer als %d MB", maxImportBytes>>20)
	}
	ok := false
	switch ext {
	case ".png":
		ok = bytes.HasPrefix(data, []byte("\x89PNG\r\n\x1a\n"))
	case ".ogg":
		ok = bytes.HasPrefix(data, []byte("OggS"))
	case ".wav":
		ok = bytes.HasPrefix(data, []byte("RIFF")) && len(data) > 12 && string(data[8:12]) == "WAVE"
	case ".mp3":
		ok = bytes.HasPrefix(data, []byte("ID3")) || (len(data) > 1 && data[0] == 0xFF && data[1]&0xE0 == 0xE0)
	}
	if !ok {
		return fmt.Errorf("inhalt ist kein %s", strings.TrimPrefix(ext, "."))
	}
	return nil
}

// storeImport schreibt unter public/<importDir>/ mit Namen aus dem Titel; vorhandene Dateien bleiben (Suffix -2 …).
// Atomar (B-511): erst eine Temp-Datei im selben Ordner, dann ein harter Link auf den Zielnamen (scheitert, wenn der Name
// schon vergeben ist, ohne etwas zu überschreiben); der beobachtende Server sieht nie eine halbe Datei.
func storeImport(root, kind, title, ext string, data []byte) (string, error) {
	dir := filepath.Join(root, "public", filepath.FromSlash(importDir[kind]))
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return "", err
	}
	tmp, err := writeTemp(dir, data)
	if err != nil {
		return "", err
	}
	defer func() { _ = os.Remove(tmp) }()
	base := slug(title)
	for n := 1; n < 100; n++ {
		name := base + ext
		if n > 1 {
			name = fmt.Sprintf("%s-%d%s", base, n, ext)
		}
		err := publish(tmp, filepath.Join(dir, name))
		if errors.Is(err, os.ErrExist) {
			continue
		}
		if err != nil {
			return "", err
		}
		return path.Join(importDir[kind], name), nil
	}
	return "", fmt.Errorf("kein freier Dateiname für %q", base)
}

// writeTemp schreibt data in eine Temp-Datei in dir (mit Sync) und liefert ihren Pfad.
func writeTemp(dir string, data []byte) (string, error) {
	f, err := os.CreateTemp(dir, ".import-*")
	if err != nil {
		return "", err
	}
	_, werr := f.Write(data)
	if werr == nil {
		werr = f.Sync()
	}
	if cerr := f.Close(); werr == nil {
		werr = cerr
	}
	if werr != nil {
		_ = os.Remove(f.Name())
		return "", werr
	}
	_ = os.Chmod(f.Name(), 0o644) // CreateTemp legt 0600 an
	return f.Name(), nil
}

// publish macht aus der Temp-Datei tmp die Datei dst, ohne eine vorhandene zu ersetzen (os.ErrExist). Ohne Link-Unterstützung
// des Dateisystems: Umbenennen nach einer Prüfung (ein Rennen mit gleichem Namen ist dann möglich, aber nie eine halbe Datei).
func publish(tmp, dst string) error {
	err := os.Link(tmp, dst)
	if err == nil || errors.Is(err, os.ErrExist) {
		return err
	}
	if _, serr := os.Stat(dst); serr == nil {
		return os.ErrExist
	}
	return os.Rename(tmp, dst)
}

// slug macht aus einem Titel einen Dateinamen aus a–z, 0–9 und -, ohne Pfadanteile.
func slug(title string) string {
	r := strings.NewReplacer("ä", "ae", "ö", "oe", "ü", "ue", "ß", "ss")
	var b strings.Builder
	dash := false
	for _, c := range r.Replace(strings.ToLower(title)) {
		if (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') {
			b.WriteRune(c)
			dash = false
		} else if !dash && b.Len() > 0 {
			b.WriteByte('-')
			dash = true
		}
	}
	s := strings.Trim(b.String(), "-")
	if len(s) > 40 {
		s = strings.Trim(s[:40], "-")
	}
	if s == "" {
		return "import"
	}
	return s
}

func contains(list []string, s string) bool {
	for _, x := range list {
		if x == s {
			return true
		}
	}
	return false
}

// DownloadHosts sind die Quellen mit direktem Download und maschinenlesbarer Lizenz. Nur sie lädt k3c-workbench selbst;
// Links aus Treffern (auch von Agenten über assets_hit_add) erreichen so keine internen Adressen (SSRF).
var DownloadHosts = map[string]bool{"upload.wikimedia.org": true}

func hostList() string {
	hosts := make([]string, 0, len(DownloadHosts))
	for h := range DownloadHosts {
		hosts = append(hosts, h)
	}
	sort.Strings(hosts)
	return strings.Join(hosts, ", ")
}

// DownloadHit übernimmt einen Treffer mit direktem Download und erkannter Lizenz ohne Handarbeit (nur https, Limit, Signatur).
func DownloadHit(ctx context.Context, root, slotID, hitID string, client *http.Client, now time.Time) (string, error) {
	_, hit, err := importTarget(root, slotID, hitID)
	if err != nil {
		return "", err
	}
	if hit.Download == "" || hit.LicenseStatus == LicenseCheck {
		return "", fmt.Errorf("treffer %s: kein direkter Download mit erkannter Lizenz, bitte manuell laden und importieren", hitID)
	}
	u, err := url.Parse(hit.Download)
	if err != nil || u.Scheme != "https" {
		return "", fmt.Errorf("download nur über https: %q", hit.Download)
	}
	if !DownloadHosts[strings.ToLower(u.Hostname())] {
		return "", fmt.Errorf("download nur von %s, nicht von %q: bitte manuell laden und importieren", hostList(), u.Hostname())
	}
	tmp, err := fetchToTemp(ctx, client, u, path.Ext(u.Path))
	if err != nil {
		return "", err
	}
	defer func() { _ = os.Remove(tmp) }()
	return ImportFile(root, slotID, hitID, tmp, now)
}

func fetchToTemp(ctx context.Context, client *http.Client, u *url.URL, ext string) (string, error) {
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, u.String(), nil)
	if err != nil {
		return "", err
	}
	req.Header.Set("User-Agent", userAgent)
	res, err := client.Do(req)
	if err != nil {
		return "", err
	}
	defer func() { _ = res.Body.Close() }()
	if res.StatusCode != http.StatusOK {
		return "", fmt.Errorf("download: HTTP %d", res.StatusCode)
	}
	f, err := os.CreateTemp("", "k3c-import-*"+strings.ToLower(ext))
	if err != nil {
		return "", err
	}
	_, werr := io.Copy(f, io.LimitReader(res.Body, maxImportBytes+1))
	if cerr := f.Close(); werr == nil {
		werr = cerr
	}
	return f.Name(), werr
}
