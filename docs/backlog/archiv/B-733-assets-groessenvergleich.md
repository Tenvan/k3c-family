# B-733 · Katalog und Review zeigen ein Asset im Größenvergleich mit dem Monarch vor einem Biom

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** WZ43
- **Projekt:** WZG
- **Erstellt:** 2026-11-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

Katalog (`tools/k3c-workbench/frontend/src/assets/catalog/`) und Review (`.../assets/review/`) zeigen ein Asset nur für sich: im Katalog doppelt vergrößert, im Review mit fester Höhe. Wie groß es im Spiel neben dem Monarch wirkt, sieht man nicht. Wunsch 🧑 2026-11-07 (aus B-732).

## Ziel

Beim Ansehen eines Assets sofort sehen, ob es im Spiel die richtige Größe hat.

## Beteiligte und Zielgruppen

🧑 und Claude-Sessions, die Assets im Katalog und Review beurteilen.

## Anforderungen

- Ansicht „Größenvergleich“ in Katalog und Review: ein Rahmen in der Höhe der Spielszene, mit Maßstab in der Höhe und in der Breite (Units, 1 Unit = 32 px).
- Das Asset steht in Spielgröße auf dem Boden, der Monarch steht rechts daneben, ihm zugewandt.
- Hintergrund: Biom auswählbar (Biome aus `data/biomes/`, Hintergrund wie im Spiel).
- Im Review Vorher und Nachher je im Größenvergleich oder umschaltbar (Entscheidung offen).

## Nicht-Ziele

Ändern von Größen aus der Ansicht (nur Ansehen, Entscheidung 005).

## Regeln und Einschränkungen

Domäne DEV. UI nur über `@webkit`, Datei ≤ 400, Funktion ≤ 60 Zeilen. Größen aus den Daten (Spec, `data/paper.json`), nicht im Code geraten. Umgesetzt mit #514, Beleg in WZ43.3.

- **Bereich:** `tools/k3c-workbench/frontend/src/assets/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Goblin-Anführer gewählt, Biom Wald → Szene in Spielhöhe, Goblin links am Boden, Monarch rechts zu ihm gedreht, Maßstab zeigt Höhe in Units.

## Ausnahme- und Fehlerfälle

Asset ohne Bodenbezug (Hintergrund, Requisite ohne Pivot): Hinweis statt Vergleich. Biom ohne Hintergrund: neutraler Boden.

## Akzeptanzkriterien

- **AC-01** Katalog: Größenvergleich mit Rahmen, Maßstab hoch und breit, Monarch rechts zugewandt.
- **AC-02** Biom-Auswahl wechselt den Hintergrund.
- **AC-03** Review zeigt den Größenvergleich für Vorher und Nachher.

## Offene Fragen

Spielgröße je Asset und Darstellung im Review: Default (Claude, 2026-11-12, Meldung im Nachgang) wie mit #514 umgesetzt; 🧑 ändert bei Bedarf.

## Notizen

Kontext: B-732 (Katalog-Cache und größere Vorschau).
