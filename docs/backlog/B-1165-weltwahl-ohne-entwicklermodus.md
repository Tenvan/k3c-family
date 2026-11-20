# B-1165 · Die Weltwahl zeigt alle Welten aus data/worlds, nicht nur die Standard-Welt

- **Domäne:** CLI
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

Ralf (2026-11-20): Aus den bestehenden Welten lässt sich nicht auswählen, auch im Entwicklermodus ist Rom nicht wählbar. Getestet wird immer aus dem dist-Ordner. Vorgabe: Immer, aber definitiv im Dev-Mode, sind alle Welten auswählbar, egal wie viel dort schon umgesetzt ist.

Stand im Quelltext (nicht im Spiel geprüft): `dist/k3c-game/data/worlds/` enthält `rom.json` und `standard.json`. `Manager::world_list` (`k3c-room/src/room/worlds.rs`) nimmt im Dev-Mode jede Welt-Datei, lässt aber jede Datei weg, die sich nicht laden lässt (kein Fehler, kein Log). Verdacht: `rom.json` ist für den Loader ungültig und fällt still heraus; zweiter Verdacht: Der Ordner `data/worlds` wird relativ zum Arbeitsordner gesucht (`worlds_fs`) und beim Start der Session festgelegt. Zu tun: Ursache im dist bestätigen, Welten neben der EXE suchen, im Dev-Mode jede Datei in der Liste zeigen (auch eine nicht ladbare, mit Grund), ohne Dev-Mode alle fertigen Welten. Sengoku, Nil und Britannien gibt es als Datei noch nicht (WE3, B-960).

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
