# B-644 · sim_test wartet wie check_run auf das Ende und meldet Fortschritt je Tag

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** BAS
- **Erstellt:** 2026-10-26
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`sim_test start` kehrt sofort zurück, der Stand kommt nur über `status` (≤ 10 Zeilen), Fortschritt zählt nur fertige Läufe (`PROGRESS n/m`, `simrun/offline.go`). Ein Insel-Lauf (40 Tage, ein Seed) meldet bis zum Ende nichts. `check_run` dagegen wartet im Aufruf, meldet Fortschritt und liefert nach höchstens 4 min eine Lauf-ID (`run=<ID>` wartet weiter).

## Ziel

Ein Testlauf lässt sich wie `check_run` in einem Aufruf bis zum Ende begleiten, mit Fortschritt auch innerhalb eines langen Laufs.

## Beteiligte und Zielgruppen

Claude und Ralf (Meilenstein-Läufe BAS).

## Anforderungen

- `sim_test start` mit `wait: true` und `sim_test {action: wait, id}` warten wie `check_run` auf das Ende; mit progressToken alle 20 s Lebenszeichen bis zum Ende, ohne nach 4 min die ID.
- Ohne `wait` bleibt das heutige Verhalten. Fortschritt je Tag ist nicht nötig (linearer Prozess, Lebenszeichen genügt).

## Nicht-Ziele

Online-Läufe (Spielserver) umbauen.

## Regeln und Einschränkungen

Workbench-Code (DEV); Neustart der Workbench melden.

## Beispiele

`sim_test {action: start, bots: coop2, world: standard, days: 40, wait: true}` → nach 4 min „läuft, Tag 17/40, ID run-1“.

## Ausnahme- und Fehlerfälle

Abbruch des Aufrufs: der Lauf läuft weiter (wie heute).

## Akzeptanzkriterien

- **AC-01** Test: Wartemodus meldet Lebenszeichen, liefert den Endstatus bzw. beim Zeitlimit die ID (`simtest_wait_test.go`).

## Offene Fragen

keine (Default: Name `wait`, Limit wie `check_run`).

## Notizen

Auslöser: Frage von Ralf nach dem Referenzlauf BS11.4. Umgesetzt nach seinem Hinweis „so wie der check_run“; `task check:diff` bis zur Klärung pausiert, nur gezielte `go test`. Workbench-Neustart nötig.
