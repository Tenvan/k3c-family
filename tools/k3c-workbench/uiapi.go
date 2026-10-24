package main

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

// HTTP-Schicht der Oberfläche für den Client-Server-Betrieb (-headless, -dev): dieselben Methoden wie die
// Wails-Bindings, als POST auf uiPrefix+"rpc/{Method}" mit den Argumenten als JSON-Array; Antwort {result} oder
// {error}. Ereignisse kommen als Server-Sent Events (uiapi_events.go). Vorbild: tools/k3c-studio/backend/rpc.go.

// uiPrefix aller Adressen der Oberfläche.
const uiPrefix = "/api/dev/"

// uiHeader verlangt jeder POST: ein fremder Web-Auftritt kann ihn ohne Vorabfrage (CORS-Preflight) nicht setzen, der
// Server beantwortet keine. k3c-workbench startet Prozesse, deshalb dazu nur Loopback, nur localhost/Loopback-IP als Host
// und nur Anfragen derselben Seite (uiSameSite), auch von anderen lokalen Seiten (Spiel, Studio) nicht.
const uiHeader = "X-K3C-Dev"

const maxUIRPCBytes = 1 << 20 // Größe eines Aufrufs (Prompt für Claude, Task-Argumente)

// uiMethods sind die Methoden von App, die die Oberfläche aufrufen darf: genau die von GoApp in
// frontend/src/api/wails.ts (TestUIMethodsWieWailsTS hält beide gleich). TaskList, TaskState u. a. bleiben MCP-intern.
var uiMethods = map[string]bool{
	"Info": true, "Services": true, "ServicesReload": true, "ServiceStart": true, "ServiceStop": true,
	"ServiceRestart": true, "ServicesStartAll": true, "ServicesStopAll": true, "ServiceLogLevels": true,
	"Sources": true, "ConsoleTail": true, "LogsQuery": true, "LogsErrors": true,
	"McpOverview": true, "McpRestart": true, "McpInstructions": true, "McpCalls": true, "McpUsage": true,
	"Tasks": true, "TasksReload": true, "TaskStart": true, "TaskStop": true, "TaskRuns": true,
	"TaskGatesView": true, "TaskGateToggle": true, "TaskGateSave": true,
	"PlanningData": true, "PlanningDocs": true, "PlanningSet": true, "PlanningDoc": true, "PlanningGet": true,
	"GitHubStatus": true, "WorktreeCheck": true, "OpenInClaude": true, "StudioOpen": true, "Resources": true,
	"GitOverview": true, "GitAction": true, "GitDeleteBranches": true, "GitPRs": true, "GitPRCreate": true, "GitPRAction": true,
	"GitCheckMerged": true, "GitStamp": true, "TicketCreate": true, "TicketRecent": true,
}

// uiHandler beantwortet alle Adressen unter uiPrefix.
func (a *App) uiHandler() http.Handler {
	mux := http.NewServeMux()
	mux.HandleFunc("GET "+uiPrefix+"events", a.uiEvents)
	mux.HandleFunc("POST "+uiPrefix+"rpc/{method}", a.uiRPC)
	return uiGuard(mux)
}

func uiGuard(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch {
		case !uiLoopback(r.RemoteAddr):
			uiFail(w, http.StatusForbidden, "k3c-workbench ist nur auf diesem Rechner erreichbar")
		case r.URL.RawPath != "": // kodierter Pfad: so schlüpft eine Anfrage an einem Filter vorbei
			uiFail(w, http.StatusForbidden, "Kodierter Pfad nicht erlaubt")
		case !uiHostAllowed(r.Host): // DNS-Rebinding: ein fremder Name, der auf diesen Rechner zeigt
			uiFail(w, http.StatusForbidden, "k3c-workbench antwortet nur auf localhost oder eine Loopback-Adresse")
		case !uiSameSite(r): // eine andere Seite im Browser, auch eine lokale auf einem anderen Port
			uiFail(w, http.StatusForbidden, "Anfrage einer fremden Seite")
		case r.Method == http.MethodPost && r.Header.Get(uiHeader) != "1":
			uiFail(w, http.StatusForbidden, "Kopfzeile "+uiHeader+": 1 fehlt")
		default:
			next.ServeHTTP(w, r)
		}
	})
}

