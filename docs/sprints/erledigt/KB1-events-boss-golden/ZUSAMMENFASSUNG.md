# KB1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Kampf-Nachträge: Mond-Events, Endboss-Bau, Golden-Läufe, Feuergeist
- **Projekt:** KMP

## Ziel

Mond-Events und Gegner verhalten sich in allen Stufen und Biomen wie im Regelwerk beschrieben und sind durch Tests abgesichert. Am Ende sichtbar: `task check:go` grün, neue Golden-Dateien für Tiefe 3 und 4 mit Begründung im Commit.

## Ergebnis

2026-10-25, Claude (KB1.5): `task check` und `task check:go` grün; `check:race` lokal übersprungen (kein C-Compiler). AC-01 erfüllt durch KB1.1, AC-02 durch KB1.2 (bestehende Tests), AC-03 durch KB1.3 (zwei neue Golden-Läufe, 17,9 MiB), AC-04 durch KB1.4 (Regeldatei). Review ohne schwere Befunde; `bosse.md` ist nachgezogen, der Stichtag „Morgengrauen“ ist vermerkt. Gestapelt auf W12 (#479). Keine neuen Tickets.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | Die Vollmond-Belohnung gilt für die bespielten Stufen, der Blutmond verstärkt Boss-Flächen … |
| AC-02 | geprüft | Die Lage des Endboss-Baus ist entschieden und im Generator mit Test umgesetzt (B-343). |
| AC-03 | geprüft | Golden-Läufe für Eisenstollen und Kristallhöhle liegen unter `testdata/golden/` (B-327). |
| AC-04 | geprüft | Der Feuergeist hinterlässt eine Flammen-Fläche, oder `docs/rules/gegner.md` ist angepasst … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-379 | erledigt | AC-01 erfüllt | Die Vollmond-Belohnung gilt für die Stufen, in denen gespielt wird |
| B-380 | erledigt | AC-01 erfüllt | Der Blutmond verstärkt auch die Flächenangriffe der Bosse |
| B-343 | erledigt | AC-02 geprüft | Der Bau des Endbosses liegt an der inneren Kante des Ausgangs-Chunks |
| B-327 | erledigt | AC-03 geprüft | Golden-Läufe decken Eisenstollen und Kristallhöhle ab |
| B-328 | erledigt | AC-04 geprüft | Der Feuergeist hinterlässt eine Flammen-Fläche |
