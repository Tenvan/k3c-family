# B-000 · Kurzer Titel als Aussage

- **Domäne:** REG | SIM | SRV | CLI | PLAT | INF | DEV | PAP
- **Typ:** Idee | Problem | Schuld | Frage
- **Prio:** hoch | mittel | niedrig | ?
- **Umgebung:** offline | live | ? (offline: ohne laufende Dienste prüfbar – Code, Unit-/Mock-Tests, Werkzeuge ohne Serverzugriff, worktree-tauglich; live: braucht laufenden Server, Browser oder Gerät)
- **Status:** offen | eingeplant | erledigt | verworfen
- **Sprint:** – (oder SP01, R1 …)
- **Projekt:** – (oder Kürzel eines Projekts, z. B. GRA; gilt auch ohne Sprint)
- **Erstellt:** JJJJ-MM-TT
- **Spec:** Entwurf | freigegeben | rückwirkend
- **Revision:** 1
- **Freigabe:** – (bei `freigegeben`: Datum und Quelle, z. B. „2026-10-02 🧑 PR #20“)
- **Duplikat von:** – (optional: genau eine ID B-000; `plan_set` setzt dann Status verworfen)
- **Ersetzt:** – (optional: IDs B-000, B-001 der Tickets, die dieses ersetzt)

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

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
