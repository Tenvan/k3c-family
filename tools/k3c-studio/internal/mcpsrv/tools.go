package mcpsrv

import (
	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// register ist der einzige Katalog aller Tools des Studios: Name, Beschreibung, Annotations und Handler.
// Ein neues Tool kommt nur hier hinzu; Zähler, Parameter-Hinweis und Oberfläche lesen es von hier.
func register(s *Server) {
	registerEngine(s)
	registerAssets(s)
	registerAssetsResearch(s)
	registerAssetsImport(s)
	registerHub(s)
	registerWorld(s)
}

// registerEngine ist das Engine-Tool (B-047, aus k3c-workbench): rechnet im Prozess des Studios über engine/simcli
// (B-924), ohne laufenden Server.
func registerEngine(s *Server) {
	add(s, &mcp.Tool{
		Name: "level_generate",
		Description: "Erzeugt ein Level mit der Engine des Studios (kein Server nötig): Kopfzeile, eine Zeile je Abschnitt, " +
			"Objekte nach Art, Warnungen der Prüfung. Gleicher Seed ergibt dasselbe Level wie im Spiel. " +
			"Nutze es bei: Level eines Seeds prüfen. Statt: Spiel starten und Level von Hand ansehen.",
		Annotations: readOnly(),
	}, s.levelGenerate)
}
