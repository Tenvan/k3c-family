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
- **Revision:** 2
- **Freigabe:** –

## Ausgangslage

Bei der Sichtung von W6.2 (2026-10-11, 🧑) stehen die Bauplätze im Hub zu eng zusammen, besonders die Treppen. In `data/hub.json › sites` liegen Treppe hoch (+16), Kaserne (+20), Treppe runter (+24) und Schmiede (+28) je nur 4 Units auseinander, das Mindestmaß der Zahlziele (≥ 4 Units, Glossar › Zahlziel) ist also gerade eingehalten, Preisschilder und Symbole überlappen aber.

## Ziel

Die Treppen haben so viel Abstand, dass Preisschild, Wartegrund-Symbol und Text (W6.2) sich nicht überlappen und jeder Platz einzeln bezahlbar bleibt: ≥ 8 Units zu jedem anderen Zahlziel derselben Tiefe (W11.1, umgesetzt). Der Hub bleibt ±50 Units breit. Gebäude, die sich nach den Maßen aus GEB (6 Kacheln) überdecken, obwohl ihre Zahlziele 4 Units auseinander stehen, sind der Rest dieses Problems und stehen in B-446.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Treppe hoch (ab Tiefe 1) und Treppe runter haben mindestens 8 Units zu jedem anderen Zahlziel derselben Tiefe (Burg, Händler, Plätze, Angebote, Linien), bei voller Streuung der Linien.
- Treppe hoch und Händler-Kaufen (nur Tiefe 0) teilen sich ein Raster (+16); die Linien bleiben (Kristallhöhle: Portale ab 135, Camps ohne freien Chunk).
- Deterministisch; Golden und Spielstand-Fixtures (`testdata/saves/v*`) auf die neuen Offsets, keine Migration alter Stände (Beschluss 🧑).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Go-Test: Jede Treppe liegt mindestens 8 Units von jedem anderen Zahlziel derselben Tiefe entfernt, bei voller Streuung (`TestSiteDataTreppenAbstand`, `TestSiteDataAbstandBeiVollerStreuung` in `engine/sim/site_data_test.go`); Golden grün (W11.1, fertig).

## Offene Fragen

Geklärt mit 🧑 (2026-10-14): Treppen ≥ 8 Units, gemessen von Zahlziel zu Zahlziel; die Breite der Treppen-Grafik (GEB5-Annahme: 4 × 4 und 4 × 3 Kacheln) füllt den Abstand nicht aus (kleinster Rand 2 Units zur Burg), die Offsets aus W11.1 bleiben also gültig. Offen (2026-10-16): Gebäude-Überdeckung Kaserne ↔ Schmiede (2 Units) → B-446.

## Notizen

Revision 2 (2026-10-16, Entwurf): Ziel und Kriterium auf den umgesetzten Teil (Treppen) geschärft, Platzhalter-Kriterium ersetzt; Rest nach B-446 ausgelagert. Die frühere Abhängigkeit von der Gebäude-Grafik (GR3) gilt jetzt GEB5 (Maße der übrigen Gebäude auf `origin/develop`; die Treppen-Maße 4 × 4 und 4 × 3 sind GEB5-Annahmen, GEB5.3 offen).
