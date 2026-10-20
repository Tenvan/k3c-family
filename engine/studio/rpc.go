package studio

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net"
	"net/http"
	"os"
	"path/filepath"
	"reflect"
	"strings"
	"sync"
	"time"

	assets "k3c/tools/assetkit"
)

// Prefix aller Adressen des Studios am Server.
const Prefix = "/api/studio/"

// header verlangt jede schreibende Anfrage (POST): ein fremder Web-Auftritt kann ihn ohne Vorabfrage (CORS-Preflight)
// nicht setzen, der Server beantwortet keine. So kann keine fremde Seite im Browser des Nutzers das Studio bedienen.
const header = "X-K3C-Studio"

const (
	maxRPCBytes    = 4 << 20  // Größe eines Aufrufs (Chunk-Pläne, Gebäude-Planer)
	maxImportBytes = 64 << 20 // Größe einer importierten Datei
)

// rpcPrefixes nennen die Methoden von App, die die Oberfläche aufrufen darf.
var rpcPrefixes = []string{"Assets", "World", "Plan", "Hub", "Chat"}

// bus verteilt Ereignisse (assets:changed) an die offenen Event-Ströme.
type bus struct {
	mu   sync.Mutex
	subs map[chan string]struct{}
}

func newBus() *bus { return &bus{subs: map[chan string]struct{}{}} }

func (b *bus) publish(name string) {
	b.mu.Lock()
	defer b.mu.Unlock()
	for c := range b.subs {
		select {
		case c <- name:
		default: // voller Puffer: die Oberfläche lädt ohnehin neu
		}
	}
}

func (b *bus) subscribe() (chan string, func()) {
	c := make(chan string, 8)
	b.mu.Lock()
	b.subs[c] = struct{}{}
	b.mu.Unlock()
	return c, func() { b.mu.Lock(); delete(b.subs, c); b.mu.Unlock() }
}

// Handler beantwortet alle Adressen unter Prefix. Er erreicht nur Loopback (das Studio schreibt in den Checkout);
// remote erlaubt weitere Adressen (Umgebung K3C_STUDIO_LAN).
func (a *App) Handler(remote bool) http.Handler {
	mux := http.NewServeMux()
	mux.HandleFunc("GET "+Prefix+"ping", func(w http.ResponseWriter, _ *http.Request) { reply(w, http.StatusOK, "ok", nil) })
	mux.HandleFunc("GET "+Prefix+"events", a.events)
	mux.HandleFunc("POST "+Prefix+"rpc/{method}", a.rpc)
	mux.HandleFunc("POST "+Prefix+"import", a.importFile)
	mux.HandleFunc("POST "+Prefix+"chat", a.chatSend)
	mux.Handle("GET "+Prefix+"public/", http.StripPrefix(strings.TrimSuffix(Prefix, "/")+"/public", publicHandler(a.root)))
	return guard(mux, remote)
}

func guard(next http.Handler, remote bool) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if !remote && !isLoopback(r.RemoteAddr) {
			fail(w, http.StatusForbidden, "Das Studio ist nur auf dem Rechner des Servers erreichbar")
			return
		}
		if r.URL.RawPath != "" { // kodierter Pfad: so schlüpfte eine Anfrage an einem Filter vor dem Server vorbei
			fail(w, http.StatusForbidden, "Kodierter Pfad nicht erlaubt")
			return
		}
		if !hostAllowed(r.Host, remote) { // DNS-Rebinding: ein fremder Name, der auf diesen Rechner zeigt
			fail(w, http.StatusForbidden, "Das Studio antwortet nur auf localhost oder eine IP-Adresse")
			return
		}
		if r.Method == http.MethodPost && r.Header.Get(header) == "" {
			fail(w, http.StatusForbidden, "Kopfzeile "+header+" fehlt")
			return
		}
		next.ServeHTTP(w, r)
	})
}

// hostAllowed erlaubt localhost und Loopback-IPs; mit remote (LAN) jede IP-Adresse, nie einen anderen Namen.
func hostAllowed(hostport string, remote bool) bool {
	host, _, err := net.SplitHostPort(hostport)
	if err != nil {
		host = strings.Trim(hostport, "[]")
	}
	ip := net.ParseIP(host)
	return host == "localhost" || (ip != nil && (remote || ip.IsLoopback()))
}

func isLoopback(remoteAddr string) bool {
	host, _, err := net.SplitHostPort(remoteAddr)
	if err != nil {
		host = remoteAddr
	}
	ip := net.ParseIP(host)
	return ip != nil && ip.IsLoopback()
}

