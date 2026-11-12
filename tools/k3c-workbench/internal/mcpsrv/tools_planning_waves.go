package mcpsrv

import (
	"context"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-workbench/internal/planning"
)

// registerPlanWaves sind die lesenden Wellen-Tools (B-783): plan_waves und plan_queue.
func registerPlanWaves(s *Server) {
	add(s, &mcp.Tool{
		Name: "plan_waves",
		Description: "Arbeitswellen nach docs/arbeitsweise.md › Parallelität und Wellen aus den lokalen Dateien: Kandidaten sind offene " +
			"autonome Sessions aktiver Sprints (Dateimenge aus „Erlaubte Dateien“, Sperre aus der Sprint-README), freigegebene " +
			"Tickets ohne Sprint und freigegebene geplante Sprints mit „Braucht 🧑: –“. Ebene aus „Abhängig von“, Kollision bei " +
			"überlappendem Bereich (Präfix, ohne Sammeldateien) oder gleicher Sperre; Rang, dann Prio, dann Reihenfolge entscheidet. " +
			"Sessions in Arbeit stehen in Welle 1 und belegen Plätze (slots, höchstens 4). Je zurückgestelltem Knoten der Grund, " +
			"dazu die Knoten in keiner Welle (Bereich fehlt, wartet auf …, Fehler: unbekannte ID). project und domain grenzen nur " +
			"die Anzeige ein. Nutze es bei: Welle planen oder starten. Statt: Wellen von Hand aus Tickets und Sprints rechnen.",
		Annotations: readOnly(),
	}, func(ctx context.Context, in planning.WavesFilter) (string, error) {
		return planning.Waves(s.ws(ctx).root, in)
	})
	add(s, &mcp.Tool{
		Name: "plan_queue",
		Description: "Ralf-Queue nach docs/arbeitsweise.md › Ralf-Queue: Freigaben (Specs im Entwurf mit „Braucht 🧑: Freigabe“), " +
			"Entscheidungen und Abnahmen je Gerät (PC, Xbox/TV, Pi, Mini-PC, Workshop, Veröffentlichung, unbekannt): offene " +
			"Mensch-Sessions aktiver Sprints mit dem Gerät aus einer Zeile „Gerät:“ im Abschnitt Kontext, dazu Tickets und Sprints " +
			"mit „Braucht 🧑: Abnahme“ oder „Gerät“. device zeigt nur die Abnahmen an einem Gerät. " +
			"Nutze es bei: 🧑 gesammelt vorlegen, was Freigabe, Entscheidung oder Abnahme braucht. Statt: Mensch-Sessions und Specs einzeln suchen.",
		Annotations: readOnly(),
	}, func(ctx context.Context, in planning.QueueFilter) (string, error) {
		return planning.Queue(s.ws(ctx).root, in)
	})
}
