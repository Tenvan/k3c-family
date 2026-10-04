# B-220 · Ein neuer Schild ersetzt den laufenden nur, wenn er größer ist

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** niedrig
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Iron Wall (500 HP) und Divine Shield (100 HP) nutzen denselben Wirkungstyp `shield` (`castShield` in `engine/sim/skills_tank.go`). Er setzt `Player.Shield` und `ShieldFor` immer neu. S1.2b sah vor, dass ein neuer Schild den alten nur ersetzt, wenn er größer ist; `skills_tank.go` lag nicht in den erlaubten Dateien von S1.2b. Wer beide Linien lernt, kann heute einen laufenden 500er-Schild mit Divine Shield auf 100 senken.

## Ziel

Ein zweiter Schild-Skill verschlechtert einen laufenden Schild nicht.

## Beteiligte und Zielgruppen

Spieler mit Punkten in Tank- und Heiler-Linie; Entscheidung über die Regel 🧑.

## Anforderungen

- `castShield` übernimmt den neuen Schild nur, wenn seine HP größer sind als der Rest des laufenden Schilds (sonst bleibt der alte, die Abklingzeit startet trotzdem).
- Deterministisch, je Spieler.

## Nicht-Ziele

Schild auf andere Spieler wirken (Divine Shield bleibt beim Wirkenden).

## Regeln und Einschränkungen

`docs/rules/monarch.md` § 3; Werte nur in `data/monarch.json`; Datei ≤ 400, Funktion ≤ 60.

## Beispiele

Iron Wall läuft mit 400 Rest-HP, Divine Shield gewirkt → Schild bleibt 400. Schild leer oder 50 Rest-HP → Divine Shield setzt 100.

## Ausnahme- und Fehlerfälle

Gleich große Schilde → offen, ob die Dauer erneuert wird (🧑).

## Akzeptanzkriterien

- **AC-01** Go-Test: laufender Schild 400, Divine Shield → `Shield` bleibt 400; bei 50 → 100.

## Offene Fragen

Gilt die Regel auch für Dauer (längerer, aber kleinerer Schild)? 🧑

## Notizen

Gefunden in S1.2b (Sprint S1).
