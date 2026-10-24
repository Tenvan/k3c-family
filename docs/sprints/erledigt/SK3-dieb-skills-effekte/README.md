# SK3 · SIM, CLI, DEV · Dieb-Skills Backstab und Smoke Bomb, Skill-Effekte, Skill-Ring im Studio

- **Status:** erledigt
- **Projekt:** SKL
- **Domäne:** SIM, CLI, DEV
- **Reife:** bereit
- **Tickets:** B-577, B-578, B-562
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-23, Revision 1

## Ausgangslage

Der Dieb hat keine aktiven Skills (B-577), Skills zeigen nur einen Ring je Klasse (B-578), das Studio zeigt die Lücke statt des Skill-Rings (B-562).

## Ziel

Dieb mit Backstab und Smoke Bomb, eigener Effekt je Skill, Studio zeigt den Skill-Ring. Am Ende sichtbar: Dieb-Skills im Spiel und im Studio.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

B-577, B-578, B-562 › Anforderungen.

## Nicht-Ziele

Weitere Klassen.

## Regeln und Einschränkungen

Hängt von SK2 ab (Preset und Darstellung Dieb). Werte nur in `data/monarch.json`; Golden-Änderung begründet; 🧑 bestätigt die Unsichtbarkeits-Regel vor der Freigabe (B-577).

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `presets.thief.activeSkills` nennt `backstab` und `smokeBomb`, Sim-Tests, Golden begründet (B-577/AC-01, AC-02).
- **AC-02** Jeder Skill hat einen eigenen Effekt (B-578/AC).
- **AC-03** Die Monarch-Seite des Studios zeigt den Skill-Ring des Spiels (B-562/AC).

## Offene Fragen

Verhalten der Gegner gegen Unsichtbare (kein Ziel?), entscheidet 🧑 (B-577).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SK3.1 | `SK3.1-dieb-skills-sim.md` | Umsetzung | autonom | fertig |
| SK3.2 | `SK3.2-skill-effekte.md` | Umsetzung | autonom | fertig |
| SK3.3 | `SK3.3-studio-skill-ring.md` | Umsetzung | autonom | fertig |


## Abnahme

- 2026-10-23, Claude autonom: AC-01 bis AC-03 umgesetzt, Review (Sonnet) ohne schwere Befunde, `task check:diff` grün.
- Offen bei 🧑: Unsichtbarkeits-Regel bestätigen (Annahmen in SK3.1), Skill-Effekte am TV ansehen, BAL-Lauf für Backstab/Smoke Bomb (Balancing zurückgestellt).
