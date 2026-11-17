# B-1024 · Rust-Datenprüfungen, die Tabellen aus Balance oder Sim brauchen, fehlen noch in k3c-data

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU25
- **Projekt:** RSU
- **Erstellt:** 2026-11-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU3.4/RU3.5 laden alle Spiel-Daten in `rust/crates/k3c-data` und machen die Prüfungen, die nur die Datei selbst oder andere Daten-Dateien brauchen. Einige Go-Prüfungen beim Laden hängen an Tabellen und Logik aus Paketen, die erst spätere Sprints portieren; sie fehlen in Rust noch:

- `balance-hints.json`: Ziel-Kennzahl der Hebel (`measures`, engine/balance/targets.go).
- `balance-targets.json`: Kennzahl und Art (`measures`), Bot (`knownBot`) und Spielerzahl (`checkPlayers`), Grad (`checkGrade`), Welt (`loadWorld`), Dev-Hilfen (`checkHelps`).
- `hub.json`: `hub_validate.go`/`hub_validate_layout.go` (Gebäude-Planer, braucht Sim-Logik).

Weitere Nachträge aus RU3.5 stehen im Ergebnis von RU3.5.

## Ziel

Die Rust-Ladefunktionen lehnen dieselben Dateien ab wie Go, mit denselben Texten.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Port nach `docs/standards/rust-port.md`; die Prüfungen kommen in die Crate, die die Tabelle besitzt (z. B. `k3c-balance`), und rufen `k3c-data` zum Lesen.

## Nicht-Ziele

Prüfungen ändern oder verschärfen.

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

- **AC-01** Jede oben genannte Prüfung hat in Rust einen Test mit dem Fehlertext aus Go; die Liste ist leer oder jeder Rest hat ein eigenes Ticket.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
