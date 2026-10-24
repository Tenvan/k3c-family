# WZ28 · DEV · Offline-Werkzeuge: Slot-Details, Ticket-Button ohne KI, Balancing-Seite

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-594, B-595, B-596
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-24, Ralf im Chat (Revision 1)

## Ausgangslage

Ralf (2026-10-24): Die Grafik-Seite zeigt zu wenig zu den Slots (B-594); er muss auf einem Notebook ohne Internet und Claude testen und Tickets anlegen (B-595) sowie Balancing-Simulationen mit Berichten fahren können (B-596).

## Ziel

Slot-Details auf der Grafik-Seite, ein Ticket-Button ohne KI auf jeder Seite von Workbench und Studio und eine Balancing-Seite für Kampf und Wirtschaft mit Berichten und automatischem Abstimmen der Werte, alles ohne Internet und ohne Claude. Am Ende sichtbar: Notebook ohne Netz legt Tickets an und fährt einen Balancing-Lauf mit Bericht.

## Beteiligte und Zielgruppen

Ralf; Agent DEV; 🧑 entscheidet die offenen Fragen und nimmt am Notebook ab.

## Anforderungen

B-594, B-595, B-596 › Anforderungen.

## Nicht-Ziele

Neue Balancing-Regeln (BAL), KI-Zuordnung per Chat, Synchronisation zwischen Rechnern (Git).

## Regeln und Einschränkungen

Atomic Design über `@webkit`; keine externen Abrufe zur Laufzeit (Schriften, Skripte lokal gebündelt); Texte zweisprachig; Studio-Frontend ohne EXE-Neubau änderbar.

- **Ticket-Kern geteilt:** Das Studio kann `tools/k3c-workbench/internal/planning` nicht importieren (Go-`internal`, eigenes Modul). Vorlage, Nummer und Index für neue Tickets ziehen in ein Paket `tools/assetkit/ticket` (beide Apps binden `assetkit` schon ein). `planning.Create` legt Tickets danach über dieses Paket an, eine zweite Nummernvergabe entsteht nicht. Offline zählt nur der lokale Stand (Backlog und Archiv); den Abgleich mit Worktrees und `origin` macht die Workbench wie bisher.
- **Balancing-Läufe über einen Weg (B-348):** Die Laufsteuerung von `sim_test` (`internal/mcpsrv/simtest*.go`: Bau des Balance-Binary, Start, Abbruch, Berichte) zieht in ein Paket der Workbench (`internal/simrun`), das MCP-Tool und Balancing-Seite gemeinsam nutzen. Die Warteschlange kommt dort hinzu; die Grenze von zwei gleichzeitigen Läufen je Checkout bleibt. Wert-Varianten nutzen `internal/balance/vary.go` (nur im Speicher).
- **Diagramme ohne neue Abhängigkeit:** kleine SVG-Bausteine in `@webkit` (Linie, Balken, Korridor-Band); keine Chart-Bibliothek.
- **Atlas lesen:** `public/atlas/atlas.json` wird heute nur geschrieben (`tools/atlas`); das Studio-Backend liest sie neu, schreibt sie nicht.

## Beispiele

Slot `shardling` gewählt → Detailbereich mit Atlas-Definition; Button „Ticket“ → B-nnn im Backlog; Balancing-Lauf 20 Seeds → Bericht Pass/Fail.

## Ausnahme- und Fehlerfälle

Atlas fehlt → Hinweis `task gen:atlas`. Backlog nicht beschreibbar → Eingabe bleibt. Lauf bricht ab → Teilbericht.

## Akzeptanzkriterien

