# B-584 · Bauplätze zeigen kleine Bauschilder statt grauer Rahmen

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** GEB16
- **Projekt:** GRA
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Unbezahlte Bauplätze zeigen einen grauen Rahmen (`siteView.ts`, Umriss). Ralf (2026-10-24): hässlich.

## Ziel

Jeder Bauplatz zeigt ein kleines Bauschild (Brett am Stock) ohne Text, in Varianten.

## Beteiligte und Zielgruppen

Spielende; 🧑 entscheidet über das Aussehen.

## Anforderungen

- Bauschild statt Rahmen an Bauplätzen, ohne Text.
- Drei Varianten: Verteidigung (Mauer, Turm, Tor), Siedlung (übrige Gebäude), Treppen.
- Stock halb so lang, doppelt so dick (Ralf, 2026-10-24).
- Das Schild steht nur an Bauplätzen, die jetzt gebaut werden dürfen (`buildableRules.ts`, Spiegel von `sitePayable`/`lineOpen`).

## Nicht-Ziele

Eigene Pixel-Grafik aus dem Studio (später möglich); Schilder für Abbau-Objekte (Bäume, Felsen) – Ralf nannte sie „evtl.“.

## Regeln und Einschränkungen

Nur zeichnen (ADR 001); Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

Unbezahlte Farm → kleines braunes Schild am Boden statt Umriss.

## Ausnahme- und Fehlerfälle

nicht relevant, nur Darstellung.

## Akzeptanzkriterien

- **AC-01** Kein grauer Rahmen mehr an Bauplätzen; Schild je Variante (Test `signVariant`).
- **AC-02** 🧑 sieht das Schild im Spiel und bestätigt oder nennt Änderungen.

## Offene Fragen

Entschieden 2026-10-24 (🧑): Der Client zeigt nur, was der Server meldet; die Client-Regel `buildableRules.ts` wird durch das Server-Feld `payable` ersetzt (B-587). Eine Abbau-Variante mit Axt und Spitzhacke kommt als B-588.

## Notizen

Auf Wunsch von Ralf sofort umgesetzt (2026-10-24): `buildSignView.ts`, `buildableRules.ts`. Spec: rückwirkend. Die Anzeige-Regel spiegelt den Server; ein Protokollfeld `payable` wäre genauer (eigenes Ticket, falls Abweichungen auffallen).
