# B-328 · Der Feuergeist hinterlässt eine Flammen-Fläche

- **Domäne:** SIM
- **Typ:** Frage
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** KMP
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`docs/rules/gegner.md` § 1 nennt für den Feuergeist „Fernkampf mit Flammen-Fläche“. K1.3 hat ihn laut Session als `ranged` + `kiting` angelegt (`data/enemies.json`); eine Flammen-Fläche gibt es in der Simulation nicht, und § 2 kennt dafür kein Trait.

## Ziel

Der Feuergeist hat die Fähigkeit, die das Regelwerk ihm gibt, oder das Regelwerk streicht sie.

## Beteiligte und Zielgruppen

Spieler, Balancing (REG); 🧑 entscheidet.

## Anforderungen

- Wirkung festlegen: Radius, Dauer, Schaden je Sekunde der Fläche (Werte in `data/enemies.json`), deterministisch, 2+ Spieler.

## Nicht-Ziele

Anzeige der Fläche (K5), Bosse (K2).

## Regeln und Einschränkungen

Werte nur in `data/`, Logik in `engine/sim/`; „der Elite hat genau eine Fähigkeit“ (`gegner.md` § 2).

## Beispiele

Geschoss trifft → am Ziel brennt 3 s lang eine Fläche mit Radius 2, 5 Schaden je Sekunde (Werte vermutet).

## Ausnahme- und Fehlerfälle

Fläche hinter einer Mauer → offen.

## Akzeptanzkriterien

- **AC-01** 🧑 hat entschieden; bei Umsetzung prüft ein Test die Fläche laut Daten.

## Offene Fragen

Flammen-Fläche umsetzen (welche Werte) oder aus § 1 streichen? 🧑

## Notizen

–
