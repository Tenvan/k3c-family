package mcpsrv

import (
	"context"
	"errors"
	"fmt"
	"io"
	"net"
	"net/http"
	"net/netip"
	"net/url"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"syscall"
	"time"
)

// Entfernte Server (B-771): Die logs_*-Tools lesen mit `server` den Dev-Endpunkt eines laufenden Spielservers (Raspberry Pi) statt der
// lokalen Dateien. Nur GET gegen /api/dev/*, nur bekannte Namen oder Heimnetz-Adressen, keine Weiterleitungen, kein Proxy, und der
// Dialer prüft die tatsächlich verbundene Adresse (schützt auch vor einem Namen, der auf eine öffentliche IP zeigt).
const (
	remoteTimeout  = 10 * time.Second
	remoteBodyMax  = 2 << 20 // größte gelesene Antwort; der Server liefert höchstens 1 MiB Log
	remoteRows     = 1000    // so viele Zeilen je Datei holt ein Aufruf höchstens (Grenze des Servers)
	remoteMiB      = 8       // so viele MiB vom Ende der Datei durchsucht der Server höchstens
	remoteEnv      = "K3C_REMOTE_SERVERS"
	remoteDefaults = "pi=http://pi-gaming:8080"
)

// remoteTarget ist ein aufgelöster entfernter Server: Kurzname (nur [A-Za-z0-9_.-], nie „.“ oder „..“) und Basisadresse.
type remoteTarget struct{ name, base string }

// shortName erlaubt als Kurzname nur Buchstaben, Ziffern, _ und - (kein Punkt: der Name wird zum Ordnernamen).
var shortName = regexp.MustCompile(`^[A-Za-z0-9_-]+$`)

// remoteServers sind die bekannten Server (Kurzname → Basisadresse): pi = pi-gaming:8080, weitere aus K3C_REMOTE_SERVERS
// ("name=host:port,name2=http://host:port"). Ungültige Namen werden übersprungen.
func remoteServers() map[string]string {
	known := map[string]string{}
	for _, spec := range strings.Split(remoteDefaults+","+os.Getenv(remoteEnv), ",") {
		name, addr, ok := strings.Cut(strings.TrimSpace(spec), "=")
		if !ok || !shortName.MatchString(name) || addr == "" {
			continue
		}
		if !strings.Contains(addr, "://") {
			addr = "http://" + addr
		}
		known[name] = strings.TrimRight(addr, "/")
	}
	return known
}

// resolveRemote macht aus `server` ein Ziel: ein bekannter Kurzname oder host:port im Heimnetz (Name ohne Punkt, localhost oder
// private IP). Alles andere lehnt es ab und nennt die gültigen Werte.
func resolveRemote(spec string) (remoteTarget, error) {
	spec = strings.TrimSpace(spec)
	known := remoteServers()
	if base, ok := known[spec]; ok {
		return remoteTarget{name: spec, base: base}, nil
	}
	if host, port, err := net.SplitHostPort(spec); err == nil && homeHost(host) {
		if n, err := strconv.Atoi(port); err == nil && n > 0 && n < 65536 {
			kind := "n-" // Name; IP-Adressen bekommen ein anderes Präfix, damit 10.0.0.1 und 10_0_0_1 nie denselben Namen tragen
			if _, err := netip.ParseAddr(host); err == nil {
				kind = "ip-"
			}
			return remoteTarget{name: kind + sourceSafe(host+"_"+port), base: "http://" + net.JoinHostPort(host, port)}, nil
		}
	}
	names := make([]string, 0, len(known))
	for n := range known {
		names = append(names, n)
	}
	sort.Strings(names)
	return remoteTarget{}, fmt.Errorf("server %q unbekannt; gültig: %s oder host:port im Heimnetz (Name ohne Punkt, localhost, private IP)", spec, strings.Join(names, ", "))
}

// allDigits: Ein Host aus Ziffern (2130706433) oder in Hex-Schreibweise (0x7f000001) ist keine IP-Adresse für netip, löst aber manche
// Systeme als solche auf; er gilt nie als Name.
var allDigits = regexp.MustCompile(`^(0[xX][0-9a-fA-F]+|[0-9]+)$`)

