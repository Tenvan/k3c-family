# WZG · Werkzeuge

- **Status:** aktiv
- **Rang:** 1
- **Ziel-Tickets:** –

## Ziel

Die Entwickler-Werkzeuge (k3c-dev mit MCP und Planung, Werkzeug-Seiten unter `src/tools/`) decken Planung, Prüfung und Diagnose ab. Dazu kommt ein eigenes Asset-Studio (WZ6–WZ10, B-459–B-464), getrennt von der Workbench, weil Ralf dort laufend ändert: Seiten für Grafik und Sound, Welt-Betrachter mit Chunk- und Live-Vorschau, Chunk-Planung je Biom, Gebäude-Planer für die Freischaltung je Hub-Stufe sowie Vorschauen für Monarch, Gegner und Bosse.

## Sprints

In Abarbeitungs-Reihenfolge; der erste nicht erledigte Sprint ist der nächste. Jeder Sprint nennt das Projekt im Feld `Projekt`.

| Sprint | Thema | Status |
|---|---|---|
| M11 | MCP-Seite: alle Tools mit Statistik, Zeitfilter | erledigt |
| DBG3 | Dungeon-Master-Seite /dm | erledigt |
| MON2 | Monitoring-Seite mit Dashboard | erledigt |
| PL2 | Werkzeug-Seiten in der gewählten Sprache | erledigt |
| DV1 | Domäne DEV für k3c-dev, Sprint ohne Prio und Einschiebbar | erledigt |
| WT1 | Worktree-Sessions richten k3c-dev-Tools per Argument auf ihren Checkout | erledigt |
| WZ2 | Planungs-Werkzeuge: eindeutige Nummern, Fahrplan, Flags, Prüfläufe | erledigt |
| WZ3 | Review-Nachträge zu WZ2: Filter-Zählung, Splitter-Test, Kleinigkeiten | erledigt |
| WZ4 | Splitter-Test in DOM-Umgebung | erledigt |
| WZ5 | Planungsseite im Look der erp-qs-Workbench | erledigt |
| WZ6 | Asset-Studio als eigene EXE mit Seiten für Grafik und Sound | erledigt |
| WZ7 | Welt-Betrachter im Asset-Studio mit Chunk- und Live-Vorschau | erledigt |
| WZ8 | Chunks je Insel und Stufe planen: Seed oder feste Objekte | aktiv |
| WZ16 | Welt-Betrachter: eine gemeinsame Ansicht mit Sprungleiste, Zoom und Gebäude-Filter | erledigt |
| WZ15 | Hub-Planung Vorlauf: Regeltabelle erzeugt, Prüfung ohne globalen Zustand | geplant |
| WZ9 | Gebäude-Planer für die Freischaltung je Hub-Stufe | geplant |
| WZ10 | Abspiel-Bausteine und Monarch-Vorschau im Asset-Studio | aktiv |
| WZ11 | Gegner- und Boss-Vorschau im Asset-Studio, Figuren-Seiten entfernen | aktiv |
| WZ12 | Planungs-Werkzeug: Platzhalter-Zeilen und Worktree-Pills | aktiv |
| WZ13 | Glossar und Regeln: Verweis für das Morgengrauen auf B-470 | geplant |
| WZ14 | Geplante Chunks tragen Ruinen und Gegnerlager | geplant |

## Nicht-Ziele

Testläufe mit Bots, `sim_test` und Lasttest gehören zu TST (abgetrennt 2026-10-08).

## Notizen

Entscheidungen, Zurückstellungen (Grund und Datum bei `ruht`). Darf leer bleiben (`–`).
