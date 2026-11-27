//! Welten-Editor der Oberfläche (Port von `app_world.go`): die 15 Methoden `World*` mit ihren Einträgen der
//! Aufruf-Tabelle und der Wächter für `world:changed`. Die Methoden blockieren (Spiel-EXE); Command und UI-API rufen die
//! Tabelle außerhalb der Laufzeit-Threads.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use k3c_kit::uikit;
use serde_json::{Value, json};

use super::rpc::encode;
use super::{App, EV_WORLD, Emit, emit_to};
use crate::world::{self, Changed, CreateView, Detail, Editor, Info, LevelView, PicksView, ValuesView, Watcher};

#[cfg(test)]
#[path = "world_tests.rs"]
mod tests;

/// Takt des Welten-Wächters (B-1216).
const WORLD_POLL: Duration = Duration::from_secs(1);

impl App {
    /// Welten-Editor der Repo-Wurzel.
    fn worlds(&self) -> Editor {
        self.wait();
        Editor::new(&self.root, (self.world_engine)(&self.root))
    }

    /// Änderungen an `data/worlds/` und den Basis-Dateien erreichen die Oberfläche als Ereignis `world:changed`, egal ob
    /// Editor, MCP oder Hand sie schrieben (B-1216).
    pub(super) fn watch_worlds(&self) -> Watcher {
        let forward: Option<Emit> = self.forward.clone();
        let bus = Arc::clone(&self.bus);
        world::watch(self.root.clone(), WORLD_POLL, move |ids| {
            emit_to(forward.as_ref(), &bus, EV_WORLD, json!(Changed { ids }));
        })
    }

    /// Log einer Änderung: ohne `error` die Meldung `saved`, sonst `rejected` mit dem Grund.
    fn note(&self, error: &str, [rejected, saved]: [&str; 2], fields: &[(&str, Value)]) {
        let mut attrs = vec![("ns", json!("dev"))];
        attrs.extend_from_slice(fields);
        if error.is_empty() {
            self.logger().info(saved, &attrs);
        } else {
            attrs.push(("error", json!(error)));
            self.logger().warn(rejected, &attrs);
        }
    }

    /// Die Welten, die Standard-Welt zuerst (Methode `WorldList`).
    pub fn world_list(&self) -> Vec<Info> {
        self.worlds().list()
    }

    /// Kopf und Inseln der Welt `id` (Methode `WorldDetail`).
    pub fn world_detail(&self, id: &str) -> Detail {
        self.worlds().load(id)
    }

    /// Level der Insel in der Stufe zum Seed der Engine (Methode `WorldLevel`).
    pub fn world_level(&self, id: &str, island: &str, stage: i64, seed: &str) -> LevelView {
        self.worlds().level(id, island, stage, seed)
    }

    /// Werte der Welt mit Basiswert und Override je Feld (Methode `WorldValues`).
    pub fn world_values(&self, id: &str) -> ValuesView {
        self.worlds().values(id)
    }

    /// Übersteuert ein Feld der Welt mit `value` (JSON); `null` setzt es zurück. Geschrieben wird nur eine gültige Welt
    /// (Methode `WorldOverrideSet`).
    pub fn world_override_set(&self, id: &str, file: &str, path: &[String], value: &str) -> ValuesView {
        let view = self.worlds().set_override(id, file, path, value);
        let field = format!("{file}.{}", path.join("."));
        let texts = ["🚫 Override abgelehnt", "💾 Override gespeichert"];
        self.note(&view.error, texts, &[("world", json!(id)), ("field", json!(field))]);
        view
    }

    /// Legt eine neue Welt an (Methode `WorldCreate`, B-954).
    pub fn world_create(&self, id: &str, name: &str, extends: &str) -> CreateView {
        let view = self.worlds().create_view(id, name, extends);
        let texts = ["🚫 Welt nicht angelegt", "💾 Welt angelegt"];
        self.note(&view.error, texts, &[("world", json!(id))]);
        view
    }

