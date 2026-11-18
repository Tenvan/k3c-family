# B-1085 · Welt-Datei lesen: null- und Array-Randfälle weichen von Go ab

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU10
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Beim strengen Lesen der Welt-Datei (`island/worlds.rs`, `worlds_decode.rs`) weichen Randfälle von Go ab (Review RU10.5): (1) `"skins": null` wird angenommen, Go lehnt ab (`WorldFile.skins` ist ein `Value`, fehlt und null sind nicht unterscheidbar; `k3c-data` liegt außerhalb des Bereichs); (2) `T::deserialize(&Value)` nimmt Structs auch aus JSON-Arrays an (Insel als Array, `depthScaling: [1,1,1]`, `nature: []`), Go meldet `kein gültiges JSON`; (3) `natureByDepth: {"0": null}` lehnt Rust ab, Go nimmt leere Werte; eine Insel `null` meldet Rust als JSON-Fehler statt `id fehlt`.

## Ziel

Die Randfälle verhalten sich wie in Go.

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

- **AC-01** Die genannten Fälle verhalten sich wie in Go (je ein Test in `worlds_tests.rs`), Reihenfolge und Text der übrigen Meldungen bleiben.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
