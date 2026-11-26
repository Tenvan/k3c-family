//! Der einzige Katalog aller Tools des Studios (Port von `tools.go`): Name, Beschreibung, Annotations und Handler.
//! Ein neues Tool kommt nur hier hinzu; spätere Sessions ergänzen ihre Gruppe als weiteren `tool_router`, tragen die
//! Namen in [`CATALOG`] ein und kürzen `PENDING_TOOLS` in `tests/mcp_tools_golden.rs`.

use rmcp::handler::server::common::Extension;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::service::{RequestContext, RoleServer};
use rmcp::{tool, tool_router};

use super::kit::Workspace;
use super::tools_engine::{self, LevelIn};
use super::tools_gamedata::{self, ReportIn};
use super::tools_world::{self, ChunksIn, CreateIn, DeleteIn, IdIn, OverrideIn, SetIn, blocking};
use super::tools_world_story::{self, StoryIn};
use crate::world::{Editor, EngineFactory};

/// Empfänger der Tool-Methoden.
#[derive(Clone)]
pub struct Studio {
    /// Engine-Seite des Welten-Editors.
    pub(super) world_engine: EngineFactory,
}

impl Studio {
    /// Welten-Editor des Checkouts eines Aufrufs.
    fn worlds(&self, ws: &Workspace) -> Editor {
        Editor::new(&ws.root, (self.world_engine)(&ws.root))
    }
}

/// Reihenfolge der Tools in den Zählern wie in Go `register`.
pub(super) const CATALOG: &[&str] = &[
    "level_generate",
    "reports_list",
    "report_read",
    "saves_list",
    "world_list",
    "world_get",
    "world_create",
    "world_delete",
    "world_set",
    "world_override_set",
    "world_chunks_set",
    "world_story_set",
    "world_check",
];

/// Alle Routen.
pub(super) fn router() -> ToolRouter<Studio> {
    Studio::engine_router() + Studio::gamedata_router() + Studio::world_read_router() + Studio::world_write_router()
}

/// Das Engine-Tool (B-047): rechnet mit der Spiel-EXE des Checkouts, ohne laufenden Server.
#[tool_router(router = engine_router)]
impl Studio {
    #[tool(
        description = "Erzeugt ein Level mit der Spiel-EXE des Checkouts (k3c-game sim level, kein Server nötig; fehlt sie oder ist sie veraltet, nennt der Fehler den Bau-Befehl): Kopfzeile, eine Zeile je Abschnitt, Objekte nach Art, Warnungen der Prüfung. Gleicher Seed ergibt dasselbe Level wie im Spiel. Nutze es bei: Level eines Seeds prüfen. Statt: Spiel starten und Level von Hand ansehen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn level_generate(
        &self,
        Parameters(input): Parameters<LevelIn>,
        Extension(ws): Extension<Workspace>,
        context: RequestContext<RoleServer>,
    ) -> Result<String, String> {
        let canceled = context.ct.cancelled_owned();
        tools_engine::level_generate(&ws, &input, |o| tools_engine::game_call(o, canceled)).await
    }
}

/// Die lesenden Tools für Berichte und Spielstände (B-063).
#[tool_router(router = gamedata_router)]
impl Studio {
    #[tool(
        description = "Xbox-Berichte der Gamepad-Testseite (reports/*.json), neueste zuerst: Datum, Gerät, Controller, FPS. Nutze es bei: neue Berichte von der Xbox finden. Statt: ls reports/.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn reports_list(&self, Extension(ws): Extension<Workspace>) -> Result<String, String> {
        tools_gamedata::reports_list(&ws)
    }

    #[tool(
        description = "Ein Xbox-Bericht verdichtet: Controller mit gesehenen Tasten, FPS je Sprite-Stufe, Vollbild, Zurück-Navigationen. Nutze es bei: Controller- oder FPS-Befund eines Berichts. Statt: JSON-Datei lesen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn report_read(
        &self,
        Parameters(input): Parameters<ReportIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        tools_gamedata::report_read(&ws, &input)
    }

    #[tool(
        description = "Spielstände (saves/ oder K3C_SAVES_DIR) mit Stufe, Tag, Spielern und Datum, Sicherungen eingeschlossen. Nutze es bei: welcher Spielstand existiert und wie weit er ist. Statt: saves/ durchsuchen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn saves_list(&self, Extension(ws): Extension<Workspace>) -> Result<String, String> {
        tools_gamedata::saves_list(&ws)
    }
}

/// Die lesenden Welten-Tools (B-954).
#[tool_router(router = world_read_router)]
impl Studio {
    #[tool(
        description = "Welten unter data/worlds/ mit ID, Name, Eltern-Welt und Zahl der Inseln, die Standard-Welt zuerst. Nutze es bei: welche Welten es gibt, vor world_create. Statt: data/worlds/ auflisten und Dateien öffnen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn world_list(&self, Extension(ws): Extension<Workspace>) -> Result<String, String> {
        let editor = self.worlds(&ws);
        blocking(move || tools_world::list(&editor)).await
    }

    #[tool(
        description = "Eine Welt: Kopf, Inseln mit Tiefen und Biomen, geplante Chunks, die Geschichte (Wortschatz und Zeilen je Wendepunkt, fehlende als „fehlt“), dazu der Text der Datei. Nutze es bei: vor jeder Änderung an einer Welt. Statt: data/worlds/<id>.json lesen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn world_get(
        &self,
        Parameters(input): Parameters<IdIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        let editor = self.worlds(&ws);
        blocking(move || tools_world::get(&editor, &input.id)).await
    }

    #[tool(
        description = "Prüft die Datei einer Welt wie das Laden im Spiel (strenge Felder, Inseln, Tiefen, Bosse, Häfen, Chunks, Geschichte) und mit allen Nachfolger-Welten; fehlende Einträge der Geschichte kommen als Hinweis, nicht als Fehler; ändert nichts. Nutze es bei: nach Änderungen, bevor die Welt gespielt wird. Statt: Spiel starten und auf den Ladefehler warten.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn world_check(
        &self,
        Parameters(input): Parameters<IdIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        let editor = self.worlds(&ws);
        blocking(move || tools_world::check(&editor, &input.id)).await
    }
}

