package simrun

import "time"

// Ereignisse der Balancing-Seite (B-662): Das Register meldet Änderungen an Läufen (Zustand, Phase, Fortschritt,
// Zwischenstand) und an Berichten über Rückrufe; die Workbench macht daraus balance:runs und balance:reports. Ein Lauf
// meldet sich höchstens einmal je notifyEvery; die letzte Änderung kommt immer an (nachlaufend).

// notifyEvery ist der Mindestabstand zweier Meldungen eines Laufs (höchstens etwa 2 je Sekunde).
const notifyEvery = 500 * time.Millisecond

// Notify sind die Rückrufe des Registers; sie laufen in eigener Goroutine, nie unter der Sperre des Registers.
type Notify struct {
	Run     func(root string, info Info) // ein Lauf hat sich geändert (auch: wartender entfernt)
	Reports func(root string)            // Berichte oder Baselines des Checkouts haben sich geändert
}

// SetNotify setzt die Rückrufe und ersetzt frühere.
func (g *Registry) SetNotify(n Notify) {
	g.mu.Lock()
	defer g.mu.Unlock()
	g.notify = n
}

// changed meldet eine Änderung von run, gedrosselt auf notifyEvery; unter g.mu.
func (g *Registry) changed(run *Run) {
	if g.notify.Run == nil || run.notePending {
		return
	}
	run.notePending = true
	time.AfterFunc(max(notifyEvery-time.Since(run.noted), 0), func() {
		g.mu.Lock()
		run.notePending, run.noted = false, time.Now()
		fn, info := g.notify.Run, run.info()
		g.mu.Unlock()
		if fn != nil {
			fn(run.root, info)
		}
	})
}

// reportsChanged meldet geänderte Berichte des Checkouts; unter g.mu.
func (g *Registry) reportsChanged(root string) {
	if fn := g.notify.Reports; fn != nil {
		go fn(root)
	}
}
