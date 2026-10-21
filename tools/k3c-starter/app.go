package main

// App ist die Bindung ans Fenster: State, Start und Stop rufen die Oberfläche (frontend/src/api/wails.ts) über window.go.main.App auf.
type App struct {
	sup *supervisor
	// err ist die Meldung, wenn Server-EXE oder dist/ fehlen: Das Fenster öffnet trotzdem und nennt den Grund.
	err error
}

func newApp(lay layout, err error) *App {
	a := &App{err: err}
	if err == nil {
		a.sup = newSupervisor(lay)
	}
	return a
}

// start startet den Server beim Öffnen des Fensters (Doppelklick genügt).
func (a *App) start() {
	if a.sup != nil {
		a.sup.Start()
	}
}

// stop beendet den Server beim Schließen des Fensters.
func (a *App) stop() {
	if a.sup != nil {
		a.sup.Stop()
	}
}

// State ist der Zustand für das Fenster.
func (a *App) State() Status {
	if a.sup == nil {
		return Status{Message: a.err.Error(), Log: []string{}, URLs: []string{}}
	}
	return a.sup.State()
}

// Start startet den Server (Schalter im Fenster).
func (a *App) Start() { a.start() }

// Stop beendet den Server (Schalter im Fenster).
func (a *App) Stop() { a.stop() }
