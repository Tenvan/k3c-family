# B-287 · Hub-Stufe 4 und 5 sind mit dem Beutel-Maximum bezahlbar

- **Domäne:** REG
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RG1
- **Projekt:** –
- **Erstellt:** 2026-10-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Beutel fasst höchstens 100 Gold (`data/economy.json` › `purse.maxGold`). Gezahltes Gold bleibt nur, solange die Bezahl-Taste gehalten wird; wer loslässt, bekommt seinen offenen Betrag zurück (`refundPending`, `engine/sim/economy.go`). Der Hub-Ausbau kostet laut `data/hub.json` › `levels` 200 Gold (Stufe 4) und 400 Gold (Stufe 5). Zwei Spieler bringen zusammen höchstens 200 Gold auf einmal: Stufe 4 nur mit vollen Beuteln, Stufe 5 erst mit vier Spielern (gefunden in W1.1).

## Ziel

Jede Hub-Stufe ist mit zwei Spielern erreichbar.

## Beteiligte und Zielgruppen

Spieler (2 Monarchen); 🧑 entscheidet den Weg, REG pflegt die Werte.

## Anforderungen

- Hub-Stufe 4 und 5 sind mit zwei Spielern bezahlbar (Weg offen, siehe Offene Fragen).

## Nicht-Ziele

Feintuning aller Gebäudekosten (B-015).

## Regeln und Einschränkungen

Werte nur in `data/`; ändert sich die Mechanik (Teilzahlung bleibt stehen), ist es ein SIM-Ticket.

## Beispiele

Zwei Spieler mit je 100 Gold stehen an der Burg, Hub-Stufe 4 → Ausbau auf 5 lässt sich bezahlen.

## Ausnahme- und Fehlerfälle

nicht relevant (reine Wert- bzw. Regelfrage).

## Akzeptanzkriterien

- **AC-01** Test: Zwei Spieler mit vollem Beutel bezahlen den Ausbau auf jede Hub-Stufe.

## Offene Fragen

Weg (🧑): Kosten für Stufe 4/5 senken, Beutel-Maximum erhöhen oder gezahltes Gold am Ausbau stehen lassen (Teilzahlung über mehrere Besuche, SIM).

## Notizen

Werte aus `docs/rules/materialien-gebaeude.md` § 2 (Q44).
