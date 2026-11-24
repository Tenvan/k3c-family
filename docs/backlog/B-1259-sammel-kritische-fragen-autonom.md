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
2. **Schiffs-Animation (B-1244):** Sichtprüfung durch Ralf am Gerät nötig.
3. **Standbild `hud-pad-tastatur` (still_hud):** Die Go-Referenz ist seit HB1 veraltet; der Radar-Streifen ist ausgenommen (`tests/common/hud_radar.rs`). Default: Ausnahme bleibt. Entscheidung: Referenz neu setzen oder Ausnahme dauerhaft.
4. **e2e-Referenz (B-1258):** `ablauf_wie_go_zwei_laeufe_gleich` ist ausgesetzt. Default: Neuaufnahme aus dem heutigen Rust-Stand in einer eigenen Session.
5. **Assets GS3.2 und GS4.3:** Studio-MCP muss laufen. Entscheidung: Studio für diese Sessions starten, oder Assets zurückstellen.
6. **WE6.6 Umfang:** Skins je Welt sind auf `sprint/we6-6-skins` committet (`7b37e4fec`), nicht gemergt und nicht gebaut verifiziert (Linker-Fehler 1104 im frischen Build-Ordner). Offen: Objekt-Skins, Endboss und Bau je Biom. Entscheidung: Umfang bestätigen; Rendering ohne Grafik lässt sich nur per Standbild prüfen.
7. **Sichtprüfungen:** WE6.4 und SI7 (Oberfläche im Studio), Workbench- und Studio-EXE neu bauen.
8. **WE8 Zielkorridore:** Balancing ist gesperrt; die Werte bleiben vorläufig.
9. **Override-Name Welt-Events (WE8.7):** Die drei Events stehen in `data/welt-events.json` und werden über `overrides › welt-events` eingeschaltet, nicht über `overrides › events` wie B-991 und B-992 es nennen. Grund: das Go-Golden `events_wie_go` muss bytegleich bleiben. Default: so belassen, Spec nachziehen. Entscheidung: Spec anpassen oder Golden neu schreiben.
10. **Sandsturm und Monarch (WE8.7):** Der Sandsturm bremst den Monarchen und fliehende Gegner im Fest, wie der Taifun. Default: so belassen. Entscheidung: im Spiel prüfen, dann bestätigen oder die Regel auf Truppen und Gegner begrenzen.
11. **Vorbestehende Rot-Stellen auf `main`:** vier Sim-Tests (`island::worlds_tests` ×2, `island::takt_tests::insel_nach_1000_schritten_wie_go`, `story::mod_tests::standard_welt_erzaehlt_eine_geschichte`) und `task lint` (Playtest- und Studio-Frontend). Default: eigenes Aufräum-Ticket, nicht in Feature-PRs mischen. Entscheidung: Priorität.
12. **Offene Sessions ohne Ausführung in dieser Runde:** WE8.5, WE8.8, WE6.6 (Rest), GS3.2, GS4.3, GS4.6, GS4.7, Reviews GS3.6, GS4.5, WE6.5, WE8.6, RU36.5. Entscheidung: Reihenfolge und ob Studio-MCP-Sessions warten.

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

- **AC-01**: Jede der zwölf Fragen hat eine Entscheidung oder einen dokumentierten Default.
- **AC-02**: Die betroffenen Sessions (B-1244, B-1258, WE6.6, WE8.5, WE8.8, GS3.2, GS4.3, GS4.6, GS4.7, Reviews) sind nach der Entscheidung eingeplant oder abgeschlossen.

## Offene Fragen

13. **Welt-Events nicht über das Studio setzbar (WE8.5):** `world_override_set` kennt `welt-events` nicht (nur die Go-Dateiliste). Entscheidung: Studio-Liste um `welt-events` erweitern (eigenes Ticket, Klasse A) oder Events über `events` setzen? Default: eigenes Ticket anlegen, Event-Zuordnung bis dahin offen. Entscheidung: 🧑.
14. **Skins-Zuordnung (WE8.5):** B-996 nennt Variante → Asset, `skins` ist Basis-Asset → Ersatz-Asset (docs/rules/welten.md § 2.3). Ersatz-Assets wie `gegner/ninja` fehlen. Default: Skins nur, wo ein Ersatz-Asset existiert; sonst Feindnamen allein. Entscheidung: 🧑.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
