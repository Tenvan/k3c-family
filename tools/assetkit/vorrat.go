package assets

import (
	"bytes"
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"io/fs"
	"net/http"
	"net/url"
	"os"
	"path"
	"path/filepath"
	"strings"
	"sync"
	"time"
)

// VorratDir ist der Vorrat der Vorschlagsliste (B-601, WZ29.6): ein Cache unter .work/ (gitignored), je Art (audio/,
// grafik/) und Quelle ein Unterordner. Nicht gewählte Kandidaten bleiben hier und kommen nie in Git; erst die Übernahme
// (vorrat_adopt.go) kopiert eine Datei nach public/.
const VorratDir = ".work/vorrat"

// vorratIndex merkt je geladenem Eintrag Quelle, Lizenz, Größe, Prüfsumme und Ladezeit.
const vorratIndex = "index.json"

const vorratComment = "Vorrat der Vorschlagsliste (B-601): geladene Dateien mit Quelle, Lizenz, Größe, SHA-256 und Ladezeit. Nur lokal, nie in Git."

// Status eines Eintrags aus Liste und Index.
const (
	VorratOpen    = "offen"
	VorratLoaded  = "geladen"
	VorratBlocked = "gesperrt"
	VorratDropped = "verworfen"
)

// vorratExt sind die Endungen, die der Vorrat lädt (Pakete nur zum Auspacken von Hand).
var vorratExt = map[string]bool{".png": true, ".ogg": true, ".mp3": true, ".wav": true, ".zip": true, ".7z": true}

// vorratKindDir ist der Ordner je Art unter .work/vorrat/.
var vorratKindDir = map[string]string{KindGraphic: "grafik", KindSound: "audio"}

// VorratEntry ist eine geladene Datei des Vorrats.
type VorratEntry struct {
	ID      string `json:"id"`
	Source  string `json:"quelle"`
	License string `json:"lizenz"`
	Size    int64  `json:"groesse"`
	SHA256  string `json:"pruefsumme"`
	Loaded  string `json:"geladen"`
	File    string `json:"datei"`            // relativ zu .work/vorrat/
	Public  string `json:"public,omitempty"` // nach der Übernahme: Datei relativ zu public/
}

// VorratIndex ist der Inhalt von .work/vorrat/index.json.
type VorratIndex struct {
	Comment string        `json:"_comment"`
	Entries []VorratEntry `json:"eintraege"`
}

// vorratMu reiht Lesen und Schreiben des Index hintereinander (Downloads im Hintergrund, Übernahme, Aufräumen).
var vorratMu sync.Mutex

func vorratPath(root, rel string) string {
	return filepath.Join(root, filepath.FromSlash(VorratDir), filepath.FromSlash(rel))
}

// LoadVorrat liest den Index; fehlt er, ist der Vorrat leer.
func LoadVorrat(root string) (*VorratIndex, error) {
	b, err := os.ReadFile(vorratPath(root, vorratIndex))
	if errors.Is(err, fs.ErrNotExist) {
		return &VorratIndex{Comment: vorratComment}, nil
	}
	if err != nil {
		return nil, err
	}
	var v VorratIndex
	if err := json.Unmarshal(b, &v); err != nil {
		return nil, fmt.Errorf("%s/%s: %w", VorratDir, vorratIndex, err)
	}
	return &v, nil
}

func (v *VorratIndex) save(root string) error {
	file := vorratPath(root, vorratIndex)
	if err := os.MkdirAll(filepath.Dir(file), 0o755); err != nil {
		return err
	}
	b, err := json.MarshalIndent(v, "", "  ")
	if err != nil {
		return err
	}
	return writeAtomic(file, append(b, '\n'))
}

func (v *VorratIndex) find(id string) *VorratEntry {
	for i := range v.Entries {
		if v.Entries[i].ID == id {
			return &v.Entries[i]
		}
	}
	return nil
}

