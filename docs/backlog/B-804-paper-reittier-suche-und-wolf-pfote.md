# B-804 · tools/paper findet für das Reittier Wolf das Reittier statt des Gegners, Gegner-Wölfe haben Pfoten

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`findeReittier` in `tools/paper/rig/reiten.ts` durchsucht die Nachbar-Gruppen alphabetisch; „gegner“ kommt vor „reittiere“. Die Monarch-Spec findet für `"reittier": "wolf"` deshalb den Gegner-Wolf statt des Reittiers (belegt in NG3.1). Außerdem haben `gegner/wolf` und `gegner/alphawolf` kein `proportionen.fuss` und fallen auf `huf` zurück (Spitze 3,4), die Reittier-Wölfe haben `pfote` (2,2).

## Ziel

Das Reittier Wolf des Monarchen nutzt das Reittier-Asset, nicht den Gegner. Die Gegner-Wölfe haben Pfoten statt Hufen.

## Beteiligte und Zielgruppen

🧑 (Ralf) gibt die Spec frei und nimmt Änderungen an der Grafik ab; betroffen sind Entwicklung (`tools/paper`) und das native Spiel, das die Rigs lädt.

## Anforderungen

- Die Suche in `findeReittier` bevorzugt die Gruppe `reittiere`, danach die übrigen Nachbar-Gruppen.
- Die Specs von `gegner/wolf` und `gegner/alphawolf` setzen `proportionen.fuss` auf `pfote` (nur über `assets_set`).
- Deterministisch, kein Einfluss auf andere Reittiere.

## Nicht-Ziele

Keine neue Reittier-Grafik, keine Änderung anderer Gegner-Specs, kein Umbau der Rig-Suche über die Gruppenreihenfolge hinaus.

## Regeln und Einschränkungen

Specs nur über `assets_*` (Entscheidung 005). Sperre `golden`, weil sich Grafik-Golden ändern können; Golden nach Regel aktualisieren (`docs/arbeitsweise.md` › Sim, Balance und Gen).

Planungsangaben für die Arbeitswellen:

- **Bereich:** `tools/paper/rig/**`, `public/grafik/gegner/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** golden

## Beispiele

- Monarch-Spec mit `"reittier": "wolf"` → die Animationen `*-wolf` nutzen das Reittier `reittiere/wolf`.
- `gegner/wolf` mit `proportionen.fuss: pfote` → Fußspitze 2,2 statt 3,4.

## Ausnahme- und Fehlerfälle

- Gibt es in `reittiere` kein passendes Asset, greift die bisherige Suche in den übrigen Gruppen (kein Fehler).
- Findet sich nirgends ein Treffer, bleibt die bisherige Fehlermeldung.

## Akzeptanzkriterien

- **AC-01** Test in `tools/paper`: Die Monarch-Animationen `*-wolf` nutzen das Reittier (Gruppe `reittiere`), nicht den Gegner.
- **AC-02** `gegner/wolf` und `gegner/alphawolf` haben `proportionen.fuss: pfote`; geänderte Grafik-Golden sind nach Regel aktualisiert.

## Offene Fragen

keine

## Notizen

Gefunden in NG3.1, steht auch in B-791 › Notizen.
