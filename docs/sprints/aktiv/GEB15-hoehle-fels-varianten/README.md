# GEB15 · DEV · Höhlen-Wurzeln, Felsen und Minen: mehr Varianten, größer

- **Status:** aktiv
- **Projekt:** GEB
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-582, B-583
- **Start-Commit:** 819c53d3
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-25, Revision 1 (Ziel von Ralf: Specs genehmigt)

## Ausgangslage

Aus der Abnahme GEB10.5 (Ralf, 2026-10-24): Höhlen-Wurzeln sind zu wenige und alle gleich (B-582); Minen und Felsen sollen größer sein und zwei bis drei Varianten mehr haben (B-583).

## Ziel

Höhle, Mine und Felsen wirken abwechslungsreich. Am Ende sichtbar: Studio-Vorschau und Spiel mit mehreren Wurzel- und Fels-Varianten.

## Beteiligte und Zielgruppen

Spielende; Agent DEV; 🧑 entscheidet die offenen Werte und nimmt ab.

## Anforderungen

B-582 › Anforderungen, B-583 › Anforderungen.

## Nicht-Ziele

Neue Objekt-Arten, Balancing.

## Regeln und Einschränkungen

Asset-Weg über das Studio; Generator deterministisch; Golden mit Begründung; Werte in `data/`.

## Beispiele

Höhle → verschieden lange Wurzeln, dichter. Mine → größere Felsen in mehreren Formen.

## Ausnahme- und Fehlerfälle

nicht relevant, nur Darstellung.

## Akzeptanzkriterien

- **AC-01** Mindestens drei Wurzel-Varianten, Höhle dichter (B-582/AC-01, B-582/AC-02).
- **AC-02** Fels und Mine-Fels größer, je mindestens zwei neue Varianten (B-583/AC-01, B-583/AC-02).
- **AC-03** Ralf hat AC-01 und AC-02 am Gerät abgenommen.

## Offene Fragen

Dichte, Größenfaktor und Variantenzahl: Defaults in B-582 und B-583; 🧑 bestätigt bei der Abnahme. Spec-Freigabe steht aus.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GEB15.1 | `GEB15.1-hoehlen-wurzeln.md` | Umsetzung | autonom | fertig |
| GEB15.2 | `GEB15.2-felsen-minen.md` | Umsetzung | autonom | fertig |
| GEB15.3 | `GEB15.3-review.md` | Review | autonom | offen |
| GEB15.4 | `GEB15.4-abnahme-geraet.md` | Workshop | Mensch | offen |

- GEB15.1 (DEV) Höhlen-Wurzeln: Varianten und Dichte (AC-01).
- GEB15.2 (DEV) Felsen und Minen: größer, neue Varianten (AC-02).
- GEB15.3 Review: alle Kriterien prüfen.
- GEB15.4 (Mensch) Abnahme am Gerät (AC-03).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
