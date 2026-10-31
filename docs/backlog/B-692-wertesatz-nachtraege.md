# B-692 · Welt-Wertesatz: Nachträge aus dem Review BAL8.2

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** BAL
- **Erstellt:** 2026-10-31
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Review BAL8.2 (2026-10-31), nicht behobene Punkte mittlerer Schwere:
1. `skillCatalog`/`skillIndex` (`engine/sim/monarch_data.go:63`) bleiben auf der Basis; Overrides von `monarch.skills.*` wirken in der Sim nicht.
2. Spielstand-Prüfung nutzt die Basis: `validateLearned` (`island_save_v3.go:94`, TierPoints) und `validateEconomy` (`save_economy.go:135`, Rüstungsstufen). Eine Welt mit eigenen `tierPoints` oder kürzerer `armor`-Liste kann gültige Stände ablehnen oder ungültige annehmen.
3. `level.RuinHP` (`engine/level/ruins.go:88`) und der Level-Generator (`level/lines.go:28`, `level/sites.go:47`, `sim/world_lines.go:73,84`) lesen die eingebetteten Dateien; Overrides auf `economy.payRangeUnits`, `hub.sitePlacement` und Gebäude-HP einer Ruine wirken dort nicht. Entweder in `docs/rules/welten.md` als nicht übersteuerbar festhalten (und im Merge sperren) oder durchreichen.
4. `UseData` (`data.go` ~181) lädt in die bestehenden Basis-Maps hinein (`b := buildings` kopiert nur den Header); `defaultValues()` hält jetzt Zeiger darauf. Maps vor dem Laden klonen. `goals.json` lädt `UseData` nicht.
5. `k3c-server -check-data` (`cmd/k3c-server/restart.go`) und `tools/k3c-workbench/internal/balance/island_world.go` lösen `overrides` und `extends` nicht auf.
6. `World.values()` ohne `vals` allokiert pro Aufruf (nur Test-Gerüst).

## Ziel

Die sechs Punkte sind je behoben oder bewusst dokumentiert.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Je Punkt: Test oder Eintrag in `docs/rules/welten.md` als Ausnahme.
- **AC-02** `task check:diff` grün.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
