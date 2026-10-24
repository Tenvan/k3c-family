# B-588 · Abbau-Schilder zeigen eine Axt für Bäume und eine Spitzhacke für Felsen

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** GEB16
- **Projekt:** GEB
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Bauschilder (B-584) gibt es nur an Bauplätzen. Ralf (2026-10-24): Für den Abbau schön wären eine Axt und eine Spitzhacke als Symbol.

## Ziel

Ein Abbau-Schild zeigt eine Axt (Baum) bzw. eine Spitzhacke (Fels, Erz).

## Beteiligte und Zielgruppen

Spielende; 🧑 entscheidet über Aussehen und Ort; Grafik über das Asset-Studio.

## Anforderungen

- Zwei Symbole als Pixel-Grafik: Axt, Spitzhacke.
- Zeichnung als Abbau-Variante des Bauschilds.

## Nicht-Ziele

Neue Abbau-Regeln; Wimpel der Markierung (GEB10).

## Regeln und Einschränkungen

Nur zeichnen (ADR 001); Grafik über das Studio; Client zeigt nur, was der Server meldet (`marked`, B-587-Prinzip).

## Beispiele

Markierter Baum → Axt-Schild am Baum.

## Ausnahme- und Fehlerfälle

Grafik fehlt → kein Symbol (nie ein falsches).

## Akzeptanzkriterien

- **AC-01** Axt und Spitzhacke liegen als Grafik im Katalog und in der Studio-Vorschau.
- **AC-02** Das Schild erscheint am vorgesehenen Ort (Beobachtung am Gerät).

## Offene Fragen

Wo genau steht das Schild: an jedem markierten Baum/Fels (Default, ersetzt oder ergänzt den Wimpel) oder an jedem abbaubaren? Bei 14–20 Bäumen je Abschnitt wäre „jeder abbaubare“ zu voll; 🧑 entscheidet bei der Spec.

## Notizen

Aus der Abnahme am 2026-10-24. Spec: Entwurf.
