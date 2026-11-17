# B-1060 · Der Rust-Client spielt Effekte aus Ausschnitten des Ton-Atlas

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU30
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Go spielt Ton-Effekte als Ausschnitte eines dekodierten Atlas. Im Rust-Client nimmt `k3c_platform::AudioOut::load` nur ganze Dateien; `decode`/`Pcm` sind nicht öffentlich. RU17.5 hat Bank, Ausgabe und Mix portiert und spielt Effekte aus Einzeldateien (`public/audio/kandidaten/`); `load_atlas` und der Test `load_bank_atlas_clips_match_sprites` stehen auf `ignore`.

## Ziel

Ralf entscheidet den Weg; danach spielt der Client alle Effekte wie Go, ohne `ignore` im Paket `audio`.

## Beteiligte und Zielgruppen

🧑 entscheidet, Agenten setzen um.

## Anforderungen

Option (a, Vorschlag): `k3c-platform` bekommt `load_range(bytes, start_secs, dur_secs)`; in `audio/bank.rs` ändert sich nur `load_atlas`, der ignorierte Test wird eingeschaltet.
Option (b): Einzeldateien bleiben dauerhaft; `load_atlas` und der ignorierte Test entfallen, der Sound-Atlas wird für das Rust-Spiel nicht mehr gebraucht.

## Nicht-Ziele

Neue Ton-Inhalte, Mix-Änderungen.

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-platform/src/**`, `rust/crates/k3c-game/src/audio/**`
- **Abhängig von:** RU17
- **Braucht 🧑:** Entscheidung
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant

## Ausnahme- und Fehlerfälle

Fehlt eine Effekt-Datei, bleibt der Slot stumm wie in Go.

## Akzeptanzkriterien

- **AC-01** `grep -rn 'ignore' rust/crates/k3c-game/src/audio` findet nichts; `cargo test -p k3c-game audio` grün.

## Offene Fragen

- Weg (a) oder (b)? Haltepunkt aus RU17.5 (2026-11-18).

## Notizen

Aus RU17.5 (Commit 3d468c8c8). Eingeplant in RU30, damit RU17 ohne Warten auf die Entscheidung abschließen kann.
