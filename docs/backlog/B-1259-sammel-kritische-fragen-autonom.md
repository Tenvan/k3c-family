# B-1259 · Sammelticket: kritische Fragen aus der autonomen Arbeit, von Ralf zu entscheiden

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WE6
- **Projekt:** WZG
- **Erstellt:** 2026-11-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Bei der autonomen Abarbeitung (Ziel: „Rest autonom ohne Rückfragen“) sind Fragen offen geblieben, die nur Ralf entscheiden kann. Sie stehen hier gesammelt; wo ein Default nach best practice möglich war, ist er angegeben und gilt bis zur Entscheidung.

1. **Schiffbau-Startwerte (B-1244):** Holzmengen 8/12/16/20/24, fünf Zahlungsstufen, Stapellauf 5 Gold. Balancing ist gesperrt. Default: Werte wie in B-1244 festgehalten, Balancing später mit Freigabe.
2. **Schiffs-Animation (B-1244):** Sichtprüfung durch Ralf am Gerät nötig, Anzeige nicht im Bereich der Session.
3. **Standbild `hud-pad-tastatur` (still_hud):** Die Go-Referenz ist seit HB1 veraltet; der Radar-Streifen ist für dieses Bild ausgenommen (`tests/common/hud_radar.rs`). Default: Ausnahme bleibt, bis Referenz neu gesetzt wird. Entscheidung: Referenz neu setzen oder Ausnahme dauerhaft.
4. **e2e-Referenz (B-1258):** `ablauf_wie_go_zwei_laeufe_gleich` ist ausgesetzt. Entscheidung: Neuaufnahme aus dem heutigen Rust-Stand freigeben (Default: ja, Neuaufnahme in einer eigenen Session).
5. **Assets GS3.2 und GS4.3:** Studio-MCP muss laufen. Entscheidung: Studio für diese Sessions starten, oder die Assets zurückstellen.
6. **WE6.6 Umfang:** Skins je Welt sind im Rust-Spiel committet (`7b37e4fec` auf `sprint/we6-6-skins`). Offen: Objekt-Skins und Endboss je Biom. Entscheidung: Umfang bestätigen.
7. **Sichtprüfungen:** WE6.4 und SI7 (Oberfläche im Studio), Workbench- und Studio-EXE neu bauen. Ohne diese Prüfung gelten die Sessions als umgesetzt, aber nicht abgenommen.
8. **WE8 Zielkorridore:** Events (Überfall, Sandsturm, Nebel) brauchen Zielkorridore. Balancing ist gesperrt, deshalb bleiben die Werte vorläufig. Entscheidung: Balancing-Freigabe, wann.

## Ziel

Ralf kann in einem Durchgang alle Fragen beantworten, die die autonome Arbeit angehalten haben. Nutzen: Die Session-Arbeit läuft danach ohne Rückfragen weiter.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

1. Jede Frage oben hat eine Entscheidung von Ralf oder bleibt beim Default.
2. Nach einer Entscheidung wird die betroffene Session umgesetzt oder eine Folge-Session angelegt.
3. Neue offene Fragen kommen in dieses Ticket, nicht in ein neues.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01**: Jede der acht Fragen hat eine Entscheidung oder einen dokumentierten Default.
- **AC-02**: Die betroffenen Sessions (B-1244, B-1258, WE6.6, GS3.2, GS4.3, WE8) sind nach der Entscheidung eingeplant oder abgeschlossen.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
