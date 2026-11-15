# B-958 · Welt Polar 1900: Expedition auf Eisschollen mit knappem Holz

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** verworfen
- **Sprint:** WE3
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Weltvorschlag 🧑 2026-11-15 aus dem Test des Welten-Editors.

## Ziel

Welt `polar`: Polarexpedition um 1900.
- **Inseln/Biome:** Küsteneis (crystal), Packeis-Schollen (crystal), Gletscherhöhle (cave); Wechsel per Expeditionsschiff.
- **Gegner/Portale:** Portale als Eisspalten; Eisbären und Schneestürme (Event) als Gefahr.
- **Angriffsmodus:** `general` (Expeditionsleiter mit Banner).
- **Besonderheiten:** Holz knapp (wenig Bäume, Holz-Gold seltener), Truhen als Vorratslager häufiger, lange Nächte.
- **Editor-Voraussetzungen (B-954):** `world_create`, Biome/Tiefen, Overrides (Natur-Dichte niedrig, Tag/Nacht-Länge, Truhen); Assets Expeditionszelt, Schiff im Eis über `assets_*`.

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
- **Größe:** S
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

Verworfen 🧑 2026-11-15: Das Spiel hat noch keine Winterzeit; bei Bedarf nach einem Winter-Ticket neu planen.
