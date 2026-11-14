# WZ32 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Balancing-Seite: feste Fläche, eine Liste, Lauf-Editor mit Varianten-Tabelle
- **Projekt:** WZG

## Ziel

Die Balancing-Seite ist in jeder Fensterbreite bedienbar: feste Fläche mit Liste und Detail, Berichte erkennbar und gesammelt verwaltbar, ein Lauf entsteht im Lauf-Editor mit Varianten-Tabelle und Daten-Browser. Am Ende sichtbar: Workbench im Browser-Pane mit `?mock=1` bei 1.440, 960 und 600 px wie im Mock; Vitest und Go-Tests grün.

## Ergebnis

2026-10-29: AC-01 bis AC-10 erfüllt (Ergebnisse WZ32.1 bis WZ32.5), `task check` und `task check:go` grün. 3 schwere Review-Befunde behoben (Geister-Lauf in der Liste, abgelehnter Start schloss den Editor, Textmodus verlor Zeilen). Neue Tickets: B-668, B-669. Manuelle Abnahme am PC durch 🧑 steht aus.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | Bericht liefert Auftrag, Label, Lauf-ID und getroffene Ziele (B-662/AC-01). |
| AC-02 | erfüllt | Sammel-Bewerten, Sammel-Löschen und Baseline aus der Oberfläche (B-662/AC-02, AC-03). |
| AC-03 | erfüllt | Ereignisse statt Abfrage, Werte aller Dateien, Option „je Variante“ (B-662/AC-04, AC-05). |
| AC-04 | erfüllt | Umbruch unter 1.100 px und feste Fläche mit eigenen Scroll-Bereichen (B-658/AC-01, AC-02, … |
| AC-05 | erfüllt | Eine Liste mit Suche, Chips und Mehrfachauswahl, Vergleich von 2 bis 4 Berichten (B-660/AC … |
| AC-06 | erfüllt | Detail mit fünf Reitern, Erneut starten, Als Variante übernehmen (B-660/AC-04, AC-05). |
| AC-07 | erfüllt | Lauf-Editor mit Ausgangspunkt, Abschnitten und Fußleiste (B-661/AC-01, AC-06). |
| AC-08 | erfüllt | Varianten-Tabelle mit Prüfung, Bereich als Abstimmung, Textmodus verlustfrei (B-661/AC-02 … |
| AC-09 | erfüllt | Daten-Browser mit Suche über alle Dateien (B-661/AC-05). |
| AC-10 | erfüllt | `task check` und `task check:go` grün. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-658 | verworfen | AC-04 erfüllt | Die Balancing-Seite stellt Konfiguration und Berichte unter 1.100 px untereinand … |
| B-659 | verworfen | AC-04 erfüllt | Die Balancing-Seite füllt die Fläche, nur Liste und Detail scrollen |
| B-660 | verworfen | AC-05 erfüllt, AC-06 erfüllt | Läufe und Berichte stehen in einer Liste mit Filter und Mehrfachauswahl, der Ber … |
| B-661 | verworfen | AC-07 erfüllt, AC-08 erfüllt, AC-09 erfüllt | Der Lauf-Editor setzt Varianten als Tabelle mit Basiswert und Daten-Browser |
| B-662 | erledigt | AC-01 erfüllt, AC-02 erfüllt, AC-03 erfüllt | Die Workbench liefert Auftrag und Label je Bericht und verwaltet Berichte gesamm … |
