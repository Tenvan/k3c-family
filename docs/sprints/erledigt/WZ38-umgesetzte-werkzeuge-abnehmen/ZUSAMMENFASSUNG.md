# WZ38 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Umgesetzte Werkzeug-Tickets abnehmen: Welten-Editor, Studio-Seiten, Git-Seite, check:diff
- **Projekt:** WZG

## Ziel

🧑 nimmt die umgesetzten Werkzeug-Tickets in einem Durchgang ab, danach sind sie `erledigt`. Am Ende sichtbar: Welten-Editor, Studio-Seiten und Git-Seite am PC.

## Ergebnis

2026-10-31: Kriterien gegen Code und Tests belegt (WZ38.1), Abnahme am PC durch Ralf (WZ38.2). Neue Tickets aus den Befunden: B-689 (Inspektor scrollt nicht in sich), B-690 (Testlücken), B-691 (Doku Norton, codemap).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Welten-Editor erfüllt `B-663/AC-01` bis `B-667/AC-05`. |
| AC-02 | geprüft | Studio-Seiten Planer, Monarch, Gegner erfüllen `B-670` bis `B-672`. |
| AC-03 | geprüft | Git-Seite erfüllt `B-580/AC-01` bis `AC-05`. |
| AC-04 | geprüft | `check:diff` ist wie in `B-619` beschrieben schnell (Norton-Ausnahme gesetzt, Workbench ne … |
| AC-05 | – | 🧑 hat AC-01 bis AC-04 abgenommen. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-663 | erledigt | AC-01 geprüft | Der Welten Editor bricht nach der Breite seiner Seite um und zeigt keine waagrec … |
| B-664 | erledigt | – | Der Welten Editor trennt Welt-Struktur und Stufe bearbeiten und speichert an ein … |
| B-665 | erledigt | – | Im Modus Stufe bearbeiten stehen eine große Live-Vorschau, eine schmale Chunk-Vo … |
| B-666 | erledigt | – | Gebäude-Zustände der Live-Vorschau lassen sich je Art, per Voreinstellung und je … |
| B-667 | erledigt | AC-01 geprüft | Ein Klick auf einen Bauplatz in der Live-Vorschau wählt ihn im Inspektor |
| B-670 | erledigt | AC-02 geprüft | Der Gebäude-Planer zeigt Bühne, Matrix und Inspektor auf einen Blick, ohne dass … |
| B-671 | erledigt | – | Die Monarch-Seite hat eine Werkzeugleiste und eine große Bühne, die Skills liege … |
| B-672 | erledigt | AC-02 geprüft | Die Gegner-Seite zeigt Gegner als kompakte Tabelle mit Inspektor, Lücken als Zah … |
| B-580 | erledigt | AC-03 geprüft | Die Workbench hat eine Git-Seite für dieses Repo |
| B-619 | erledigt | AC-04 geprüft | check:diff nutzt den Go-Testcache und lässt lange Tests aus |
