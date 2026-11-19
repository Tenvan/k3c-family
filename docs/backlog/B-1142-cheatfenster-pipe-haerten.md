# B-1142 · Cheat-Fenster-Pipe härten: Zugriffsrechte, Pipe-Squatting, blockierendes Schreiben

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU56
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Review RU56.4: `win32/pipe.rs` erzeugt die Pipe ohne `SECURITY_ATTRIBUTES` (Standard-DACL: Everyone und Anonymous lesend), der Name `k3c-devwin-<id>` ist vorhersagbar, und `WriteFile` blockiert den einzigen I/O-Thread, wenn ein Client lebt, aber nicht liest.

## Ziel

Welches Ergebnis, welcher Nutzen (oder welcher Schaden verschwindet)? Kein Umsetzungsschritt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **AC-01** Pipe nur für den Besitzer (SDDL `D:P(A;;GA;;;OW)` über `SECURITY_ATTRIBUTES`, Feature `Win32_Security` ist gepinnt).
- **AC-02** Das Spiel prüft als Client per `GetNamedPipeServerProcessId`, dass der Server das eigene gestartete Fenster ist (Pipe-Squatting); der Pipe-Name enthält eine Zufallskomponente.
- **AC-03** Schreiben mit Zeitgrenze (overlapped I/O oder Verbindung nach Frist kappen), kein Deadlock bei vollem Puffer.

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

Aus dem Review von RU56.4; Risiko gering (lokal, `PIPE_REJECT_REMOTE_CLIENTS`). Umschalter-Zustand geht beim Reiterwechsel verloren, Tab-Taste springt nicht zwischen Controls: ebenfalls hier oder in B-1134 prüfen.
