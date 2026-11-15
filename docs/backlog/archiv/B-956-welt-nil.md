# B-956 · Welt Nil: Ägypten mit Oasen-Inseln, Nilbarke und Sandstürmen

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

Welt `nil`: Ägypten am Nil.
- **Inseln/Biome:** Oasen am Fluss (forest mit Palmen), Wüstenrand (mine als Sandstein), Grabkammern (cave) als Tiefe; 3 Inseln, Wechsel per Nilbarke (Schiff).
- **Gegner/Portale:** Portale als Grabeingänge; Sandsturm als Event (Sicht und Tempo sinken), Skorpione/Mumien als Gegner-Varianten.
- **Angriffsmodus:** `groups`.
- **Besonderheiten:** Gold eher aus Handel/Händler, weniger Holz (Overrides auf Holz-Gold aus B-952), Bauten aus Stein.
- **Editor-Voraussetzungen (B-954):** `world_create`, Biome/Tiefen, Overrides (Natur, Wirtschaft, Events); Assets Palme, Barke, Obelisk über `assets_*`; ggf. neues Event „Sandsturm“ (SIM-Ticket).

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

Sandsturm als neues Event: eigenes SIM-Ticket oder mit vorhandenen Events (Nebel/Nacht) nähern? Default: zuerst nähern. (🧑)

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
