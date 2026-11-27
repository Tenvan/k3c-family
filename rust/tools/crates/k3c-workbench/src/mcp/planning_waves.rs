//! Die lesenden Wellen-Tools `plan_waves`, `plan_queue` und `plan_check` (Port von `tools_planning_waves.go`).

use k3c_kit::mcp::Workspace;
use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};

use super::planning::in_workspace;
use super::tools::Workbench;
use crate::planning::{self, CheckIn, QueueFilter, WavesFilter};

#[tool_router(router = planning_waves_router, vis = "pub(super)")]
impl Workbench {
    #[tool(
        description = "Arbeitswellen nach docs/arbeitsweise.md › Parallelität und Wellen aus den lokalen Dateien: \
            Kandidaten sind offene autonome Sessions aktiver Sprints (Dateimenge aus „Erlaubte Dateien“, Sperre aus der \
            Sprint-README), freigegebene Tickets ohne Sprint und freigegebene geplante Sprints mit „Braucht 🧑: –“. Ebene \
            aus „Abhängig von“, Kollision bei überlappendem Bereich (Präfix, ohne Sammeldateien) oder gleicher Sperre; \
            Rang, dann Prio, dann Reihenfolge entscheidet. Zwei Lanes: zweit (Second Worker, autonom) und haupt \
            (Hauptrechner). Jeder Knoten bekommt eine Lane: Session mit Lane zweit gehört zweit, anderer Lane-Wert haupt, \
            Sim-Domäne nur zweit, sonst die erste freie (zweit zuerst). Sessions in Arbeit stehen in Welle 1 und belegen \
            ihre Lane. wave_count zeigt die ersten N Wellen (Standard 1). Je zurückgestelltem Knoten der Grund, dazu die \
            Knoten in keiner Welle (Bereich fehlt, wartet auf …, Fehler: unbekannte ID). project und domain grenzen nur \
            die Anzeige ein. Nutze es bei: Welle planen oder starten. Statt: Wellen von Hand aus Tickets und Sprints \
            rechnen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn plan_waves(
        &self,
        Parameters(input): Parameters<WavesFilter>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        in_workspace(ws, move |root| planning::waves(&root, &input)).await
    }

    #[tool(
        description = "Ralf-Queue nach docs/arbeitsweise.md › Ralf-Queue: Freigaben (Specs im Entwurf mit „Braucht 🧑: \
            Freigabe“), Entscheidungen und Abnahmen je Gerät (PC, unbekannt): offene Mensch-Sessions aktiver Sprints mit \
            dem Gerät aus einer Zeile „Gerät:“ im Abschnitt Kontext, dazu Tickets und Sprints mit „Braucht 🧑: Abnahme“ \
            oder „Gerät“. device zeigt nur die Abnahmen an einem Gerät. Nutze es bei: 🧑 gesammelt vorlegen, was \
            Freigabe, Entscheidung oder Abnahme braucht. Statt: Mensch-Sessions und Specs einzeln suchen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn plan_queue(
        &self,
        Parameters(input): Parameters<QueueFilter>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        in_workspace(ws, move |root| planning::queue(&root, &input)).await
    }

    #[tool(
        description = "Prüft die Planungsangaben eines Tickets oder Sprints für Arbeitswellen (docs/arbeitsweise.md › \
            Planungsangaben) und nennt die Mängel: Planungsangaben oder einzelne Zeilen fehlen, Bereich leer, Umgebung: ? \
            (beim Sprint je offener Session), Größe: L, unbekannte ID oder Freitext in „Abhängig von“, Bereich überlappt \
            mit einem anderen offenen Ticket. Archivierte Tickets prüft es nicht; es ändert nichts. Nutze es bei: vor der \
            Freigabe einer Spec und vor plan_waves. Statt: Regeln und Einschränkungen von Hand prüfen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn plan_check(
        &self,
        Parameters(input): Parameters<CheckIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        in_workspace(ws, move |root| planning::check(&root, &input)).await
    }
}
