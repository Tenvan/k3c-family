# RU67 · DEV · assetkit nach Rust: Asset-Logik des Studios

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-25

## Ausgangslage

Entwurf. Abhängig von RU65. `tools/assetkit` (rund 4.000 Zeilen Go, 2.000 Zeilen Tests) enthält die Asset-Logik des Studios; Papierschnitt-Renderer (`tools/paper`) ist TypeScript und bleibt.

## Ziel

Die Asset-Logik liegt als Rust-Crate mit portierten Tests; die Dateien unter `public/grafik` und die Golden bleiben unverändert (Port ohne Verhaltensänderung).

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft.

## Anforderungen

Port von `tools/assetkit` (Asset-Logik des Studios: Spezifikationen, Katalog, Quellen, Prüfungen) nach einem Rust-Crate, Verhalten wie Go, Tests mitportiert. Der Papierschnitt-Renderer `tools/paper` (TypeScript) und die Dateien unter `public/grafik` bleiben unverändert.

## Nicht-Ziele

Studio-Backend und MCP-Tools `assets_*` (RU70). Entfernen des Go-Codes (RU71).

## Regeln und Einschränkungen

Port ohne Verhaltensänderung; Go ist das Orakel. Datei ≤ 400, Funktion ≤ 60 Zeilen. Neue Crates nur nach Annahme von Entscheidung 014.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/crates/k3c-assetkit/**`, `rust/Cargo.toml`, `rust/Cargo.lock`
- **Abhängig von:** RU65, RU66
- **Braucht 🧑:** –
- **Sperre:** deps

## Beispiele

Dieselbe Asset-Spec ergibt in Rust denselben Katalogeintrag und dieselben Prüfmeldungen wie in Go.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go (Abweichung beheben oder Ticket).

## Akzeptanzkriterien

- **AC-01** `assetkit` liegt als Rust-Crate vor, die portierten Tests sind grün (`cargo test`, `task rust:lint`).
- **AC-02** Kein Go-Code geändert; `task check:diff` grün.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Stichpunkte (Reife Entwurf): 1. Datenmodell und Spezifikationen (AC-01). 2. Katalog, Quellen, Prüfungen (AC-01). 3. Review (AC-02).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
