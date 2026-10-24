# B-585 · Wald-Chunks stehen voller Bäume

- **Domäne:** REG
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** GEB16
- **Projekt:** GEB
- **Erstellt:** 2026-10-24
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-25

## Ausgangslage

B-531/B-540 hat den Wald auf `tree [6, 9]` je Waldabschnitt (50 Units) gebracht. Ralf (2026-10-24): deutlich enger, ein Wald-Chunk soll voller Bäume sein.

## Ziel

Wald-Abschnitte wirken wie dichter Wald.

## Beteiligte und Zielgruppen

Spielende; 🧑 legt die Dichte fest und nimmt ab.

## Anforderungen

- Waldabschnitt noch dichter als `tree [14, 20]` (Ralf, 2026-10-24: „noch dichter“); neuer Wert in der Umsetzungs-Session festlegen und von 🧑 bestätigen lassen. Default: `[22, 30]`, Bachabschnitt `[5, 8]`.
- Golden mit Begründung neu schreiben; die Eingaben von `campaign-ausbau.json` markieren genug Bäume am Hub für die Werkstatt.
- Das Holz-Balancing ist ausdrücklich nicht Teil davon (B-589, eigener Sprint).

## Nicht-Ziele

Baumgröße (B-541), Holz-Balancing (`sim_test` später).

## Regeln und Einschränkungen

Werte in `data/`; Golden nur mit Begründung; deterministisch.

## Beispiele

Waldabschnitt → 14 bis 20 Bäume statt 6 bis 9.

## Ausnahme- und Fehlerfälle

Zu viel Holz im Balancing → Werte nachziehen (eigenes Ticket).

## Akzeptanzkriterien

- **AC-01** Waldabschnitte haben mehr Bäume als heute (`deco_test.go` mit dem neuen Wert).
- **AC-02** Golden-Läufe grün mit neuen Werten.
- **AC-03** 🧑 sieht den Wald im Spiel dicht genug (offen).

## Offene Fragen

Endwert der Dichte: Default `[22, 30]`, 🧑 bestätigt am Gerät.

## Notizen

Aus der Abnahme am 2026-10-24. Spec: Entwurf. Verweis: B-531, B-540.
