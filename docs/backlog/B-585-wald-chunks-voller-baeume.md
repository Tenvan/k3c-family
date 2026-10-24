# B-585 · Wald-Chunks stehen voller Bäume

- **Domäne:** REG
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** GEB16
- **Projekt:** GEB
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-531/B-540 hat den Wald auf `tree [6, 9]` je Waldabschnitt (50 Units) gebracht. Ralf (2026-10-24): deutlich enger, ein Wald-Chunk soll voller Bäume sein.

## Ziel

Wald-Abschnitte wirken wie dichter Wald.

## Beteiligte und Zielgruppen

Spielende; 🧑 legt die Dichte fest und nimmt ab.

## Anforderungen

- `data/biomes/forest.json`: Waldabschnitt `tree [14, 20]`, Bachabschnitt `tree [3, 6]`; Lichtung und Felsen unverändert.
- Golden mit Begründung neu schreiben; die Eingaben von `campaign-ausbau.json` laufen zu den neuen Bäumen (fünf Bäume am Hub markieren).

## Nicht-Ziele

Baumgröße (B-541), Holz-Balancing (`sim_test` später).

## Regeln und Einschränkungen

Werte in `data/`; Golden nur mit Begründung; deterministisch.

## Beispiele

Waldabschnitt → 14 bis 20 Bäume statt 6 bis 9.

## Ausnahme- und Fehlerfälle

Zu viel Holz im Balancing → Werte nachziehen (eigenes Ticket).

## Akzeptanzkriterien

- **AC-01** Waldabschnitte haben 14 bis 20 Bäume (`deco_test.go`).
- **AC-02** Golden-Läufe grün mit neuen Werten.
- **AC-03** 🧑 sieht den Wald im Spiel dicht genug (offen).

## Offene Fragen

Holz-Balancing mit `sim_test` nicht gefahren; bei Bedarf Folgeticket.

## Notizen

Aus der Abnahme am 2026-10-24. Spec: Entwurf. Verweis: B-531, B-540.
