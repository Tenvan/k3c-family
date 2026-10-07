// Package serverapi liest die Diagnose-Schnittstelle des Go-Servers (GET /api/status, B-027). Nur lesend.
// Die JSON-Formen entsprechen engine/room (Status, Summary, Failure); k3c-dev ist ein eigenes Modul und hält sie lokal.
package serverapi

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"net/url"
	"time"
)

// Umgebung wie beim Server: Token K3C_STATUS_TOKEN, Adresse K3C_SERVER_URL, sonst 127.0.0.1:K3C_HTTP_PORT (8080).
const (
	EnvURL   = "K3C_SERVER_URL"
	EnvPort  = "K3C_HTTP_PORT"
	EnvToken = "K3C_STATUS_TOKEN"
)

// Client fragt einen laufenden Server. Das Token wird nie in Fehlermeldungen oder Texten genannt.
type Client struct {
	Base  string
	Token string
	HTTP  *http.Client
}

// FromEnv baut den Client aus der Umgebung (get = os.Getenv).
func FromEnv(get func(string) string) *Client {
	base := get(EnvURL)
	if base == "" {
		port := get(EnvPort)
		if port == "" {
			port = "8080"
		}
		base = "http://127.0.0.1:" + port
	}
	return &Client{Base: base, Token: get(EnvToken), HTTP: &http.Client{Timeout: 5 * time.Second}}
}

// TickMs ist die Tick-Dauer in Millisekunden.
type TickMs struct {
	Last float64 `json:"last"`
	P99  float64 `json:"p99"`
}

// Room ist ein Raum in /api/status.
type Room struct {
	Code     string   `json:"code"`
	Name     string   `json:"name"`
	Depth    int      `json:"depth"`
	Taken    int      `json:"taken"`
	Free     int      `json:"free"`
	Running  bool     `json:"running"`
	Devices  int      `json:"devices"`
	Monarchs []string `json:"monarchs"`
	Tick     int      `json:"tick"`
	TickMs   TickMs   `json:"tickMs"`
}

// Failure ist ein abgestürzter Raum.
type Failure struct {
	Code  string `json:"code"`
	Name  string `json:"name"`
	Error string `json:"error"`
	At    string `json:"at"`
}

// Status ist /api/status.
type Status struct {
	Version  string    `json:"version"`
	UptimeS  int64     `json:"uptimeS"`
	Saves    int       `json:"saves"`
	Reports  int       `json:"reports"`
	Rooms    []Room    `json:"rooms"`
	Failures []Failure `json:"failures"`
	CPU      *float64  `json:"cpu"` // Prozent einer CPU; fehlt, wo der Server keine Quelle hat (B-175)
}

// Summary ist /api/status?room=CODE.
type Summary struct {
	Code    string         `json:"code"`
	Depth   int            `json:"depth"`
	Tick    int            `json:"tick"`
	Phase   string         `json:"phase"`
	Day     int            `json:"day"`
	Gold    []int          `json:"gold"`
	Troops  map[string]int `json:"troops"`
	Enemies int            `json:"enemies"`
	Castle  float64        `json:"castleHp"`
	Wave    int            `json:"wave"`
}

// Status liefert den Zustand des Servers.
func (c *Client) Status(ctx context.Context) (Status, error) {
	var st Status
	err := c.get(ctx, "/api/status", &st)
	return st, err
}

// Room liefert den verdichteten Zustand eines Raums.
func (c *Client) Room(ctx context.Context, code string) (Summary, error) {
	var s Summary
	err := c.get(ctx, "/api/status?room="+url.QueryEscape(code), &s)
	if errors.Is(err, errRoomMissing) {
		return s, failf("Raum %s nicht gefunden", code)
	}
	return s, err
}

var errRoomMissing = errors.New("raum fehlt")

func (c *Client) get(ctx context.Context, path string, out any) error {
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, c.Base+path, nil)
	if err != nil {
		return failf("Adresse ungültig: %s", c.Base)
	}
	req.Header.Set("Authorization", "Bearer "+c.Token)
	res, err := c.HTTP.Do(req)
	if err != nil {
		return failf("Server nicht erreichbar unter %s", c.Base)
	}
	defer func() { _ = res.Body.Close() }()
	switch res.StatusCode {
	case http.StatusOK:
		if err := json.NewDecoder(res.Body).Decode(out); err != nil {
			return failf("Antwort von %s nicht lesbar", c.Base)
		}
		return nil
	case http.StatusUnauthorized:
		return failf("401, Token prüfen (%s)", EnvToken)
	case http.StatusNotFound:
		// Mit Token antwortet der Server auf einen unbekannten Raum mit {"error":"Raum nicht gefunden"}, ohne Token mit leerem 404.
		var body struct {
			Error string `json:"error"`
		}
		if json.NewDecoder(res.Body).Decode(&body) == nil && body.Error == "Raum nicht gefunden" {
			return errRoomMissing
		}
		return failf("Diagnose am Server aus (%s dort nicht gesetzt)", EnvToken)
	}
	return failf("Server antwortet mit %d", res.StatusCode)
}

// meldung ist ein Fehlertext für Menschen (großgeschrieben, deutsch), kein Go-Fehlertext.
type meldung string

func (m meldung) Error() string { return string(m) }

func failf(format string, args ...any) error { return meldung(fmt.Sprintf(format, args...)) }
