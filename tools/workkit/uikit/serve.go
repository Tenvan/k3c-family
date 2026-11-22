package uikit

import (
	"context"
	"errors"
	"fmt"
	"net"
	"net/http"
	"strconv"
	"strings"
	"time"

	"k3c/engine/devrun"
)

const (
	// listenWait wartet auf den Port, den ein eben beendeter Vorgänger (Neustart im -dev) noch hält.
	listenWait = 10 * time.Second
	// stopTimeout begrenzt das Warten auf offene Anfragen beim Beenden.
	stopTimeout = 2 * time.Second
)

// ResolvePort liest den Port der Oberflächen-API aus raw (Wert der Umgebungsvariable env); leer heißt def.
func ResolvePort(env, raw string, def int) (int, error) {
	raw = strings.TrimSpace(raw)
	if raw == "" {
		return def, nil
	}
	port, err := strconv.Atoi(raw)
	if err != nil || port < 1 || port > 65535 {
		return 0, fmt.Errorf("%s=%q ist keine gültige Portzahl (1–65535)", env, raw)
	}
	return port, nil
}

// Listen öffnet 127.0.0.1:port; ein Vorgänger, der den Port noch hält, bekommt etwas Zeit (devrun.Listen).
func Listen(env string, port int) (net.Listener, error) {
	ln, err := devrun.Listen("tcp", net.JoinHostPort("127.0.0.1", strconv.Itoa(port)), listenWait)
	if err != nil {
		return nil, fmt.Errorf("oberflächen-port %d nicht verfügbar (%w); einen anderen über %s setzen", port, err, env)
	}
	return ln, nil
}

// PageURL ist die Adresse des Listeners als http-URL mit abschließendem Schrägstrich.
func PageURL(ln net.Listener) string { return "http://" + ln.Addr().String() + "/" }

// Serve beantwortet h auf ln, bis ctx endet (nil) oder der Server scheitert (Fehler). Beim Ende schließt es die
// SSE-Ströme und wartet höchstens stopTimeout auf offene Anfragen.
func Serve(ctx context.Context, ln net.Listener, h http.Handler) error {
	reqCtx, endStreams := context.WithCancel(context.Background()) // beendet die SSE-Ströme beim Stoppen
	defer endStreams()
	srv := &http.Server{Handler: h, ReadHeaderTimeout: 10 * time.Second,
		BaseContext: func(net.Listener) context.Context { return reqCtx }}
	served := make(chan error, 1)
	go func() { served <- srv.Serve(ln) }()
	var err error
	select {
	case <-ctx.Done():
	case err = <-served:
	}
	endStreams()
	shutCtx, cancel := context.WithTimeout(context.Background(), stopTimeout)
	_ = srv.Shutdown(shutCtx)
	cancel()
	if errors.Is(err, http.ErrServerClosed) {
		return nil
	}
	return err
}
