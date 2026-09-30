# R1 · REG · Regelwerk I – Fundament

- **Status:** geplant
- **Domäne:** REG
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-004, B-005, B-021, B-025
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`game-design.md` ist die einzige Regelquelle und widerspricht Entscheidung 001 (B-005). Sonst stecken Regeln im TS-Code (`src/world/sim/`) und in `data/`.

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

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- R1.1 Ist-Regelwerk: Regeln aus `src/world/sim/` und `data/` gegen `game-design.md` abgleichen; Widersprüche als Tickets; Gliederung für `docs/rules/` (AC-03).
- R1.2 🧑 Workshop Kern-Loop & Wirtschaft: Gold/Material, Besitz im gemischten Koop (2–4+ Spieler), Tag/Nacht, Wellen, Taste X (AC-01, AC-04).
- R1.3 🧑 Workshop Stufen & Niederlage: Tiefen, Aggressionspool, Strafen, Ziel der Kampagne (AC-02).
- R1.4 🔍 Review + Beschluss, Umsetzungs-Tickets für SIM (Go) und CLI (AC-05, alle).

## Abnahme

–
