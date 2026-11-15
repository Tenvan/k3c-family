# B-952 · Holzfällen bringt 1 bis 3 Gold und ist am Anfang die Haupteinnahme

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WI1
- **Projekt:** WIR
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Hinweis 🧑 2026-11-15: Holzfällen bringt kein Gold. In K2C ist es am Anfang die Haupteinnahme; der Gold-Drop der Gegner soll nur Zusatz sein (Belohnung, Noteinkunft).

## Ziel

Jeder gefällte Baum wirft 1 bis 3 Goldmünzen ab (Zufall über `engine/rng`, Werte in `data/economy.json`); Gegner-Drop wird reduziert.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `engine/sim/**`, `data/**`, `docs/rules/**`, `testdata/golden/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** golden
- Balancing-Läufe gesperrt: Werte vorläufig (`provisional`).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Baum fällen erzeugt 1–3 Münzen am Boden, deterministisch (Test).
- **AC-02** Gegner-Drop gesenkt, Werte in `data/`, Begründung in `docs/rules/wirtschaft.md`.

## Offene Fragen

Um wie viel sinkt der Gegner-Drop? Default: halbiert. (🧑)

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
