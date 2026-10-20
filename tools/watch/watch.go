package main

import (
	"context"
	"errors"
	"io/fs"
	"log/slog"
	"os"
	"os/exec"
	"path/filepath"
	"slices"
	"strings"
	"time"
)

// fileSig ist, woran sich eine Änderung erkennen lässt.
type fileSig struct {
	mod  int64
	size int64
}

// snapshot liest Änderungszeit und Größe aller beobachteten Dateien. Versteckte Ordner (.git …), node_modules,
// Testdateien und nicht lesbare Pfade zählen nicht; ein fehlender Pfad ist nicht schlimm (er kann später entstehen).
func snapshot(cfg config) map[string]fileSig {
	out := map[string]fileSig{}
	add := func(path string, d fs.DirEntry) {
		if skipped(path, cfg.Skip) {
			return
		}
		if info, err := d.Info(); err == nil {
			out[filepath.ToSlash(path)] = fileSig{mod: info.ModTime().UnixNano(), size: info.Size()}
		}
	}
	for _, p := range cfg.Paths {
		info, err := os.Stat(p)
		switch {
		case err != nil:
			continue
		case !info.IsDir():
			add(p, fs.FileInfoToDirEntry(info)) // eine einzeln genannte Datei zählt immer
		default:
			_ = filepath.WalkDir(p, func(path string, d fs.DirEntry, err error) error {
				if err != nil {
					return nil
				}
				if d.IsDir() {
					if name := d.Name(); path != p && (strings.HasPrefix(name, ".") || name == "node_modules") {
						return filepath.SkipDir
					}
					return nil
				}
				if watched(path, cfg.Exts) {
					add(path, d)
				}
				return nil
			})
		}
	}
	return out
}

// skipped sagt, ob path eine ausgenommene Datei ist oder unter einem ausgenommenen Ordner liegt.
func skipped(path string, skip []string) bool {
	p := filepath.ToSlash(filepath.Clean(path))
	for _, s := range skip {
		s = filepath.ToSlash(filepath.Clean(s))
		if p == s || strings.HasPrefix(p, s+"/") {
			return true
		}
	}
	return false
}

// watched sagt, ob eine Datei aus einem Ordner beobachtet wird: passende Endung, keine Testdatei.
func watched(path string, exts []string) bool {
	if strings.HasSuffix(path, "_test.go") {
		return false
	}
	return len(exts) == 0 || slices.Contains(exts, strings.ToLower(filepath.Ext(path)))
}

// changed nennt eine Datei, die neu, geändert oder weg ist (die erste in Namensreihenfolge); "" heißt: nichts hat sich geändert.
func changed(before, after map[string]fileSig) string {
	var names []string
	for p, sig := range after {
		if old, ok := before[p]; !ok || old != sig {
			names = append(names, p)
		}
	}
	for p := range before {
		if _, ok := after[p]; !ok {
			names = append(names, p)
		}
	}
	if len(names) == 0 {
		return ""
	}
	slices.Sort(names)
	return names[0]
}

// proc ist ein laufendes Kind (Test-Naht).
type proc interface {
	Stop()
	Done() <-chan struct{} // schließt, sobald das Kind beendet ist
}

// supervisor baut und startet neu.
type supervisor struct {
	cfg   config
	log   *slog.Logger
	build func(ctx context.Context) error
	start func() (proc, error)
	cur   proc
}

func newSupervisor(cfg config, log *slog.Logger) *supervisor {
	s := &supervisor{cfg: cfg, log: log}
	s.build = func(ctx context.Context) error { return runBuild(ctx, cfg.Build) }
	s.start = func() (proc, error) { return startProc(cfg.Cmd, cfg.Grace) }
	return s
}

// run baut und startet einmal, beobachtet dann die Dateien und baut bei jeder Änderung neu, bis ctx endet.
func (s *supervisor) run(ctx context.Context) error {
	defer s.stop()
	s.log.Info("🚀 Watcher läuft", "paths", strings.Join(s.cfg.Paths, ","), "exe", s.cfg.Cmd[0])
	base := snapshot(s.cfg)
	s.rebuild(ctx)
	tick := time.NewTicker(s.cfg.Poll)
	defer tick.Stop()
	for {
		select {
		case <-ctx.Done():
			s.log.Info("🛑 Watcher beendet")
			return nil
		case <-tick.C:
		}
		s.reap()
		now := snapshot(s.cfg)
		file := changed(base, now)
		if file == "" {
			continue
		}
		s.log.Info("⏳ Änderung erkannt", "file", file)
		base = s.settle(ctx, now)
		if ctx.Err() != nil {
			continue
		}
		s.rebuild(ctx)
	}
}

// reap vergisst ein Programm, das sich von selbst beendet hat; es startet erst bei der nächsten Änderung wieder.
func (s *supervisor) reap() {
	if s.cur == nil {
		return
	}
	select {
	case <-s.cur.Done():
		s.log.Warn("💥 Programm beendet, wartet auf die nächste Änderung")
		s.cur = nil
	default:
	}
}

