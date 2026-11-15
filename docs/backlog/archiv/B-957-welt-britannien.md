# B-957 · Welt Britannien: Kelten mit Steinkreisen, Mooren und Druiden

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WE3
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Weltvorschlag 🧑 2026-11-15 aus dem Test des Welten-Editors.

## Ziel

Welt `britannien`: Kelten in Britannien.
- **Inseln/Biome:** Hochmoor (forest, feucht), Steinkreis-Hügel (forest/mine), Hügelgräber (cave); 3 Inseln.
- **Gegner/Portale:** Portale als Nebel-Steinkreise; römische Späher und Moorgeister als Gegner-Varianten.
- **Angriffsmodus:** `groups`.
- **Besonderheiten:** Heiler-Linie (Druiden) stärker gewichtet, Monolithen als Bauplatz-Dekor.
- **Editor-Voraussetzungen (B-954):** `world_create`, Biome/Tiefen, Overrides (Natur, Skill-Gewichte); Assets Steinkreis, Rundhaus über `assets_*`.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `data/worlds/**`, `public/grafik/**`, `data/paper.json`
- **Abhängig von:** B-954
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Welt per MCP angelegt und `world_check` grün.
- **AC-02** Spielbar bis Inselwechsel (Sichtprüfung 🧑).

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