// reply antwortet {"result": …} bzw. {"error": …}.
func reply(w http.ResponseWriter, status int, result any, err error) {
	body := map[string]any{"result": result}
	if err != nil {
		body = map[string]any{"error": err.Error()}
	}
	w.Header().Set("Content-Type", "application/json")
	w.Header().Set("Cache-Control", "no-store")
	w.WriteHeader(status)
	_ = json.NewEncoder(w).Encode(body)
}

func fail(w http.ResponseWriter, status int, msg string) { reply(w, status, nil, errors.New(msg)) }

// rpc ruft die Methode {method} von App mit den JSON-Argumenten aus dem Rumpf (ein Array) auf.
func (a *App) rpc(w http.ResponseWriter, r *http.Request) {
	name := r.PathValue("method")
	m := reflect.ValueOf(a).MethodByName(name)
	if !allowed(name) || !m.IsValid() {
		fail(w, http.StatusNotFound, "Unbekannte Methode "+name)
		return
	}
	if strings.HasPrefix(name, "Chat") && !isLoopback(r.RemoteAddr) { // startet ein Modell mit der Anmeldung von 🧑
		fail(w, http.StatusForbidden, chatLoopbackMsg)
		return
	}
	var raw []json.RawMessage
	if err := json.NewDecoder(io.LimitReader(r.Body, maxRPCBytes)).Decode(&raw); err != nil {
		fail(w, http.StatusBadRequest, "Argumente ungültig: "+err.Error())
		return
	}
	args, err := decodeArgs(m.Type(), raw)
	if err != nil {
		fail(w, http.StatusBadRequest, err.Error())
		return
	}
	result, err := splitResult(m.Call(args))
	if err != nil {
		reply(w, http.StatusUnprocessableEntity, nil, err)
		return
	}
	reply(w, http.StatusOK, result, nil)
}

func allowed(name string) bool {
	for _, p := range rpcPrefixes {
		if strings.HasPrefix(name, p) {
			return true
		}
	}
	return false
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

var errorType = reflect.TypeOf((*error)(nil)).Elem()

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

// events ist ein Server-Sent-Events-Strom mit den Ereignissen des Studios.
func (a *App) events(w http.ResponseWriter, r *http.Request) {
	fl, ok := w.(http.Flusher)
	if !ok {
		fail(w, http.StatusInternalServerError, "kein Strom möglich")
		return
	}
	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-store")
	ch, unsub := a.bus.subscribe()
	defer unsub()
	_, _ = fmt.Fprint(w, ": offen\n\n")
	fl.Flush()
	tick := time.NewTicker(20 * time.Second)
	defer tick.Stop()
	for {
		select {
		case <-r.Context().Done():
			return
		case name := <-ch:
			_, _ = fmt.Fprintf(w, "event: %s\ndata: {}\n\n", name)
		case <-tick.C:
			_, _ = fmt.Fprint(w, ": halten\n\n")
		}
		fl.Flush()
	}
}

// importFile übernimmt die im Rumpf hochgeladene Datei zu einem Recherche-Treffer (Abfrage id, hit, name = Dateiname).
// Die Endung bestimmt die Art; ImportFile prüft Größe, Format und Lizenz.
func (a *App) importFile(w http.ResponseWriter, r *http.Request) {
	q := r.URL.Query()
	ext := strings.ToLower(filepath.Ext(filepath.Base(q.Get("name"))))
	if ext == "" || len(ext) > 5 || strings.ContainsAny(ext, `/\`) {
		fail(w, http.StatusBadRequest, "Dateiname ohne gültige Endung")
		return
	}
	dir, err := os.MkdirTemp("", "k3c-studio-import-")
	if err != nil {
		fail(w, http.StatusInternalServerError, err.Error())
		return
	}
	defer func() { _ = os.RemoveAll(dir) }()
	src := filepath.Join(dir, "upload"+ext)
	if err := saveBody(src, http.MaxBytesReader(w, r.Body, maxImportBytes)); err != nil {
		fail(w, http.StatusBadRequest, err.Error())
		return
	}
	rel, err := assets.ImportFile(a.root, q.Get("id"), q.Get("hit"), src, time.Now())
	if err != nil {
		reply(w, http.StatusUnprocessableEntity, nil, err)
		return
	}
	reply(w, http.StatusOK, rel, nil)
}

func saveBody(path string, body io.Reader) error {
	f, err := os.Create(path)
	if err != nil {
		return err
	}
	_, werr := io.Copy(f, body)
	if cerr := f.Close(); werr == nil {
		werr = cerr
	}
	if werr != nil {
		return errors.New("Datei zu groß oder unvollständig: " + werr.Error())
	}
	return nil
}
