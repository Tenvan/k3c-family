# B-955 · Welt Sengoku: Japan der Kriegerzeit mit Burgen, Reisfeldern und Ninja-Überfällen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
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

Welt `sengoku`: Japan der Kriegerzeit.
- **Inseln/Biome:** Reisfeld-Ebene (forest), Bambuswälder (forest, dichte Natur), Bergfestung (ironhold) als letzte Insel; 3 Inseln, Tiefen 0–2.
- **Gegner/Portale:** Portale als Torii-Schreine; Ninja-Überfälle als Nacht-Event (schnelle Diebe), Samurai-Wachen als Lager.
- **Angriffsmodus:** `general` (Daimyo mit Banner).
- **Besonderheiten:** Burgausbau stark (Hub-Stufen früher), Reis als Holz-Ersatz kosmetisch.
- **Editor-Voraussetzungen (B-954):** `world_create`, Inseln/Tiefen/Biome setzen, `combat.mode`, Overrides (Natur-Dichte, Event-Gewichte); neue Assets (Torii, Pagode) über `assets_*`.

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
