# B-409 · Bauplätze im Hub stehen zu eng, besonders die Treppen

- **Domäne:** REG
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** W11
- **Projekt:** WRT
- **Erstellt:** 2026-10-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Bei der Sichtung von W6.2 (2026-10-11, 🧑) stehen die Bauplätze im Hub zu eng zusammen, besonders die Treppen. In `data/hub.json › sites` liegen Treppe hoch (+16), Kaserne (+20), Treppe runter (+24) und Schmiede (+28) je nur 4 Units auseinander, das Mindestmaß der Zahlziele (≥ 4 Units, Glossar › Zahlziel) ist also gerade eingehalten, Preisschilder und Symbole überlappen aber.

## Ziel

Bauplätze, besonders die Treppen, haben so viel Abstand, dass Preisschild, Wartegrund-Symbol und Text (W6.2) sich nicht überlappen und jeder Platz einzeln bezahlbar bleibt. Der Hub bleibt ±50 Units breit (oder die Breite wird bewusst geändert).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

- Abstand und Offsets hängen von der Breite der Gebäude-Grafiken ab (Beschluss 🧑 2026-10-11): Erst wenn die Gebäude-Grafiken (GR3) feststehen, wird der neue Mindestabstand der Zahlziele (heute 4 Units) festgelegt. Neue Offsets in `data/hub.json`, Regel in `docs/rules/materialien-gebaeude.md § 3`, Golden-Update dann mit der Grafik-Anbindung planen.
- Reicht der Hub (±50) dafür, oder wandern Plätze auf die Linien außerhalb? Offen bis zur Grafik.

## Notizen

Abhängigkeit: Gebäude-Grafik (Projekt GRA, Sprint GR3). Nicht vor der Grafik-Anbindung einplanen; dann Breite je Gebäude messen und die Offsets daraus ableiten.
