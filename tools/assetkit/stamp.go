package assets

import (
	"bufio"
	"context"
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"os"
	"path/filepath"
	"strings"
	"time"
)

// ErrStale lehnt ein Speichern auf veraltetem Stand ab; der Text geht so an die Oberfläche (B-459 › Ausnahme- und
// Fehlerfälle). Eigener Typ, weil Go-Fehlertexte sonst klein beginnen.
var ErrStale error = staleError{}

type staleError struct{}

func (staleError) Error() string { return "Register geändert, bitte erneut speichern" }

// FindRoot sucht ab dir aufwärts das Verzeichnis mit dem go.mod des Spiels (module k3c).
func FindRoot(dir string) (string, error) {
	for {
		if isGameModule(filepath.Join(dir, "go.mod")) {
			return dir, nil
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			return "", errors.New("repo-wurzel nicht gefunden: kein go.mod mit \"module k3c\" über " + dir)
		}
		dir = parent
	}
}

func isGameModule(path string) bool {
	f, err := os.Open(path)
	if err != nil {
		return false
	}
	defer func() { _ = f.Close() }()
	sc := bufio.NewScanner(f)
	for sc.Scan() {
		if line := strings.TrimSpace(sc.Text()); strings.HasPrefix(line, "module ") {
			return strings.TrimSpace(strings.TrimPrefix(line, "module ")) == "k3c"
		}
	}
	return false
}

// Stamp ist der Stand des Slot-Registers: SHA-256 des Datei-Inhalts (hex).
func Stamp(root string) (string, error) {
	b, err := os.ReadFile(filepath.Join(root, RegisterPath))
	if err != nil {
		return "", err
	}
	sum := sha256.Sum256(b)
	return hex.EncodeToString(sum[:]), nil
}

// SetChoicesIf ist SetChoices mit Schreibregel: gespeichert wird nur, wenn das Register noch den Stand stand hat,
// sonst ErrStale ohne Schreibzugriff. Liefert den Slot und den neuen Stand.
//
// ponytail: mu reiht nur Schreiber dieses Prozesses; zwischen Stamp und Save kann ein anderer Prozess (MCP der
// Workbench) noch schreiben, und der neue Stand wird frisch von der Platte gehasht. Das Fenster ist wenige
// Dateizugriffe breit, Watch meldet einen solchen Fremdschreiber trotzdem; eine Datei-Sperre wäre plattformabhängig.
// Unter Windows kann ein Lesezugriff hier auch ein gleichzeitiges os.Rename eines anderen Prozesses scheitern lassen
// (kein FILE_SHARE_DELETE); das galt schon für die Lesezugriffe der Workbench-Seite.
func SetChoicesIf(root, id string, choices []Choice, stand string) (*Slot, string, error) {
	mu.Lock()
	defer mu.Unlock()
	cur, err := Stamp(root)
	if err != nil {
		return nil, "", err
	}
	if cur != stand {
		return nil, "", ErrStale
	}
	slot, err := setChoices(root, id, choices)
	if err != nil {
		return nil, "", err
	}
	next, err := Stamp(root)
	return slot, next, err
}

// Watch prüft im Takt every, ob sich das Slot-Register geändert hat, und ruft dann onChange. Größe und
// Änderungszeit sind der schnelle Vorfilter, erst der Hash entscheidet; eine fehlende Datei gilt als eigener Stand.
// Der Stand beim Aufruf ist der Ausgangsstand. Endet ctx, endet die Goroutine.
//
// ponytail: Polling statt Dateisystem-Ereignissen wie planning.Watch in k3c-dev; fsnotify wäre eine neue
// Abhängigkeit, und ein Stat je Sekunde kostet nichts.
func Watch(ctx context.Context, root string, every time.Duration, onChange func()) {
	path := filepath.Join(root, RegisterPath)
	lastInfo, lastHash := probe(path, root)
	go func() {
		t := time.NewTicker(every)
		defer t.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-t.C:
				info, _ := os.Stat(path)
				if sameFile(info, lastInfo) {
					continue
				}
				hash, err := Stamp(root)
				if err != nil && info != nil {
					continue // gesperrt (Rename eines anderen Schreibers): im nächsten Takt neu versuchen
				}
				lastInfo = info
				if hash != lastHash {
					lastHash = hash
					onChange()
				}
			}
		}
	}()
}

func probe(path, root string) (os.FileInfo, string) {
	info, _ := os.Stat(path)
	hash, _ := Stamp(root)
	return info, hash
}

func sameFile(a, b os.FileInfo) bool {
	if a == nil || b == nil {
		return a == nil && b == nil
	}
	return a.Size() == b.Size() && a.ModTime().Equal(b.ModTime())
}
