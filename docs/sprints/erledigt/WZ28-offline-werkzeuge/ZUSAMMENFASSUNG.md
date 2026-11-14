# WZ28 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Offline-Werkzeuge: Slot-Details, Ticket-Button ohne KI, Balancing-Seite
- **Projekt:** WZG

## Ziel

Slot-Details auf der Grafik-Seite, ein Ticket-Button ohne KI auf jeder Seite von Workbench und Studio und eine Balancing-Seite für Kampf und Wirtschaft mit Berichten und automatischem Abstimmen der Werte, alles ohne Internet und ohne Claude. Am Ende sichtbar: Notebook ohne Netz legt Tickets an und fährt einen Balancing-Lauf mit Bericht.

## Ergebnis

- 2026-10-24, Review-Session WZ28.7 (autonom, Claude): AC-01–AC-07, AC-09–AC-14 belegt (Session-Ergebnisse, Offline-Nachweis `offline_test.go`, `tests/offlineTools.test.ts`); AC-08 offen bei WZ28.8 (Ralf am Notebook). - `task check` und `task check:go` grün; `check:race` nur in der CI (lokal CGO aus). - Behoben im Review: unlesbare `reports/ratings.json` blockiert Liste und Löschen nicht mehr; vorher `findAssignment` (TestGoNestingDepth). - Neue Tickets: B-615, B-616. Balancing-Seite (Hilfen, Dateien, Bot je Spieler, Berichte) hat Ralf im Mock abgenommen. - 2026-10-31, WZ28.8: AC-08 von Ralf ( …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Die Grafik-Seite zeigt je Slot Name, Beschreibung, Status, Sheet- und Atlas-Definition, Pa … |
| AC-02 | – | Gemeinsamer Ticket-Baustein und Backend legen Tickets nach Vorlage mit Seitenkontext an (B … |
| AC-03 | – | Der Ticket-Button steht auf allen Seiten von Workbench und Studio (B-595/AC-02). |
| AC-04 | – | Das Balancing-Backend startet, zeigt und bricht Läufe ab, auch mit Wert-Varianten, und lie … |
| AC-05 | – | Die Balancing-Seite in der Workbench konfiguriert Läufe, zeigt Fortschritt und Berichte, v … |
| AC-06 | – | Sweep und automatische Suche liefern Vorschläge mit Diff, ohne `data/` zu ändern (B-596/AC … |
| AC-07 | – | Alle drei Teile laufen ohne Netzwerk, geprüft per Test und ohne externe Anfragen (B-595/AC … |
| AC-08 | offen | Ralf hat Ticket-Button und Balancing-Seite am Notebook ohne Internet abgenommen. |
| AC-09 | geprüft | Die Studio-Tests sind grün und `check_run task:check:diff` läuft mit Studio-Anteil durch ( … |
| AC-10 | geprüft | Hilfen und Beispiele: Jedes Eingabefeld der Balancing-Konfiguration hat Hilfetext und Hilf … |
| AC-11 | geprüft | Datei-Liste mit Popups: Die Balancing-Seite listet alle Dateien, die das Balancing beeinfl … |
| AC-12 | geprüft | Beschreibungen in `docs/data/`: alle 17 Dateien beschrieben, Pfade geprüft (B-608/AC-01, B … |
| AC-13 | geprüft | Bot je Spieler: Offline-Läufe geben jedem Spieler ein eigenes Profil (`saver+passive`), Se … |
| AC-14 | geprüft | Berichte verwalten: einzeln bewerten und nach Rückfrage löschen, Vergleich mit Vergleichsb … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-594 | erledigt | AC-01 – | Die Grafik-Seite zeigt je Slot Name, Atlas-Definition und alles für die Zuordnun … |
| B-595 | erledigt | AC-02 –, AC-03 –, AC-07 – | Jede Seite hat einen Button, der ohne KI ein Ticket zur Seite anlegt |
| B-596 | erledigt | AC-04 –, AC-05 –, AC-06 –, AC-07 – | Eine Balancing-Seite fährt Simulationen und Berichte ohne Internet und ohne Clau … |
| B-606 | erledigt | AC-09 geprüft | Die Studio-Tests sind nach dem dichten Wald wieder grün und check_run hängt nich … |
| B-608 | erledigt | AC-12 geprüft | Jede balancing-relevante Datei in data/ hat eine Beschreibung mit Aufbau und Wer … |
| B-609 | erledigt | AC-10 geprüft | Die Balancing-Konfiguration erklärt jedes Eingabefeld |
| B-610 | erledigt | AC-11 geprüft | Die Balancing-Seite listet die Konfigurationsdateien, die das Balancing beeinflu … |
| B-611 | erledigt | AC-13 geprüft | Ein Balancing-Lauf kann für jeden Spieler einen eigenen Bot wählen |
| B-612 | erledigt | AC-14 geprüft | Balancing-Berichte lassen sich einzeln löschen und bewerten, der Vergleich erklä … |
