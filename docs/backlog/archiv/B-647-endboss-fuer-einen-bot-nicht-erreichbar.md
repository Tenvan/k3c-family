# B-647 · Ein einzelner Bot kann den Endboss nicht besiegen: Truppen erreichen ihn nicht, Monarch heilt nicht

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** BS3
- **Projekt:** BAS
- **Erstellt:** 2026-10-26
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-26, Chat, durch 🧑, Revision 1

## Ausgangslage

Der Stufen-Test für ein komplettes Spiel mit einem Bot (BS3.4, `task balance:fullgame STUFE=4`, `K3C_FULLGAME=voll`) kommt bis zum Endboss, besiegt ihn aber nicht (Stand nach B-646, 86 s je Lauf). Befunde: (1) Der Endboss `crystalHeartWarden` (10 024 HP) steht am Bau (`speed 0`) rund 200 Units vom Hub entfernt; die Truppen sitzen am Hub und erreichen ihn nicht, nur der Monarch kann ihn schlagen (10 Schaden je 0,7 s, `monarch.json` › `attack`). (2) Der Monarch regeneriert nur über Passive und Heiler (`monarch_passives.go`); ein Bot, der nur `sim.PlayerCommand` sendet, kann keine Skills lernen (Skill-Punkte aus Bossen bleiben ungenutzt). Nach den Beschwörungen steht der Monarch dauerhaft bei 25 % HP und geht nie zum Bau. (3) Selbst mit auf 300 HP gesetztem Endboss (`K3C_FULLGAME=boss`) bleibt der Test rot, weil der Monarch nicht kämpft.

## Ziel

Ein einzelner Bot kann die Insel 1 komplett durchspielen (Endboss besiegt), damit steht fest, dass die Simulation durchspielbar ist; danach folgen die Werte der Münzskala und coop2.

## Beteiligte und Zielgruppen

Claude (Bot und Simulation), 🧑 entscheidet über die Wege.

## Anforderungen

- Entscheidung nötig, wie ein Bot den Endboss erreicht. Optionen (ungeprüft): (A) Truppen begleiten den Monarchen zum Bau (neues Verhalten in der Sim: Kämpfer folgen dem Monarchen im Boss-Kampf); (B) Bot lernt Skills über eine neue Eingabe in `sim.PlayerCommand` (z. B. `LearnSkill`), damit Regeneration, Heilung und mehr Schaden verfügbar sind; (C) Endboss bewegt sich auf den Hub zu, sobald er erscheint (Truppen und Mauern kämpfen mit); (D) für den Test: Dev-Hilfe, die den Endboss schwächt oder dem Bot Skills schenkt (nur `fullgame`, kein Spielverhalten).
- Empfehlung (ungeprüft): D für den schnellen Nachweis, danach B oder C als Regel, abhängig vom Spielgefühl.

## Nicht-Ziele

Balance der Münzskala; Mehrspieler.

## Regeln und Einschränkungen

Bots nur über `sim.PlayerCommand`; deterministisch; Entscheidung über Spielverhalten bei 🧑 (`bosse.md` § 1: „Bosse bleiben ein Gruppenkampf“).

## Beispiele

`K3C_FULLGAME=voll task balance:fullgame STUFE=4`: Endboss `10024/10024` bis Tag 41, Monarch HP 25.

## Ausnahme- und Fehlerfälle

Der Monarch stirbt am Bau: Respawn an der Burg, Endboss bleibt aktiv (`boss_endboss.go`).

## Akzeptanzkriterien

- **AC-01** `task balance:fullgame STUFE=4` (ohne `boss`, mit `voll`): Endboss besiegt, `Island.EndbossDefeated`.
- **AC-02** Stufe 5 des Tests: Der Bot besiegt den Endboss ohne Dev-Hilfen (`K3C_FULLGAME=ohne`) — später.

## Offene Fragen

Welcher Weg (A bis D)? 🧑 entscheidet; Default: D jetzt, B danach.

## Notizen

Verwandt: B-645 (Bots), B-646 (Wellen-Lawine, erledigt).

**Erledigt 2026-10-26 (Entscheidung 🧑: zuerst Weg D, danach B, dann A):**
- Weg D: `K3C_FULLGAME=boss` (Endboss auf 300 HP, Monarch geheilt) als Nachweis der Kette.
- Weg B: Der Tester gibt dem Bot die Aktion eines Menschen im Menü (`sim.LearnSkill`, `bot_learn.go`), Tank-Linie.
- Weg A: Begleitschutz (`engine/sim/unit_escort.go`, `data/bosses.json` › `lair.escortUnits` 160): Kommt ein lebender Monarch auf 160 Units an den kämpfenden Endboss, marschieren Schützen und Krieger der Stufe zu ihm und bleiben, bis er stirbt. Test `TestTruppenBegleitenDenMonarchenZumEndboss`. Der Bot baut vor dem Gang zum Bau erst 8 Kämpfer auf.
- Ergebnis: Mit Dev-Bau Stufe 5 und vollem Vorrat (`K3C_FULLGAME=voll`, Standard) besiegt ein einzelner Bot den Endboss an Tag 19 (41 s), alle vier Stufen des Tests sind grün. Ohne jede Hilfe (`ohne`) bleibt es offen (Goblin-Anführer fällt nicht): das ist der Balance-Meilenstein BS3.4/B-645.
