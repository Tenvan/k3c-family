# B-1190 · Abnahme HB1: HUD-Baustellen, Biome, Landstreicher-Lager, Hafen (PC-Abnahme, Sim, Gen, Race)

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WN4
- **Projekt:** WZN
- **Erstellt:** 2026-11-21
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint HB1 (B-1186 HUD, B-1187 Biome und Landstreicher-Lager, B-1188 Hafen) ist umgesetzt, die Review HB1.6 ist durch. Offen sind PC-Abnahme und die in Sessions gesperrten Läufe (B-742).

## Ziel

Abnahme-Ticket HB1 (genau eins): (1) PC-Abnahme durch Ralf; (2) `task check:race`; (3) `task gen:paper` samt Golden; (4) Sim und Balance nur über `sim_test` (Taverne max 5 Landstreicher, Hafen nah am Lager verändern Balance). Rot: neues Ticket.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **AC-01** Abnahme am PC (Ralf): Baustellen-Texte klein, Kosten als Icons, Ausbau x/n, Händler mit zwei Ständen sichtbar, Gebäudename über dem Gebäude bei Monarch davor, Biom-Zeile am Radar in jeder Stufe, Lager zwischen Bäumen mit Taverne-Platz nach Rodung, Start am Hafen (1 bis 2 Chunks vom Lager), Hafen bewachsen.
- **AC-02** `task check:race` und `task gen:paper` samt Golden grün (oder Befund als Ticket).
- **AC-03** Offene Review-Befunde entschieden: (a) Gebäudename überlappt Bogen-/Hammer-Preis (Name nun bei h+2,4 Units, im Bild prüfen); (b) Kosten nur durch Rot als zu wenig markiert, kein zweites Signal; (c) Kosten-Icons allokieren je Bild (`price_cost`); (d) Baustellen-Text SMALL 0,55 am TV im Split-Screen lesbar?; (e) `near_harbor` kann bei Kollision mit anderem Planchunk auf Slot ±1/±2 oder sehr kleiner Insel zu Err führen; (f) Späteinsteiger (`add_island_player`) starten ebenfalls am Hafen; (g) Hafen-Bäume lassen Plätze (SITE_GAP 1) frei, Dichte ist Konstante `TREES_PER_UNIT` im Code statt Datenwert; (h) „muss freigeräumt werden“ ohne Bau-Sperre am Schiff; (i) Händlerfigur selbst fehlt, nur die Stände; (j) Biome außer Wald haben keine Baumflächen.
- **AC-04** Rote Tests nicht aus HB1: `alloc_je_bild`, `eingebettete_welten_gueltig`, `insel_nach_1000_schritten_wie_go`, `e2e_flow`, drei `planning.test.ts`-Tests.

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

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
