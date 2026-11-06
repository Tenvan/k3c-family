# B-705 · Das Vorher-Bild im Review rendert mit dem Stil der Basis

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Paper-CLI liest die Stil-Datei immer aus der Repo-Wurzel (`tools/paper/render/stil.ts`, `STIL_PFAD`). Das Review rendert das Vorher-Bild aus einem Schnappschuss-Baum der Basis, aber mit dem aktuellen Stil. Ein reiner Stilwechsel zeigt deshalb keinen Pixel-Unterschied (D-RV1-3, gemessen in RV1).

## Ziel

Vorher-Bilder im Review entstehen mit dem Stil der Basis.

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

- **AC-01** `paper frame` und `paper render` nehmen `--stil <datei>`; ohne Option gilt `public/grafik/stil.paper.json`.
- **AC-02** Das Review übergibt beim Vorher-Bild die Stil-Datei des Schnappschusses; ein Test mit reinem Stilwechsel zeigt `pixelAnteil > 0`.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
