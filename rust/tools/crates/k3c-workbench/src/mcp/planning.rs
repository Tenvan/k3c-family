//! Tools `plan_list`, `plan_get`, `plan_next`, `plan_create`, `plan_set`, `plan_section`, `plan_sections` und
//! `plan_delete` (Port von `tools_planning.go`) auf [`crate::planning`]. Die Planung arbeitet synchron und sperrt ihre
//! Schreibwege selbst; hier läuft sie nur in einem blockierenden Faden.

use std::collections::BTreeMap;
use std::path::PathBuf;

use k3c_kit::mcp::Workspace;
use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;

use super::tools::Workbench;
use crate::planning::{self, Filter, NewDoc, NextFilter};

#[cfg(test)]
#[path = "planning_tests.rs"]
mod tests;

/// Eingabe von `plan_get` und `plan_delete`.
#[derive(Deserialize, JsonSchema)]
pub(super) struct IdIn {
    #[schemars(
        description = "Ticket B-123 (doppelte Nummer: B-123-kurzname), Sprint M8, Session M8.1 oder Projekt GRA"
    )]
    id: String,
}

#[derive(Deserialize, JsonSchema)]
pub(super) struct SetIn {
    #[schemars(
        description = "Ticket B-123 (doppelte Nummer: B-123-kurzname), Sprint M8, Session M8.1 oder Projekt GRA"
    )]
    id: String,
    #[schemars(description = "Kopf-Felder wie in der Vorlage, z. B. {\"Status\": \"fertig\"}")]
    fields: BTreeMap<String, String>,
}

#[derive(Deserialize, JsonSchema)]
pub(super) struct SectionIn {
    #[schemars(
        description = "Ticket B-123 (doppelte Nummer: B-123-kurzname), Sprint M8, Session M8.1 oder Projekt GRA"
    )]
    id: String,
    #[schemars(description = "Abschnitt der Vorlage ohne ##, z. B. Ausgangslage, Ergebnis")]
    section: String,
    #[schemars(description = "neuer Inhalt (Markdown, Unterüberschriften ab ###)")]
    text: String,
}

#[derive(Deserialize, JsonSchema)]
pub(super) struct SectionsIn {
    #[schemars(
        description = "Ticket B-123 (doppelte Nummer: B-123-kurzname), Sprint M8, Session M8.1 oder Projekt GRA"
    )]
    id: String,
    #[schemars(
        description = "Abschnitt der Vorlage ohne ## → neuer Inhalt (Markdown, Unterüberschriften ab ###), z. B. \
            {\"Ziel\": \"…\", \"Kontext\": \"…\"}"
    )]
    sections: BTreeMap<String, String>,
}

/// Ruft die Planung auf der Wurzel des Checkouts in einem blockierenden Faden.
pub(super) async fn in_workspace(
    ws: Workspace,
    call: impl FnOnce(PathBuf) -> Result<String, String> + Send + 'static,
) -> Result<String, String> {
    match tokio::task::spawn_blocking(move || call(ws.root)).await {
        Ok(result) => result,
        // Der Server macht aus der Panik ein Fehler-Ergebnis.
        Err(e) if e.is_panic() => std::panic::resume_unwind(e.into_panic()),
        Err(e) => Err(e.to_string()),
    }
}

/// Es gibt keine ungeplanten Tickets (Regel 🧑 2026-11-14): Ein neues Ticket nennt gleich Projekt und Sprint; fehlt
/// der passende Sprint, zuerst `plan_create` kind sprint.
fn ticket_planned(input: &NewDoc) -> Result<(), String> {
    if input.kind != "ticket" {
        return Ok(());
    }
    let missing: Vec<&str> = ["Projekt", "Sprint"]
        .into_iter()
        .filter(|key| {
            let value = input.fields.get(*key).map_or("", |v| v.trim());
            matches!(value, "" | "–" | "-")
        })
        .collect();
    if missing.is_empty() {
        return Ok(());
    }
    Err(format!(
        "neues Ticket ohne {}: es gibt keine ungeplanten Tickets, Projekt und Sprint gleich mitgeben (fehlt der \
         Sprint, zuerst plan_create kind sprint)",
        missing.join(" und ")
    ))
}

