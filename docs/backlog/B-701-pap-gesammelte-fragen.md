# B-701 · PAP: kritische Fragen und vorab getroffene Entscheidungen werden gesammelt von Ralf bestätigt

- **Domäne:** DEV
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** PS1
- **Projekt:** PAP
- **Erstellt:** 2026-11-06
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-06, Chat (🧑: gesammelter Fragen-Task)

## Ausgangslage

🧑 hat am 2026-11-06 festgelegt: Bis zum Sichtprüfungs-Sprint PS1 entscheidet Claude offene Punkte im Projekt PAP mit einem Default und arbeitet weiter; kritische Fragen werden hier gesammelt statt einzeln gestellt.

## Ziel

🧑 bestätigt oder korrigiert alle vorab getroffenen Entscheidungen in einem Durchgang (PS1.1 oder früher auf Wunsch).

## Beteiligte und Zielgruppen

🧑 entscheidet; Agenten des Projekts PAP tragen neue Entscheidungen hier ein.

## Anforderungen

- Jede Entscheidung als Zeile `D-nn · Thema · Default · Alternative · betroffene Sprints · Status (offen|bestätigt|geändert)` im Abschnitt Notizen.
- Agenten, die einen Default wählen, tragen ihn hier ein und nennen ihn im Ergebnis ihrer Session.
- **Kritisch** (Arbeit stoppt, statt Default): Kosten, Lizenz/Rechte, Datenverlust, Änderung an `main` oder Veröffentlichung.

## Nicht-Ziele

Sichtprüfungen selbst (PS1.2–PS1.4).

## Regeln und Einschränkungen

Memory-Regel „Autonom: Specs selbst freigeben“ gilt; Änderungen nach 🧑-Entscheid laufen als neue Tickets im Projekt PAP.

## Beispiele

MG4 braucht einen Rig-Typ für die Spinne → Default `requisite` mit Keyframes, Zeile D-08, MG4 arbeitet weiter.

## Ausnahme- und Fehlerfälle

Eine spätere Korrektur durch 🧑 macht Arbeit ungültig → Folgeticket im Projekt PAP, kein Rückbau ohne Ticket.

## Akzeptanzkriterien

- **AC-01** Jede Zeile D-nn hat den Status `bestätigt` oder `geändert` (mit Folgeticket).

## Offene Fragen

Siehe Notizen; keine blockiert die Umsetzung (Defaults gelten bis PS1.1).

## Notizen

| Nr. | Thema | Default (Claude) | Alternative | Sprints | Status |
|---|---|---|---|---|---|
| D-01 | Neue Abhängigkeiten | `@resvg/resvg-js@2.6.2`, `ajv@8.17.1` | eigener Validator ohne ajv | PV1 | offen |
| D-02 | Arbeitsweise | Domäne PAP, Session-Felder `Lane` und `Modell`, max. 4 Agenten parallel | alles in DEV, seriell | PV1 | offen |
| D-03 | Laufzeit Renderer | bun (`bun tools/paper/bin/paper.ts`) | Node mit Build | PV1, RD1 | offen |
| D-04 | Ausgaben im Repo | PNGs + Manifeste committet, Index je Gruppe | nur Specs committen, Build erzeugt PNGs | PV1, MG*, US1 | offen |
| D-05 | Golden-Regel | exakte Hashes, bei Plattform-Abweichung Toleranz 0,1 % Pixel | nur Toleranz | PV1, RD1 | offen |
| D-06 | Altes Register | ab MC1 eingefroren, Entfernung in CU1 | parallel weiter pflegbar | MC1, CU1 | offen |
| D-07 | Neue Papier-Materialien | `smaragd`, `purpur`, `stein`, `holz` mit von Claude gewählten Tönen | 🧑 wählt Töne | PV1 | offen |
| D-08 | Rig für Spinne, Golem | `requisite` mit Keyframes | neuer Rig-Typ `vielbeiner` | MG4, MG5 | offen |
| D-09 | Schatten auf Boden/Hintergrund | Phaser-4-Filter, sonst abgedunkelte Sprite-Kopie | im PNG backen | US1 | offen |
| D-10 | Workbench während der Umsetzung | eine Workbench aus der Repo-Wurzel im Dev-Modus; Agenten bauen keine eigene; neue MCP-Tools gelten erst nach Merge | eigene Instanz je Worktree | alle | offen |
| D-11 | Datei-Watch | Server ignoriert `*.paper.json`, Neustart nur bei geänderten PNGs | alles beobachten | US1 | offen |
| D-12 | Chat und „In Claude öffnen“ | Studio-Chat entfällt, „In Claude öffnen“ bleibt und wird zusätzlich Prompt-Button | beides weg | WB1 | offen |
