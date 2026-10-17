package assets

import (
	"fmt"
	"os"
	"path"
	"path/filepath"
	"strings"
	"time"
)

// creditHeader ist die Tabelle des Import-Abschnitts; Ordner nennt die Datei (kandidaten/<datei>), Status die Lizenz-Prüfung.
const creditHeader = "| Ordner | Pack | Urheber | Lizenz | Quelle | Status |\n|---|---|---|---|---|---|\n"

const importIntro = "Mit k3c-dev importierte Kandidaten (B-298): je Datei eine Zeile. „Lizenz prüfen“ gilt, bis 🧑 die Lizenz in k3c-dev bestätigt.\n\n"

// cell macht Text tabellentauglich (kein |, kein Zeilenumbruch).
func cell(s, fallback string) string {
	s = strings.TrimSpace(strings.NewReplacer("|", "/", "\r", " ", "\n", " ").Replace(s))
	if s == "" {
		return fallback
	}
	return s
}

func creditStatus(h Hit, now time.Time) string {
	switch h.LicenseStatus {
	case LicenseConfirmed:
		return "bestätigt 🧑 " + now.Format("2006-01-02")
	case LicenseKnown:
		return "Lizenz erkannt (" + cell(h.Source, "Quelle") + ")"
	}
	return "Lizenz prüfen"
}

// addCredit trägt die Datei sofort in die Credit-Datei ein (B-165), im Abschnitt „Importe (k3c-dev)“.
func addCredit(root, kind, rel string, h Hit, now time.Time) error {
	file := filepath.Join(root, "public", filepath.FromSlash(creditFile[kind]))
	b, err := os.ReadFile(file)
	if err != nil && !os.IsNotExist(err) {
		return err
	}
	text := strings.ReplaceAll(string(b), "\r\n", "\n")
	if !strings.Contains(text, importSection) {
		text = strings.TrimRight(text, "\n") + "\n\n" + importSection + "\n\n" + importIntro + creditHeader
	}
	row := fmt.Sprintf("| %s | %s | %s | %s | %s | %s |\n", creditFolder(kind, rel), cell(h.Title, "ohne Titel"),
		cell(h.Artist, "unbekannt"), cell(h.License, "unbekannt"), cell(h.Link, "–"), creditStatus(h, now))
	return writeAtomic(file, []byte(strings.TrimRight(text, "\n")+"\n"+row))
}

// creditFolder ist der Pfad relativ zum Ordner der Credit-Datei (grafik/kandidaten/x.png → kandidaten/x.png).
func creditFolder(kind, rel string) string {
	return strings.TrimPrefix(rel, path.Dir(creditFile[kind])+"/")
}

// markImported vermerkt am Treffer die importierte Datei.
func markImported(root, slotID, hitID, rel string) error {
	return updateHit(root, slotID, hitID, func(h *Hit) { h.File = rel })
}

func updateHit(root, slotID, hitID string, change func(*Hit)) error {
	mu.Lock()
	defer mu.Unlock()
	r, err := LoadResearch(root)
	if err != nil {
		return err
	}
	for i := range r.Hits[slotID] {
		if r.Hits[slotID][i].ID == hitID {
			change(&r.Hits[slotID][i])
			return r.save(root)
		}
	}
	return fmt.Errorf("treffer %q zu %s nicht gefunden", hitID, slotID)
}

// ConfirmLicense bestätigt die Lizenz eines Treffers (nur 🧑 über die Oberfläche): Status am Treffer und in der Credit-Zeile.
func ConfirmLicense(root, slotID, hitID string, now time.Time) error {
	reg, err := Load(root)
	if err != nil {
		return err
	}
	slot := reg.Find(slotID)
	if slot == nil {
		return fmt.Errorf("unbekannter Asset-Slot %q", slotID)
	}
	var file string
	if err := updateHit(root, slotID, hitID, func(h *Hit) {
		h.LicenseStatus = LicenseConfirmed
		file = h.File
	}); err != nil {
		return err
	}
	if file == "" {
		return nil // noch nicht importiert: die Credit-Zeile entsteht beim Import mit „bestätigt“
	}
	return setCreditStatus(root, slot.Kind, creditFolder(slot.Kind, file), "bestätigt 🧑 "+now.Format("2006-01-02"))
}

func setCreditStatus(root, kind, folder, status string) error {
	file := filepath.Join(root, "public", filepath.FromSlash(creditFile[kind]))
	b, err := os.ReadFile(file)
	if err != nil {
		return err
	}
	lines := strings.Split(strings.ReplaceAll(string(b), "\r\n", "\n"), "\n")
	for i, l := range lines {
		if !strings.HasPrefix(l, "| "+folder+" |") {
			continue
		}
		cells := strings.Split(strings.Trim(l, "|"), "|")
		cells[len(cells)-1] = " " + status + " "
		lines[i] = "|" + strings.Join(cells, "|") + "|"
		return writeAtomic(file, []byte(strings.Join(lines, "\n")))
	}
	return fmt.Errorf("credit-zeile %s fehlt in %s", folder, creditFile[kind])
}
