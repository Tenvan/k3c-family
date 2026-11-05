# BAL9 · DEV, SIM, REG · Meilenstein 1: Titan über acht Seeds, castleHeld im Fenster, Bot-Performance

- **Status:** aktiv
- **Projekt:** BAL
- **Domäne:** DEV, SIM, REG
- **Reife:** bereit
- **Tickets:** B-656, B-657, B-654
- **Start-Commit:** 29493645
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-31, Ralf im Chat (Revision 1)

## Ausgangslage

Der Meilenstein-Lauf coop2 standard scheitert am Titan und an `holz` und `schiff` (B-656, Lauf run-1 Fail); die Kennzahl `castleHeld` misst über den ganzen Lauf statt im Fenster (B-657); Bot-Performance ist teilweise gelöst, `wanted` und `nodeFor` stehen noch (B-654).

## Ziel

Der Meilenstein-Lauf wird über 8 Seeds sauber beurteilt, `castleHeld` misst nur Nacht 1 bis 5, der Bot-Anteil ist schneller. Am Ende sichtbar: Bericht mit Pass je Kennzahl oder begründet neu gefassten Zielen.

## Beteiligte und Zielgruppen

Wer balanct; Agenten REG, DEV, SIM; 🧑 entscheidet über neu gefasste Ziele.

## Anforderungen

`B-656 › Anforderungen`, `B-657 › Anforderungen`, `B-654 › Anforderungen`.

## Nicht-Ziele

Neue Mechaniken; Balancing-Runde mit Spieleabend (BR1, BR2).

## Regeln und Einschränkungen

Alle Testläufe nur über `sim_test`, Sim-Session eigene Session vor der Abnahme; Bot-Verhalten bleibt byte-gleich (B-654); Coop nur über Multiplikatoren.

## Beispiele

Lauf mit 8 Seeds, `M4` als Basis → Titan in n von 8 Seeds besiegt, Bericht nennt Ampel.

## Ausnahme- und Fehlerfälle

Ziel nicht erreichbar → Ziel begründet neu fassen statt Wert blind drehen.

## Akzeptanzkriterien

- **AC-01** Der Meilenstein-Lauf wird über 8 Seeds gegen `M4` beurteilt, `holz` und `schiff` grün oder begründet neu gefasst (`B-656/AC-01` bis `AC-04`).
- **AC-02** `castleHeld` wertet nur Nacht 1 bis 5 (`B-657/AC-01` bis `AC-03`).
- **AC-03** `wanted` und `nodeFor` ohne Map-Zugriffe je Tick, Lauf byte-gleich (`B-654/AC-01`).

## Offene Fragen

keine. Entschieden (🧑, 2026-10-31): Bleibt `holz` oder `schiff` rot, fasst der Sprint das Ziel mit Begründung neu, die Werte bleiben; 🧑 bestätigt die neue Fassung im Bericht.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BAL9.1 | `BAL9.1-castleheld-fenster.md` | Umsetzung | autonom | fertig |
| BAL9.2 | `BAL9.2-bot-performance.md` | Umsetzung | autonom | fertig |
| BAL9.3 | `BAL9.3-meilenstein-acht-seeds.md` | Umsetzung | autonom | in Arbeit |
| BAL9.4 | `BAL9.4-review.md` | Review | autonom | offen |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- BAL9.1 (DEV) `castleHeld` im Fenster (AC-02).
- BAL9.2 (SIM) Bot-Performance (AC-03).
- BAL9.3 (REG) Meilenstein-Lauf mit 8 Seeds, Werte und Ziele (AC-01).
- BAL9.4 Review, alle Kriterien, schließt ab.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