#[tool_router(router = planning_router, vis = "pub(super)")]
impl Workbench {
    #[tool(
        description = "Sprints mit Sessions und Tickets als je eine Zeile, filterbar nach kind, status, domain, sprint, \
            projekt; archive mit erledigten Tickets. Sprints mit Projekt nach Rang und Platz im Projekt, ohne Projekt \
            dahinter. kind projekt: Projekte nach Rang mit Sprints und Fortschritt. Tickets beginnen mit der Zeile \
            `Tickets: <gezeigt> von <alle nicht archivierten> · <n> offen · <n> eingeplant · <n> ungeplant` \
            (offen/eingeplant = Status, ungeplant = ohne Sprint); sprint `ungeplant` (oder –) zeigt nur Tickets ohne \
            Sprint. Nutze es bei: Überblick über Sprints, Sessions und Tickets. Statt: docs/sprints und docs/backlog \
            durchsuchen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn plan_list(
        &self,
        Parameters(input): Parameters<Filter>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        in_workspace(ws, move |root| planning::list(&root, &input)).await
    }

    #[tool(
        description = "Ein Ticket, eine Sprint-README, eine Session oder ein Projekt als Markdown, mit Pfad in der ersten \
            Zeile. Nutze es bei: eine Planungsdatei lesen. Statt: Datei suchen und lesen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn plan_get(
        &self,
        Parameters(input): Parameters<IdIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        in_workspace(ws, move |root| planning::get(&root, &input.id)).await
    }

    #[tool(
        description = "Nächste Session nach docs/arbeitsweise.md › Autonomer Ablauf, Schritt 1, aus den lokalen Dateien: \
            aktive Projekte nach Rang, ihre aktiven Sprints in Tabellen-Reihenfolge, erste Session offen und autonom mit \
            erledigten Abhängigkeiten, die mit keiner Session in Arbeit kollidiert (wie plan_waves: gleiche Sperre der \
            Sprint-README oder überlappende Erlaubte Dateien; ohne Dateimenge noch Domäne bzw. Lane); domain grenzt ein. \
            Antwort: Kopf (ID, Pfad, Sprint, Domäne, Lane, Umgebung, Modell, Branch sprint/<präfix>, Kriterien), \
            Session-Datei, ihre Kriterien aus der Sprint-README und höchstens drei parallel mögliche Sessions (ohne \
            Kollision untereinander, je mit Modell); sonst der Grund je Projekt. Je aktivem Sprint gilt der Stand von \
            refs/remotes/origin/sprint/<präfix> (README, Sessions; kein fetch), sonst der lokale Stand; die Antwort nennt \
            das je Sprint. Nutze es bei: Start einer autonomen Session. Statt: Projekte, Sprint-READMEs und \
            Session-Dateien einzeln lesen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn plan_next(
        &self,
        Parameters(input): Parameters<NextFilter>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        in_workspace(ws, move |root| planning::next(&root, &input)).await
    }

    #[tool(
        description = "Legt Ticket (nächste Nummer, Index), Sprint (in geplant/, Fahrplan), Session (Session-Tabelle) \
            oder Projekt (docs/projekte, Übersicht, Rang = letzter + 1) als Kopie der Vorlage an. Inhalte danach mit \
            plan_sections füllen. Nutze es bei: neues Ticket, neuer Sprint, neue Session, neues Projekt. Statt: Vorlage \
            von Hand kopieren.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn plan_create(
        &self,
        Parameters(input): Parameters<NewDoc>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        ticket_planned(&input)?;
        in_workspace(ws, move |root| planning::create(&root, &input)).await
    }

    #[tool(
        description = "Setzt Kopf-Felder (Titel (nur Sprint und Ticket: ändert die Überschrift samt Fahrplan, \
            Projekt-Tabelle und Index), Status, Prio, Sprint, Projekt, Reife, Spec, Revision, Freigabe …; Sprints haben \
            keine Prio) und zieht nach: Index, Session-Tabelle, Fahrplan, Archiv bei erledigt/verworfen, Sprint-Ordner \
            beim Status, Sprint-Tabelle des Projekts. Session: Status (auch verworfen), Domäne (REG, SIM, SRV, CLI, \
            PLAT, INF, DEV, PAP; Feld, Überschrift und Fahrplan des Sprints folgen; am Sprint nicht setzbar), Lane (– \
            oder Kurzname a-z, 0-9, -) und Modell (–, haiku, sonnet, opus, fable), beide auch in älteren Dateien ohne das \
            Feld. Projekt: Status, Rang (die anderen rücken lückenlos), Sprints (neue Reihenfolge, kommagetrennt). \
            Ticket-Relationen: Duplikat von (eine ID, setzt verworfen), Ersetzt (IDs, kommagetrennt); geprüft auf ID, \
            Selbstverweis, Zyklus. Nutze es bei: Status, Prio oder andere Kopf-Felder ändern. Statt: Datei, Index und \
            Fahrplan von Hand bearbeiten.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn plan_set(
        &self,
        Parameters(input): Parameters<SetIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        in_workspace(ws, move |root| planning::set(&root, &input.id, &input.fields)).await
    }

    #[tool(
        description = "Ersetzt den Inhalt eines Abschnitts (## Name) eines Tickets, Sprints, einer Session oder eines \
            Projekts. Nutze es bei: Abschnitt wie Ergebnis oder Ziel füllen. Statt: Datei von Hand bearbeiten.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn plan_section(
        &self,
        Parameters(input): Parameters<SectionIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        in_workspace(ws, move |root| {
            planning::section(&root, &input.id, &input.section, &input.text)
        })
        .await
    }

    #[tool(
        description = "Ersetzt mehrere Abschnitte (## Name) eines Tickets, Sprints, einer Session oder eines Projekts in \
            einem Aufruf; erst werden alle Abschnitte geprüft, ein unbekannter bricht ab, ohne etwas zu schreiben. Nutze \
            es bei: Spec oder Session mit mehreren Abschnitten füllen. Statt: plan_section je Abschnitt.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn plan_sections(
        &self,
        Parameters(input): Parameters<SectionsIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        in_workspace(ws, move |root| planning::sections(&root, &input.id, &input.sections)).await
    }

    #[tool(
        description = "Löscht einen Sprint-Entwurf in geplant/, eine Session darin oder ein Projekt ohne Sprints und \
            Tickets. Tickets nie: plan_set Status verworfen. Nutze es bei: Entwurf verwerfen. Statt: Dateien und \
            Verweise von Hand löschen.",
        annotations(
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn plan_delete(
        &self,
        Parameters(input): Parameters<IdIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        in_workspace(ws, move |root| planning::delete(&root, &input.id)).await
    }
}
