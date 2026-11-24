package mcpsrv

import (
	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// register ist der einzige Katalog aller Tools des Studios: Name, Beschreibung, Annotations und Handler.
// Ein neues Tool kommt nur hier hinzu; Zähler, Parameter-Hinweis und Oberfläche lesen es von hier.
func register(s *Server) {
	registerEngine(s)
	registerSim(s)
	registerSimTest(s)
	registerGameData(s)
	registerAssets(s)
	registerAssetsResearch(s)
	registerAssetsImport(s)
	registerHub(s)
	registerWorld(s)
}

// registerEngine ist das Engine-Tool (B-047, aus k3c-workbench): rechnet mit der Spiel-EXE des Checkouts
// (`k3c-game sim level`, RU34.2), ohne laufenden Server.
func registerEngine(s *Server) {
	add(s, &mcp.Tool{
		Name: "level_generate",
		Description: "Erzeugt ein Level mit der Spiel-EXE des Checkouts (k3c-game sim level, kein Server nötig; fehlt sie oder ist sie veraltet, nennt der Fehler den Bau-Befehl): Kopfzeile, eine Zeile je Abschnitt, " +
			"Objekte nach Art, Warnungen der Prüfung. Gleicher Seed ergibt dasselbe Level wie im Spiel. " +
			"Nutze es bei: Level eines Seeds prüfen. Statt: Spiel starten und Level von Hand ansehen.",
		Annotations: readOnly(),
	}, s.levelGenerate)
}

// registerSim sind die Engine-Tools des Balancings (B-047, seit WZ63.5 aus k3c-workbench): rechnen im Prozess des
// Studios über engine/simcli (B-924), ohne laufenden Server.
func registerSim(s *Server) {
	add(s, &mcp.Tool{
		Name: "sim_run",
		Description: "Deterministischer Simulationslauf mit der Spiel-EXE des Checkouts (k3c-game sim run, kein Server nötig), höchstens 100000 Ticks (30/s): " +
			"Tag, Welle, Gold, Truppen, Verluste, Ende. Optional Eingaben je Monarch und Tick-Bereich (höchstens 100 Segmente). " +
			"Mit head: true öffnet sich das Spiel und zeigt den Lauf (speed 1 bis 64), sonst läuft er headless. " +
			"Bis auf Widerruf gesperrt (Beschluss 🧑 2026-11-11, aufheben: Studio mit K3C_SIM_ENTSPERRT=1). " +
			"Nutze es bei: kurzer deterministischer Regel-Check. Statt: go run oder ein Spiel von Hand.",
		Annotations: readOnly(),
	}, s.simRun)
	add(s, &mcp.Tool{
		Name: "replay_run",
		Description: "Spielt eine Replay-Datei des Balancing-Testers (task balance:run -- --replay-dir) mit der Engine des Studios ohne Bot ab: " +
			"Endzustand-Hash, Burgfall-Tick, Vergleich mit der Aufnahme, Warnung bei anderem Datenstand. Nur Dateien im Repo. " +
			"Nutze es bei: Replay nachrechnen oder Abweichung finden. Statt: Balancing-Binary in der Shell.",
		Annotations: readOnly(),
	}, s.replayRun)
}

// registerGameData sind die lesenden Tools für Berichte und Spielstände (B-063, seit WZ63.5 aus k3c-workbench).
func registerGameData(s *Server) {
	add(s, &mcp.Tool{
		Name: "reports_list",
		Description: "Xbox-Berichte der Gamepad-Testseite (reports/*.json), neueste zuerst: Datum, Gerät, Controller, FPS. " +
			"Nutze es bei: neue Berichte von der Xbox finden. Statt: ls reports/.",
		Annotations: readOnly(),
	}, s.reportsList)
	add(s, &mcp.Tool{
		Name: "report_read",
		Description: "Ein Xbox-Bericht verdichtet: Controller mit gesehenen Tasten, FPS je Sprite-Stufe, Vollbild, Zurück-Navigationen. " +
			"Nutze es bei: Controller- oder FPS-Befund eines Berichts. Statt: JSON-Datei lesen.",
		Annotations: readOnly(),
	}, s.reportRead)
	add(s, &mcp.Tool{
		Name: "saves_list",
		Description: "Spielstände (saves/ oder K3C_SAVES_DIR) mit Stufe, Tag, Spielern und Datum, Sicherungen eingeschlossen. " +
			"Nutze es bei: welcher Spielstand existiert und wie weit er ist. Statt: saves/ durchsuchen.",
		Annotations: readOnly(),
	}, s.savesList)
}
