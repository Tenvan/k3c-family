# B-576 · Jede Klasse hat einen eigenen Skill-Sound

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** SO8
- **Projekt:** SND
- **Erstellt:** 2026-10-23
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Entscheidung 🧑 zu B-558 (S5, KR4.1): Statt des gemeinsamen `sfx:skill` bekommt jede Klasse einen eigenen Skill-Sound.

## Ziel

Der Skill jeder Klasse klingt eigen.

## Beteiligte und Zielgruppen

Spielende; Agent DEV (Slots, Katalog) und CLI (Auslöser); 🧑 bestätigt die Auswahl beim Hören.

## Anforderungen

- Ein Slot je Skill-Linie (Tank, Zauberer, Heiler, Dieb) in `data/assets.json` (`sfx:skill:tank`, `sfx:skill:mage`, `sfx:skill:healer`, `sfx:skill:thief`), Kandidaten aus CC0-Quellen (Kenney), Auswahl durch den Agenten als Default, Bestätigung durch 🧑 beim Hören.
- Auslöser ohne Protokolländerung: `SkillCasts` (`src/scenes/effects/skillCasts.ts`) erkennt das Wirken eines aktiven Skills am Anstieg der Abklingzeit und kennt die Linie des Skills (`MONARCH.skills[id].line`); daraus spielt der Client den Slot der Linie.
- Gemeinsamer `sfx:skill` bleibt als Rückfall (Linie ohne Slot, Ton nicht geladen); `sfx:skill` wird weiter für `skillPoint` genutzt.
- Nach der Slot-Änderung `task gen:soundatlas`, Credits-Zeilen für neue Dateien.

## Nicht-Ziele

Eigener Sound je einzelnem Skill; Protokoll- und Server-Änderungen; Lautheits-Feinschliff über Bus-Gain.

## Regeln und Einschränkungen

Client rechnet nichts, vergleicht nur Zustände (ADR 001); der Ton folgt dem Spieler-Index: nur der wirkende Spieler löst ihn aus, Entfernung/Split-Screen-Dämpfung wie bei den anderen Effekten (`attenuation`); zwei Spieler gleichzeitig; Credits-Test grün; Slots wie vorhanden `sfx:*` im Katalog.

## Beispiele

Heiler wirkt `groupHeal` → Heiler-Ton; Tank wirkt `taunt` → Tank-Ton; Linie ohne Slot → `sfx:skill`.

## Ausnahme- und Fehlerfälle

Passiver Skill gelernt → kein Ton (wie bisher kein Wirken). Slot-Datei fehlt → Rückfall `sfx:skill`, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Vier Slots `sfx:skill:<linie>` mit Kandidaten und Credit im Katalog; Atlas neu erzeugt (Test: Katalog und Credits).
- **AC-02** Das Wirken eines aktiven Skills spielt den Ton seiner Linie, ohne Slot den gemeinsamen (Vitest der Abbildung Skill → Slot).
- **AC-03** 🧑 hat die vier Klassen-Töne gehört und bestätigt oder geändert.

## Offene Fragen

keine. Die Auswahl trifft der Agent als Default (CC0, kindgerecht, passend: Tank dumpf/metallisch, Zauberer hell/mystisch, Heiler weich/aufsteigend, Dieb kurz/leise), 🧑 bestätigt beim Hören.

## Notizen

SO7.1 (2026-10-23): Voraussetzung war ein Skill-Ereignis oder Klassenfeld im Protokoll. Neu 2026-10-24: Der Client kennt die Linie über `MONARCH.skills[id].line` und erkennt das Wirken in `SkillCasts` (SK2.3, B-578); ein Protokollfeld ist nicht nötig. Spec: Entwurf. Sprint SO8.
