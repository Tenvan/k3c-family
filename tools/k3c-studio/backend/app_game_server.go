package backend

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net"
	"net/http"
	"net/url"
	"os"
	"strconv"
	"strings"
	"time"
)

// „Im Spiel öffnen“ auf einem Spielserver (B-513): statt in der Engine des Studios läuft das Spiel auf dem Spielserver
// dieses Rechners. Erkannt wird er über GET /api/health am Standardport (K3C_HTTP_PORT, sonst 8080) auf Loopback; die
// Heimnetz-Adresse ist nur Anzeige. Angesprochen wird nie eine andere als eine Loopback-Adresse.

// defaultServerPort ist der Port des Spielservers ohne K3C_HTTP_PORT (wie cmd/k3c-server).
const defaultServerPort = "8080"

// gameClient fragt den Spielserver (Gesundheit, Dev-API). Weiterleitungen folgt er nie, sonst ginge eine Anfrage an
// eine fremde Adresse, obwohl nur Loopback erlaubt ist.
var gameClient = &http.Client{
	Timeout:       15 * time.Second,
	CheckRedirect: func(*http.Request, []*http.Request) error { return http.ErrUseLastResponse },
}

// probeTimeout begrenzt die Erkennung, damit die Liste schnell erscheint.
const probeTimeout = 2 * time.Second

// GameServer ist ein Spielserver für „Im Spiel öffnen“: Base ist die Loopback-Adresse (geht ans Binding), LAN und Host
// sind nur Anzeige; Dev meldet den Dev-Mode (nötig für „voll ausgebaut“ und andere Welten).
type GameServer struct {
	Base      string `json:"base"`
	LAN       string `json:"lan"`
	Host      string `json:"host"`
	Reachable bool   `json:"reachable"`
	Dev       bool   `json:"dev"`
	Version   string `json:"version"`
}

// GameServersView sind die geprüften Spielserver.
type GameServersView struct {
	Servers []GameServer `json:"servers"`
}

// gameServerBase ist die Loopback-Adresse des Spielservers: Test-Naht serverBase, sonst Port aus K3C_HTTP_PORT oder 8080.
func (a *App) gameServerBase() string {
	if a.serverBase != "" {
		return a.serverBase
	}
	port := os.Getenv("K3C_HTTP_PORT")
	if n, err := strconv.Atoi(port); err != nil || n < 1 || n > 65535 {
		port = defaultServerPort
	}
	return "http://127.0.0.1:" + port
}

// WorldGameServers prüft den Spielserver dieses Rechners (Binding): erreichbar, Dev-Mode, Version.
func (a *App) WorldGameServers() GameServersView {
	base := a.gameServerBase()
	ctx, cancel := context.WithTimeout(a.context(), probeTimeout)
	defer cancel()
	s := GameServer{Base: base, Host: hostName()}
	if u, err := url.Parse(base); err == nil {
		s.LAN = lanAddr(u.Port())
	}
	var health struct {
		Version string `json:"version"`
	}
	if getJSON(ctx, base+"/api/health", &health) == nil {
		s.Reachable, s.Version = true, health.Version
		s.Dev = serverDev(ctx, base)
	}
	return GameServersView{Servers: []GameServer{s}}
}

// WorldOpenGameServer startet das Level auf dem Spielserver unter base (Binding, nur Loopback): neu oder voll ausgebaut
// (POST /api/dev/fullsave, nur im Dev-Mode des Servers); eine andere als die Standard-Welt nur im Dev-Mode. Die
// Landingpage des Servers öffnet `game.html` mit dem Spielstand.
func (a *App) WorldOpenGameServer(base, seed, biome, mode string) OpenGameResult {
	root, err := loopbackBase(base)
	if err != nil {
		return OpenGameResult{Error: err.Error()}
	}
	world := a.activeWorld()
	if err := checkOpenGame(seed, biome, mode, world); err != nil {
		return OpenGameResult{Error: err.Error()}
	}
	ctx := a.context()
	if err := checkServer(ctx, root); err != nil {
		return OpenGameResult{Error: err.Error()}
	}
	if world != standardWorld && !serverDev(ctx, root) {
		return OpenGameResult{Error: "Andere Welten startet der Spielserver nur im Dev-Mode (Server mit K3C_DEV starten)"}
	}
	save := seed
	if mode == modeFull {
		if save, err = createFullSave(ctx, root, seed); err != nil {
			return OpenGameResult{Error: err.Error()}
		}
	}
	query := url.Values{"autostart": {"1"}, "save": {save}}
	if world != standardWorld {
		query.Set("fresh", "1")
		query.Set("world", world)
	}
	return OpenGameResult{URL: root + "/#" + url.QueryEscape("game.html?"+query.Encode()), Save: save}
}

// errServerDown ist der Hinweis bei einem Server, der nicht antwortet.
func errServerDown(root string) error {
	return hint(fmt.Sprintf("Spielserver läuft nicht (%s). Starte ihn mit `task start` oder wähle die interne Engine", root))
}

// getJSON fragt url per GET und liest die Antwort nach v; jeder Status außer 200 ist ein Fehler.
func getJSON(ctx context.Context, target string, v any) error {
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, target, nil)
	if err != nil {
		return err
	}
	res, err := gameClient.Do(req)
	if err != nil {
		return err
	}
	defer func() { _ = res.Body.Close() }()
	if res.StatusCode != http.StatusOK {
		return fmt.Errorf("status %d", res.StatusCode)
	}
	return json.NewDecoder(res.Body).Decode(v)
}

// checkServer prüft vor dem Öffnen, ob der Spielserver unter root antwortet (GET /api/health).
func checkServer(ctx context.Context, root string) error {
	var health map[string]any
	if getJSON(ctx, root+"/api/health", &health) != nil {
		return errServerDown(root)
	}
	return nil
}

// serverDev fragt den Dev-Mode des Servers (GET /api/dev); jeder Fehler heißt: kein Dev-Mode.
func serverDev(ctx context.Context, root string) bool {
	var out struct {
		Dev bool `json:"dev"`
	}
	return getJSON(ctx, root+"/api/dev", &out) == nil && out.Dev
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
		return "", hint("Voll ausgebaut geht nur im Dev-Mode des Spielservers (Server mit K3C_DEV starten)")
	case res.StatusCode != http.StatusOK && out.Error != "":
		return "", hint(out.Error)
	case res.StatusCode != http.StatusOK:
		return "", hint(fmt.Sprintf("Fehler %d vom Spielserver", res.StatusCode))
	case !strings.HasPrefix(out.Save, "voll-") || !gameSaveName.MatchString(out.Save):
		return "", hint("Die Antwort des Servers nennt keinen Spielstand")
	}
	return out.Save, nil
}

// hostName ist der Rechnername (nur Anzeige), leer wenn unbekannt.
func hostName() string {
	name, _ := os.Hostname()
	return name
}

// lanAddr ist die erste private IPv4-Adresse dieses Rechners mit port (nur Anzeige: so erreichen Geräte im Heimnetz den
// Server), leer ohne eine.
func lanAddr(port string) string {
	addrs, err := net.InterfaceAddrs()
	if err != nil {
		return ""
	}
	for _, addr := range addrs {
		if ipnet, ok := addr.(*net.IPNet); ok && ipnet.IP.To4() != nil && ipnet.IP.IsPrivate() {
			return net.JoinHostPort(ipnet.IP.String(), port)
		}
	}
	return ""
}