// uiHostAllowed erlaubt localhost und Loopback-IPs, nie einen anderen Namen oder eine LAN-Adresse.
func uiHostAllowed(hostport string) bool {
	host, _, err := net.SplitHostPort(hostport)
	if err != nil {
		host = strings.Trim(hostport, "[]")
	}
	ip := net.ParseIP(host)
	return host == "localhost" || (ip != nil && ip.IsLoopback())
}

// uiSameSite: Origin (falls gesendet) ist genau der Host der Anfrage, Sec-Fetch-Site (falls gesendet) same-origin oder
// none. Hinter Vite (changeOrigin aus) sind beide localhost:5191; curl und Go-Clients senden keines. Vorbild: gameGuard
// in tools/k3c-studio/server.go.
func uiSameSite(r *http.Request) bool {
	if o := r.Header.Get("Origin"); o != "" {
		u, err := url.Parse(o)
		if err != nil || u.Host != r.Host {
			return false
		}
	}
	site := r.Header.Get("Sec-Fetch-Site")
	return site == "" || site == "same-origin" || site == "none"
}

func uiLoopback(remoteAddr string) bool {
	host, _, err := net.SplitHostPort(remoteAddr)
	if err != nil {
		host = remoteAddr
	}
	ip := net.ParseIP(host)
	return ip != nil && ip.IsLoopback()
}

// uiReply antwortet {"result": …} bzw. {"error": …}.
func uiReply(w http.ResponseWriter, status int, result any, err error) {
	body := map[string]any{"result": result}
	if err != nil {
		body = map[string]any{"error": err.Error()}
	}
	w.Header().Set("Content-Type", "application/json")
	w.Header().Set("Cache-Control", "no-store")
	w.WriteHeader(status)
	_ = json.NewEncoder(w).Encode(body)
}

func uiFail(w http.ResponseWriter, status int, msg string) { uiReply(w, status, nil, errors.New(msg)) }

// uiRPC ruft die Methode {method} von App mit den JSON-Argumenten aus dem Rumpf (ein Array) auf.
func (a *App) uiRPC(w http.ResponseWriter, r *http.Request) {
	name := r.PathValue("method")
	m := reflect.ValueOf(a).MethodByName(name)
	if !uiMethods[name] || !m.IsValid() {
		uiFail(w, http.StatusNotFound, "Unbekannte Methode "+name)
		return
	}
	defer a.res.Track("ui:" + name)()
	var raw []json.RawMessage
	if err := json.NewDecoder(io.LimitReader(r.Body, maxUIRPCBytes)).Decode(&raw); err != nil {
		uiFail(w, http.StatusBadRequest, "Argumente ungültig: "+err.Error())
		return
	}
	args, err := uiDecodeArgs(m.Type(), raw)
	if err != nil {
		uiFail(w, http.StatusBadRequest, err.Error())
		return
	}
	result, err := uiSplitResult(m.Call(args))
	if err != nil {
		uiReply(w, http.StatusUnprocessableEntity, nil, err)
		return
	}
	uiReply(w, http.StatusOK, result, nil)
}

func uiDecodeArgs(t reflect.Type, raw []json.RawMessage) ([]reflect.Value, error) {
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

var uiErrorType = reflect.TypeFor[error]()

// uiSplitResult trennt den Wert vom Fehler (Rückgabewert vom Typ error).
func uiSplitResult(out []reflect.Value) (any, error) {
	var result any
	for _, o := range out {
		if o.Type() != uiErrorType {
			result = o.Interface()
		} else if !o.IsNil() {
			return nil, o.Interface().(error)
		}
	}
	return result, nil
}
