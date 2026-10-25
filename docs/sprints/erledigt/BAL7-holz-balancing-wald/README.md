# BAL7 · REG · Holz-Balancing nach dem dichteren Wald

- **Status:** erledigt
- **Projekt:** BAL
- **Domäne:** REG
- **Reife:** bereit
- **Tickets:** B-589
- **Start-Commit:** 819c53d3
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-25, Revision 1 (Ziel von Ralf; nur Messung und Vorschlag, Werte entscheidet Ralf in BAL7.2)

## Ausgangslage

Nach GEB16 steht der Wald viel dichter (B-585); das Holz-Balancing wurde dort bewusst nicht gemessen (B-589).

## Ziel

Der Holzfluss liegt im Zielkorridor; Werte sind nachgezogen. Am Ende sichtbar: Messbericht aus `sim_test` und gesetzte Werte in `data/`.

## Beteiligte und Zielgruppen

🧑 entscheidet die Werte; Agent REG misst.

## Anforderungen

B-589 › Anforderungen.

## Nicht-Ziele

Wald-Dichte (B-585), andere Rohstoffe.

## Regeln und Einschränkungen

Testläufe nur über `sim_test`; Werte in `data/`; Golden mit Begründung. Läuft erst nach GEB16.

## Beispiele

Zu viel Holz → Holz je Baum senken oder Kosten anheben.

## Ausnahme- und Fehlerfälle

Messung im Korridor → nichts ändern.

## Akzeptanzkriterien

- **AC-01** Messbericht Holz je Nacht und Hub-Stufen-Zeit (B-589/AC-01).
- **AC-02** Werte von 🧑 entschieden und umgesetzt oder verworfen (B-589/AC-02).

## Offene Fragen

Zielkorridor laut `docs/rules`; 🧑 bestätigt. Spec-Freigabe steht aus.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BAL7.1 | `BAL7.1-holzfluss-messen.md` | Umsetzung | autonom | fertig |
| BAL7.2 | `BAL7.2-werte-entscheiden.md` | Workshop | Mensch | fertig |
| BAL7.3 | `BAL7.3-review.md` | Review | autonom | fertig |

- BAL7.1 (REG) Holzfluss messen über `sim_test`, Wertvorschlag (AC-01).
- BAL7.2 (Mensch) Werte entscheiden (AC-02).
- BAL7.3 Werte umsetzen und Review.

## Abnahme

2026-10-25, Claude: AC-01 durch BAL7.1 (run-7, Holz 85 % im Korridor). AC-02 durch BAL7.2 (vorentschieden: keine Wertänderung) und BAL7.3 (nichts umzusetzen). Keine neuen Tickets.
