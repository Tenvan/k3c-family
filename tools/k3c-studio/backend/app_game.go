package backend

import (
	"errors"
	"fmt"
	"net"
	"net/url"
	"regexp"

	"k3c/engine/room"
)

// „Im Spiel öffnen“ des Welt-Betrachters (B-460, WZ7.4, AC-06): das Level neu oder voll ausgebaut im Browser starten,
// über die Landingpage des Studio-Servers mit eingebundener Engine (B-507/AC-02). Nur Loopback: das Studio öffnet nie
// eine fremde Adresse.

const (
	modeNew  = "new"  // neues Spiel mit diesem Seed (die Lobby lädt einen vorhandenen Spielstand gleichen Namens)
	modeFull = "full" // Spielstand „voll ausgebaut“ in der Engine des Studios anlegen und laden (B-315)
)

var (
	// Namensformat eines Spielstands (wie SAVE_FORMAT in src/scenes/lobbyLogic.ts): der Seed eines neuen Spiels ist dieser Name.
	gameSaveName = regexp.MustCompile(`^[a-z0-9-]{1,32}$`)
	// Für „voll ausgebaut“ muss `voll-<seed>` ins Namensformat passen (wie fullSeed in engine/room).
	gameFullSeed = regexp.MustCompile(`^[a-z0-9-]{1,27}$`)
)

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
// (`full`: legt den Spielstand `voll-<seed>` in der eingebundenen Engine an, B-507/AC-02). Die Landingpage unter base
// (der Server des Studios) öffnet danach `game.html` mit diesem Spielstand. Ist eine andere als die Standard-Welt
// gewählt, startet `new` ein frisches Spiel in ihr (`fresh=1&world=<id>`, WZ18.6). Kein Spielserver nötig.
func (a *App) WorldOpenGame(base, seed, biome, mode string) OpenGameResult {
	root, err := loopbackBase(base)
	if err != nil {
		return OpenGameResult{Error: err.Error()}
	}
	world := a.activeWorld()
	if err := checkOpenGame(seed, biome, mode, world); err != nil {
		return OpenGameResult{Error: err.Error()}
	}
	if a.engine == nil {
		return OpenGameResult{Error: errNoEngine.Error()}
	}
	save := seed
	if mode == modeFull {
		if save, err = a.engine.Rooms.FullSave(seed); err != nil {
			return OpenGameResult{Error: fullSaveHint(err).Error()}
		}
	}
	query := url.Values{"autostart": {"1"}, "save": {save}}
	if world != standardWorld { // neues Spiel in der gewählten Welt (WZ18.5)
		query.Set("fresh", "1")
		query.Set("world", world)
	}
	return OpenGameResult{URL: gameURL(root, query), Save: save}
}

// errNoEngine ist der Hinweis eines Studios ohne eingebundene Engine (nur Tests ohne New).
const errNoEngine = hint("Das Studio hat keine Engine gestartet")

// checkOpenGame prüft Art, Biom und Seed vor dem Start.
func checkOpenGame(seed, biome, mode, world string) error {
	switch mode {
	case modeNew:
		if biome != "forest" {
			return hint("Start nur für den Wald (Tiefe 0)")
		}
		if !gameSaveName.MatchString(seed) {
			return hint("Seed muss aus Kleinbuchstaben, Ziffern und - bestehen (höchstens 32 Zeichen)")
		}
	case modeFull:
		if world != standardWorld {
			return hint("Voll ausgebaut gibt es nur für die Standard-Welt")
		}
		if !gameFullSeed.MatchString(seed) {
			return hint("Für „voll ausgebaut“ muss der Seed aus Kleinbuchstaben, Ziffern und - bestehen (höchstens 27 Zeichen)")
		}
	default:
		return hint(fmt.Sprintf("Unbekannte Art %q (new, full)", mode))
	}
	return nil
}

// gameURL ist die Landingpage unter root mit `game.html?<query>` im Fragment. index.html ausdrücklich: die Wurzel des
// Studio-Servers führt zum Studio.
func gameURL(root string, query url.Values) string {
	return root + "/index.html#" + url.QueryEscape("game.html?"+query.Encode())
}

// fullSaveHint macht aus einem Fehler der Engine beim Anlegen von „voll ausgebaut“ einen Hinweis.
func fullSaveHint(err error) error {
	switch {
	case errors.Is(err, room.ErrSaveInUse):
		return hint("Spielstand ist gerade in einem Raum mit verbundenem Gerät offen")
	case errors.Is(err, room.ErrBadRequest):
		return hint("Seed ungültig")
	}
	return hint(fmt.Sprintf("Spielstand nicht angelegt: %v", err))
}
