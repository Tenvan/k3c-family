# B-622 · Fackeln gibt es als Spielobjekt mit Licht in der Nacht

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WRT
- **Erstellt:** 2026-10-25
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

W13.1 (2026-10-25) zeichnet Licht für Lagerfeuer und Fackeln nach `data/light.json`. Fackeln gibt es aber nicht als Spielobjekt, nur das Lagerfeuer (Hub vor dem Ausbau) leuchtet.

## Ziel

Fackeln stehen in der Welt (z. B. an Mauern und Türmen) und leuchten nachts.

## Beteiligte und Zielgruppen

Spielende; Agent SIM/CLI; 🧑 entscheidet, wo Fackeln stehen.

## Anforderungen

- Fackeln als Objekt (Level oder Gebäude-Zubehör), im Zustand gemeldet; der Client zeichnet sie mit dem vorhandenen Licht (`lightView.ts`).

## Nicht-Ziele

Neue Lichtberechnung.

## Regeln und Einschränkungen

Deterministisch, Werte in `data/`.

## Beispiele

Gebauter Turm → zwei Fackeln, nachts heller Kreis.

## Ausnahme- und Fehlerfälle

nicht relevant

## Akzeptanzkriterien

- **AC-01** Fackeln stehen in der Welt und leuchten nachts (Test der Platzierung, Abnahme am Gerät).

## Offene Fragen

Wo stehen Fackeln (Mauer, Turm, Burg)? Entscheidet 🧑.

## Notizen

Gefunden in W13.1.
