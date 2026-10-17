# B-458 · Hub wächst in Ausbaustufen sichtbar und kleinteilig (Lagerfeuer bis Burg, K2C-Stil)

- **Domäne:** DEV
- **Typ:** Idee
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

Der Hub hat heute fünf Stufen (GEB4: Burg und Hub-Stufen 1–5, Tor). Ralf (2026-10-17): Er soll sichtbar kleinteiliger wachsen wie in Kingdom Two Crowns: erst ein kleines Lagerfeuer mit Grill, dann ein Haus, usw. Mehr Stufen je Level sind möglich; auf den Stufen werden jeweils Gebäude und Schilde freigeschaltet.

## Ziel

Der Hub wächst in mehr, kleineren Ausbaustufen von einem Lagerfeuer mit Grill über ein Haus bis zur Burg; je Stufe sind Gebäude und Schilde freigeschaltet. Sichtbar in einer Vorschau-Reihe, danach Review durch 🧑.

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

- **AC-01** Stufenfolge und Freischaltungen (Gebäude, Schilde je Stufe) sind als Entscheidung von 🧑 in der Spec festgelegt; Stufenzahl je Level offen.
- **AC-02** Hub-Grafik je Stufe als Daten-JSON und PNG nach `docs/standards/gebaeude-grafik.md` § 6; Vorschau zeigt die Wachstumsreihe.
- **AC-03** Review-Session: 🧑 nimmt die Wachstumsreihe in der Vorschau ab (Preview- und Review-Aufgabe).
- **AC-04** Regel- und Server-Anteil (Freischaltung je Stufe) als eigenes Ticket SIM/REG, falls `data/` oder `engine/` betroffen.

## Offene Fragen

Anzahl Stufen je Level, Reihenfolge der freigeschalteten Gebäude, Schilde; Abgrenzung zu GEB4. Entscheidet 🧑.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
