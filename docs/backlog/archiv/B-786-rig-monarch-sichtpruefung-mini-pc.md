# B-786 · Rig-Monarch wird am Mini-PC geprüft und gemessen, sobald das Spiel am PC fertig ist

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** verworfen
- **Sprint:** NG9
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

## Ausgangslage

NV2.4 (Sichtprüfung und Messung des Rig-Monarchen am Mini-PC, B-778/AC-06) ist zurückgestellt: Der Mini-PC wird erst organisiert, wenn das Spiel am PC weit genug fertig ist (Ralf, 2026-11-11). Das Paket entsteht mit `task native:rig:paket` (`native/k3c-rig`, Messskript `native/measure.ps1`, `messen.cmd`).

## Ziel

Ein Rig-Monarch am Mini-PC ist mit dem Auge geprüft und gemessen; die Entscheidung, ob der Monarch als Rig ins native Spiel geht, ist notiert.

## Beteiligte und Zielgruppen

🧑 Ralf (Mini-PC beschaffen, Sichtprüfung, Messung, Entscheidung). Claude baut das Paket.

## Anforderungen

- Sichtprüfung: Schatten, Schärfe, Umhang, Sitz auf dem Pferd, getauschte Waffe und Rüstung, zwei Gamepads.
- Messung mit 2, 50 und 200 Reitern als Rig und als Sheet (`messen.cmd -Engine rig`), Bericht zurück ins Repo.

## Nicht-Ziele

Code ändern; Befunde werden eigene Tickets.

## Regeln und Einschränkungen

Entscheidung 008 (Go + Ebitengine). Wiederaufnahme erst, wenn 🧑 den Mini-PC bereitstellt.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `docs/backlog/B-778-rig-prototyp-monarch-aus-teilen.md`
- **Abhängig von:** –
- **Braucht 🧑:** Gerät
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant: Gerätetermin ohne Eingabefälle.

## Ausnahme- und Fehlerfälle

nicht relevant: Gerätetermin ohne Eingabefälle.

## Akzeptanzkriterien

- **AC-01** Sichtbefund und Messwerte vom Mini-PC stehen in B-778 › Notizen, die Entscheidung von 🧑 ist notiert (ersetzt B-778/AC-06).

## Offene Fragen

Welcher Mini-PC (CPU, GPU, Windows), welche Auflösung (TV oder Monitor), welche Gamepads, wie kommt das Paket hin und der Bericht zurück? Entscheidet 🧑, sobald der Mini-PC feststeht.

## Notizen

Verworfen 2026-11-13 (Dialog Ralf „NG9.5 entfällt“): Keine Sichtprüfung und Messung am Mini-PC; NG9.5 verworfen.
