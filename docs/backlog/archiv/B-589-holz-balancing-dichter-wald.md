# B-589 · Holz-Balancing nach dem dichteren Wald messen und nachziehen

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** BAL7
- **Projekt:** BAL
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Wald steht nach B-585 viel dichter, Holz fließt entsprechend schneller. Das Balancing wurde dort bewusst nicht gemessen (Ralf, 2026-10-24: eigener Sprint).

## Ziel

Der Holzfluss liegt wieder im Zielkorridor; Werte sind nachgezogen.

## Beteiligte und Zielgruppen

🧑 entscheidet über die Werte; Agent REG misst über `sim_test`.

## Anforderungen

- Messung von Holz je Nacht und Zeit bis zu den Hub-Stufen mit dem neuen Wald.
- Wertvorschlag (Holzmenge je Baum, Kosten, Anzahl Bäume) und Entscheidung.

## Nicht-Ziele

Die Wald-Dichte selbst (B-585).

## Regeln und Einschränkungen

Testläufe nur über `sim_test`; Werte in `data/`; Golden mit Begründung.

## Beispiele

Zu viel Holz → Holz je Baum senken oder Kosten anheben.

## Ausnahme- und Fehlerfälle

Messung im Korridor → nichts ändern, Ticket erledigt.

## Akzeptanzkriterien

- **AC-01** Messbericht Holz je Nacht und Hub-Stufen-Zeit mit dem dichteren Wald (`sim_test`).
- **AC-02** Entscheidung von 🧑 über Werte; Änderungen umgesetzt oder verworfen.

## Offene Fragen

Zielkorridor laut `docs/rules`; 🧑 bestätigt.

## Notizen

Aus B-585. Spec: Entwurf. Eigener Balancing-Sprint, nicht in GEB16.
