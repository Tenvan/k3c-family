# R1 · REG · Regelwerk I – Fundament

- **Status:** geplant
- **Domäne:** REG
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-004, B-005, B-021, B-025
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`game-design.md` ist die einzige Regelquelle und widerspricht Entscheidung 001 (B-005). Sonst stecken Regeln im Go-Code (`engine/sim/`, die TS-Simulation ist seit SP09 gelöscht) und in `data/`.

## Ziel

Das Fundament des Regelwerks ist beschlossen. Am Ende sichtbar: `docs/rules/wirtschaft.md`, `docs/rules/stufen.md`, `game-design.md` ohne Widerspruch zu Entscheidung 001.

## Beteiligte und Zielgruppen

🧑 entscheidet in Workshops, der Agent bereitet vor und fragt einzeln; die SIM-Sprints in Go nutzen das Ergebnis.

## Anforderungen

B-004 › Anforderungen (Regelwerk I), B-005, B-021 und B-025 › Anforderungen.

## Nicht-Ziele

Skills und Monarch (Regelwerk II). Keine Code-Änderungen.

## Regeln und Einschränkungen

Workshops sind Sessions mit `Agent: Mensch`. Werte bleiben in `data/`, Regeln verweisen darauf.

## Beispiele

Workshop Wirtschaft → `docs/rules/wirtschaft.md` mit Regel, Begründung und Verweis auf `data/*.json`.

## Ausnahme- und Fehlerfälle

Keine Einigung im Workshop → Frage-Ticket, Thema im nächsten Workshop.

## Akzeptanzkriterien

- **AC-01** `docs/rules/wirtschaft.md` ist beschlossen (B-004/AC-01).
- **AC-02** `docs/rules/stufen.md` mit dem Ziel der Kampagne ist beschlossen (B-025/AC-01).
- **AC-03** `game-design.md` widerspricht Entscheidung 001 nicht mehr (B-005/AC-01, B-005/AC-02).
- **AC-04** Die Belegung der Taste X ist entschieden (B-021/AC-01, B-021/AC-02).
- **AC-05** Umsetzungs-Tickets für SIM (Go) und CLI liegen im Backlog.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| R1.1 | `R1.1-ist-regelwerk.md` | Umsetzung | autonom | offen |
| R1.2 | `R1.2-workshop-wirtschaft.md` | Workshop | Mensch | offen |
| R1.3 | `R1.3-workshop-stufen.md` | Workshop | Mensch | offen |
| R1.4 | `R1.4-beschluss-tickets.md` | Umsetzung | autonom | offen |

## Abnahme

–
