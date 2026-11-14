# B-885 · Nativer Rig-Export der Bosse ist nach MG10 veraltet

- **Domäne:** PAP
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** MG10
- **Projekt:** PAP
- **Erstellt:** 2026-11-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

MG10 (B-720) hat die Boss-Specs unter `public/grafik/bosse/` neu gebaut (version 3, neue Teile wie `keule`, `hackmesser`, `zepter`, `klinge`, `kristalle`, eigene Pivots). Der eingecheckte native Rig-Export `native/k3c-rig/assets/bosse/*` (und `assets/soll/bosse/**`) steht noch auf version 1 mit Schwert und Schild; `task native:rig:export:check` weicht vermutlich ab (ungeprüft, nicht in CI).

## Ziel

Der native Rig-Export der Bosse passt wieder zu den Specs.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

`task native:rig:export` einmal laufen lassen und das Ergebnis prüfen (Gen-Lauf, daher eigenes Ticket statt MG10).

## Nicht-Ziele

Änderungen an den Specs selbst (B-720).

## Regeln und Einschränkungen

Gen nur am Feature bzw. im Abnahme-Ticket (B-742).

- **Bereich:** `native/k3c-rig/assets/**`
- **Abhängig von:** B-720
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `task native:rig:export:check` grün mit den Boss-Specs version 3.

## Offene Fragen

keine

## Notizen

Gefunden im Review MG10.4 (2026-11-14).
