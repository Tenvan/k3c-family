# B-636 · Die Farm heißt Baumschule und steht auf gerodeten Waldbauplätzen

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** BS4
- **Projekt:** BAS
- **Erstellt:** 2026-10-26
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-26, Chat, durch 🧑 mit BS2

## Ausgangslage

Die Farm (`data/buildings.json` › `farm`, Plantage `engine/sim/building_plantation.go`) steht auf zwei festen Farm-Weltplätzen bei ±52 (`data/hub.json` › `sites`, Hub-Stufe 1) und lässt dort Bäume nachwachsen. Im Client heißt sie Farm (`src/scenes/render/plantation.ts`, `buildingSprites.ts`), im Glossar „Farm“ und „Farm-Weltplatz“.

## Ziel

Wie in Kingdom Two Crowns: Die Baumschule entsteht an festen Stellen im **Wald**, sobald ihr Bauplatz von Bäumen befreit ist; erst dann kann man sie kaufen. Der Name Farm verschwindet aus Regeln, Daten, Code und Oberfläche.

## Beteiligte und Zielgruppen

Spieler (Holz-Nachschub als Belohnung fürs Roden), Claude (Regeln, Sim, Client), 🧑 entscheidet Platzierung.

## Anforderungen

- Begriff **Baumschule** im Glossar (ersetzt Farm, Farm-Weltplatz → Baumschul-Platz); Regel in `docs/rules/materialien-gebaeude.md`.
- Daten-ID `nursery` statt `farm` (Gebäude, Asset-Slot, Hub-Sites entfallen bzw. wandern in den Level-Generator).
- Der Level-Generator legt Baumschul-Plätze in Wald-Chunks des Wald-Bioms an (deterministisch, `engine/rng`); auf dem Platz stehen Bäume.
- Der Platz wird erst kaufbar, wenn alle Bäume im Platz-Streifen gefällt sind (Anzeige „erst roden“).
- Plantage-Wirkung (nachwachsende Bäume) bleibt; Werte auf der neuen Münzskala (B-635).
- Funktioniert mit 2 Spielern, Stufen und Inseln wie bisher.

## Nicht-Ziele

Neue Grafik (Projekt GEB, Ticket bei Bedarf); andere Biome als Wald.

## Regeln und Einschränkungen

Kette REG → SIM → CLI; Protokoll-Änderung (Gebäudeart, Platz-Zustand) in eigener Session. Keine Rücksicht auf alte Spielstände.

## Beispiele

Wald-Chunk mit Baumschul-Platz, 4 Bäume darauf → Bauschild grau „erst roden“; nach dem 4. Baum → Bauschild aktiv, Kauf möglich.

## Ausnahme- und Fehlerfälle

Kein Wald-Chunk im Level → Generator erzwingt mindestens einen Baumschul-Platz je Seite (Default) oder keinen (offene Frage). Nachwachsende Plantage-Bäume sperren den Platz nicht wieder.

## Akzeptanzkriterien

- **AC-01** Glossar und Regeln nennen Baumschule und Baumschul-Platz, „Farm“ kommt in `docs/rules` nicht mehr vor.
- **AC-02** `grep -ri farm data engine src` findet keine Spiel-Bezeichner mehr (Ausnahmen begründet).
- **AC-03** Generator-Test: Wald-Level hat Baumschul-Plätze mit Bäumen, deterministisch je Seed.
- **AC-04** Sim-Test: Kauf vor dem Roden abgelehnt, nach dem Roden möglich; Plantage wächst.
- **AC-05** Client zeigt Baumschule und den Hinweis „erst roden“.

## Offene Fragen

- Wie viele Baumschul-Plätze je Wald-Level und wo (Vorschlag: je Seite 1–2 im Abstand ≥ Linie 2)? → 🧑, Default: je Seite 1, gewürfelt in Wald-Chunks hinter Linie 2.
- Gibt es weiter Baumschulen auf tieferen Stufen (Höhle, Mine)? Default: nein, nur Wald.

## Notizen

Wunsch 🧑 2026-10-26.
**2026-10-29 (Claude, BS8.1 vorgezogen):** Bots roden jetzt auch Baumschul-Plätze (`bot_clear.go`: `blockedSite` + `rank`, Reihenfolge Mauer, Turm, Baumschule; Test `TestBotRodetBaumschule`); gekauft wurde sie schon über `buildOrder`. Messung `task balance:milestone` (coop2, standard, k3c-dev, 40 Tage): vorher Stufe 4 Tag 24 (Ratten-Königin Tag 23), nachher Hub 2 schon Tag 6, Burg fällt nicht mehr früh, aber Ratten-Königin erst Tag 33, Stufe 4 Tag 35, Lava-Golem nein. Ergebnis gemischt, ein Seed; die Delta-Runde `holz` gegen `M1` (BS8.2) und mehr Seeds stehen aus.