// homeHost erlaubt nur Ziele im Heimnetz: Name ohne Punkt (pi-gaming), localhost oder eine private/lokale IP.
func homeHost(host string) bool {
	if host == "" || allDigits.MatchString(host) {
		return false
	}
	if ip, err := netip.ParseAddr(host); err == nil {
		return homeIP(ip)
	}
	return !strings.Contains(host, ".") && sourceName.MatchString(host)
}

// homeIP: Loopback, private und link-lokale Adressen (auch als IPv4 in IPv6 verpackt).
func homeIP(ip netip.Addr) bool {
	ip = ip.Unmap()
	return ip.IsLoopback() || ip.IsPrivate() || ip.IsLinkLocalUnicast()
}

// homeControl prüft im Dialer die Adresse, mit der wirklich verbunden wird (nach der Namensauflösung).
func homeControl(_, address string, _ syscall.RawConn) error {
	ap, err := netip.ParseAddrPort(address)
	if err != nil || !homeIP(ap.Addr()) {
		return fmt.Errorf("ziel %s liegt nicht im Heimnetz", address)
	}
	return nil
}

// remoteClient folgt keiner Weiterleitung, nutzt keinen Proxy und verbindet nur mit Heimnetz-Adressen.
var remoteClient = &http.Client{
	CheckRedirect: func(*http.Request, []*http.Request) error { return http.ErrUseLastResponse },
	Transport: &http.Transport{
		Proxy:       nil,
		DialContext: (&net.Dialer{Timeout: remoteTimeout, Control: homeControl}).DialContext,
	},
}

// sourceSafe macht aus einer Adresse einen Ordnernamen.
func sourceSafe(s string) string {
	return strings.Map(func(r rune) rune {
		if r >= 'a' && r <= 'z' || r >= 'A' && r <= 'Z' || r >= '0' && r <= '9' || r == '-' || r == '_' {
			return r
		}
		return '_'
	}, s)
}

// get liest einen Dev-Endpunkt; Antwort ist der Text und der Kopf. Fehler nennen Adresse, Status und den Grund des Servers. Eine Antwort
// über remoteBodyMax ist ein Fehler (kein stilles Abschneiden).
func (t remoteTarget) get(ctx context.Context, path string, q url.Values) (string, http.Header, error) {
	ctx, cancel := context.WithTimeout(ctx, remoteTimeout)
	defer cancel()
	u := t.base + path
	if len(q) > 0 {
		u += "?" + q.Encode()
	}
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, u, nil)
	if err != nil {
		return "", nil, err
	}
	resp, err := remoteClient.Do(req)
	if err != nil {
		return "", nil, fmt.Errorf("server %s (%s) nicht erreichbar: %w", t.name, t.base, err)
	}
	defer func() { _ = resp.Body.Close() }()
	body, err := io.ReadAll(io.LimitReader(resp.Body, remoteBodyMax+1))
	if err != nil {
		return "", nil, fmt.Errorf("server %s: Antwort nicht lesbar: %w", t.name, err)
	}
	if len(body) > remoteBodyMax {
		return "", nil, fmt.Errorf("server %s: Antwort größer als %d MiB, abgebrochen", t.name, remoteBodyMax>>20)
	}
	if resp.StatusCode != http.StatusOK {
		return "", nil, t.statusError(resp.StatusCode, string(body))
	}
	return string(body), resp.Header, nil
}

// statusError übersetzt eine Fehlerantwort des Servers in einen Hinweis, was zu tun ist.
func (t remoteTarget) statusError(code int, body string) error {
	hint := ""
	switch {
	case code == http.StatusForbidden:
		hint = " (Dev-Mode am Server aus: K3C_DEV=1)"
	case code == http.StatusNotFound && strings.Contains(body, "Log aus"):
		hint = " (am Server ist kein Log-Ordner gesetzt: K3C_LOG_DIR)"
	case code >= 300 && code < 400:
		hint = " (Weiterleitungen werden nicht verfolgt)"
	}
	return fmt.Errorf("server %s (%s) antwortet %d: %s%s", t.name, t.base, code, clip(strings.TrimSpace(body)), hint)
}

// remoteLog ist eine Zeile von /api/dev/logs: Name der Datei auf dem Server, Bytes, Zeit.
type remoteLog struct {
	file string
	size int64
	when string
}

// knownFile: Nur diese Dateinamen vom Server werden übernommen; alles andere (auch ein manipulierter Name) fällt weg.
func knownFile(file string) bool { return file == "server" || file == "client" || file == "client.1" }

