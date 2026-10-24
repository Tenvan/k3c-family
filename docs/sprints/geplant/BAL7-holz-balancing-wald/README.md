# BAL7 · REG · Holz-Balancing nach dem dichteren Wald

- **Status:** geplant
- **Projekt:** BAL
- **Domäne:** REG
- **Reife:** Entwurf
- **Tickets:** B-589
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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
| BAL7.1 | – | Umsetzung | autonom | offen |
| BAL7.2 | – | Workshop | Mensch | offen |
| BAL7.3 | – | Review | autonom | offen |

- BAL7.1 (REG) Holzfluss messen über `sim_test`, Wertvorschlag (AC-01).
- BAL7.2 (Mensch) Werte entscheiden (AC-02).
- BAL7.3 Werte umsetzen und Review.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
