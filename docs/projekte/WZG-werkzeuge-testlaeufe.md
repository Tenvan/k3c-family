# WZG · Werkzeuge

- **Status:** aktiv
- **Rang:** 2
- **Ziel-Tickets:** –

## Ziel

Die Entwickler-Werkzeuge (k3c-workbench mit MCP und Planung, Werkzeug-Seiten unter `src/tools/`) decken Planung, Prüfung und Diagnose ab. Dazu kommt ein eigenes Asset-Studio (WZ6–WZ10, B-459–B-464), getrennt von der Workbench, weil Ralf dort laufend ändert: Seiten für Grafik und Sound, Welt-Betrachter mit Chunk- und Live-Vorschau, Chunk-Planung je Biom, Gebäude-Planer für die Freischaltung je Hub-Stufe sowie Vorschauen für Monarch, Gegner und Bosse.

## Sprints

In Abarbeitungs-Reihenfolge; der erste nicht erledigte Sprint ist der nächste. Jeder Sprint nennt das Projekt im Feld `Projekt`.

| Sprint | Thema | Status |
|---|---|---|
| M11 | MCP-Seite: alle Tools mit Statistik, Zeitfilter | erledigt |
| DBG3 | Dungeon-Master-Seite /dm | erledigt |
| MON2 | Monitoring-Seite mit Dashboard | erledigt |
| PL2 | Werkzeug-Seiten in der gewählten Sprache | erledigt |
| DV1 | Domäne DEV für k3c-workbench, Sprint ohne Prio und Einschiebbar | erledigt |
| WT1 | Worktree-Sessions richten k3c-workbench-Tools per Argument auf ihren Checkout | erledigt |
| WZ2 | Planungs-Werkzeuge: eindeutige Nummern, Fahrplan, Flags, Prüfläufe | erledigt |
| WZ3 | Review-Nachträge zu WZ2: Filter-Zählung, Splitter-Test, Kleinigkeiten | erledigt |
| WZ4 | Splitter-Test in DOM-Umgebung | erledigt |
| WZ5 | Planungsseite im Look der erp-qs-Workbench | erledigt |
| WZ6 | Asset-Studio als eigene EXE mit Seiten für Grafik und Sound | erledigt |
| WZ7 | Welt-Betrachter im Asset-Studio mit Chunk- und Live-Vorschau | erledigt |
| WZ8 | Chunks je Insel und Stufe planen: Seed oder feste Objekte | erledigt |
| WZ16 | Welt-Betrachter: eine gemeinsame Ansicht mit Sprungleiste, Zoom und Gebäude-Filter | erledigt |
| WZ15 | Hub-Planung Vorlauf: Regeltabelle erzeugt, Prüfung ohne globalen Zustand | erledigt |
| WZ9 | Gebäude-Planer für die Freischaltung je Hub-Stufe | erledigt |
| WZ10 | Abspiel-Bausteine und Monarch-Vorschau im Asset-Studio | erledigt |
| WZ11 | Gegner- und Boss-Vorschau im Asset-Studio, Figuren-Seiten entfernen | erledigt |
| WZ12 | Planungs-Werkzeug: Platzhalter-Zeilen und Worktree-Pills | erledigt |
| WZ13 | Glossar und Regeln: Verweis für das Morgengrauen auf B-470 | erledigt |
| WZ14 | Geplante Chunks tragen Ruinen und Gegnerlager | erledigt |
| WZ17 | Chunk-Editor komfortabel: Inselgrößen, getrennter Zoom, Seed-Chunk umwandeln | erledigt |
| WZ18 | Welten im Spiel: Welt-Dateien, Welt-Wahl im Raum und Welten-Editor | erledigt |
| WZ19 | Chat im Studio mit Claude Code CLI und Welt per Prompt füllen | erledigt |
| TS1 | Spielserver und Asset-Studio trennen, Server beobachtet Dateien | erledigt |
| WZ20 | k3c-workbench: Planungstest für Entwürfe, Worktree-Einträge vollständig und lesend, sim_test im Worktree | erledigt |
| WZ21 | Studio-Chat: eigene Umgebung, Grundgerüst als Instruktion, Entwurf geschützt | erledigt |
| WZ22 | Studio-Editoren: Fehler sichtbar, Im Spiel öffnen wiederholbar | erledigt |
| WZ23 | Studio-Vorschauen: Reittiere aus dem Spielcode, Monarch-Seite, Boss-Anzeigen, Gebäude-Planer abgenommen | erledigt |
| WZ24 | Geplante Chunks in Simulation und Spielstand: Regeln geklärt, Planung im Stand, eine Prüfung der Inselgröße | erledigt |
| WZ25 | Nachträge zum Server-Neustart: Räume bleiben offen, Assets ohne Build, Werkzeuge passen dazu | erledigt |
| WZ26 | CI rot beheben, Prüf- und Planungswerkzeuge härten | erledigt |
| WZ27 | Geplante Chunks: Gegnerlager, Ruinen-Kollision, Dateigrenze | erledigt |
| WZ28 | Offline-Werkzeuge: Slot-Details, Ticket-Button ohne KI, Balancing-Seite | aktiv |
| WZ30 | Oberflächen zur Abnahme: Szenen, Filter, Quellen, Chat-Karten mit Mock-Daten | aktiv |
| WZ29 | Ausbau der Sound- und Grafik-Seite: Hörproben, Vorschauen, Filter, AI-Chat | aktiv |
| WZ31 | Workbench und Studio als Web-Client: Server starten, im Browser arbeiten | aktiv |
| WZ32 | Balancing-Seite: feste Fläche, eine Liste, Lauf-Editor mit Varianten-Tabelle | aktiv |

## Nicht-Ziele

Testläufe mit Bots, `sim_test` und Lasttest gehören zu TST (abgetrennt 2026-10-08).

## Notizen

Entscheidungen, Zurückstellungen (Grund und Datum bei `ruht`). Darf leer bleiben (`–`).
