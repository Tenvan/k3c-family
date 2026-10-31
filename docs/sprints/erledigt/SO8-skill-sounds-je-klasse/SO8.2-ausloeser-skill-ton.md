# SO8.2 · Auslöser: Skill-Ton je Skill, Linie und Rückfall

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** so8/2-ausloeser-skill-ton
- **Abhängig von:** SO8.1
- **Tickets:** B-576
- **Kriterien:** AC-02, AC-03

## Ziel

Das Wirken eines aktiven Skills spielt den Ton seiner Linie (Rückfall `sfx:skill`); für passive und einzelne Skills lässt sich über `sfx:skill:<skillId>` ein Ton definieren (ohne Eintrag still).

## Kontext

Spec `docs/sprints/geplant/SO8-skill-sounds-je-klasse/README.md`, B-576. Das Wirken erkennt `src/scenes/effects/skillCasts.ts` (`SkillCasts.diff`: Abklingzeit steigt = aktiver Skill wirkt; neuer Skill in `skills` = passiver gelernt). Die Linie liefert `MONARCH.skills[id].line` (`src/model/data.ts`). Audio-Ereignisse und Slots: `src/audio/events.ts`, Kern `src/audio/audioCore.ts`; Entfernung/Dämpfung wie bei den anderen Effekten (`attenuation`).

## Erlaubte Dateien

- `src/audio/` (Abbildung Skill → Slot, neue reine Funktion mit Test), `src/scenes/effects/skillCasts.ts` und `src/scenes/GameScene.ts` (nur Anbindung), zugehörige `*.test.ts`
- keine Protokoll- oder Server-Änderung

## Nicht-Ziele

Neue Slots (SO8.1), Auswahl der Töne für passive Skills, Pegel-Feinschliff.

## Schritte

1. Reine Funktion `skillSlot(skillId, kind)` (Vorrang: `sfx:skill:<skillId>` wenn im Katalog, dann `sfx:skill:<linie>` für aktive, dann `sfx:skill` für aktive; passive ohne Skill-Slot: kein Ton) mit Vitest, inklusive Beispiel-Slot für einen passiven Skill im Test.
2. `SkillCasts` meldet Wirken und Lernen je Spieler mit Skill-Id (zusätzlich zum Ring, ohne den Ring zu ändern); GameScene spielt den Ton nur für den wirkenden Spieler, mit der Dämpfung nach Abstand.
3. Reset bei Stufenwechsel wie beim Ring: kein Ton aus dem Vergleich mit altem Stand.
4. `task check:diff` einmal vor dem Commit; Reviewer nicht nötig (unter 150 Zeilen).

## Fertig, wenn

- [ ] AC-02: Vitest der Abbildung: aktiver Skill → Linien-Slot, ohne Slot → `sfx:skill`.
- [ ] AC-03: Vitest: passiver Skill ohne Skill-Slot still, mit Skill-Slot spielt er ihn.

## Prüfen

`task test -- skill`, `task check:diff`.

## Ergebnis

- AC-02 umgesetzt: reine Funktion `skillSlot` in `src/audio/skills.ts` (eigener Slot vor Linie vor `sfx:skill`); `SkillCasts` meldet Wirken/Lernen per Callback, `GameScene.playSkill` spielt den Slot am Ort des Spielers mit `attenuation` und `nightGain`. Reset bei Stufenwechsel wie beim Ring.
- AC-02 geprüft: Vitest `src/audio/skills.test.ts` (taunt → Tank-Slot, ohne Linien-Slot → `sfx:skill`, Katalog hat einen Slot je Klasse), `skillCasts.test.ts` (Meldung je Spieler, nicht aus dem ersten Zustand); Typecheck und Lint grün.
- AC-03 geprüft: Vitest: `regeneration` ohne Slot still, mit `sfx:skill:regeneration` spielt er ihn, der Skill-Slot hat Vorrang vor der Linie.
- Abweichung: Schritt 1 nennt `skillSlot(skillId, kind)`, die Art liest die Funktion aus `MONARCH.skills`, der zweite Parameter ist das Slot-Register (testbar). `events.test.ts` nimmt `sfx:skill:*` aus der Erreichbarkeit über `cueFor` aus.
