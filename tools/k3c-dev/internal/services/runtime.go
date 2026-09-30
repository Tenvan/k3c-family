package services

import (
	"context"
	"fmt"
	"net"
	"net/http"
	"os"
	"path/filepath"
	"strconv"
	"time"

	"k3c/tools/k3c-dev/internal/console"
	"k3c/tools/k3c-dev/internal/proc"
)

// checkTimeout begrenzt eine einzelne Health-Prüfung.
const checkTimeout = 2 * time.Second

// ProcStarter startet einen Dienst über internal/proc: ohne Konsolenfenster, im Job Object, Ausgabe zeilenweise an out.
func ProcStarter(svc Service, root string, out func(stream, text string)) (Process, error) {
	cmd := proc.Command(context.Background(), svc.Command)
	cmd.Dir = filepath.Join(root, svc.Cwd)
	cmd.Env = os.Environ()
	for k, v := range svc.Env {
		cmd.Env = append(cmd.Env, k+"="+v)
	}
	p := &procProcess{cmd: cmd,
		stdout: console.NewLineWriter(func(text string) { out("stdout", text) }),
		stderr: console.NewLineWriter(func(text string) { out("stderr", text) })}
	cmd.Stdout, cmd.Stderr = p.stdout, p.stderr
	if err := cmd.Start(); err != nil {
		return nil, err
	}
	return p, nil
}

type procProcess struct {
	cmd            *proc.Cmd
	stdout, stderr *console.LineWriter
}

func (p *procProcess) PID() int { return p.cmd.Process.Pid }

func (p *procProcess) Wait() error {
	err := p.cmd.Wait()
	p.stdout.Flush()
	p.stderr.Flush()
	return err
}

func (p *procProcess) Kill() error { return p.cmd.Kill() }

// client folgt keiner Weiterleitung: ein 3xx zählt schon als gesund.
var client = &http.Client{Timeout: checkTimeout, CheckRedirect: func(*http.Request, []*http.Request) error {
	return http.ErrUseLastResponse
}}

// HealthCheck prüft einen Dienst: http = GET / mit 2xx oder 3xx, tcp = Port nimmt Verbindungen an.
func HealthCheck(ctx context.Context, svc Service) error {
	addr := net.JoinHostPort("127.0.0.1", strconv.Itoa(svc.Port))
	if svc.Health == "tcp" {
		conn, err := (&net.Dialer{Timeout: checkTimeout}).DialContext(ctx, "tcp", addr)
		if err != nil {
			return err
		}
		return conn.Close()
	}
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, "http://"+addr+"/", nil)
	if err != nil {
		return err
	}
	resp, err := client.Do(req)
	if err != nil {
		return err
	}
	_ = resp.Body.Close()
	if resp.StatusCode >= 400 {
		return fmt.Errorf("antwort %d", resp.StatusCode)
	}
	return nil
}
