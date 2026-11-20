# B-1159 · Truhen geben nur halb so viel Gold wie nach B-953 (statt zehnmal fünfmal)

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU36
- **Projekt:** RSU
- **Erstellt:** 2026-11-20
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU36.2 (🧑 2026-11-20, G-5): Die Truhen mit Fontaene passen, geben aber zu viel Gold: noch einmal auf die Hälfte reduzieren (zehnmal wird fünfmal). Wert in `data/`; Balancing und Sim-Läufe sind gesperrt, daher nur den Wert ändern.

## Ziel

Welches Ergebnis, welcher Nutzen (oder welcher Schaden verschwindet)? Kein Umsetzungsschritt.

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

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

RU36.3 (Versuch): Wert `chestGold` in `data/economy.json` von [30, 60] auf [15, 30] gesetzt, danach rot: `campaign::golden_kampagne`, `golden_kampagne_ausbau`, `golden_durchsatz`-Vergleich (Golden aus Go), `coop::chest_tests` (feste 30..=60 und Faktorwerte) und `unit::hammer_tests::truhe_wirft_fontaene`. Die Änderung ist zurückgenommen. Sie braucht neue Golden (`task go:golden` bzw. Rust-Golden nach RU41, Sperre golden) und angepasste Tests: eigene Session mit Freigabe 🧑 für die Golden-Änderung.