    /// Löscht die Welt `id`; die Oberfläche fragt vorher nach dem Namen der Welt (Methode `WorldDelete`, B-1172).
    pub fn world_delete(&self, id: &str) -> CreateView {
        let view = self.worlds().delete_view(id);
        let texts = ["🚫 Welt nicht gelöscht", "🧹 Welt gelöscht"];
        self.note(&view.error, texts, &[("world", json!(id))]);
        view
    }

    /// Setzt einen Natur-Wert der Insel-Stufe fest, `None` setzt ihn auf „aus Seed“ zurück; die Antwort ist das neue
    /// Level (Methode `WorldNatureSet`, B-891).
    pub fn world_nature_set(
        &self,
        id: &str,
        island: &str,
        stage: i64,
        key: &str,
        value: Option<i64>,
        seed: &str,
    ) -> LevelView {
        let view = self.worlds().set_nature(id, island, stage, key, value, seed);
        let texts = ["🚫 Natur-Wert abgelehnt", "💾 Natur-Wert gespeichert"];
        let fields = [("world", json!(id)), ("island", json!(island)), ("key", json!(key))];
        self.note(&view.rejected, texts, &fields);
        view
    }

    /// Setzt die Art der Welt ("spinoff" | "new", leer entfernt sie; Methode `WorldSetKind`, B-1217).
    pub fn world_set_kind(&self, id: &str, kind: &str) -> CreateView {
        let view = self.worlds().set_kind_view(id, kind);
        let texts = ["🚫 Art der Welt abgelehnt", "💾 Art der Welt gespeichert"];
        self.note(&view.error, texts, &[("world", json!(id)), ("kind", json!(kind))]);
        view
    }

    /// Setzt die Eltern-Welt; leer entfernt sie (Methode `WorldSetExtends`, B-1217).
    pub fn world_set_extends(&self, id: &str, parent: &str) -> CreateView {
        let view = self.worlds().set_extends_view(id, parent);
        let texts = ["🚫 Eltern-Welt abgelehnt", "💾 Eltern-Welt gespeichert"];
        self.note(&view.error, texts, &[("world", json!(id)), ("parent", json!(parent))]);
        view
    }

    /// Setzt die Etappe einer Insel; `None` entfernt sie (Methode `WorldIslandEtappe`, B-963).
    pub fn world_island_etappe(&self, id: &str, island: &str, etappe: Option<i64>) -> CreateView {
        let view = self.worlds().set_etappe_view(id, island, etappe);
        let texts = ["🚫 Etappe abgelehnt", "💾 Etappe gespeichert"];
        self.note(&view.error, texts, &[("world", json!(id)), ("island", json!(island))]);
        view
    }

    /// Wetter, Reittier und Einsiedler der Welt samt gültigen IDs (Methode `WorldPicks`, SI7.1).
    pub fn world_picks(&self, id: &str) -> PicksView {
        self.worlds().picks_view(id)
    }

    /// Setzt Wetter, Reittier oder Einsiedler; "" entfernt den Eintrag (Methode `WorldSetPick`, SI7.1).
    pub fn world_set_pick(&self, id: &str, key: &str, value: &str) -> CreateView {
        let view = self.worlds().set_pick_view(id, key, value);
        let texts = ["🚫 Auswahl abgelehnt", "💾 Auswahl gespeichert"];
        let fields = [("world", json!(id)), ("art", json!(key)), ("wert", json!(value))];
        // Wie Go steht der Wert nur beim Speichern im Log, die Art immer.
        let used = if view.error.is_empty() { 3 } else { 2 };
        self.note(&view.error, texts, &fields[..used]);
        view
    }

    /// Setzt den Endboss der Insel; leer entfernt ihn (Methode `WorldIslandEndBoss`, B-981).
    pub fn world_island_end_boss(&self, id: &str, island: &str, boss: &str) -> CreateView {
        let view = self.worlds().set_island_end_boss(id, island, boss);
        let texts = ["🚫 Endboss abgelehnt", "💾 Endboss gespeichert"];
        let fields = [("world", json!(id)), ("island", json!(island)), ("boss", json!(boss))];
        let used = if view.error.is_empty() { 3 } else { 2 };
        self.note(&view.error, texts, &fields[..used]);
        view
    }