// put ersetzt den Eintrag mit derselben ID oder hängt ihn an.
func (v *VorratIndex) put(e VorratEntry) {
	if old := v.find(e.ID); old != nil {
		*old = e
		return
	}
	v.Entries = append(v.Entries, e)
}

// changeVorrat liest den Index unter Sperre, ändert ihn und speichert.
func changeVorrat(root string, change func(*VorratIndex) error) error {
	vorratMu.Lock()
	defer vorratMu.Unlock()
	v, err := LoadVorrat(root)
	if err != nil {
		return err
	}
	if err := change(v); err != nil {
		return err
	}
	return v.save(root)
}

// VorratState ist der Status eines Eintrags der Liste: Liste plus Index; Entry nur bei „geladen“, Used = die übernommene
// Datei ist einem Slot unter public/ zugeordnet.
type VorratState struct {
	Status string
	Reason string
	Entry  *VorratEntry
	Used   bool
}

// VorratStates liefert den Status je ID der Liste: gesperrt (Lizenz nicht erlaubt oder in der Liste gesperrt, wird nie
// geladen), verworfen, geladen (Index und Datei vorhanden) oder offen.
func VorratStates(root string, list []Suggestion) (map[string]VorratState, error) {
	v, err := LoadVorrat(root)
	if err != nil {
		return nil, err
	}
	used, err := usedFiles(root)
	if err != nil {
		return nil, err
	}
	out := make(map[string]VorratState, len(list))
	for _, s := range list {
		st := VorratState{Status: VorratOpen}
		switch e := v.find(s.ID); {
		case !SuggestionLicenseAllowed(s.License):
			st = VorratState{Status: VorratBlocked, Reason: fmt.Sprintf("Lizenz %s nicht erlaubt (Q70)", s.License)}
		case s.Status == VorratBlocked:
			st = VorratState{Status: VorratBlocked, Reason: "in der Vorschlagsliste gesperrt (Lizenz der Quelle weicht ab)"}
		case s.Status == VorratDropped:
			st.Status = VorratDropped
		case e != nil && fileExists(vorratPath(root, e.File)):
			st = VorratState{Status: VorratLoaded, Entry: e, Used: e.Public != "" && used[e.Public]}
		}
		out[s.ID] = st
	}
	return out, nil
}

// usedFiles sind alle Dateien, die das Slot-Register einem Slot zuordnet.
func usedFiles(root string) (map[string]bool, error) {
	reg, err := Load(root)
	if err != nil {
		return nil, err
	}
	out := map[string]bool{}
	for _, s := range reg.Slots {
		for _, c := range s.Choices {
			out[c.File] = true
		}
	}
	return out, nil
}

func fileExists(p string) bool {
	st, err := os.Stat(p)
	return err == nil && st.Mode().IsRegular()
}

// CheckVorratDownload prüft einen Eintrag vor dem Download: nicht gesperrt oder verworfen, Lizenz nach Q70, Größe im
// Limit, https-Adresse und erlaubte Endung. Liefert die Adresse und die Endung.
func CheckVorratDownload(s Suggestion) (*url.URL, string, error) {
	switch {
	case !SuggestionLicenseAllowed(s.License):
		return nil, "", fmt.Errorf("%s: Lizenz %s nicht erlaubt (Q70)", s.ID, s.License)
	case s.Status == VorratBlocked || s.Status == VorratDropped:
		return nil, "", fmt.Errorf("%s ist %s", s.ID, s.Status)
	case s.Size <= 0 || s.Size > MaxSuggestionBytes:
		return nil, "", fmt.Errorf("%s: Größe %d außerhalb 1 … %d", s.ID, s.Size, MaxSuggestionBytes)
	case vorratKindDir[s.Kind] == "":
		return nil, "", fmt.Errorf("%s: Art %q unbekannt", s.ID, s.Kind)
	}
	u, err := url.Parse(s.FileURL)
	if err != nil {
		return nil, "", fmt.Errorf("%s: Datei-URL %q ungültig", s.ID, s.FileURL)
	}
	if err := VorratURLAllowed(u); err != nil {
		return nil, "", fmt.Errorf("%s: %w", s.ID, err)
	}
	ext := strings.ToLower(path.Ext(s.FileName))
	if !vorratExt[ext] {
		return nil, "", fmt.Errorf("%s: Endung %q nicht erlaubt (png, ogg, mp3, wav, zip, 7z)", s.ID, ext)
	}
	return u, ext, nil
}

