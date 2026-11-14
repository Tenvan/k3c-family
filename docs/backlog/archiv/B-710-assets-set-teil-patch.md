# B-710 · assets_set kann einzelne Teile nicht ändern, ohne die ganze Teile-Liste zu senden

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** WZ43
- **Projekt:** WZG
- **Erstellt:** 2026-11-06
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

MG1.2: Die Nase des Monarchen (`kopf`, Lage `punkt`) und die Kronen-Füllung nutzen `koenigsrot` wie der Umhang. Um die Nase auf `haut` umzustellen, müsste `assets_set` per JSON-Merge-Patch die ganze Liste `teile` (rund 780 Zeilen) neu senden, weil Listen ersetzt werden (RFC 7396). Für MG2–MG8 ist das die häufigste Art von Änderung.

## Ziel

`assets_set` ändert ein Teil gezielt, z. B. `teile` als Objekt nach `id` (`{teile: {kopf: {lagen: [...]}}}`) oder ein eigener Pfad `teile[kopf].lagen[3]`.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `assets_set` nimmt `teile` auch als Objekt nach `id` an und ändert nur die genannten Teile; Reihenfolge und übrige Teile bleiben.
- Unbekannte `id` wird mit Feld und gültigen IDs abgelehnt; die Liste als Ganzes bleibt erlaubt.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Domäne DEV (Werkzeug), Ticket-Projekt PAP; Klasse A (Problem). Vertrag `docs/standards/paper-mcp.md` nachziehen.

- **Bereich:** `tools/k3c-workbench/internal/paperassets/**`, `tools/k3c-workbench/internal/mcpsrv/tools_assets*.go`, `docs/standards/paper-mcp.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Test in `tools/k3c-workbench`: Patch nur für `kopf` ändert genau dieses Teil, Reihenfolge und andere Teile bleiben.

## Offene Fragen

Form des Patches: Default (Claude, 2026-11-12) Objekt `teile` nach `id`.

## Notizen

Folge-Änderung danach: Nase des Monarchen auf `haut`/`schatten`, damit Varianten den Umhang tauschen können (D-MG1-03).

Verwandt (Schema, Lane renderer): `tausch` verlangt mindestens einen Eintrag, daher ist `spieler1` ein Tausch auf sich selbst und die fünf `@spieler1`-PNGs sind byte-gleiche Kopien der Basis (rund 1,2 MB doppelt); ein leerer Tausch oder ein Alias würde das vermeiden.
