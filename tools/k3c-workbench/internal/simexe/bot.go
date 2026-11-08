package simexe

import (
	"bufio"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"time"

	"k3c/tools/k3c-workbench/internal/console"
	"k3c/tools/k3c-workbench/internal/proc"
)

// decideTimeout begrenzt eine Antwort von k3c-sim bot (Test-Naht); sonst hielte ein hängendes Profil Gerät und Pool.
var decideTimeout = 30 * time.Second

// Command ist das Kommando eines Monarchen aus `k3c-sim bot`; Felder wie sim.PlayerCommand.
type Command struct {
	Player int     `json:"player"`
	MoveX  float64 `json:"moveX"`
	Sprint bool    `json:"sprint"`
	Pay    bool    `json:"pay"`
	Attack bool    `json:"attack"`
	Skill  int     `json:"skill"`
}

// Decider entscheidet Kommandos für Monarchen auf dem Zustand einer Stufe (Test-Naht der Bot-Geräte).
type Decider interface {
	Decide(state json.RawMessage, players []int) ([]Command, error)
}

// Bot ist ein laufender Prozess `k3c-sim bot --bot <name>`: eine Anfrage je Zeile auf stdin, eine Antwort je Zeile
// auf stdout. Die Profile sind ohne eigenen Zustand; mehrere Geräte teilen sich einen Prozess oder einen Pool (pool.go).
type Bot struct {
	mu     sync.Mutex
	cmd    *proc.Cmd
	stdin  io.WriteCloser
	out    *bufio.Scanner
	stderr *console.LineWriter

	tailMu sync.Mutex
	tail   []string
	err    error // erster Fehler; danach antwortet Decide nur noch damit
}

// StartBot startet den Entscheider für das Profil name; ctx beendet ihn (Job-Objekt samt Baum).
func StartBot(ctx context.Context, root, name string) (*Bot, error) {
	if err := Check(root); err != nil {
		return nil, err
	}
	b := &Bot{}
	b.cmd = proc.Command(ctx, []string{filepath.Join(root, Rel(0)), "bot", "--bot", name})
	b.cmd.Dir = root
	b.cmd.Env = append(os.Environ(), Env(root)...)
	b.stderr = console.NewLineWriter(func(line string) {
		b.tailMu.Lock()
		b.tail = last(append(b.tail, line), tailLines)
		b.tailMu.Unlock()
	})
	b.cmd.Stderr = b.stderr
	var err error
	if b.stdin, err = b.cmd.StdinPipe(); err != nil {
		return nil, err
	}
	stdout, err := b.cmd.StdoutPipe()
	if err == nil {
		err = b.cmd.Start()
	}
	if err != nil {
		_ = b.stdin.Close()
		return nil, fmt.Errorf("k3c-sim bot nicht gestartet: %w", err)
	}
	b.out = bufio.NewScanner(stdout)
	b.out.Buffer(make([]byte, 0, 64<<10), 8<<20)
	return b, nil
}

// Decide schickt den Zustand und liefert die Kommandos in der Reihenfolge von players (fehlende Monarchen ohne).
func (b *Bot) Decide(state json.RawMessage, players []int) ([]Command, error) {
	ids, err := json.Marshal(players)
	if err != nil {
		return nil, err
	}
	// von Hand zusammengesetzt: json.Marshal prüfte den großen Zustand jedes Mal erneut
	req := make([]byte, 0, len(state)+len(ids)+32)
	req = append(append(append(append(req, `{"state":`...), state...), `,"players":`...), ids...)
	req = append(req, '}')
	b.mu.Lock()
	defer b.mu.Unlock()
	if b.err != nil {
		return nil, b.err
	}
	timer := time.AfterFunc(decideTimeout, func() { _ = b.cmd.Kill() }) // hängt das Profil, endet der Prozess
	defer timer.Stop()
	if _, err := b.stdin.Write(append(req, '\n')); err != nil {
		return nil, b.fail("Anfrage nicht zustellbar")
	}
	if !b.out.Scan() {
		if !timer.Stop() {
			return nil, b.fail(fmt.Sprintf("keine Antwort binnen %s", decideTimeout))
		}
		return nil, b.fail("keine Antwort")
	}
	ev, ok := ParseEvent(b.out.Text())
	switch {
	case !ok:
		return nil, b.fail("Antwort nicht lesbar")
	case ev.Event == "error":
		return nil, b.fail(ev.Message)
	case ev.Event != "commands":
		return nil, b.fail("unerwartete Antwort " + ev.Event)
	}
	return ev.Commands, nil
}

// fail merkt den ersten Fehler samt letzten stderr-Zeilen (unter mu).
func (b *Bot) fail(reason string) error {
	b.tailMu.Lock()
	tail := strings.Join(b.tail, " | ")
	b.tailMu.Unlock()
	msg := "k3c-sim bot: " + reason
	if tail != "" {
		msg += ": " + tail
	}
	b.err = fmt.Errorf("%s", msg)
	return b.err
}

// Close beendet den Prozess samt Baum (Job-Objekt); die Profile haben keinen Zustand, der verloren ginge.
func (b *Bot) Close() {
	_ = b.stdin.Close()
	_ = b.cmd.Kill()
	_ = b.cmd.Wait()
	b.stderr.Flush()
}
