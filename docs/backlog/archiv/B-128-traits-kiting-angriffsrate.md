# B-128 · Die Gegner-Traits aoe, swarm, phases und Kiting wirken, die Angriffsrate steht je Gegner in den Daten

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** K1
- **Projekt:** –
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, mit Sprint K1

## Ausgangslage

Die Traits `aoe`, `swarm`, `phases`, `pack`, `stealsWood` und `flying` haben keine Wirkung; es gibt kein Kiting; alle Gegner greifen mit 1/s an (`engine/sim/enemies.go`, `data/waves.json`); das Tor existiert nicht (`docs/rules/archiv/ist-gegner-bosse.md`).

## Ziel

Flächenschlag, Schwarm, Phasenwechsel und Kiting wirken wie beschlossen, jeder Gegner hat seine eigene Angriffsrate, das Tor blockiert wie die Mauer. Nutzen: Elite-Gegner haben Eigenarten (`docs/rules/gegner.md` §§ 2–4).

## Beteiligte und Zielgruppen

Spieler; Werte pflegt REG, Feintuning B-099.

## Anforderungen

- `aoe`: Flächenschlag Radius 3 Units alle 4 s (Höhlentroll, Kristallwächter); `swarm`: ein Schwarm-Platz spawnt 4 Gegner (Rattenschwarm, Splitterwicht); `phases`: alle 8 s für 2 s unverwundbar und ignoriert Mauern (Minengeist).
- Kiting: Fernkämpfer halten 8 Units Abstand zu Spielern und Truppen (Goblin Archer, Feuergeist).
- `data/enemies.json`: `attacksPerSecond` je Gegner (Standard 1); `pack` und `stealsWood` werden aus den Daten entfernt.
- Das Tor (B-116) blockiert Gegner wie die Mauer, eigene Bürger und Spieler passieren (Q63, `docs/rules/gegner.md` § 4); übrige Gebäude sind Ziele für Gegner mit `prefersBuildings`.
- Ereignis `enemyKilled` (Art, Ort) für Messung und Anzeige.

## Nicht-Ziele

Neue Gegnerarten (B-129), Bosse (B-130), Events (B-131), Darstellung (B-132).

## Regeln und Einschränkungen

`docs/rules/gegner.md`; Werte nur in `data/`; Tor-Wirkung zusammen mit B-116. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Höhlentroll schlägt alle 4 s mit Radius 3: mehrere Truppen nehmen Schaden; der Goblin Archer weicht einem Krieger aus und schießt weiter.

## Ausnahme- und Fehlerfälle

Kiting an der Mauer oder am Rand → bleibt stehen (kein Absturz); Schwarm bei Platzmangel → spawnt in Reihe.

## Akzeptanzkriterien

- **AC-01** Test je Trait (aoe, swarm, phases, Kiting): Wirkung laut Daten, deterministisch.
- **AC-02** Test: Angriffsrate je Gegner aus den Daten; ohne Eintrag 1/s.
- **AC-03** Test: Tor blockiert Gegner, eigene Bürger und Spieler passieren; übrige Gebäude werden nach Regel angegriffen.
- **AC-04** Test: `enemyKilled` wird je Tod gemeldet.
- **AC-05** Golden-Daten aktualisiert; `task check:go` grün.

## Offene Fragen

- Warnzeit vor dem Flächenschlag: Startwert legt K1.1 in `data/enemies.json` fest, 🧑 bestätigt.

## Notizen

Aus R4.2. Zusammen mit B-116 (Tor) und B-013 (Elite-KI) planen.
