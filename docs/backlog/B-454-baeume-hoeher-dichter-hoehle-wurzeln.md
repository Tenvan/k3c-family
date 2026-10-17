# B-454 · Bäume höher und dichter, Höhle mit eigener Unterwelt-Flora (Wurzeln)

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** GEB
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ralf hat GEB6 in der Vorschau gesehen (2026-10-17): Die Bäume (1 × 2,25 Kacheln) sind zu niedrig und stehen nicht dicht genug; in Höhlen passen Wald-Bäume und -Büsche nicht ins Unterirdische. Quellen: `public/grafik/gebaeude/` (`tanne*`, `pappel*`, `birke*`, `busch`), Look-Wahl in `src/scenes/` (Stufe 2 = Höhle).

## Ziel

Bäume sind etwa 3/4 der Szenenhöhe hoch und stehen dichter. Die Höhle zeigt eigene Pflanzen statt Baum-Umfärbung: Wurzeln an Wand und Boden, passende Büsche. Danach Vorschau und Review durch 🧑.

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

- **AC-01** Baumhöhe ca. 3/4 der Szenenhöhe; Dichte höher als heute (Maß und Dichte als Entscheidung in der Spec, Regeldatei § 1 nachgezogen).
- **AC-02** Höhlen-Flora (Wurzeln an Wand und Boden, Büsche) als Daten-JSON und PNG, im Spiel nur in der Höhle (Mine nach Absprache).
- **AC-03** 🧑 nimmt Vorschau und Spiel ab (Review-Session mit Vorschau).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
