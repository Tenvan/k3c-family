# B-676 · Das Studio zeigt Skills am Wirkort und DoTs und Buffs an Gegnern und Monarchen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** KB5
- **Projekt:** WZG
- **Erstellt:** 2026-10-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Monarch-Seite des Studios (B-671) spielt Skills am Monarchen, weil das Spiel sie dort zeichnet; DoTs, Buffs und Kontrolleffekte hat das Spiel noch nicht. Mit B-674 (Wirkort) und B-675 (Status-Effekte) bekommt das Spiel beides.

## Ziel

Im Studio sieht 🧑 jeden Skill so wie im Spiel: Flächenzauber am Ziel, Status-Effekte über die Dauer an Gegnern und Monarchen.

## Beteiligte und Zielgruppen

- 🧑 Ralf prüft Skill-Grafik im Studio.
- Sessions der Domäne DEV setzen um.

## Anforderungen

- Monarch-Seite: Option „Ziel-Gegner“ (Gegner aus der Gegner-Seite, z. B. Goblin) in Reichweite des Skills. Flächenzauber erscheinen am Ziel mit dem Radius des Skills (wie B-674).
- DoTs und Kontrolleffekte werden am Ziel-Gegner über ihre Dauer gezeigt (Sturm-Fläche, Betäubung, Verlangsamung, Spott), Buffs am Monarchen (Schild, Aura, Regeneration, lastStand, frostArmor), mit der Darstellung des Spiels aus `src/scenes/effects/` (B-675), Schleife/Tempo/Einzelbild wie heute.
- Gegner-Seite: Modus „Status“ zeigt die Status-Effekte an der gewählten Gegnergruppe.
- Was das Spiel nicht zeichnet, bleibt eine Lücke.

## Nicht-Ziele

- Eigene Darstellungen im Studio ohne Gegenstück im Spiel.

## Regeln und Einschränkungen

- Domäne DEV: `tools/k3c-studio/frontend/src/monarch/`, `enemies/`, `playback/`.
- Darstellung aus dem Spiel importieren, nicht nachbauen.
- Erst nach B-674 und B-675.

## Beispiele

- fireball mit Ziel-Gegner Goblin in 8 Units → Ring mit 3 Units Radius am Goblin.
- shieldBash → Sterne 2 s über dem Goblin.
- ironWall → Schild-Blase am Monarchen 10 s.

## Ausnahme- und Fehlerfälle

- Ziel außerhalb der Reichweite → „Skill ohne Ziel“ wie im Spiel.
- Gegner-Grafik fehlt → Platzhalter-Rahmen, Effekt trotzdem.

## Akzeptanzkriterien

- **AC-01** Vitest: Flächenzauber mit Ziel-Gegner erscheinen am Ziel mit dem Radius des Skills.
- **AC-02** Vitest: jeder Status-Effekt aus B-675 lässt sich an Gegner bzw. Monarch abspielen oder zeigt seine Lücke.
- **AC-03** Abnahme durch 🧑 im Studio.

## Offene Fragen

- Ziel-Gegner frei wählbar oder fest Goblin? Default: frei aus der Gegnerliste. Entscheidet 🧑.

## Notizen

- Aus der Rückfrage von 🧑 am 2026-10-30. Hängt an B-674 und B-675.
