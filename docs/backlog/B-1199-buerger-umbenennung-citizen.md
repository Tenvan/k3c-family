# B-1199 · Landstreicher und Bauer werden zum Bürger (citizen)

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WK1
- **Projekt:** NAC
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Heute gibt es `vagrant` (Landstreicher) und `peasant` (Bauer). Entscheidung Ralf (2026-11-21): beide werden ein Typ, der Bürger (`citizen`); alte Spielstände egal. Gehört zum Sprint FG1, ECO1 (B-1193/B-1194) nutzt den Bürger mit Werkzeuggürtel.

## Ziel

Nur noch `citizen` als Grundfigur; Berufe entstehen aus dem Bürger durch Ausrüstung.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **AC-01** `vagrant` und `peasant` sind in k3c-sim, k3c-data, k3c-game, Texten und Tests zu `citizen` zusammengelegt; Go-Parität-Golden mit `native_rules` aus bleiben grün.
- **AC-02** Texte heißen „Bürger“ (de) und „Citizen“ (en).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), nur Rust (Entscheidung 013), Go-Golden nur mit `native_rules` aus.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-sim/**`, `rust/crates/k3c-data/**`, `rust/crates/k3c-game/**`, `data/troops.json`, `data/economy.json`
- **Abhängig von:** B-1194
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** golden | spielstand

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