// VorratHosts sind die Hosts der Quellen (samt Unterdomänen), von denen der Vorrat lädt; auch jedes Redirect-Ziel muss
// dazugehören, damit eine Quelle nicht auf Klartext, Loopback oder Geräte im Heimnetz umleiten kann (SSRF).
var VorratHosts = []string{"opengameart.org", "kenney.nl", "freesound.org", "itch.io", "itch.zone"}

// VorratURLAllowed: nur https und nur ein Host aus VorratHosts.
func VorratURLAllowed(u *url.URL) error {
	if u.Scheme != "https" || u.Hostname() == "" {
		return fmt.Errorf("download nur über https: %q", u.String())
	}
	h := strings.ToLower(u.Hostname())
	for _, allowed := range VorratHosts {
		if h == allowed || strings.HasSuffix(h, "."+allowed) {
			return nil
		}
	}
	return fmt.Errorf("host %q ist keine erlaubte Quelle", h)
}

// VorratRedirect ist der CheckRedirect des Download-Clients: jedes Ziel wie die Start-URL prüfen, höchstens 5 Sprünge.
func VorratRedirect(req *http.Request, via []*http.Request) error {
	if len(via) >= 5 {
		return errors.New("zu viele Weiterleitungen")
	}
	return VorratURLAllowed(req.URL)
}

// Progress meldet die bisher geladenen Bytes eines Downloads.
type Progress func(loaded int64)

// VorratDownload lädt einen Eintrag in den Vorrat: erst in eine Temp-Datei im Zielordner (Größe laut Liste, Limit,
// Signatur), dann atomar umbenannt und in den Index eingetragen. Ein Fehler lässt nichts zurück; ist der Eintrag schon
// geladen und passt die Prüfsumme der Datei, passiert nichts.
func VorratDownload(ctx context.Context, root string, s Suggestion, client *http.Client, now time.Time, progress Progress) (VorratEntry, error) {
	u, ext, err := CheckVorratDownload(s)
	if err != nil {
		return VorratEntry{}, err
	}
	if e, ok := vorratHave(root, s.ID); ok {
		return e, nil
	}
	stem := strings.TrimSuffix(path.Base(s.FileName), path.Ext(s.FileName))
	rel := path.Join(vorratKindDir[s.Kind], slug(s.Source), slug(s.ID+"-"+stem)+ext)
	full := vorratPath(root, rel)
	if err := os.MkdirAll(filepath.Dir(full), 0o755); err != nil {
		return VorratEntry{}, err
	}
	tmp, sum, err := fetchVorrat(ctx, client, u, filepath.Dir(full), progress)
	if err != nil {
		return VorratEntry{}, err
	}
	defer func() { _ = os.Remove(tmp) }() // nach dem Umbenennen ohne Wirkung
	if err := checkVorratFile(tmp, ext, s.Size); err != nil {
		return VorratEntry{}, fmt.Errorf("%s: %w", s.ID, err)
	}
	if err := os.Rename(tmp, full); err != nil {
		return VorratEntry{}, err
	}
	e := VorratEntry{ID: s.ID, Source: s.Source, License: s.License, Size: s.Size, SHA256: sum,
		Loaded: now.UTC().Format(time.RFC3339), File: rel}
	err = changeVorrat(root, func(v *VorratIndex) error { v.put(e); return nil })
	if err != nil {
		_ = os.Remove(full)
	}
	return e, err
}

