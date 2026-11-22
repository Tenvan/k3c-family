// Package uikit ist die HTTP-Schicht der Oberfläche einer Wails-App im Client-Server-Betrieb (-headless, -dev):
// dieselben Methoden wie die Wails-Bindings, als POST auf Prefix+"rpc/{Method}" mit den Argumenten als JSON-Array;
// Antwort {result} oder {error}. Ereignisse kommen als Server-Sent Events (events.go). Gemeinsam für k3c-workbench
// und k3c-studio (WZ63.3); die Oberfläche spricht sie über tools/webkit/lib/devRpc.ts an.
package uikit

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net"
	"net/http"
	"net/url"
	"reflect"
	"strings"
)

// Prefix aller Adressen der Oberfläche.
const Prefix = "/api/dev/"

// Header verlangt jeder POST: ein fremder Web-Auftritt kann ihn ohne Vorabfrage (CORS-Preflight) nicht setzen, der
// Server beantwortet keine. Die Apps starten Prozesse, deshalb dazu nur Loopback, nur localhost/Loopback-IP als Host
// und nur Anfragen derselben Seite (SameSite), auch von anderen lokalen Seiten (Spiel, andere App) nicht.
const Header = "X-K3C-Dev"

// MaxRPCBytes begrenzt die Größe eines Aufrufs (Prompt für Claude, Task-Argumente).
const MaxRPCBytes = 1 << 20

// API beantwortet die Aufrufe der Oberfläche für Target (die App mit den Bindings).
type API struct {
	App     string          // Name in Meldungen, z. B. "k3c-workbench"
	Target  any             // Empfänger der Methoden (Zeiger auf die App)
	Methods map[string]bool // freigegebene Methoden: genau die von GoApp in frontend/src/api/wails.ts (CheckMethods)
	Bus     *Bus            // Ereignisse für den Strom
	// Track misst einen Aufruf (Ressourcen-Monitor der Workbench); nil misst nichts.
	Track func(name string) func()
}

// Handler beantwortet alle Adressen unter Prefix, die Oberfläche selbst liefert Vite.
func (api *API) Handler() http.Handler {
	mux := http.NewServeMux()
	mux.HandleFunc("GET "+Prefix+"events", api.Bus.serve)
	mux.HandleFunc("POST "+Prefix+"rpc/{method}", api.rpc)
	mux.HandleFunc("GET "+Prefix, func(w http.ResponseWriter, _ *http.Request) { Fail(w, http.StatusNotFound, "Unbekannte Adresse") })
	return Guard(api.App, mux)
}

// Guard lässt nur Anfragen dieses Rechners an derselben Seite durch (siehe Header).
func Guard(app string, next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch {
		case !Loopback(r.RemoteAddr):
			Fail(w, http.StatusForbidden, app+" ist nur auf diesem Rechner erreichbar")
		case r.URL.RawPath != "": // kodierter Pfad: so schlüpft eine Anfrage an einem Filter vorbei
			Fail(w, http.StatusForbidden, "Kodierter Pfad nicht erlaubt")
		case !HostAllowed(r.Host): // DNS-Rebinding: ein fremder Name, der auf diesen Rechner zeigt
			Fail(w, http.StatusForbidden, app+" antwortet nur auf localhost oder eine Loopback-Adresse")
		case !SameSite(r): // eine andere Seite im Browser, auch eine lokale auf einem anderen Port
			Fail(w, http.StatusForbidden, "Anfrage einer fremden Seite")
		case r.Method == http.MethodPost && r.Header.Get(Header) != "1":
			Fail(w, http.StatusForbidden, "Kopfzeile "+Header+": 1 fehlt")
		default:
			next.ServeHTTP(w, r)
		}
	})
}