/// Die schreibenden Welten-Tools (B-954, B-975).
#[tool_router(router = world_write_router)]
impl Studio {
    #[tool(
        description = "Legt eine neue Welt an (data/worlds/<id>.json): eine Insel island1 mit Tiefen 0-4, Kampf groups/5, optional von einer Eltern-Welt erbend und mit Geschichte (story); eine vorhandene Welt wird nie überschrieben. Danach mit world_set, world_override_set, world_chunks_set und world_story_set ausbauen. Nutze es bei: neue Welt entwerfen. Statt: Welt-Datei von Hand schreiben.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn world_create(
        &self,
        Parameters(input): Parameters<CreateIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        let editor = self.worlds(&ws);
        blocking(move || tools_world::create(&editor, input)).await
    }

    #[tool(
        description = "Löscht die Datei einer Welt (data/worlds/<id>.json) endgültig (sie liegt in Git); braucht confirm=true. Abgelehnt werden die festen Welten (standard und jede Welt aus data/paper.json › weltbilder) und Welten mit Erben (andere Welten, deren extends auf sie zeigt; die Antwort nennt sie). Spielstände der Welt bleiben, die Antwort warnt vor ihnen. Nutze es bei: eine Test- oder Entwurfs-Welt entfernen. Statt: die Datei von Hand löschen.",
        annotations(
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn world_delete(
        &self,
        Parameters(input): Parameters<DeleteIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        let editor = self.worlds(&ws);
        blocking(move || tools_world::delete(&editor, &input)).await
    }

    #[tool(
        description = "Ändert Kopf (name, gewaesser = Gewässer der Welt: Fluss, See, Küste oder Meer, notes, extends, kind = spinoff für Ableger oder Episode einer Eltern-Welt | new für komplett neue Welt, combat, skins = Basis-Asset → Ersatz-Asset) und Inseln (anhängen oder Felder ändern: Tiefen oder Biome, Skalierung, plannedChunks, chunkCounts, endBoss) einer Welt; der ganze Patch wird angewendet und dann einmal geprüft wie beim Laden im Spiel, bei Fehler bleibt die Datei. Insel anhängen in einem Aufruf: den Hafen der Insel davor mitgeben, z. B. islands: [{id: island1, plannedChunks: {\"0\": [{slot: -7, kind: harbor}]}}, {id: island2}]. Nutze es bei: Inseln, Tiefen, Biome, Kampfvariante einer Welt festlegen. Statt: Welt-Datei von Hand ändern.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn world_set(
        &self,
        Parameters(input): Parameters<SetIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        let editor = self.worlds(&ws);
        blocking(move || tools_world::set(&editor, input)).await
    }

    #[tool(
        description = "Übersteuert ein Basis-Feld (data/<file>.json › path, z. B. economy › purse › startGold) in den `overrides` der Welt; null oder der Basiswert setzt es zurück; geprüft mit allen Nachfolger-Welten. Erlaubte Dateien: buildings, economy, enemies, events, goals, hub, monarch, ship, troops, waves, welt-events; gesperrt sind u. a. hub › sitePlacement und economy › payRangeUnits (docs/rules/welten.md § 1.3). Nutze es bei: Werte einer Welt anders als die Basis (Gold, Preise, Händler, Handwerker). Statt: data/*.json ändern (gilt für alle Welten).",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn world_override_set(
        &self,
        Parameters(input): Parameters<OverrideIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        let editor = self.worlds(&ws);
        blocking(move || tools_world::override_set(&editor, &input)).await
    }

    #[tool(
        description = "Setzt die geplanten Chunks (plannedChunks) und/oder die Chunk-Anzahl (chunkCounts) einer Tiefe einer Insel; [] bzw. 0 entfernt. Jede Insel außer der letzten braucht in Tiefe 0 einen Hafen-Chunk (kind harbor). Nutze es bei: Chunk-Planung einer Insel-Stufe. Statt: plannedChunks in der Datei von Hand bearbeiten.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn world_chunks_set(
        &self,
        Parameters(input): Parameters<ChunksIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        let editor = self.worlds(&ws);
        blocking(move || tools_world::chunks_set(&editor, input)).await
    }

    #[tool(
        description = "Setzt die Geschichte einer Welt (story: lexicon, start, etappen, end, bottles) und ihrer Inseln (islands: arrival, boss, done) in einem Aufruf oder je Teil; fehlt ein Feld, bleibt es, [] entfernt den Eintrag. Geschrieben wird nur, wenn die Prüfung wie beim Laden im Spiel besteht (docs/rules/welten.md § 2.2), sonst nennt der Fehler Feld und Zeile; alle übrigen Felder der Datei bleiben byte-gleich. Die Antwort nennt gesetzte, entfernte und noch fehlende Einträge. Nutze es bei: Wortschatz und Zeilen der Geschichte einer Welt schreiben oder ändern. Statt: story in der Welt-Datei von Hand bearbeiten.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn world_story_set(
        &self,
        Parameters(input): Parameters<StoryIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        let editor = self.worlds(&ws);
        blocking(move || tools_world_story::story_set(&editor, input)).await
    }
}
