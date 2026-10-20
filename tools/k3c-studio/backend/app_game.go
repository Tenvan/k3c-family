package backend

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net"
	"net/http"
	"net/url"
	"regexp"
	"strings"
	"time"
)

// „Im Spiel öffnen“ des Welt-Betrachters (B-460, WZ7.4, AC-06): das Level neu oder voll ausgebaut im Browser starten,
// über die Landingpage des Spielservers. Nur Loopback: das Studio spricht und öffnet nie eine fremde Adresse.

const (
	modeNew  = "new"  // neues Spiel mit diesem Seed (die Lobby lädt einen vorhandenen Spielstand gleichen Namens)
	modeFull = "full" // Spielstand „voll ausgebaut“ über die Dev-API des Servers anlegen und laden (B-315)
)

var (
	// Namensformat eines Spielstands (wie SAVE_FORMAT in src/scenes/lobbyLogic.ts): der Seed eines neuen Spiels ist dieser Name.
	gameSaveName = regexp.MustCompile(`^[a-z0-9-]{1,32}$`)
	// Für „voll ausgebaut“ muss `voll-<seed>` ins Namensformat passen (wie fullSeed in engine/room).
	gameFullSeed = regexp.MustCompile(`^[a-z0-9-]{1,27}$`)
)

// gameClient fragt den Spielserver (Gesundheit, Dev-API). Das Zeitlimit hält das Fenster nicht auf; Weiterleitungen
// folgt er nie, sonst ginge eine Anfrage an eine fremde Adresse, obwohl nur Loopback erlaubt ist.
var gameClient = &http.Client{
	Timeout:       15 * time.Second,
	CheckRedirect: func(*http.Request, []*http.Request) error { return http.ErrUseLastResponse },
}

// OpenGameResult ist das Ergebnis von WorldOpenGame: die geöffnete Adresse und der geladene Spielstand, oder ein Hinweis
// in Error (nichts wurde geöffnet).
type OpenGameResult struct {
	URL   string `json:"url"`
	Save  string `json:"save"`
	Error string `json:"error"`
}

// hint ist ein Hinweis für die Seite in ganzen Sätzen (großer Anfangsbuchstabe, kein Fehlerwert im Sinn von ST1005).
type hint string

func (h hint) Error() string { return string(h) }

// loopbackBase prüft die Adresse des Spielservers: http(s), nur Loopback (localhost, 127.0.0.0/8, ::1), ohne Zugangsdaten,
// Pfad und Abfrage. Liefert „schema://host:port“.
func loopbackBase(raw string) (string, error) {
	u, err := url.Parse(raw)
	if err != nil || (u.Scheme != "http" && u.Scheme != "https") || u.Host == "" || u.User != nil {
		return "", hint(fmt.Sprintf("Adresse des Spielservers ungültig: %q", raw))
	}
	host := u.Hostname()
	if ip := net.ParseIP(host); host != "localhost" && (ip == nil || !ip.IsLoopback()) {
		return "", fmt.Errorf("nur Loopback erlaubt (localhost, 127.0.0.1, ::1), nicht %q", host)
	}
	return u.Scheme + "://" + u.Host, nil
}

