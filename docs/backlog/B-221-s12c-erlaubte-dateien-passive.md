# B-221 · S1.2c braucht für die Passive weitere erlaubte Dateien

- **Domäne:** SIM
- **Typ:** Frage
- **Prio:** hoch
- **Status:** offen
- **Sprint:** S1
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

S1.2c (`docs/sprints/aktiv/S1-monarch-schlag-skills/S1.2c-passive.md`) soll die zwölf Passiven wirken lassen, erlaubt
aber an Tests nur `engine/sim/passives_test.go`. Die Tests aus S1.2a/S1.2b lernen Passive als Füller, um das
Tier-Gating zu erfüllen, und prüfen Zahlen, die sich ändern, sobald die Passiven wirken:

- `engine/sim/skills_caster_test.go` (`mageSkills` mit `arcanePower`, `spellEcho`): `TestCasterFireballUndMeteor`
  erwartet −30/−200, mit Arcane Power +20 % wären es −36/−240; Spell Echo würfelt und kann doppelt treffen.
- `engine/sim/skills_test.go` › `tankPlayer` (`armorAura`, `thickSkin`, `regeneration`): `TestTankIronWall` erwartet
  „555 Schaden = 550 nach Verteidigung“, mit Armor Aura (+2 auch für den Besitzer) wären es 548; Regeneration und
  Thick Skin ändern HP in den Tests über `Step`.
- `engine/sim/skills_healer_test.go` (`healingAura`, `blessings`, `holyGround`): `TestHealerHeiltZauberer` erwartet
  Fireball −30 und Zauberer-HP genau 90; Blessings (+10 %) und Arcane Power (+20 %) machen −39, Healing Aura heilt im
  selben Tick 2/30 HP dazu.
- `frostArmor` soll „im Gegnerangriff“ wirken; der Nahkampftreffer steht in `engine/sim/enemies.go` › `attack`,
  das ebenfalls nicht erlaubt ist (Umweg ohne diese Datei: in `damagePlayer` das unmittelbar vorangehende
  `strike`-Ereignis des Gegners auswerten – möglich, aber eine Auslegung).

## Ziel

S1.2c lässt sich autonom abarbeiten, ohne Dateien außerhalb der erlaubten Liste zu ändern.

## Beteiligte und Zielgruppen

Planung und Freigabe 🧑; umsetzender Agent von S1.2c.

## Anforderungen

- Die Session-Datei S1.2c nennt alle Dateien, die die Passiven zwangsläufig berühren.

## Nicht-Ziele

Werte der Passiven (B-119, B-099).

## Regeln und Einschränkungen

`docs/arbeitsweise.md` › Autonomer Ablauf (nur erlaubte Dateien); Spec-Änderung nur durch 🧑.

## Beispiele

Erlaubte Dateien um `engine/sim/skills_test.go`, `engine/sim/skills_tank_test.go`, `engine/sim/skills_caster_test.go`,
`engine/sim/skills_healer_test.go` (nur Erwartungen an die Passiven anpassen) und `engine/sim/enemies.go` (nur der
Aufruf für Frost Armor in `attack`) erweitert → S1.2c ist umsetzbar.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Planungsfrage.

## Akzeptanzkriterien

- **AC-01** `S1.2c-passive.md` › Erlaubte Dateien enthält die oben genannten Test-Dateien und `engine/sim/enemies.go`
  (oder eine andere von 🧑 gewählte Lösung), Status der Session wieder `offen`.

## Offene Fragen

Dürfen die S1.2a/S1.2b-Tests angepasst werden (Erwartungen mit Passiven), oder sollen sie die Passiven nach dem Lernen
gezielt aus `Player.Skills` entfernen? Frost Armor über `enemies.go` oder über das `strike`-Ereignis? 🧑

## Notizen

Gefunden beim Start von S1.2c (Sprint S1); Umsetzung nicht begonnen, damit `sprint/s1` grün bleibt.
