# B-1021 · Abnahme RU1: Rust-Workspace, check:race und CI-Lauf prüfen

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** BAL11
- **Projekt:** BAL
- **Erstellt:** 2026-11-16
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU1 liefert den leeren Rust-Workspace samt Tasks, check:diff-Bahn und CI-Job. Lokal waren nur Build, Test, clippy und fmt dran; Race-Detector, der echte CI-Lauf und Sim/Balance/Gen gehören nach Regel B-742 in dieses Abnahme-Ticket.

## Ziel

Nach dem Sprint-PR von RU1 sind die nicht lokal prüfbaren Punkte abgenommen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Läufe nur als Abnahme des Sprints RU1, keine Sim- oder Balance-Läufe (Sperre 2026-11-11).

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

- **AC-01** Sim: verschoben (Sperre 2026-11-11), RU1 enthält keine Sim.
- **AC-02** Balance: verschoben (Sperre 2026-11-11).
- **AC-03** Gen (`gen:paper`, Golden): nicht betroffen, RU1 ändert keine Grafik.
- **AC-04** `task check:race` ist grün (lokal ohne gcc nur übersprungen, dann so vermerken).
- **AC-05** Der CI-Job „Rust · fmt · clippy · Tests (Windows)“ ist nach dem Merge auf `main` grün; Go-Jobs unverändert grün. Offen: `rustup show` auf dem Runner und die Laufzeit ohne Cache.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Lane `zweit`: Der Second Worker fährt dieses Ticket (Sim, Gen und Race sind auf dem Hauptrechner gesperrt).
