//! Asset-Logik des Studios (Port von `tools/assetkit`, RU67). Die Go-Fassung ist das Orakel; Verhalten und Dateiformate
//! bleiben gleich (Leitfaden `docs/standards/rust-werkzeuge.md`).
//!
//! Auf der Wurzel liegen die Namen des Go-Pakets `assets` (`k3c_assetkit::load` wie `assets.Load`):
//! - Slot-Register der Sounds (`data/sounds.json`): je Slot die Zuordnungen nach Präferenz, nur die erste ist aktiv.
//! - Stand des Registers mit Schreibregel und Beobachtung ([`stamp()`], [`watch`]).
//! - Zielmaß der Grafik-Slots ([`set_target_if`]).
//! - Asset-Katalog über `public/` ([`catalog()`], [`search`]) und Credit-Zeilen ([`confirm_license`]).
//! - Web-Recherche ([`Searcher`], [`run_research`]) mit Treffer-Ablage und Lizenz-Einordnung.
//! - Vorschlagsliste lesen und prüfen ([`load_suggestions`], [`check_suggestions`]).
//! - Datei-Import und Treffer-Download ([`import_file`], [`download_hit`]).
//! - Vorrat der Vorschlagsliste: laden, übernehmen, aufräumen ([`vorrat_download()`], [`vorrat_adopt()`], [`vorrat_cleanup`]).
//! - Vorschläge anhängen und verwerfen, Recherche-Übernahme ([`add_suggestions`], [`drop_suggestion`], [`import_research`]).
//! - Kandidaten eines Slots auflisten und zuordnen ([`candidates()`], [`assign_candidate`]).
//!
//! Downloads erwarten einen Client ohne eigene Weiterleitungen ([`net::client`]).
//!
//! Eigene Module bleiben die Go-Pakete [`ticket`], [`windowstate`] und [`browser`]; [`net`] baut die HTTP-Clients.

pub mod browser;
mod candidates;
mod candidates_assign;
mod candidates_hits;
mod catalog;
mod credits;
mod error;
mod fsutil;
mod gojson;
mod import;
mod import_download;
pub mod net;
mod pngsize;
mod register;
mod research;
mod research_add;
mod sources;
mod stamp;
mod suggestions;
mod suggestions_add;
mod suggestions_write;
mod target;
pub mod ticket;
mod vorrat;
mod vorrat_adopt;
mod vorrat_download;
pub mod windowstate;

#[cfg(test)]
mod repo_tests;
#[cfg(test)]
mod testutil;
#[cfg(test)]
mod write_tests;
#[cfg(test)]
mod writepaths_tests;

pub use candidates::{
    Candidate, CandidateOptions, MARK_BLOCKED, MARK_CHECK, MARK_FREE, MARK_SA, ORIGIN_CATALOG, ORIGIN_RESEARCH,
    ORIGIN_STOCK, ORIGIN_SUGGESTION, STAGE_GAME, STAGE_NET, STAGE_STOCK, candidates,
};
pub use candidates_assign::{AssignOptions, assign_candidate};
pub use catalog::{Asset, catalog, license_flag, search};
pub use credits::{LICENSE_CONFIRMED, confirm_license};
pub use error::{Error, Result};
pub use import::import_file;
pub use import_download::{DOWNLOAD_HOSTS, DOWNLOAD_PORTS, download_hit};
pub use register::{
    CATEGORIES, Choice, KIND_GRAPHIC, KIND_SOUND, REGISTER_PATH, Rect, Register, STATUS_ASSIGNED, STATUS_GAP,
    STATUS_MISSING, Size, Slot, load, set_choices, status, write_atomic,
};
pub use research::{
    Hit, LICENSE_CHECK, LICENSE_EXCLUDED, LICENSE_KNOWN, RESEARCH_PATH, Research, ResearchResult, add_hits,
    classify_license, drop_hit, hits_of, load_research, run_research,
};
pub use research_add::{import_research, plan_research_import, research_candidates};
pub use sources::{Searcher, new_searcher};
pub use stamp::{WatchHandle, find_root, set_choices_if, stamp, watch};
pub use suggestions::{
    MAX_SUGGESTION_BYTES, MIN_SUGGESTIONS, SUGGESTION_SOURCES, SUGGESTION_STATUS, SUGGESTIONS_PATH, Suggestion,
    Suggestions, check_suggestions, load_suggestions, suggestion_license_allowed, suggestion_targets,
};
pub use suggestions_add::{
    ERR_SUGGESTIONS_CHANGED, SizeFunc, SkippedSuggestion, SuggestionAdd, add_suggestions, head_size, plan_suggestions,
};
pub use suggestions_write::drop_suggestion;
pub use target::{MAX_TARGET_UNITS, Units, set_target_if};
pub use vorrat::{
    VORRAT_BLOCKED, VORRAT_DIR, VORRAT_DROPPED, VORRAT_LOADED, VORRAT_OPEN, VorratEntry, VorratIndex, VorratState,
    load_vorrat, vorrat_states,
};
pub use vorrat_adopt::{ERR_VORRAT_PACKAGE, vorrat_adopt, vorrat_adopt_if, vorrat_cleanup, vorrat_cleanup_preview};
pub use vorrat_download::{
    Progress, VORRAT_HOSTS, check_vorrat_download, vorrat_download, vorrat_redirect, vorrat_url_allowed,
};