// remoteLogs liest die Liste der Log-Dateien des Servers; unbekannte Namen werden verworfen.
func (t remoteTarget) remoteLogs(ctx context.Context) ([]remoteLog, error) {
	body, _, err := t.get(ctx, "/api/dev/logs", nil)
	if err != nil {
		return nil, err
	}
	var out []remoteLog
	for _, row := range strings.Split(strings.TrimSpace(body), "\n") {
		f := strings.Fields(row)
		if len(f) != 3 || !knownFile(f[0]) {
			continue
		}
		size, _ := strconv.ParseInt(f[1], 10, 64)
		out = append(out, remoteLog{file: f[0], size: size, when: f[2]})
	}
	return out, nil
}

// sourceOf ist der Quellname, den die lokalen Tools für eine Datei des Servers kennen: server → k3c-server, client → k3c-client.
func sourceOf(file string) string { return "k3c-" + file }

// fileOf ist die Umkehrung: k3c-client → client. false bei einem Namen, den der Server nicht kennt.
func fileOf(source string) (string, bool) {
	file, ok := strings.CutPrefix(source, "k3c-")
	return file, ok && knownFile(file)
}

// tempDir legt für einen Aufruf einen eigenen Ordner unter .work/remote-logs an (ignoriert, kein Systemtemp wegen Norton); parallele
// Aufrufe überschreiben sich so nie. done räumt ihn auf.
func (s *Server) tempDir(t remoteTarget) (dir string, done func(), err error) {
	base := filepath.Join(s.cfg.Root, ".work", "remote-logs")
	if err := os.MkdirAll(base, 0o755); err != nil {
		return "", nil, err
	}
	dir, err = os.MkdirTemp(base, t.name+"-")
	if err != nil {
		return "", nil, err
	}
	return dir, func() { _ = os.RemoveAll(dir) }, nil
}

// mirrorRemote holt von jeder Log-Datei des Servers die letzten Zeilen in einen eigenen Ordner (Wurzel mit logs/*.jsonl). done räumt auf.
func (s *Server) mirrorRemote(ctx context.Context, t remoteTarget) (root string, done func(), err error) {
	list, err := t.remoteLogs(ctx)
	if err != nil {
		return "", nil, err
	}
	root, done, err = s.tempDir(t)
	if err != nil {
		return "", nil, err
	}
	dir := filepath.Join(root, "logs")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		done()
		return "", nil, err
	}
	for _, l := range list {
		q := url.Values{"file": {l.file}, "limit": {strconv.Itoa(remoteRows)}, "mib": {strconv.Itoa(remoteMiB)}}
		body, _, err := t.get(ctx, "/api/dev/log", q)
		if err != nil {
			done()
			return "", nil, err
		}
		if body != "" {
			body += "\n"
		}
		if err := os.WriteFile(filepath.Join(dir, sourceOf(l.file)+".jsonl"), []byte(body), 0o644); err != nil {
			done()
			return "", nil, err
		}
	}
	return root, done, nil
}

// logRoot liefert die Wurzel, unter der die Log-Tools logs/*.jsonl lesen: ohne `server` die des Checkouts, sonst eine Kopie des
// entfernten Servers. note steht über der Antwort und nennt die Quelle; done räumt die Kopie auf (immer aufrufen).
func (s *Server) logRoot(ctx context.Context, server, source, local string) (root, note string, done func(), err error) {
	nop := func() {}
	if server == "" {
		return local, "", nop, nil
	}
	t, err := resolveRemote(server)
	if err != nil {
		return "", "", nop, err
	}
	if _, ok := fileOf(source); source != "" && !ok {
		return "", "", nop, fmt.Errorf("service %q: %w", source, errRemoteSource)
	}
	root, done, err = s.mirrorRemote(ctx, t)
	if err != nil {
		return "", "", nop, err
	}
	return root, fmt.Sprintf("Quelle: Server %s (%s) · je Datei die letzten %d Zeilen", t.name, t.base, remoteRows), done, nil
}

// withNote setzt die Quellenzeile über die Antwort.
func withNote(note, text string) string {
	if note == "" {
		return text
	}
	return note + "\n" + text
}

// errRemoteSource meldet einen Dienstnamen, den es auf dem Server nicht gibt.
var errRemoteSource = errors.New("auf einem entfernten Server gibt es nur k3c-server, k3c-client und k3c-client.1")