// HostAllowed erlaubt localhost und Loopback-IPs, nie einen anderen Namen oder eine LAN-Adresse.
func HostAllowed(hostport string) bool {
	host, _, err := net.SplitHostPort(hostport)
	if err != nil {
		host = strings.Trim(hostport, "[]")
	}
	ip := net.ParseIP(host)
	return host == "localhost" || (ip != nil && ip.IsLoopback())
}

// SameSite: Origin (falls gesendet) ist genau der Host der Anfrage, Sec-Fetch-Site (falls gesendet) same-origin oder
// none. Hinter Vite (changeOrigin aus) sind beide localhost:<Vite-Port>; curl und Go-Clients senden keines.
func SameSite(r *http.Request) bool {
	if o := r.Header.Get("Origin"); o != "" {
		u, err := url.Parse(o)
		if err != nil || u.Host != r.Host {
			return false
		}
	}
	site := r.Header.Get("Sec-Fetch-Site")
	return site == "" || site == "same-origin" || site == "none"
}

// Loopback meldet, ob die Gegenstelle auf diesem Rechner liegt.
func Loopback(remoteAddr string) bool {
	host, _, err := net.SplitHostPort(remoteAddr)
	if err != nil {
		host = remoteAddr
	}
	ip := net.ParseIP(host)
	return ip != nil && ip.IsLoopback()
}

// Reply antwortet {"result": …} bzw. {"error": …}.
func Reply(w http.ResponseWriter, status int, result any, err error) {
	body := map[string]any{"result": result}
	if err != nil {
		body = map[string]any{"error": err.Error()}
	}
	w.Header().Set("Content-Type", "application/json")
	w.Header().Set("Cache-Control", "no-store")
	w.WriteHeader(status)
	_ = json.NewEncoder(w).Encode(body)
}

// Fail antwortet mit einem Fehlertext.
func Fail(w http.ResponseWriter, status int, msg string) { Reply(w, status, nil, errors.New(msg)) }

// rpc ruft die Methode {method} von Target mit den JSON-Argumenten aus dem Rumpf (ein Array) auf.
func (api *API) rpc(w http.ResponseWriter, r *http.Request) {
	name := r.PathValue("method")
	m := reflect.ValueOf(api.Target).MethodByName(name)
	if !api.Methods[name] || !m.IsValid() {
		Fail(w, http.StatusNotFound, "Unbekannte Methode "+name)
		return
	}
	if api.Track != nil {
		defer api.Track("ui:" + name)()
	}
	var raw []json.RawMessage
	if err := json.NewDecoder(io.LimitReader(r.Body, MaxRPCBytes)).Decode(&raw); err != nil {
		Fail(w, http.StatusBadRequest, "Argumente ungültig: "+err.Error())
		return
	}
	args, err := decodeArgs(m.Type(), raw)
	if err != nil {
		Fail(w, http.StatusBadRequest, err.Error())
		return
	}
	result, err := splitResult(m.Call(args))
	if err != nil {
		Reply(w, http.StatusUnprocessableEntity, nil, err)
		return
	}
	Reply(w, http.StatusOK, result, nil)
}

func decodeArgs(t reflect.Type, raw []json.RawMessage) ([]reflect.Value, error) {
	if len(raw) != t.NumIn() {
		return nil, fmt.Errorf("erwartet %d Argumente, bekommen %d", t.NumIn(), len(raw))
	}
	args := make([]reflect.Value, len(raw))
	for i, r := range raw {
		p := reflect.New(t.In(i))
		if err := json.Unmarshal(r, p.Interface()); err != nil {
			return nil, fmt.Errorf("ungültiges Argument %d: %w", i+1, err)
		}
		args[i] = p.Elem()
	}
	return args, nil
}

var errorType = reflect.TypeFor[error]()

// splitResult trennt den Wert vom Fehler (Rückgabewert vom Typ error).
func splitResult(out []reflect.Value) (any, error) {
	var result any
	for _, o := range out {
		if o.Type() != errorType {
			result = o.Interface()
		} else if !o.IsNil() {
			return nil, o.Interface().(error)
		}
	}
	return result, nil
}
