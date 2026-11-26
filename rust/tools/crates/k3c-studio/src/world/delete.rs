//! Welten löschen (Port von `delete.go`, WZ62.6, B-1172). Abgelehnt werden die festen Welten (Standard und jede Welt in
//! `data/paper.json` › weltbilder), eine fehlende Datei und Welten mit Erben. Die Datei wird endgültig gelöscht, sie
//! liegt in Git. Spielstände unter `saves/`, die die Welt nutzen, bleiben unberührt und stehen als Warnung in der
//! Antwort.

use serde::Deserialize;

use super::edit::Warnings;
use super::overrides::write_lock;
use super::values::parse_entries;
use super::{CreateView, Editor, STANDARD_ID, WORLDS_DIR, check_id, null_default};

impl Editor {
    /// Löscht die Datei der Welt `id`; die Antwort sind die Warnungen.
    pub fn delete(&self, id: &str) -> Result<Warnings, String> {
        // Nur nach dem ID-Muster entsteht ein Pfad: nichts außerhalb von `data/worlds/`.
        check_id(id)?;
        let fixed = self.fixed_ids();
        if id == STANDARD_ID || fixed.iter().any(|f| f == id) {
            return Err(format!(
                "die Welt {id:?} ist eine feste Welt und lässt sich nicht löschen"
            ));
        }
        // Ohne lesbare paper.json ist nicht klar, welche Welten fest sind: lieber nichts löschen.
        if fixed.is_empty() {
            return Err(
                "data/paper.json fehlt oder enthält keine weltbilder: die festen Welten sind unbekannt, nichts gelöscht"
                    .to_owned(),
            );
        }
        let _guard = write_lock();
        let path = self.file(id);
        if std::fs::metadata(&path).is_err() {
            return Err(format!("{WORLDS_DIR}/{id}.json gibt es nicht"));
        }
        let heirs = self.heirs(id);
        if !heirs.is_empty() {
            return Err(format!(
                "die Welt {id:?} hat Erben ({}); erst deren extends ändern oder sie löschen",
                heirs.join(", ")
            ));
        }
        std::fs::remove_file(&path).map_err(|e| format!("{WORLDS_DIR}/{id}.json: remove {}: {e}", path.display()))?;
        let saves = self.saves_using(id);
        Ok((!saves.is_empty()).then(|| {
            vec![format!(
                "Spielstände nutzen die gelöschte Welt und lassen sich nicht mehr laden: {}",
                saves.join(", ")
            )]
        }))
    }

    /// Die Welten (IDs, sortiert wie die Liste), deren `extends` auf `id` zeigt.
    fn heirs(&self, id: &str) -> Vec<String> {
        // Eintragsweise statt über ein serde-Struct: Bei doppeltem `extends` gilt wie in Go der letzte Wert; ein
        // Lesefehler hier gäbe sonst eine Welt mit Erben zum Löschen frei.
        let extends = |world: &str| {
            let raw = std::fs::read(self.file(world)).ok()?;
            let entries = parse_entries(&raw).ok()?;
            let (_, value) = entries.into_iter().rfind(|(key, _)| key == "extends")?;
            serde_json::from_str::<String>(value.get()).ok()
        };
        self.list()
            .into_iter()
            .filter(|w| !w.missing && w.id != id && extends(&w.id).as_deref() == Some(id))
            .map(|w| w.id)
            .collect()
    }

    /// Die Namen der Spielstände unter `saves/`, deren Feld `world` `id` ist, sortiert.
    fn saves_using(&self, id: &str) -> Vec<String> {
        #[derive(Default, Deserialize)]
        #[serde(default)]
        struct Head {
            #[serde(deserialize_with = "null_default")]
            world: String,
        }
        let Ok(entries) = std::fs::read_dir(self.root.join("saves")) else {
            return Vec::new();
        };
        let mut out: Vec<String> = entries
            .flatten()
            .filter_map(|entry| {
                let name = entry.file_name().to_string_lossy().into_owned();
                let stem = name.strip_suffix(".json")?;
                let head = serde_json::from_slice::<Head>(&std::fs::read(entry.path()).ok()?).ok()?;
                (head.world == id).then(|| stem.to_owned())
            })
            .collect();
        out.sort();
        out
    }

    /// Wie [`Editor::delete`] mit der Antwort für die Seite.
    pub fn delete_view(&self, id: &str) -> CreateView {
        let (warnings, error) = match self.delete(id) {
            Ok(warnings) => (warnings.unwrap_or_default(), String::new()),
            Err(error) => (Vec::new(), error),
        };
        CreateView {
            id: id.to_owned(),
            warnings: Some(warnings),
            error,
        }
    }
}

#[cfg(test)]
#[path = "delete_tests.rs"]
mod tests;
