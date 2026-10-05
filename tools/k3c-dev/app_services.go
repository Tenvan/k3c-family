package main

import (
	"errors"
	"fmt"
	"path/filepath"

	"k3c/tools/k3c-dev/internal/services"
)

// ServicesView ist die Dienste-Seite beim Laden (B-068). Error ist gesetzt, wenn services.json nicht geladen wurde;
// dann ist die Liste leer und die Seite zeigt statt der Karten eine Hinweiskarte.
type ServicesView struct {
	Services []services.Status `json:"services"`
	Error    string            `json:"error"`
}

// Services liefert alle Dienste in der Reihenfolge der Konfiguration (Binding).
func (a *App) Services() ServicesView {
	a.wait()
	if a.ctl == nil {
		return ServicesView{Services: []services.Status{}, Error: a.servicesErr().Error()}
	}
	return ServicesView{Services: a.ctl.Statuses()}
}

func (a *App) servicesErr() error {
	if a.svcErr != nil {
		return a.svcErr
	}
	return errors.New("keine Dienste konfiguriert")
}

func (a *App) controller() (*services.Controller, error) {
	a.wait()
	if a.ctl == nil {
		return nil, a.servicesErr()
	}
	return a.ctl, nil
}

// ServicesReload liest services.json neu und gleicht die Dienste ab (Binding). Läuft ein geänderter Dienst, gilt
// seine neue Konfiguration erst nach dem nächsten Start (Error der Ansicht bleibt leer, der Hinweis steht im Log).
func (a *App) ServicesReload() (ServicesView, error) {
	ctl, err := a.controller()
	if err != nil {
		return ServicesView{}, fmt.Errorf("%w; k3c-dev neu starten", err)
	}
	list, err := services.Load(filepath.Join(a.root, "tools", "k3c-dev", "services.json"))
	if err != nil {
		a.log.Error("💥 dienste nicht neu geladen: "+err.Error(), "ns", "svc")
		return ServicesView{}, err
	}
	pending := ctl.Reload(a.svcCtx, services.Shift(list, 0))
	a.log.Info("🔁 dienste neu geladen", "ns", "svc", "erstBeimNaechstenStart", pending)
	return ServicesView{Services: ctl.Statuses()}, nil
}

// ServiceStart startet einen Dienst und wartet, bis er gesund ist (Binding). Den Zustand unterwegs melden Ereignisse.
func (a *App) ServiceStart(name string) (services.Status, error) {
	ctl, err := a.controller()
	if err != nil {
		return services.Status{}, err
	}
	return ctl.Start(a.svcCtx, name)
}

// ServiceStop stoppt einen Dienst; einen übernommenen nur mit force, das die Oberfläche erst nach dem
// Bestätigungsdialog setzt (Binding).
func (a *App) ServiceStop(name string, force bool) (services.Status, error) {
	ctl, err := a.controller()
	if err != nil {
		return services.Status{}, err
	}
	return ctl.Stop(a.svcCtx, name, force)
}

// ServiceRestart startet einen Dienst neu (Binding).
func (a *App) ServiceRestart(name string) (services.Status, error) {
	ctl, err := a.controller()
	if err != nil {
		return services.Status{}, err
	}
	return ctl.Restart(a.svcCtx, name)
}

// ServicesStartAll startet alle, die nicht schon laufen, parallel (Binding).
func (a *App) ServicesStartAll() error {
	ctl, err := a.controller()
	if err != nil {
		return err
	}
	return ctl.StartAll(a.svcCtx)
}

// ServicesStopAll stoppt alle eigenen rückwärts; übernommene bleiben (Binding).
func (a *App) ServicesStopAll() error {
	ctl, err := a.controller()
	if err != nil {
		return err
	}
	ctl.StopAll(a.svcCtx)
	return nil
}

// ServiceLogLevels zählt die Log-Einträge der letzten 60 min je Level (Binding).
func (a *App) ServiceLogLevels(name string) (services.LevelCounts, error) {
	ctl, err := a.controller()
	if err != nil {
		return services.LevelCounts{}, err
	}
	counts, ok, err := ctl.LogLevels(name)
	if err == nil && !ok {
		err = fmt.Errorf("%s hat kein Log", name)
	}
	return counts, err
}
