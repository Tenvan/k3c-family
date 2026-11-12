package mcpsrv

import (
	"fmt"
	"os"
)

// Sim-Sperre im Werkzeug (B-788): Simulationen und Balancing sind bis auf Widerruf gesperrt (Beschluss 🧑 2026-11-11).
// sim_test start und sim_run lehnen ab, solange die Workbench nicht mit K3C_SIM_ENTSPERRT=1 läuft.

// simUnlockEnv ist der Schalter, der die Sperre aufhebt (Umgebung der Workbench, Wert 1).
const simUnlockEnv = "K3C_SIM_ENTSPERRT"

// simLocked meldet, ob die Sperre gilt.
func simLocked() bool { return os.Getenv(simUnlockEnv) != "1" }

// simLockErr ist die Ablehnung für tool, solange die Sperre gilt; sonst nil.
func simLockErr(tool string) error {
	if !simLocked() {
		return nil
	}
	return fmt.Errorf("%s abgelehnt: Simulationen und Balancing sind bis auf Widerruf gesperrt (Beschluss 🧑 2026-11-11, "+
		"docs/arbeitsweise.md › Sim, Balance und Gen); kein Lauf gestartet. Aufheben nur durch 🧑: Workbench mit %s=1 starten",
		tool, simUnlockEnv)
}