    /// Ersetzt die Skins der Welt; ein leerer Satz entfernt sie (Methode `WorldSkins`, B-984).
    pub fn world_skins(&self, id: &str, skins: &BTreeMap<String, String>) -> CreateView {
        let view = self.worlds().set_skins(id, skins);
        let texts = ["🚫 Skins abgelehnt", "💾 Skins gespeichert"];
        let fields = [("world", json!(id)), ("anzahl", json!(skins.len()))];
        let used = if view.error.is_empty() { 2 } else { 1 };
        self.note(&view.error, texts, &fields[..used]);
        view
    }
}

pub(super) fn list(app: &App, args: &[Value]) -> Result<Value, String> {
    uikit::args::<()>(args.to_vec())?;
    encode(app.world_list())
}

pub(super) fn detail(app: &App, args: &[Value]) -> Result<Value, String> {
    let (id,): (String,) = uikit::args(args.to_vec())?;
    encode(app.world_detail(&id))
}

pub(super) fn level(app: &App, args: &[Value]) -> Result<Value, String> {
    let (id, island, stage, seed): (String, String, i64, String) = uikit::args(args.to_vec())?;
    encode(app.world_level(&id, &island, stage, &seed))
}

pub(super) fn values(app: &App, args: &[Value]) -> Result<Value, String> {
    let (id,): (String,) = uikit::args(args.to_vec())?;
    encode(app.world_values(&id))
}

pub(super) fn override_set(app: &App, args: &[Value]) -> Result<Value, String> {
    let (id, file, path, value): (String, String, Option<Vec<String>>, Value) = uikit::args(args.to_vec())?;
    let path = path.unwrap_or_default();
    encode(app.world_override_set(&id, &file, &path, &value.to_string()))
}

pub(super) fn create(app: &App, args: &[Value]) -> Result<Value, String> {
    let (id, name, extends): (String, String, String) = uikit::args(args.to_vec())?;
    encode(app.world_create(&id, &name, &extends))
}

pub(super) fn delete(app: &App, args: &[Value]) -> Result<Value, String> {
    let (id,): (String,) = uikit::args(args.to_vec())?;
    encode(app.world_delete(&id))
}

pub(super) fn nature_set(app: &App, args: &[Value]) -> Result<Value, String> {
    type Args = (String, String, i64, String, Option<i64>, String);
    let (id, island, stage, key, value, seed): Args = uikit::args(args.to_vec())?;
    encode(app.world_nature_set(&id, &island, stage, &key, value, &seed))
}

pub(super) fn set_kind(app: &App, args: &[Value]) -> Result<Value, String> {
    let (id, kind): (String, String) = uikit::args(args.to_vec())?;
    encode(app.world_set_kind(&id, &kind))
}

pub(super) fn set_extends(app: &App, args: &[Value]) -> Result<Value, String> {
    let (id, parent): (String, String) = uikit::args(args.to_vec())?;
    encode(app.world_set_extends(&id, &parent))
}

pub(super) fn island_etappe(app: &App, args: &[Value]) -> Result<Value, String> {
    let (id, island, etappe): (String, String, Option<i64>) = uikit::args(args.to_vec())?;
    encode(app.world_island_etappe(&id, &island, etappe))
}

pub(super) fn picks(app: &App, args: &[Value]) -> Result<Value, String> {
    let (id,): (String,) = uikit::args(args.to_vec())?;
    encode(app.world_picks(&id))
}

pub(super) fn set_pick(app: &App, args: &[Value]) -> Result<Value, String> {
    let (id, key, value): (String, String, String) = uikit::args(args.to_vec())?;
    encode(app.world_set_pick(&id, &key, &value))
}

pub(super) fn island_end_boss(app: &App, args: &[Value]) -> Result<Value, String> {
    let (id, island, boss): (String, String, String) = uikit::args(args.to_vec())?;
    encode(app.world_island_end_boss(&id, &island, &boss))
}

pub(super) fn skins(app: &App, args: &[Value]) -> Result<Value, String> {
    let (id, skins): (String, Option<BTreeMap<String, String>>) = uikit::args(args.to_vec())?;
    encode(app.world_skins(&id, &skins.unwrap_or_default()))
}
