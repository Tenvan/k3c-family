# B-1261 · Flaschenpost: Zielpool und Markierung über alle Stufen

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** GS4
- **Projekt:** WZG
- **Erstellt:** 2026-11-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Was ist heute so? Problem oder Idee in 1–5 Sätzen, mit Dateipfaden, wenn es um Code geht.

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

Umgesetzt in der Sim (Rust, `rust/crates/k3c-sim/src/bottle/`):
- Zielpool über alle Stufen: `bottle::pool` liest jede noch vorhandene Truhe (`pickups` mit Art `chest`) aller Stufen. Eine Insel mit einem Spieler kann so Flaschen bekommen (vorher nur Doppel-Truhen mit zwei Monarchen).
- Markierung trägt Stufe und Ort (`BottleTarget`); Spielstand Version 14 bekommt das Feld `targetStage` (fehlt bei 0).
- Markierung bleibt bestehen, solange die Truhe da ist; sie endet mit `bottleTargetReached` entweder bei Erreichen (`player` = Index) oder bei verlorener Truhe (`player` -1, `gone` true).
- Bei Gleichstand gewinnt der höchste Spieler-Index (R4).
- Aufheben, Abend und Rhythmus unverändert; der Pool wird vor dem Stufentakt gesetzt (`bottle::prepare`), der Takt läuft auf Inselebene (`bottle::step_bottle`).
- Tests: 16 Flaschenpost-Tests grün, darunter Zielpool auf anderer Stufe, Einzelspieler, Markierung ohne Monarch, Ziel auf anderer Stufe erreicht, verlorenes Ziel mit Ereignis.
- Vollsuite k3c-sim: 1193 grün, 4 vorab bekannte Fehler (eingebettete Welten, Standardwelt, Takt 1000 Schritte, Geschichte), unverändert.
- Nicht geprüft: task check:diff (Lint steht auf main rot), Golden, GS4.3/GS4.7 (Darstellung).
