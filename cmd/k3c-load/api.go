package main

import (
	"context"
	"encoding/json"
	"net/http"
	"strings"
	"time"
)

// api fragt GET /api/status (Bearer-Token). Das Token steht nie in einem Fehlertext.
type api struct {
	base, token string
	http        *http.Client
}

func newAPI(base, token string) *api {
	return &api{base: base, token: token, http: &http.Client{Timeout: 5 * time.Second}}
}

// statusRoom ist der Teil einer Raumzeile aus /api/status, den das Werkzeug braucht.
type statusRoom struct {
	Code    string `json:"code"`
	Name    string `json:"name"`
	Devices int    `json:"devices"`
	Tick    int    `json:"tick"`
	TickMs  struct {
		Last float64 `json:"last"`
		P99  float64 `json:"p99"`
	} `json:"tickMs"`
	Stages []struct {
		Phase string `json:"phase"`
	} `json:"stages"`
}

// phase ist die Phase der ersten Stufe (die Bots spielen dort).
func (r statusRoom) phase() string {
	if len(r.Stages) == 0 {
		return ""
	}
	return r.Stages[0].Phase
}

// serverStatus ist der Teil von /api/status, den das Werkzeug braucht; CPU fehlt, wo der Server keine Quelle hat.
type serverStatus struct {
	Rooms []statusRoom `json:"rooms"`
	CPU   *float64     `json:"cpu"`
}

func (a *api) rooms(ctx context.Context) ([]statusRoom, error) {
	st, err := a.status(ctx)
	return st.Rooms, err
}

func (a *api) status(ctx context.Context) (serverStatus, error) {
	var body serverStatus
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, a.base+"/api/status", nil)
	if err != nil {
		return body, failf("Adresse ungültig: %s", a.base)
	}
	req.Header.Set("Authorization", "Bearer "+a.token)
	res, err := a.http.Do(req)
	if err != nil {
		return body, failf("Server nicht erreichbar unter %s", a.base)
	}
	defer func() { _ = res.Body.Close() }()
	switch res.StatusCode {
	case http.StatusOK:
	case http.StatusUnauthorized:
		return body, failf("Token falsch (401 von %s)", a.base)
	case http.StatusNotFound:
		return body, failf("Diagnose am Server aus (%s dort nicht gesetzt)", envToken)
	default:
		return body, failf("Server antwortet mit %d", res.StatusCode)
	}
	if err := json.NewDecoder(res.Body).Decode(&body); err != nil {
		return body, failf("Status von %s nicht lesbar", a.base)
	}
	return body, nil
}

// waitDisconnected wartet bis zu 5 s, bis kein Raum mit prefix mehr ein verbundenes Gerät hat.
func (a *api) waitDisconnected(ctx context.Context, prefix string) error {
	var open []string
	for range 50 {
		rooms, err := a.rooms(ctx)
		if err != nil {
			return err
		}
		open = open[:0]
		for _, r := range rooms {
			if strings.HasPrefix(r.Name, prefix) && r.Devices > 0 {
				open = append(open, r.Name)
			}
		}
		if len(open) == 0 {
			return nil
		}
		time.Sleep(100 * time.Millisecond)
	}
	return failf("Test-Räume noch verbunden: %s", strings.Join(open, ", "))
}
