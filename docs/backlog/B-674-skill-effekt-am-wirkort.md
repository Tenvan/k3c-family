# B-674 · Flächenzauber erscheinen dort, wo die Simulation sie wirkt, nicht am Monarchen

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** KB5
- **Projekt:** KMP
- **Erstellt:** 2026-10-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Aktive Flächenzauber treffen in der Simulation am Wirkort: `spellCenter` (`engine/sim/skills_caster.go`) sucht den nächsten Gegner in Reichweite (`range` aus `data/monarch.json`: fireball 10, meteor 15, iceWall 8, lightningStorm 10) und wirkt dort im Radius. Der Client zeichnet den Skill aber immer am Monarchen: `src/scenes/effects/skillCasts.ts` erkennt das Wirken am Vergleich der Snapshots und kennt nur `p.x` des Spielers (Ring in `GROUND_Y - RING_PX`). Das Studio (B-671) zeigt den Skill deshalb ebenfalls am Monarchen. Geprüft am 2026-10-30.

## Ziel

Ein Flächenzauber ist dort zu sehen, wo er Schaden macht, und so groß wie sein Radius; Selbst- und Nahkampf-Skills bleiben am Monarchen.

## Beteiligte und Zielgruppen

- 🧑 Ralf und Familie spielen; sie müssen sehen, wo ein Zauber trifft.
- Sessions der Domänen SIM, SRV, CLI setzen um.

## Anforderungen

- Die Simulation meldet jedes Wirken eines aktiven Skills als Ereignis mit Spieler, Skill, Wirkort `x` (Units) und Radius (`GameEvent`, Protokoll laut `docs/protocol.md`, Version erhöhen).
- Ohne Ziel in Reichweite wie heute „Skill ohne Ziel“ (`castFailed`), kein Flächeneffekt.
- Der Client zeichnet den Effekt am Wirkort; der Radius des Rings folgt dem Radius des Skills (in Units × `UNIT_PX`), Farbe und Bewegung aus `skillLooks.ts`.
- Skills ohne Fläche (heal auf Ziel, shield, stealth, taunt, backstab) zeigen den Effekt am Ziel bzw. am Monarchen wie heute.
- Ton (SO8) bleibt am Wirken gekoppelt.

## Nicht-Ziele

- Neue Skill-Looks oder neue Werte.
- Status-Effekte über die Dauer (B-675).

## Regeln und Einschränkungen

- Determinismus: Ereignis aus der Simulation, kein Raten im Client.
- 2 Spieler: Ereignisse je Spieler getrennt, Split-Screen zeigt beide.
- Protokolländerung: Review vor dem Commit.

## Beispiele

- Monarch bei x 0, Gegner bei x 8, fireball (range 10, radius 3) → Ring mit 3 Units Radius bei x 8.
- meteor ohne Gegner in 15 Units → „Skill ohne Ziel“, kein Ring.

## Ausnahme- und Fehlerfälle

- Ziel stirbt im selben Takt → Ereignis trägt den Ort, an dem gewirkt wurde.
- Alter Client ohne neues Ereignis → Protokollversion lehnt ab wie üblich.

## Akzeptanzkriterien

- **AC-01** Go-Test: Flächenzauber erzeugen ein Ereignis mit Wirkort und Radius; ohne Ziel keines (`engine/sim`).
- **AC-02** Protokoll und Golden-Daten nachgezogen (`docs/protocol.md`, `task go:golden` mit Begründung).
- **AC-03** Vitest: Der Client zeichnet den Effekt am gemeldeten Ort mit dem Radius des Skills.
- **AC-04** Abnahme durch 🧑 im Spiel: fireball und meteor erscheinen am Gegner.

## Offene Fragen

- Erkennung im Client über Snapshot-Vergleich (`skillCasts.ts`) ganz ersetzen? Default: ja, das Ereignis ist die einzige Quelle. Entscheidet 🧑.

## Notizen

- Aus der Rückfrage von 🧑 am 2026-10-30 zur Monarch-Seite des Studios (B-671).
- Folge-Ticket fürs Studio: B-676.