// WorldOpenGame startet das Level im Spiel (Binding): neu (`new`: nur Wald, Seed im Namensformat) oder voll ausgebaut
// (`full`: legt über POST /api/dev/fullsave den Spielstand `voll-<seed>` an, nur im Dev-Mode des Servers). Die Landingpage
// des Servers unter base öffnet danach `game.html` mit diesem Spielstand im Browser. Ist eine andere als die Standard-Welt
// gewählt, startet `new` ein frisches Spiel in ihr (`fresh=1&world=<id>`, WZ18.6).
func (a *App) WorldOpenGame(base, seed, biome, mode string) OpenGameResult {
	root, err := loopbackBase(base)
	if err != nil {
		return OpenGameResult{Error: err.Error()}
	}
	save, world := seed, a.activeWorld()
	switch mode {
	case modeNew:
		if biome != "forest" {
			return OpenGameResult{Error: "Start nur für den Wald (Tiefe 0)"}
		}
		if !gameSaveName.MatchString(seed) {
			return OpenGameResult{Error: "Seed muss aus Kleinbuchstaben, Ziffern und - bestehen (höchstens 32 Zeichen)"}
		}
	case modeFull:
		if world != standardWorld {
			return OpenGameResult{Error: "Voll ausgebaut gibt es nur für die Standard-Welt"}
		}
		if !gameFullSeed.MatchString(seed) {
			return OpenGameResult{Error: "Für „voll ausgebaut“ muss der Seed aus Kleinbuchstaben, Ziffern und - bestehen (höchstens 27 Zeichen)"}
		}
	default:
		return OpenGameResult{Error: fmt.Sprintf("Unbekannte Art %q (new, full)", mode)}
	}
	if err := checkServer(a.context(), root); err != nil {
		return OpenGameResult{Error: err.Error()}
	}
	if mode == modeFull {
		var err error
		if save, err = createFullSave(a.context(), root, seed); err != nil {
			return OpenGameResult{Error: err.Error()}
		}
	}
	query := url.Values{"autostart": {"1"}, "save": {save}}
	if world != standardWorld { // neues Spiel in der gewählten Welt (Dev-Mode des Servers, WZ18.5)
		query.Set("fresh", "1")
		query.Set("world", world)
	}
	target := root + "/#" + url.QueryEscape("game.html?"+query.Encode())
	return OpenGameResult{URL: target, Save: save}
}

// errServerDown ist der Hinweis bei einem Server, der nicht antwortet.
func errServerDown(root string) error {
	return hint(fmt.Sprintf("Spielserver läuft nicht (%s). Starte ihn mit `task dev` und `task start`", root))
}

// checkServer prüft vor dem Öffnen, ob der Spielserver unter root antwortet (GET /api/health): sonst öffnete der Browser
// eine Seite, die nicht lädt.
func checkServer(ctx context.Context, root string) error {
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, root+"/api/health", nil)
	if err != nil {
		return err
	}
	res, err := gameClient.Do(req)
	if err != nil {
		return errServerDown(root)
	}
	_ = res.Body.Close()
	if res.StatusCode != http.StatusOK {
		return errServerDown(root)
	}
	return nil
}

// createFullSave legt den Spielstand „voll ausgebaut“ über die Dev-API des Servers an und liefert seinen Namen; jeder
// Fehler wird ein Hinweis (kein Dev-Mode, Raum belegt, Server nicht erreichbar).
func createFullSave(ctx context.Context, root, seed string) (string, error) {
	body, _ := json.Marshal(map[string]string{"seed": seed})
	req, err := http.NewRequestWithContext(ctx, http.MethodPost, root+"/api/dev/fullsave", bytes.NewReader(body))
	if err != nil {
		return "", err
	}
	req.Header.Set("Content-Type", "application/json")
	res, err := gameClient.Do(req)
	if err != nil {
		return "", errServerDown(root)
	}
	defer func() { _ = res.Body.Close() }()
	var out struct {
		Save  string `json:"save"`
		Error string `json:"error"`
	}
	_ = json.NewDecoder(res.Body).Decode(&out)
	switch {
	case res.StatusCode == http.StatusForbidden:
		return "", hint("Voll ausgebaut geht nur im Dev-Mode (Server mit K3C_DEV starten)")
	case res.StatusCode != http.StatusOK && out.Error != "":
		return "", fmt.Errorf("%s", out.Error)
	case res.StatusCode != http.StatusOK:
		return "", hint(fmt.Sprintf("Fehler %d vom Spielserver", res.StatusCode))
	case !strings.HasPrefix(out.Save, "voll-") || !gameSaveName.MatchString(out.Save):
		return "", hint("Die Antwort des Servers nennt keinen Spielstand")
	}
	return out.Save, nil
}