- **AC-01** Die Grafik-Seite zeigt je Slot Name, Beschreibung, Status, Sheet- und Atlas-Definition, Pack, Quelle, Lizenz, Verwendung (B-594/AC-01 bis AC-03).
- **AC-02** Gemeinsamer Ticket-Baustein und Backend legen Tickets nach Vorlage mit Seitenkontext an (B-595/AC-01, B-595/AC-03).
- **AC-03** Der Ticket-Button steht auf allen Seiten von Workbench und Studio (B-595/AC-02).
- **AC-04** Das Balancing-Backend startet, zeigt und bricht Läufe ab, auch mit Wert-Varianten, und liefert die neuen Kennzahlen für Kampf und Wirtschaft (B-596/AC-01, B-596/AC-04, B-596/AC-07).
- **AC-05** Die Balancing-Seite in der Workbench konfiguriert Läufe, zeigt Fortschritt und Berichte, vergleicht und exportiert (B-596/AC-02, B-596/AC-03).
- **AC-06** Sweep und automatische Suche liefern Vorschläge mit Diff, ohne `data/` zu ändern (B-596/AC-06).
- **AC-07** Alle drei Teile laufen ohne Netzwerk, geprüft per Test und ohne externe Anfragen (B-595/AC-04, B-596/AC-05).
- **AC-08** Ralf hat Ticket-Button und Balancing-Seite am Notebook ohne Internet abgenommen.

## Offene Fragen

keine. Entschieden (🧑, 2026-10-24): nächste freie Ticket-Nummer genügt (Ralf arbeitet allein), Balancing-Seite in der Workbench, Kennzahlen nach Bedarf, Schwerpunkt automatisiertes Balancing; Ticket-Kern in `tools/assetkit/ticket`, Laufsteuerung als `internal/simrun` für MCP und Seite, Diagramme als eigene SVG-Bausteine; Zuordnungs-Tabellen werden direkt aus `docs/assets/zuordnung*.md` gelesen (B-594).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ28.1 | `WZ28.1-slot-details.md` | Umsetzung | autonom | offen |
| WZ28.2 | `WZ28.2-ticket-baustein.md` | Umsetzung | autonom | offen |
| WZ28.3 | `WZ28.3-ticket-button-alle-seiten.md` | Umsetzung | autonom | offen |
| WZ28.4 | `WZ28.4-balancing-backend.md` | Umsetzung | autonom | offen |
| WZ28.5 | `WZ28.5-balancing-seite.md` | Umsetzung | autonom | offen |
| WZ28.6 | `WZ28.6-automatisches-balancing.md` | Umsetzung | autonom | offen |
| WZ28.7 | `WZ28.7-review.md` | Review | autonom | offen |
| WZ28.8 | `WZ28.8-abnahme-notebook-offline.md` | Workshop | Mensch | offen |

- WZ28.1 (DEV) Slot-Details im Studio: Backend-Aufbereitung aus Atlas, `sprites.json`, `assets.json`, Zuordnung; Detailbereich (AC-01).
- WZ28.2 (DEV) Ticket-Baustein: Go-Paket zum Anlegen nach Vorlage, `TicketButton` und Dialog in `@webkit`, Kontext-Provider (AC-02).
- WZ28.3 (DEV) Einbau des Ticket-Buttons in alle Seiten von Workbench und Studio, Seitenliste-Test (AC-03).
- WZ28.4 (DEV) Balancing-Backend ohne MCP für Kampf und Wirtschaft: Lauf starten, Status, Abbruch, Warteschlange, Berichte speichern, Wert-Varianten, neue Kennzahlen (Kampf und Wirtschaft) (AC-04, AC-05).
- WZ28.5 (DEV) Balancing-Seite in der Workbench: Konfiguration, Fortschritt, Bericht mit Diagrammen, Vergleich, Export (AC-05).
- WZ28.6 (DEV) Automatisches Balancing für Kampf- und Wirtschafts-Werte: Parameter-Sweep, Suche gegen die Zielkorridore, Vorschlag mit Diff und Begründung, Seite dazu (AC-06).
- WZ28.7 (DEV) Review und Offline-Nachweis: Test, dass keine externen Anfragen entstehen (AC-07).
- WZ28.8 (Mensch) Abnahme am Notebook ohne Internet (AC-08).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
