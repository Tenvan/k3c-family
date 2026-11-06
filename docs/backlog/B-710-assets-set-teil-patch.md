# B-710 · assets_set kann einzelne Teile nicht ändern, ohne die ganze Teile-Liste zu senden

- **Domäne:** PAP
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

MG1.2: Die Nase des Monarchen (`kopf`, Lage `punkt`) und die Kronen-Füllung nutzen `koenigsrot` wie der Umhang. Um die Nase auf `haut` umzustellen, müsste `assets_set` per JSON-Merge-Patch die ganze Liste `teile` (rund 600 Zeilen) neu senden, weil Listen ersetzt werden (RFC 7396). Für MG2–MG8 ist das die häufigste Art von Änderung.

## Ziel

`assets_set` ändert ein Teil gezielt, z. B. `teile` als Objekt nach `id` (`{teile: {kopf: {lagen: [...]}}}`) oder ein eigener Pfad `teile[kopf].lagen[3]`.

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

- **AC-01** Test in `tools/k3c-workbench`: Patch nur für `kopf` ändert genau dieses Teil, Reihenfolge und andere Teile bleiben.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Folge-Änderung danach: Nase des Monarchen auf `haut`/`schatten`, damit Varianten den Umhang tauschen können (D-MG1-03).
