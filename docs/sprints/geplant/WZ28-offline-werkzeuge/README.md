# WZ28 · DEV · Offline-Werkzeuge: Slot-Details, Ticket-Button ohne KI, Balancing-Seite

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-594, B-595, B-596
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ralf (2026-10-24): Die Grafik-Seite zeigt zu wenig zu den Slots (B-594); er muss auf einem Notebook ohne Internet und Claude testen und Tickets anlegen (B-595) sowie Balancing-Simulationen mit Berichten fahren können (B-596).

## Ziel

Slot-Details auf der Grafik-Seite, ein Ticket-Button ohne KI auf jeder Seite von Workbench und Studio und eine Balancing-Seite mit Berichten, alles ohne Internet und ohne Claude. Am Ende sichtbar: Notebook ohne Netz legt Tickets an und fährt einen Balancing-Lauf mit Bericht.

## Beteiligte und Zielgruppen

Ralf; Agent DEV; 🧑 entscheidet die offenen Fragen und nimmt am Notebook ab.

## Anforderungen

B-594, B-595, B-596 › Anforderungen.

## Nicht-Ziele

Neue Balancing-Regeln (BAL), KI-Zuordnung per Chat, Synchronisation zwischen Rechnern (Git).

## Regeln und Einschränkungen

Atomic Design über `@webkit`; Backend-Logik in `tools/assetkit` bzw. `internal/` der Apps; keine externen Abrufe zur Laufzeit (Schriften, Skripte lokal gebündelt); Testläufe nur über den Weg von `sim_test`; Texte zweisprachig; Studio-Frontend ohne EXE-Neubau änderbar.

## Beispiele

Slot `shardling` gewählt → Detailbereich mit Atlas-Definition; Button „Ticket“ → B-nnn im Backlog; Balancing-Lauf 20 Seeds → Bericht Pass/Fail.

## Ausnahme- und Fehlerfälle

Atlas fehlt → Hinweis `task gen:atlas`. Backlog nicht beschreibbar → Eingabe bleibt. Lauf bricht ab → Teilbericht.

## Akzeptanzkriterien

- **AC-01** Die Grafik-Seite zeigt je Slot Name, Beschreibung, Status, Sheet- und Atlas-Definition, Pack, Quelle, Lizenz, Verwendung (B-594/AC-01 bis AC-03).
- **AC-02** Gemeinsamer Ticket-Baustein und Backend legen Tickets nach Vorlage mit Seitenkontext an (B-595/AC-01, B-595/AC-03).
- **AC-03** Der Ticket-Button steht auf allen Seiten von Workbench und Studio (B-595/AC-02).
- **AC-04** Das Balancing-Backend startet, zeigt und bricht Läufe ab, auch mit Wert-Varianten (B-596/AC-01, B-596/AC-04).
- **AC-05** Die Balancing-Seite konfiguriert Läufe, zeigt Fortschritt und Berichte, vergleicht und exportiert (B-596/AC-02, B-596/AC-03).
- **AC-06** Alle drei Teile laufen ohne Netzwerk, geprüft per Test und ohne externe Anfragen (B-595/AC-04, B-596/AC-05).
- **AC-07** Ralf hat Ticket-Button und Balancing-Seite am Notebook ohne Internet abgenommen.

## Offene Fragen

1. Nummern-Kollision bei zwei Rechnern (B-595, Default: nächste freie Nummer). 2. Balancing-Seite in der Workbench (Default) oder im Studio (B-596). 3. Pflicht-Kennzahlen im Bericht (B-596, Default aus `zielkorridore.md`). Spec-Freigabe steht aus.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ28.1 | – | Umsetzung | autonom | offen |
| WZ28.2 | – | Umsetzung | autonom | offen |
| WZ28.3 | – | Umsetzung | autonom | offen |
| WZ28.4 | – | Umsetzung | autonom | offen |
| WZ28.5 | – | Umsetzung | autonom | offen |
| WZ28.6 | – | Review | autonom | offen |
| WZ28.7 | – | Workshop | Mensch | offen |

- WZ28.1 (DEV) Slot-Details im Studio: Backend-Aufbereitung aus Atlas, `sprites.json`, `assets.json`, Zuordnung; Detailbereich (AC-01).
- WZ28.2 (DEV) Ticket-Baustein: Go-Paket zum Anlegen nach Vorlage, `TicketButton` und Dialog in `@webkit`, Kontext-Provider (AC-02).
- WZ28.3 (DEV) Einbau des Ticket-Buttons in alle Seiten von Workbench und Studio, Seitenliste-Test (AC-03).
- WZ28.4 (DEV) Balancing-Backend ohne MCP: Lauf starten, Status, Abbruch, Berichte speichern, Wert-Varianten (AC-04).
- WZ28.5 (DEV) Balancing-Seite: Konfiguration, Fortschritt, Bericht mit Diagrammen, Vergleich, Export (AC-05).
- WZ28.6 (DEV) Review und Offline-Nachweis: Test, dass keine externen Anfragen entstehen (AC-06).
- WZ28.7 (Mensch) Abnahme am Notebook ohne Internet (AC-07).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
