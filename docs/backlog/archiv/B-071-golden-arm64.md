# B-071 · Die Golden-Tests laufen auch auf arm64 grün

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** F2
- **Projekt:** –
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint F2

## Ausgangslage

Go darf `x*y + z` zu einer FMA-Anweisung fusionieren (Go-Spezifikation, arithmetische Operatoren); auf arm64 (Raspberry Pi)
tut der Compiler das, auf amd64 nicht. Dann weicht das Ergebnis von JavaScript ab. SP04 legt als Regel fest, Produkte
vor einer Addition mit `float64(…)` zu runden. Die CI testet aber nur auf amd64, ein Verstoß fiele erst auf dem Pi auf.

## Ziel

Eine Abweichung durch FMA fällt vor dem Pi-Betrieb (SP11) in einem Test auf.

## Beteiligte und Zielgruppen

Entwickler oder Agent; Spieler am Pi (sonst andere Level als im Browser).

## Anforderungen

- `go test ./engine/...` läuft mit `GOARCH=arm64` (CI mit arm64-Runner oder Emulation) über alle Golden-Daten.

## Nicht-Ziele

Andere Architekturen als amd64 und arm64.

## Regeln und Einschränkungen

Entscheidung 001; Golden-Daten aus `testdata/golden/` (SP04); keine neue Abhängigkeit ohne Zustimmung von 🧑.

## Beispiele

Ein Ausdruck `a + b*c` ohne `float64(…)` in `engine/level` → der arm64-Lauf meldet Seed und Feld.

## Ausnahme- und Fehlerfälle

Kein arm64-Runner verfügbar → Lauf unter Emulation (qemu), langsamer, aber vollständig.

## Akzeptanzkriterien

- **AC-01** Ein CI-Job führt die Go-Golden-Tests für arm64 aus und ist grün.

## Offene Fragen

Runner: nativer GitHub-arm64-Runner `ubuntu-24.04-arm` (Repo ist öffentlich, dort nach Kenntnisstand kostenlos), Rückfall qemu-Emulation, wenn der Job nicht anspringt (Vorschlag vom 2026-10-03, 🧑 bestätigt mit der Freigabe von F2).

## Notizen

Entstanden bei der Planung von SP04 (2026-09-30). Spätestens vor SP11 umsetzen.
