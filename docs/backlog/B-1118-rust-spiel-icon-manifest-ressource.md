# B-1118 · Rust-EXE trägt Spiel-Icon und Manifest (Ressourcen-Crate braucht Freigabe)

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU30
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Go bettet `native/k3c-game/winres/icon.png` und das Manifest (`winres.json`: Name `K3C.Game`, DPI-Bewusstsein per monitor v2, long-path-aware) als Windows-Ressource ein. Die Rust-EXE (`k3c-game`) hat beides nicht. Ein Ressourcen-Crate ist eine neue Abhängigkeit und braucht Freigabe von Ralf (RU30.6, Haltepunkt). Das Titelleisten-Icon des Fensters setzt Rust nicht (kein API in `k3c-platform`).

## Ziel

Die Rust-EXE trägt Icon und Manifest wie die Go-EXE. Vorschlag: `winresource` nur unter `cfg(windows)` als `build-dependency` mit fester Version; `rust/crates/k3c-game/build.rs`; Icon `assets/icon.ico` aus `native/k3c-game/winres/icon.png` (Konvertierung dokumentieren); Manifest mit denselben Angaben wie `winres.json`.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Ralf gibt `winresource` (oder ein anderes Ressourcen-Crate) mit fester Version frei.
- **AC-02** `build.rs` bettet `assets/icon.ico` und das Manifest (`K3C.Game`, per monitor v2, long-path-aware) nur unter `cfg(windows)` ein; Builds auf anderen Plattformen bleiben unberührt.
- **AC-03** Die gebaute `k3c-game.exe` zeigt das Spiel-Icon im Explorer; das Manifest ist in der EXE lesbar.
- **AC-04** Das Titelleisten-Icon braucht ein API in `k3c-platform`; als eigenes Ticket der Fenster-API entscheiden.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
