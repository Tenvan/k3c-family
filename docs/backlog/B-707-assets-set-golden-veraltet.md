# B-707 · assets_set und assets_render lassen golden.json der Gruppe veraltet

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

Gefunden in MG1.1: `assets_set monarch {beschreibung: …}` rendert und schreibt Spec und `manifest.json` (neuer `specHash`/`renderKey`), aber `public/grafik/figuren/golden.json` behält den alten `renderKey`. Erst `task gen:paper` zieht ihn nach; bis dahin schlägt `tools/paper/golden/public.test.ts` fehl.

## Ziel

Nach `assets_set`, `assets_create`, `assets_delete` und `assets_render` passt `golden.json` der Gruppe zur Ausgabe.

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

- **AC-01** Test in `tools/k3c-workbench`: nach `assets_set` mit geänderter Beschreibung stimmt `golden.json[id].renderKey` mit `manifest.json` überein.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Workaround bis zur Behebung: nach jeder Asset-Änderung `task gen:paper` (MG-Sprints).
