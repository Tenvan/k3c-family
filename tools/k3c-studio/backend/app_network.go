package backend

import (
	"fmt"
	"time"
)

// Netz-Modus des Studios (Schalter in der Kopfzeile): auto = Erreichbarkeitsprobe, online/offline = fest gesetzt.
const (
	NetworkAuto    = "auto"
	NetworkOnline  = "online"
	NetworkOffline = "offline"
)

// NetworkView ist der Netz-Zustand: gewählter Modus und ob das Studio damit offline ist (Quellen, Chat-Vorschläge).
type NetworkView struct {
	Mode    string `json:"mode"`
	Offline bool   `json:"offline"`
}

// SourceNetwork liefert Modus und Zustand (Binding); im Modus auto mit Probe (höchstens alle 30 s).
func (a *App) SourceNetwork() NetworkView {
	st := a.sourceState()
	st.mu.Lock()
	mode := st.mode
	st.mu.Unlock()
	if mode == "" {
		mode = NetworkAuto
	}
	return NetworkView{Mode: mode, Offline: st.offline(a.context())}
}

// SourceSetNetwork setzt den Modus (Binding); auto verwirft das gemerkte Probe-Ergebnis und fragt neu.
func (a *App) SourceSetNetwork(mode string) (NetworkView, error) {
	if mode != NetworkAuto && mode != NetworkOnline && mode != NetworkOffline {
		return NetworkView{}, fmt.Errorf("unbekannter Netz-Modus %q (auto, online, offline)", mode)
	}
	st := a.sourceState()
	st.mu.Lock()
	st.mode, st.checked, st.down = mode, time.Time{}, false
	st.mu.Unlock()
	return a.SourceNetwork(), nil
}
