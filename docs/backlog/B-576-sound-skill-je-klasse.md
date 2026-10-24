# B-576 · Jede Klasse hat einen eigenen Skill-Sound

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** SO8
- **Projekt:** SND
- **Erstellt:** 2026-10-23
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-24, Chat (Ralf), Revision 2 (mit Ton für passive Skills)

## Ausgangslage

Entscheidung 🧑 zu B-558 (S5, KR4.1): Statt des gemeinsamen `sfx:skill` bekommt jede Klasse einen eigenen Skill-Sound.

## Ziel

Der Skill jeder Klasse klingt eigen.

## Beteiligte und Zielgruppen

Spielende; Agent DEV (Slots, Katalog) und CLI (Auslöser); 🧑 bestätigt die Auswahl beim Hören.

## Anforderungen

- Ein Slot je Skill-Linie (Tank, Zauberer, Heiler, Dieb) in `data/assets.json` (`sfx:skill:tank`, `sfx:skill:mage`, `sfx:skill:healer`, `sfx:skill:thief`), Kandidaten aus CC0-Quellen (Kenney), Auswahl durch den Agenten als Default, Bestätigung durch 🧑 beim Hören.
- Auslöser ohne Protokolländerung: `SkillCasts` (`src/scenes/effects/skillCasts.ts`) erkennt das Wirken eines aktiven Skills am Anstieg der Abklingzeit und kennt die Linie des Skills (`MONARCH.skills[id].line`); daraus spielt der Client den Slot der Linie.
- **Passive Skills:** Auch für sie ist ein Ton definierbar (Entscheidung 🧑, 2026-10-24: wird später mit dem AI-Chat gesetzt). Die Abbildung kennt dafür optionale Slots je Skill (`sfx:skill:<skillId>`, Vorrang vor dem der Linie); solange keiner gesetzt ist, bleibt der passive Skill still. Auslöser eines passiven Skills ist wie bei seinem Ring das neue Erscheinen in `skills` (Lernen); das Studio zeigt die möglichen Slots (leer = ohne Ton) und erlaubt, einen zuzuordnen.
- Gemeinsamer `sfx:skill` bleibt als Rückfall für aktive Skills (Linie ohne Slot, Ton nicht geladen); `sfx:skill` wird weiter für `skillPoint` genutzt.
- Nach der Slot-Änderung `task gen:soundatlas`, Credits-Zeilen für neue Dateien.

## Nicht-Ziele

Eigener Ton für alle aktiven Skills im Voraus (nur je Klasse, einzelne Skills über den optionalen Slot); Protokoll- und Server-Änderungen; Lautheits-Feinschliff über Bus-Gain; die Auswahl der Töne für passive Skills (kommt später mit dem AI-Chat).

## Regeln und Einschränkungen

Client rechnet nichts, vergleicht nur Zustände (ADR 001); der Ton folgt dem Spieler-Index: nur der wirkende Spieler löst ihn aus, Entfernung/Split-Screen-Dämpfung wie bei den anderen Effekten (`attenuation`); zwei Spieler gleichzeitig; Credits-Test grün; Slots wie vorhanden `sfx:*` im Katalog.

## Beispiele

Heiler wirkt `groupHeal` → Heiler-Ton; Tank wirkt `taunt` → Tank-Ton; Linie ohne Slot → `sfx:skill`.

## Ausnahme- und Fehlerfälle

Passiver Skill ohne gesetzten Slot → kein Ton. Skill-Slot gesetzt, Datei fehlt → bei aktiven Skills Rückfall `sfx:skill`, bei passiven still. Linie ohne Slot → `sfx:skill`.

## Akzeptanzkriterien

- **AC-01** Vier Slots `sfx:skill:<linie>` mit Kandidaten und Credit im Katalog; Atlas neu erzeugt (Test: Katalog und Credits).
- **AC-02** Das Wirken eines aktiven Skills spielt den Ton seiner Linie, ohne Slot den gemeinsamen (Vitest der Abbildung Skill → Slot).
- **AC-03** Für einen passiven (und jeden einzelnen) Skill lässt sich ein Ton über einen Slot `sfx:skill:<skillId>` definieren; ohne Eintrag bleibt er still (Vitest der Abbildung, ein Beispiel-Slot im Test).
- **AC-04** 🧑 hat die vier Klassen-Töne gehört und bestätigt oder geändert.

## Offene Fragen

keine. Die Auswahl trifft der Agent als Default (CC0, kindgerecht, passend: Tank dumpf/metallisch, Zauberer hell/mystisch, Heiler weich/aufsteigend, Dieb kurz/leise), 🧑 bestätigt beim Hören.

## Notizen

SO7.1 (2026-10-23): Voraussetzung war ein Skill-Ereignis oder Klassenfeld im Protokoll. Neu 2026-10-24: Der Client kennt die Linie über `MONARCH.skills[id].line` und erkennt das Wirken in `SkillCasts` (SK2.3, B-578); ein Protokollfeld ist nicht nötig. Spec: Entwurf. Sprint SO8.
