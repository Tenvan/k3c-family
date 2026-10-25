# B-583 · Minen und Felsen sind größer und haben zwei bis drei Varianten mehr

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

Abnahme GEB10.5 (Ralf, 2026-10-24), Punkt 7 Geschmack: Minen und Felsen könnten größer sein und zwei bis drei Variationen mehr haben.

## Ziel

Minen-Fels und Felsen sind größer und in mehreren Varianten vertreten.

## Beteiligte und Zielgruppen

Spielende; Agent DEV; 🧑 nimmt ab.

## Anforderungen

- Mine-Fels und Fels größer als heute (Maß als Wert in den Daten).
- Zwei bis drei zusätzliche Varianten je, deterministisch über den Seed gewählt.

## Nicht-Ziele

Wurzeln in der Höhle (B-582); neue Rohstoffe.

## Regeln und Einschränkungen

Asset-Weg über das Studio; deterministisch (`engine/rng`); Golden mit Begründung neu schreiben; Mindest-Schrift und Hitbox unverändert, nur Darstellung.

## Beispiele

Mine mit mehreren Felsen → verschiedene Formen, größer als bisher.

## Ausnahme- und Fehlerfälle

nicht relevant, nur Darstellung.

## Akzeptanzkriterien

- **AC-01** Fels und Mine-Fels haben je mindestens zwei zusätzliche Varianten im Katalog und in der Vorschau.
- **AC-02** Die Objekte sind sichtbar größer und abwechslungsreich (Beobachtung am Gerät).

## Offene Fragen

Wie viel größer? Default etwa 1,5-fach; 🧑 entscheidet bei der Abnahme.

## Notizen

Aus GEB10.5. Spec: Entwurf. Gemeinsamer Sprint mit B-582.