// vorratHave: Eintrag im Index, Datei vorhanden und ihre Prüfsumme passt.
func vorratHave(root, id string) (VorratEntry, bool) {
	vorratMu.Lock()
	v, err := LoadVorrat(root)
	vorratMu.Unlock()
	if err != nil {
		return VorratEntry{}, false
	}
	e := v.find(id)
	if e == nil {
		return VorratEntry{}, false
	}
	sum, err := fileSHA256(vorratPath(root, e.File))
	return *e, err == nil && sum == e.SHA256
}

func fileSHA256(p string) (string, error) {
	f, err := os.Open(p)
	if err != nil {
		return "", err
	}
	defer func() { _ = f.Close() }()
	h := sha256.New()
	if _, err := io.Copy(h, f); err != nil {
		return "", err
	}
	return hex.EncodeToString(h.Sum(nil)), nil
}

// fetchVorrat lädt u in eine Temp-Datei in dir (höchstens MaxSuggestionBytes) und liefert Pfad und SHA-256; bei einem
// Fehler ist die Temp-Datei schon entfernt.
func fetchVorrat(ctx context.Context, client *http.Client, u *url.URL, dir string, progress Progress) (string, string, error) {
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, u.String(), nil)
	if err != nil {
		return "", "", err
	}
	req.Header.Set("User-Agent", userAgent)
	res, err := client.Do(req)
	if err != nil {
		return "", "", err
	}
	defer func() { _ = res.Body.Close() }()
	if res.StatusCode != http.StatusOK {
		return "", "", fmt.Errorf("download: HTTP %d", res.StatusCode)
	}
	f, err := os.CreateTemp(dir, ".vorrat-*")
	if err != nil {
		return "", "", err
	}
	h := sha256.New()
	n, werr := io.Copy(io.MultiWriter(f, h, &countWriter{fn: progress}), io.LimitReader(res.Body, MaxSuggestionBytes+1))
	if werr == nil && n > MaxSuggestionBytes {
		werr = fmt.Errorf("datei größer als %d MB", MaxSuggestionBytes>>20)
	}
	if werr == nil {
		werr = f.Sync()
	}
	if cerr := f.Close(); werr == nil {
		werr = cerr
	}
	if werr != nil {
		_ = os.Remove(f.Name())
		return "", "", werr
	}
	_ = os.Chmod(f.Name(), 0o644) // CreateTemp legt 0600 an
	return f.Name(), hex.EncodeToString(h.Sum(nil)), nil
}

// countWriter zählt die geschriebenen Bytes und meldet sie an fn.
type countWriter struct {
	n  int64
	fn Progress
}

func (c *countWriter) Write(p []byte) (int, error) {
	c.n += int64(len(p))
	if c.fn != nil {
		c.fn(c.n)
	}
	return len(p), nil
}

// checkVorratFile prüft die Größe laut Liste und die Signatur (Kopf der Datei) zur Endung.
func checkVorratFile(file, ext string, want int64) error {
	st, err := os.Stat(file)
	if err != nil {
		return err
	}
	if st.Size() != want {
		return fmt.Errorf("größe %d statt %d laut Liste", st.Size(), want)
	}
	f, err := os.Open(file)
	if err != nil {
		return err
	}
	defer func() { _ = f.Close() }()
	head := make([]byte, 16)
	n, _ := io.ReadFull(f, head)
	return vorratSignature(head[:n], ext)
}

// vorratSignature prüft Pakete (zip, 7z) selbst, Bilder und Töne wie der Import.
func vorratSignature(head []byte, ext string) error {
	switch ext {
	case ".zip":
		if !bytes.HasPrefix(head, []byte("PK\x03\x04")) {
			return errors.New("inhalt ist kein zip")
		}
		return nil
	case ".7z":
		if !bytes.HasPrefix(head, []byte("7z\xBC\xAF\x27\x1C")) {
			return errors.New("inhalt ist kein 7z")
		}
		return nil
	}
	return checkSignature(head, ext)
}
