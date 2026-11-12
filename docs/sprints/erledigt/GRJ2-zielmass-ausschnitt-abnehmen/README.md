# GRJ2 · DEV · Grafik-Zielmaß und Ausschnitt-Rahmen abnehmen

- **Status:** erledigt
- **Projekt:** GRJ
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-684, B-685
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-31, Ralf im Chat (Revision 1)

## Ausgangslage

Zielmaß je Grafik-Slot (B-685, 71 Gebäude-Slots in `data/assets.json`) und der Rahmen im Ausschnitt-Editor (B-684) sind am 2026-10-31 umgesetzt; die Abnahme (B-684/AC-03, B-685/AC-04) steht aus.

## Ziel

🧑 nimmt Zielmaß und Ausschnitt-Rahmen ab. Am Ende sichtbar: Grafik im Studio-Vorschau-Maß, Ausschnitt wahlweise als Rahmen oder Bild.

## Beteiligte und Zielgruppen

🧑 nimmt ab; Agent DEV und CLI prüfen vorher.

## Anforderungen

`B-684 › Anforderungen`, `B-685 › Anforderungen`.

## Nicht-Ziele

Zielmaße für Gegner und Natur (laut `B-685 › Notizen` später).

## Regeln und Einschränkungen

Studio schreibt `data/assets.json`; Client zeichnet nach `ziel`; Spec „rückwirkend“, Freigabe durch 🧑.

## Beispiele

Gebäude X Stufe 3 belegt 300×200 Pixel → Grafik wird hineinskaliert.

## Ausnahme- und Fehlerfälle

Slot ohne Zielmaß → bisheriges Verhalten.

## Akzeptanzkriterien

- **AC-01** Ausschnitt zeigt wahlweise Rahmen oder Bild, Grafik passt in die Kachel (`B-684/AC-01` bis `AC-03`).
- **AC-02** Grafiken werden ins Zielmaß skaliert (`B-685/AC-01` bis `AC-04`).
- **AC-03** 🧑 hat AC-01 und AC-02 am PC abgenommen.

## Offene Fragen

keine.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- GRJ2.1 (DEV) Kriterien gegen Code und Tests belegen (AC-01, AC-02).
- GRJ2.2 (🧑) Abnahme am PC (AC-03).

## Abnahme

Verworfen 2026-11-12 (Entscheidung Ralf): B-684 durch Wegfall des Asset-Studios überholt (Entscheidung 005), B-685 abgelöst durch B-823 (Entscheidung 009); beide Tickets verworfen, keine Sessions. Status `erledigt`, weil das Werkzeug für Sprints keinen Status `verworfen` kennt.
