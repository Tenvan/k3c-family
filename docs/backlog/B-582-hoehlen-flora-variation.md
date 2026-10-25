# B-582 · Höhlen-Wurzeln sind zahlreicher und unterscheiden sich in Form und Größe

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** GEB15
- **Projekt:** GEB
- **Erstellt:** 2026-10-24
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-25

## Ausgangslage

Abnahme GEB10.5 (Ralf, 2026-10-24): Die Wurzeln in der Höhle hängen, es sind aber zu wenige und alle gleich.

## Ziel

Die Höhle wirkt abwechslungsreich: mehr Wurzeln, die sich in Form und Größe unterscheiden.

## Beteiligte und Zielgruppen

Spielende; Agent DEV (Asset-Studio, Daten); 🧑 nimmt ab.

## Anforderungen

- Mehr Wurzeln je Höhlenabschnitt (Dichte als Wert in den Daten).
- Mindestens drei Wurzel-Varianten in Form und Größe, zufällig über den deterministischen Seed gewählt.

## Nicht-Ziele

Neue Höhlen-Objekte außer Wurzeln; Mine und Felsen (B-583).

## Regeln und Einschränkungen

Asset-Weg über das Studio; Generator deterministisch (`engine/rng`); Golden mit Begründung neu schreiben; Werte in `data/`.

## Beispiele

Höhle → Wurzeln in unterschiedlicher Länge und Form, dichter als bisher.

## Ausnahme- und Fehlerfälle

nicht relevant, nur Darstellung und Platzierung.

## Akzeptanzkriterien

- **AC-01** Mindestens drei Wurzel-Varianten stehen im Katalog und in der Studio-Vorschau.
- **AC-02** Die Höhle zeigt mehr Wurzeln, die sich sichtbar unterscheiden (Beobachtung am Gerät).

## Offene Fragen

Konkrete Dichte und Anzahl der Varianten: Default 3 Varianten, Dichte etwa verdoppelt; 🧑 entscheidet bei der Abnahme.

## Notizen

Aus GEB10.5. Spec: Entwurf. Gemeinsamer Sprint mit B-583.
