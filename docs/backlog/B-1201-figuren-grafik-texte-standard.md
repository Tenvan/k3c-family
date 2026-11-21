# B-1201 · Grafik und Texte der Grundfiguren und Meister-Gebäude im Standard

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** FG1
- **Projekt:** WIR
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Zu den Kombinationen aus B-1200 und ECO1 gehören Figuren (Bürger, Handwerker, Bogenschütze, Soldat, Elite-Soldat, Elite-Bogenschütze), Gebäude (Handwerksmeister, Bogner, Waffenschmied, Rüstungsschmied) und Gegenstände (Werkzeuggürtel, Bogen, Schwert, Rüstung). Assets nur über MCP `assets_*` (Entscheidung 005).

## Ziel

Alle genannten Figuren, Gebäude und Gegenstände haben im Standard Grafik und Texte (de, en).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **AC-01** Papierschnitt-Assets liegen vor und sind in data/paper.json zugeordnet.
- **AC-02** Texte de und en für Namen, Hinweise und Regal.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Assets nur über MCP `assets_*` (Entscheidung 005); Texte de und en.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `public/grafik/**`, `data/paper.json`, `rust/crates/k3c-game/src/actors/**`, `rust/crates/k3c-game/src/hud/**`
- **Abhängig von:** B-1199
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
