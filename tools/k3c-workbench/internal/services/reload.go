package services

import (
	"context"
	"reflect"
)

func sameConfig(a, b Service) bool { return reflect.DeepEqual(a, b) }

func (c *Controller) newUnit(s Service) *unit {
	return &unit{svc: s, st: Status{Name: s.Name, Desc: s.Description, Tags: s.Tags, Port: s.Port, Health: s.HealthURL(), Log: s.Log, State: Stopped}}
}

// Reload gleicht die Dienste mit einer neu gelesenen Konfiguration ab (Knopf „Neu laden“): neue Dienste kommen
// dazu, entfernte eigene werden gestoppt und verschwinden (ein übernommener Prozess läuft unberührt weiter),
// die Reihenfolge folgt der Liste. Ein Dienst, der gerade läuft, behält seine alten Einstellungen bis zum
// nächsten Start; seine Namen liefert Reload als pending zurück.
func (c *Controller) Reload(ctx context.Context, list []Service) (pending []string) {
	old := map[string]*unit{}
	for _, u := range c.list() {
		old[u.svc.Name] = u
	}
	next := make([]*unit, 0, len(list))
	for _, s := range list {
		u, ok := old[s.Name]
		delete(old, s.Name)
		switch {
		case !ok:
			u = c.newUnit(s)
			c.set(u, func(*Status) {}) // meldet den neuen Dienst der Oberfläche
		case !sameConfig(u.svc, s):
			if p := u.status().State; p != Stopped && p != Failed {
				pending = append(pending, s.Name)
				break
			}
			u.cmd.Lock()
			u.svc = s
			u.cmd.Unlock()
			c.set(u, func(st *Status) {
				st.Desc, st.Tags, st.Port, st.Health, st.Log = s.Description, s.Tags, s.Port, s.HealthURL(), s.Log
			})
		}
		next = append(next, u)
	}
	for name, u := range old {
		if u.status().State != Adopted {
			_, _ = c.Stop(ctx, name, false)
		}
	}
	c.mu.Lock()
	c.units = next
	c.mu.Unlock()
	return pending
}
