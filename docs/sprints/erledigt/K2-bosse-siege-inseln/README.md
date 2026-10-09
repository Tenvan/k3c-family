# K2 · SIM · Bosse, Siegvarianten und Inselwechsel

- **Status:** erledigt
- **Projekt:** KMP
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** B-130, B-102, B-103
- **Start-Commit:** 7fe4f05
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-07, Chat, durch 🧑, Revision 1

## Ausgangslage

Es gibt keine Minibosse und keinen Endboss, Siegvarianten und Niederlage-Modi der Raum-Optionen sind nicht umgesetzt, ein Inselwechsel fehlt (B-130, B-102, B-103).

## Ziel

Je Stufe ein Miniboss, je Insel ein Endboss, wählbare Siegvarianten und Niederlage-Modi sowie der gemeinsame Inselwechsel sind in der Simulation spielbar.

Am Ende sichtbar: Tests je Boss, Siegvariante und Modus grün, Spielstand mit besiegten Bossen und aktueller Insel.

## Beteiligte und Zielgruppen

Spieler (Koop) und REG für Werte; 🧑 gibt die Spec frei.

## Anforderungen

B-130 › Anforderungen, B-102 › Anforderungen, B-103 › Anforderungen.

## Nicht-Ziele

Events (K3), Protokoll (K4), Anzeige (K5), Balancing (BR2).

## Regeln und Einschränkungen

Werte nur in `data/`, Logik und Tests in `engine/sim/`; deterministisch (`engine/rng`), mit 2+ Spielern gleichzeitig. Golden-Daten nach dem Golden-Ablauf (B-137) aktualisieren, Spielstand-Änderungen nach der Migrationsregel (B-137). Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`. Der Sprint bleibt in der Domäne SIM. Quelle: `docs/rules/bosse.md`, `docs/rules/stufen.md`.

Beantwortet aus dem Regelwerk (Stand 2026-10-06): Boss-Namen und alle Boss-Zahlen sind vorläufige Startwerte, 🧑 hat sie als „Vorschlag“ bestätigt (`docs/rules/bosse.md` § 1.1 und Annahmen), endgültige Werte kommen über B-099/BR2. „Gold sammeln“ zählt die Summe aller eingesammelten Münzen der Spieler (`docs/rules/stufen.md` § 3, N = 1000). Die Reihenfolge der Inseln ist fest aus den Daten (`docs/rules/stufen.md` § 1).

## Beispiele

Endboss besiegt → Inselwechsel frei → alle lebenden Spieler am Punkt → neue Insel mit neuer Tabelle.

## Ausnahme- und Fehlerfälle

Nicht alle lebenden Spieler am Punkt → Wechsel wartet. Komplett verloren → Raum endet, letzter Spielstand bleibt unverändert.

## Akzeptanzkriterien

- **AC-01** Miniboss erscheint mit Welle 5 bzw. 3, Fähigkeit und Belohnung laut Daten (Test) (B-130/AC-01).
- **AC-02** Endboss wird bei Ankunft ausgelöst, wartet sonst, wechselt die Phasen (Test) (B-130/AC-02).
- **AC-03** Boss-HP skaliert mit der Spieleranzahl der Insel (Test) (B-130/AC-03).
- **AC-04** Der Spielstand merkt besiegte Bosse und die aktuelle Insel, Bosse kehren nie zurück (Test) (B-130/AC-04, B-103/AC-04).
- **AC-05** Je Siegvariante: Bedingung nicht erfüllt, erfüllt, Ereignis genau einmal (Test) (B-102/AC-01).
- **AC-06** Je Niederlage-Modus gelten die Folgen laut Regelwerk, „Komplett verloren“ lässt den letzten Spielstand unverändert, Standard je Grad und Überschreiben je Raum (Test) (B-102/AC-02, B-102/AC-03, B-102/AC-04).
- **AC-07** Endboss besiegt gibt den Inselwechsel frei, Wechsel nur mit allen lebenden Spielern am Punkt, je Stufe ein Miniboss und der Endboss in der tiefsten (Test) (B-103/AC-01, B-103/AC-02, B-103/AC-03).
- **AC-08** Golden-Daten aktualisiert, `task check:go` grün (B-130/AC-05).

## Offene Fragen

- Anzahl n der Inseln und Inhalt weiterer Inseln (`docs/rules/stufen.md` § 1 und § 7: offen, bis Insel 1 spielbar ist; entscheidet 🧑). Nicht blockierend: `data/islands.json` enthält nur Insel 1, der Wechsel-Test (K2.3a) nutzt eine zweite Insel aus der Testdatei, der Code kennt keine feste Anzahl.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| K2.1a | `K2.1a-boss-rahmen-minibosse.md` | Umsetzung | autonom | fertig |
| K2.1b | `K2.1b-minibosse-flaechen.md` | Umsetzung | autonom | fertig |
| K2.1c | `K2.1c-endboss.md` | Umsetzung | autonom | fertig |
| K2.2a | `K2.2a-siegvarianten.md` | Umsetzung | autonom | fertig |
| K2.2b | `K2.2b-niederlage-modi.md` | Umsetzung | autonom | fertig |
| K2.3a | `K2.3a-inselwechsel.md` | Umsetzung | autonom | fertig |
| K2.3b | `K2.3b-spielstand-golden.md` | Umsetzung | autonom | fertig |
| K2.4 | `K2.4-review.md` | Review | autonom | fertig |

## Abnahme

2026-10-09, Review K2.4: AC-01 bis AC-08 mit Nachweis in K2.1a–K2.3b › Ergebnis; `task check` und `task check:go` grün.
Keine schweren Befunde; leicht: `GateOpen` nicht im Spielstand (Ereignis nach Laden erneut, ohne Wirkung).
Offen außerhalb SIM: B-344, B-345 (SRV), Anzeige über K4/K5. Neues Ticket: B-372 (`check_run` Timeout).
Version: v0.x Minor vorgeschlagen (Bosse, Siege, Niederlage-Modi und Inselwechsel wirken im Spiel).
