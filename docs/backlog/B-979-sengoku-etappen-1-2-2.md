# B-979 · Sengoku nutzt die Etappen 1, 2, 2: Insel 3 bekommt einen Hafen

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** WE5
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sengoku (`data/worlds/sengoku.json`, gitignored, Kopie im Spielpaket `dist/k3c-game/data/worlds/`) hat drei Inseln ohne Feld `etappe`, also die Etappen 1, 2, 3 (Listenplatz + 1). Etappen und Zielwahl am Hafen sind seit WE4 (B-961, B-962) in Engine und Spiel. Insel 3 (Bergfestung, Tiefen 0, 3, 4) plant in Tiefe 0 keinen Hafen-Chunk; die Prüfung in `engine/sim/island_worlds.go` (`hasHarbor`, `soleOfLastEtappe`) verlangt ihn für jede Insel außer der einzigen der letzten Etappe.

## Ziel

Sengoku zeigt die freie Zielwahl: Nach Insel 1 (Etappe 1) wählen die Spieler am Hafen zwischen Insel 2 und Insel 3 (beide Etappe 2).

## Beteiligte und Zielgruppen

Spieler (Couch-Koop, 2 Spieler) wählen das Ziel auf der Seekarte. Claude ändert die Welt über die Welten-MCP-Tools. 🧑 gibt die Spec frei.

## Anforderungen

- Insel 3 von Sengoku plant in Tiefe 0 einen Hafen-Chunk (`kind: harbor`), gesetzt mit `world_chunks_set`.
- Die Inseln tragen `etappe` 1, 2, 2, gesetzt mit `world_set`.
- `world_check sengoku` meldet keinen Fehler; die Welt lädt im Spiel.
- Geändert wird nur über `world_*`; die Kopie im Spielpaket entsteht über den Build (`task build`), nicht von Hand.
- Mit 2 Spielern gilt die Zielwahl wie in B-961 (keine neue Regel).

## Nicht-Ziele

Neue Chunks oder Biome für die Bergfestung, Etappen für Nil und Britannien, Änderungen an Engine oder Workbench (Fehler dort → eigenes Ticket).

## Regeln und Einschränkungen

Weltdateien außer `standard.json` sind gitignored und werden nur über `world_*` geändert. Keine Simulation, kein Balancing.

- **Bereich:** `data/worlds/sengoku.json`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

Insel 1 abgeschlossen (Endboss besiegt, Schiff fertig) → die Seekarte bietet Insel 2 und Insel 3 an. Beide abgeschlossen → Kampagnenende wie bisher nach der letzten Etappe.

## Ausnahme- und Fehlerfälle

Hafen-Chunk passt nicht in den Plan der Tiefe 0 (Mauerlinien, Chunk-Anzahl) → `world_chunks_set` lehnt ab; dann `chunkCounts` der Tiefe 0 um 1 erhöhen und erneut setzen. `etappe` vor dem Hafen gesetzt → Prüfung lehnt ab (Reihenfolge: erst Hafen, dann Etappen).

## Akzeptanzkriterien

- **AC-01** `world_get sengoku` zeigt die Etappen 1, 2, 2 und einen Hafen-Chunk in Tiefe 0 von Insel 3.
- **AC-02** `world_check sengoku` ohne Fehler.
- **AC-03** Im Spiel (Sichtprüfung 🧑, Teil der Abnahme von B-996): Nach Insel 1 stehen zwei Ziele auf der Seekarte.

## Offene Fragen

- Lage des Hafen-Chunks auf Insel 3 → Default: derselbe Platz wie auf Insel 1 und 2 von Sengoku (vorläufig, 🧑 bestätigt).
- Spielpaket-Kopie → Default: entsteht beim nächsten `task build`, kein eigener Schritt (vorläufig, 🧑 bestätigt).

## Notizen

Teil von B-959/B-960-Umfeld, Auftrag 🧑 2026-11-15.
