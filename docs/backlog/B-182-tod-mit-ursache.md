# B-182 · Das Ereignis playerDown nennt, was den Monarchen getötet hat

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** hoch
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`applyDamage(w, targetID, damage)` (`engine/sim/common.go`) kennt den Angreifer nicht; das Ereignis `playerDown` hat nur `player` (Index). Auch `hit` aus F3.1 nennt nur Ziel, Ort und Schaden. Der Spielmetrik-Report (B-150, Sprint S2, Session S2.3) braucht je Tod die Gegnerart („Tod durch was“, Beschluss Q12); der Raum (SRV) kann sie aus den Ereignissen nicht ablesen. Aufgefallen beim Bereitmachen von S2 (2026-10-03).

## Ziel

Jeder Tod eines Monarchen nennt im Ereignis die Ursache, damit Report und spätere Auswertung (B-160) „Tod durch was“ ohne Raten zählen.

## Beteiligte und Zielgruppen

Entwickler (SIM, SRV), 🧑 als Auswerter der Spieleabende (B-151).

## Anforderungen

- `playerDown` trägt zusätzlich die Ursache: die Gegnerart (`kind` aus `data/enemies.json`, z. B. `goblinArcher`) bei Nahkampf und Geschoss; ein anderer Wert für Ursachen ohne Gegner (z. B. Skill-Schaden, falls es den gibt).
- Deterministisch, ändert keinen Spielwert; nur das Ereignis bekommt ein Feld.
- 2+ Spieler: die Ursache gilt je Monarch.

## Nicht-Ziele

Report schreiben (B-150), Anzeige im Client, Ursache bei Truppen und Gebäuden.

## Regeln und Einschränkungen

Domäne SIM; Golden-Läufe ändern sich nur im Ereignis (Begründung im Commit-Text, Q09); Datei ≤ 400 Zeilen, Funktion ≤ 60; keine Iteration über Maps.

## Beispiele

Ein Pfeil eines `goblinArcher` tötet Monarch 1 → `{"type":"playerDown","player":1,"by":"goblinArcher"}`.

## Ausnahme- und Fehlerfälle

Mehrere Treffer im selben Tick → die Ursache ist der Treffer, der die HP auf 0 bringt. Tod ohne Gegner → fester Wert statt leer.

## Akzeptanzkriterien

- **AC-01** Test in `engine/sim`: Tod durch Nahkampf und durch Geschoss nennt die Gegnerart im Ereignis `playerDown`, auch für den Spieler mit Index 1.
- **AC-02** Golden-Läufe ändern sich nur im Feld des Ereignisses, `task check:go` ist grün.

## Offene Fragen

Feldname (`by` ist ein Vorschlag) und ob `hit` dasselbe Feld bekommt; Einplanung (vor S2.3): 🧑.

## Notizen

Anlass: S2.3 (Spielmetrik-Report) hängt davon ab, siehe `docs/sprints/geplant/S2-protokoll-skills-speichern-metrik/S2.3-spielmetrik-report.md`.
