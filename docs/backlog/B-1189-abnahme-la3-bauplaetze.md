# B-1189 · Abnahme LA3: Bauplätze nach Gebäudegröße (PC-Sichtprüfung, Race, Rust-Aufzeichnung)

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** LA3
- **Projekt:** RSU
- **Erstellt:** 2026-11-21
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint LA3 (B-1182) spreizt die Hub-Plätze nach Gebäudebreite (`k3c_data::load_game`, `data/sizes.json › bauplaetze`) und schiebt Turm und Mauer der Zone 1 hinter das äußerste Gebäude. Session-Gates (`task check`, `task check:go` ohne Lint) sind grün. Offen sind die Abnahme am PC und die in Sessions gesperrten Läufe (B-742).

## Ziel

Abnahme-Ticket des Sprints LA3 (genau eins): (1) Hub der Standard-Welt auf Stufe 4 am PC ansehen: keine Überlappung der Gebäude, Turm und Mauer am Rand, Zahlstellen erreichbar, 1 und 2 Spieler; (2) `task check:race`; (3) Go-Lint (`golangci-lint` fehlt lokal) auf einem Rechner mit Werkzeug; (4) rote Rust-Tests einordnen: `alloc_je_bild`, `e2e_flow` (skillpunkt), `insel_nach_1000_schritten_wie_go` (alle auf main rot), lokale `data/worlds/rom.json` (nicht eingecheckt) macht `eingebettete_welten_gueltig` und `alle_welt_dateien_aus_data_sind_ladbar` rot; Ticket je Befund.

## Beteiligte und Zielgruppen

🧑 Ralf (Abnahme am PC); Hauptrechner.

## Anforderungen

- **AC-01** Abnahme am PC durch 🧑: Hub in allen festen Welten ohne Überlappung (Ansicht, Zahlstellen).
- **AC-02** `task check:race` grün oder Befund als Ticket.
- **AC-03** `golangci-lint` auf der Wurzel grün.
- **AC-04** Die roten Rust-Tests sind eingeordnet und als Tickets erfasst.
- **AC-05** Folge: Die Rust-Aufzeichnung (`testdata/rust/trace`) und das Level-Layout der Zone 1 haben sich in Rust verschoben; 🧑 bestätigt das.

## Nicht-Ziele

Neue Funktionen; Sim und Balancing (gesperrt).

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

- **AC-01** Abnahme am PC durch 🧑.
- **AC-02** `task check:race` grün.
- **AC-03** `golangci-lint` grün.
- **AC-04** Rote Rust-Tests eingeordnet, Tickets erfasst.
- **AC-05** Verschiebung der Zone 1 in Rust von 🧑 bestätigt.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
