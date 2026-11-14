# B-711 · Variante des Reiters tauscht die Materialien des Reittiers nicht

- **Domäne:** PAP
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** MG10
- **Projekt:** PAP
- **Erstellt:** 2026-11-06
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: alle offenen angehen)

## Ausgangslage

MG1.2: Die Varianten `spieler2`–`spieler4` des Monarchen tauschen `koenigsblau`, die Satteldecke des Pferds (`reittiere/pferd`, Teil `sattel`, `koenigsblau`) bleibt in Reit-Animationen aber blau, weil der Tausch nur auf die Spec des Reiters wirkt (`tools/paper/rig/reiten.ts`).

## Ziel

Entscheiden, ob der Tausch einer Reiter-Variante auch auf das Reittier wirkt (Satteldecke in Spielerfarbe) oder ob das Reittier eigene Varianten bekommt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Assets nur über MCP `assets_*` (Entscheidung 005).

- **Bereich:** `public/grafik/figuren/**`, `public/grafik/reittiere/**`, `tools/paper/**`, `docs/standards/paper-spec.md`
- **Abhängig von:** B-720
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Entscheidung in `paper-spec.md` › Varianten; bei „wirkt mit“: `galopp@spieler2.png` zeigt die Satteldecke in `smaragd`.

## Offene Fragen

keine. Entschieden (Default, autonom Claude, 2026-11-14): Der Varianten-Tausch wirkt beim Reittier mit (einfach); Reittier-Varianten (MG2) bleiben ungenutzt.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
