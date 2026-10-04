package services

import (
	"context"
	"io/fs"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"time"
)

// Watch beschreibt, bei welchen Änderungen k3c-dev einen Dienst neu startet (Eintrag `watch` in services.json).
type Watch struct {
	// Paths sind Ordner oder Dateien relativ zur Repo-Wurzel; Ordner werden samt Unterordnern beobachtet.
	Paths []string `json:"paths"`
	// Ext begrenzt Dateien in Ordnern auf diese Endungen (mit Punkt, z. B. ".go"); leer = alle. Einzeln genannte Dateien zählen immer.
	Ext []string `json:"ext,omitempty"`
}

// fileSig ist, woran sich eine Änderung erkennen lässt.
type fileSig struct {
	mod  int64
	size int64
}

// snapshot liest Änderungszeit und Größe aller beobachteten Dateien. Versteckte Ordner (.git …), node_modules und
// nicht lesbare Pfade werden übersprungen; ein fehlender Pfad ist nicht schlimm (er kann später entstehen).
func snapshot(root string, w *Watch) map[string]fileSig {
	out := map[string]fileSig{}
	add := func(path string, d fs.DirEntry) {
		if info, err := d.Info(); err == nil {
			out[filepath.ToSlash(path)] = fileSig{mod: info.ModTime().UnixNano(), size: info.Size()}
		}
	}
	for _, p := range w.Paths {
		abs := filepath.Join(root, filepath.FromSlash(p))
		info, err := os.Stat(abs)
		if err != nil {
			continue
		}
		if !info.IsDir() {
			add(p, fs.FileInfoToDirEntry(info))
			continue
		}
		_ = filepath.WalkDir(abs, func(path string, d fs.DirEntry, err error) error {
			if err != nil {
				return nil
			}
			if d.IsDir() {
				if name := d.Name(); path != abs && (strings.HasPrefix(name, ".") || name == "node_modules") {
					return filepath.SkipDir
				}
				return nil
			}
			if len(w.Ext) == 0 || slices.Contains(w.Ext, strings.ToLower(filepath.Ext(path))) {
				rel, _ := filepath.Rel(root, path)
				add(rel, d)
			}
			return nil
		})
	}
	return out
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

// watchFiles beginnt (falls noch nicht), die Dateien des Dienstes zu beobachten. Aufruf nur für Dienste mit `watch`.
func (c *Controller) watchFiles(u *unit) {
	if u.svc.Watch == nil {
		return
	}
	u.mu.Lock()
	defer u.mu.Unlock()
	if u.unfiles != nil {
		return
	}
	ctx, cancel := context.WithCancel(context.Background())
	u.unfiles = cancel
	go c.filesLoop(ctx, u)
}

// unwatchFiles beendet die Beobachtung (der Dienst wurde ausdrücklich gestoppt).
func (c *Controller) unwatchFiles(u *unit) {
	u.mu.Lock()
	defer u.mu.Unlock()
	if u.unfiles != nil {
		u.unfiles()
		u.unfiles = nil
	}
}

// filesLoop prüft alle FilesEvery die Dateien. Eine Änderung wird erst nach FilesQuiet Ruhe wirksam (Speichern in
// mehreren Schritten, Formatierer); dann startet der Dienst neu. Läuft er nicht (gestoppt, übernommen), gibt es nichts
// zu tun; ist er fehlgeschlagen (z. B. Build-Fehler), versucht der nächste Neustart es erneut.
func (c *Controller) filesLoop(ctx context.Context, u *unit) {
	base := snapshot(c.opts.Root, u.svc.Watch)
	tick := time.NewTicker(c.opts.FilesEvery)
	defer tick.Stop()
	for {
		select {
		case <-ctx.Done():
			return
		case <-tick.C:
		}
		cur := snapshot(c.opts.Root, u.svc.Watch)
		file := changed(base, cur)
		if file == "" {
			continue
		}
		cur, ok := c.settle(ctx, u, cur)
		if !ok {
			return
		}
		base = cur
		c.restartForChange(ctx, u, file)
	}
}

// settle wartet, bis sich die Dateien FilesQuiet lang nicht mehr ändern, und liefert den ruhigen Stand.
func (c *Controller) settle(ctx context.Context, u *unit, cur map[string]fileSig) (map[string]fileSig, bool) {
	for {
		select {
		case <-ctx.Done():
			return nil, false
		case <-time.After(c.opts.FilesQuiet):
		}
		next := snapshot(c.opts.Root, u.svc.Watch)
		if changed(cur, next) == "" {
			return next, true
		}
		cur = next
	}
}

// restartForChange startet den Dienst wegen einer Änderung neu. Ein Stop gewinnt: er beendet die Beobachtung (ctx) und
// hält die Befehlssperre; danach ändert dieser Neustart nichts mehr.
func (c *Controller) restartForChange(ctx context.Context, u *unit, file string) {
	u.cmd.Lock()
	defer u.cmd.Unlock()
	if ctx.Err() != nil {
		return
	}
	switch u.status().State {
	case Stopped, Adopted, Stopping:
		return
	}
	c.opts.Log.Info("🔁 dienst "+u.svc.Name+": Neustart wegen Änderung an "+file, "ns", "svc")
	_, err := c.stop(u)
	if err == nil {
		_, err = c.start(ctx, u)
	}
	c.opts.Console.Add(c.source(u), "stdout", "k3c-dev: Neustart wegen Änderung an "+file)
	if err != nil {
		c.opts.Console.Add(c.source(u), "stderr", "k3c-dev: Neustart fehlgeschlagen: "+err.Error())
	}
}
