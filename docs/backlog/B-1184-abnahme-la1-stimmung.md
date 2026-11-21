# B-1184 · Abnahme LA1: Stimmung und Look-Anbindung (FPS-Messung, Gen, Race)

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** LA1
- **Projekt:** RSU
- **Erstellt:** 2026-11-21
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint LA1 ist in main (PR #795), LA1.5 von 🧑 am PC abgenommen (2026-11-21). Offen sind die in Sessions gesperrten Läufe (B-742) und eine FPS-Messung der flackernden Fackeln.

## Ziel

Abnahme-Ticket des Sprints LA1 (genau eins): (1) `task check:race`; (2) `task gen:paper` samt Grafik-Golden; (3) FPS und schlechtester Frame mit vielen Fackeln im Hub (Leistungsanzeige) gemessen; (4) rote Tests `economy_wie_go` (ChestGold), `alloc_je_bild`, `e2e_flow` (skillpunkt) auf Herkunft prüfen, Ticket je Befund.

## Beteiligte und Zielgruppen

🧑 Ralf (PC-Messung); Hauptrechner.

## Anforderungen

- **AC-01** `task check:race` grün oder Befund als Ticket.
- **AC-02** `task gen:paper` und Grafik-Golden grün oder Befund als Ticket.
- **AC-03** FPS-Messung im Hub nachts mit Fackeln: schlechtester Frame im Budget (60 Hz), Messwert im Ergebnis.
- **AC-04** Die drei roten Rust-Tests sind eingeordnet (nicht von LA1) und als Tickets erfasst.

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

- **AC-01** `task check:race` grün oder Befund als Ticket.
- **AC-02** `task gen:paper` und Grafik-Golden grün oder Befund als Ticket.
- **AC-03** FPS-Messung im Hub nachts mit Fackeln im Budget.
- **AC-04** Rote Tests eingeordnet und als Tickets erfasst.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