// settle wartet, bis cfg.Quiet lang nichts mehr geändert wurde (Speichern in mehreren Schritten, Formatierer), und
// liefert den Stand danach.
func (s *supervisor) settle(ctx context.Context, from map[string]fileSig) map[string]fileSig {
	cur := from
	for ctx.Err() == nil {
		select {
		case <-ctx.Done():
			return cur
		case <-time.After(s.cfg.Quiet):
		}
		next := snapshot(s.cfg)
		if changed(cur, next) == "" {
			return next
		}
		cur = next
	}
	return cur
}

// rebuild baut die EXE neu und tauscht sie gegen die laufende aus. Unter Windows lässt sich eine laufende EXE nicht
// überschreiben, aber umbenennen: Sie wandert beiseite, der Build schreibt am festen Pfad eine neue, und erst nach
// einem erfolgreichen Build endet das alte Programm. Ein Fehler stellt die alte EXE wieder her und lässt sie laufen.
func (s *supervisor) rebuild(ctx context.Context) {
	exe := s.cfg.Cmd[0]
	aside := ""
	if len(s.cfg.Build) > 0 { // ohne Build gibt es nichts zu tauschen, die EXE bleibt, wie sie ist
		var err error
		if aside, err = moveAside(exe); err != nil {
			s.log.Warn("🚫 laufende EXE nicht beiseitegelegt", "err", err)
		}
	}
	if len(s.cfg.Build) > 0 {
		s.log.Info("🔁 Build startet", "cmd", strings.Join(s.cfg.Build, " "))
		if err := s.build(ctx); err != nil {
			if ctx.Err() == nil { // bei Strg+C ist der abgebrochene Build kein Fehler
				s.log.Error("❌ Build fehlgeschlagen, die laufende Version bleibt", "err", err)
			}
			if err := restoreAside(exe, aside); err != nil {
				s.log.Error("💥 alte EXE nicht zurückgelegt, es läuft nichts mehr: "+exe, "err", err)
			}
			return
		}
		s.log.Info("✅ Build fertig")
	}
	s.stop()
	discard(aside)
	p, err := s.start()
	if err != nil {
		s.log.Error("💥 Start fehlgeschlagen", "err", err)
		return
	}
	s.cur = p
	s.log.Info("🚀 gestartet", "exe", exe)
}

// stop beendet das laufende Kind und wartet darauf.
func (s *supervisor) stop() {
	if s.cur == nil {
		return
	}
	s.cur.Stop()
	s.cur = nil
}

// moveAside benennt exe in exe+".old" um und liefert den neuen Pfad; "" heißt: es gab keine EXE.
func moveAside(exe string) (string, error) {
	if _, err := os.Stat(exe); err != nil {
		if errors.Is(err, fs.ErrNotExist) {
			return "", nil
		}
		return "", err
	}
	aside := exe + ".old"
	_ = os.Remove(aside) // Rest eines früheren Laufs
	if err := os.Rename(exe, aside); err != nil {
		return "", err
	}
	return aside, nil
}

// restoreAside stellt die beiseitegelegte EXE wieder her (nach einem fehlgeschlagenen Build).
func restoreAside(exe, aside string) error {
	if aside == "" {
		return nil
	}
	if err := os.Remove(exe); err != nil && !errors.Is(err, fs.ErrNotExist) { // halb geschriebene neue EXE
		return err
	}
	return os.Rename(aside, exe)
}

// discard löscht die beiseitegelegte EXE; das alte Programm ist beendet, aber Windows gibt die Datei manchmal erst
// einen Moment später frei.
func discard(aside string) {
	if aside == "" {
		return
	}
	for range 20 {
		if err := os.Remove(aside); err == nil || errors.Is(err, fs.ErrNotExist) {
			return
		}
		time.Sleep(50 * time.Millisecond)
	}
}

// runBuild führt den Build-Befehl aus; Ausgabe und Fehler gehen auf die Konsole.
func runBuild(ctx context.Context, argv []string) error {
	cmd := exec.CommandContext(ctx, argv[0], argv[1:]...)
	cmd.Stdout, cmd.Stderr = os.Stdout, os.Stderr
	prepare(cmd)
	cmd.Cancel = func() error { return killTree(cmd) } // bei Strg+C auch `go build` unter `task` beenden
	return cmd.Run()
}

// childProc ist ein gestartetes Programm.
type childProc struct {
	cmd   *exec.Cmd
	done  chan struct{}
	grace time.Duration
}

// startProc startet argv ohne zu warten.
func startProc(argv []string, grace time.Duration) (proc, error) {
	cmd := exec.Command(argv[0], argv[1:]...)
	cmd.Stdout, cmd.Stderr = os.Stdout, os.Stderr
	prepare(cmd)
	if err := cmd.Start(); err != nil {
		return nil, err
	}
	p := &childProc{cmd: cmd, done: make(chan struct{}), grace: grace}
	go func() {
		_ = cmd.Wait()
		close(p.done)
	}()
	return p, nil
}

func (p *childProc) Done() <-chan struct{} { return p.done }

// Stop bittet das Programm um ein sauberes Ende (der Server speichert dabei die Räume) und beendet es erst nach der
// Frist hart. Es wartet, bis das Programm weg ist.
func (p *childProc) Stop() {
	if requestStop(p.cmd) == nil {
		select {
		case <-p.done:
			return
		case <-time.After(p.grace):
		}
	}
	_ = p.cmd.Process.Kill()
	<-p.done
}
