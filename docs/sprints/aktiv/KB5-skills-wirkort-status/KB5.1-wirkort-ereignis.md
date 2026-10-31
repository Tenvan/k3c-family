# KB5.1 · Wirk-Ereignis mit Ort und Radius, Client zeichnet am Wirkort (B-674)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** kb5/1-wirkort-ereignis
- **Abhängig von:** –
- **Tickets:** B-674
- **Kriterien:** AC-01

## Ziel

Aktive Flächenzauber melden ein Ereignis mit Wirkort und Radius; der Client zeichnet den Skill-Effekt dort statt am Monarchen.

## Kontext

Spec: `docs/backlog/B-674-skill-effekt-am-wirkort.md`. Simulation: `engine/sim/skills.go`, `skills_caster.go` (`spellCenter`, `castAreaDamage`, `castSlow`, `castDamageOverTime`), Ereignisse und Snapshot in `engine/sim` und `engine/net`, Protokoll `docs/protocol.md`, Version in `src/online/clientProtocol.ts`. Client heute: `src/scenes/effects/skillCasts.ts` erkennt Wirken per Snapshot-Vergleich und zeichnet am Spieler; Looks in `skillLooks.ts`.

## Erlaubte Dateien

- `engine/sim/**`, `engine/net/**`
- `docs/protocol.md`, `testdata/golden/**` (nur mit `task go:golden`, Begründung im Commit)
- `src/model/**`, `src/online/clientProtocol.ts`, `src/scenes/effects/**`

## Nicht-Ziele

- Status-Effekte über die Dauer (KB5.2, KB5.3).
- Studio (KB5.4).

## Schritte

1. Ereignis `skillCast` (Spieler, Skill, x, Radius) in der Simulation beim Wirken aktiver Skills, Go-Test.
2. Protokoll und Version nachziehen, Golden-Daten.
3. Client: Effekt aus dem Ereignis am Wirkort mit Radius zeichnen, Snapshot-Erkennung entfernen, Vitest.
4. `task check:diff`, Review (Protokoll).

## Fertig, wenn

- [x] B-674 AC-01: Go-Test Ereignis mit Ort und Radius, ohne Ziel keines.
- [x] B-674 AC-02: Protokoll und Golden nachgezogen.
- [x] B-674 AC-03: Vitest Effekt am gemeldeten Ort mit Radius.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- B-674 AC-01 geprüft: `engine/sim/skills_cast_event_test.go` (fireball am Ziel mit Radius 3, meteor ohne Ziel nur `castFailed`, Ziel stirbt im selben Takt mit Ort des Wirkens, zwei Spieler getrennt, taunt am Monarchen mit Radius 0). Wirkort vor dem Wirken bestimmt (`castPlace`, `skills_caster.go`), Ereignis `skillCast` in `castSkill`; Spell Echo meldet einmal.
- B-674 AC-02 geprüft: Protokoll v10 (`docs/protocol.md`, `ProtocolVersion`, `PROTOCOL_VERSION`), Beispiel `s2c-snapshot-rueckmeldung.json` mit `skillCast`, `TestRueckmeldungEreignisseWieBeispiel` erweitert. `task go:golden` ohne Änderung: die Golden-Läufe wirken keine Skills, Hash unverändert.
- B-674 AC-03 geprüft: `skillCasts.test.ts` (Ring am gemeldeten Ort, Radius × `UNIT_PX`, ohne Fläche Größe des Looks, unbekannter Skill ignoriert, Abklingzeit allein zeigt nichts). Offene Frage aus B-674 mit Default entschieden: Ereignis ist die einzige Quelle für aktive Skills, passive Lern-Ringe bleiben aus dem Snapshot.
- Abweichung: außerhalb der erlaubten Dateien je eine Zeile in `src/scenes/GameScene.ts` (Ereignisse an `SkillCasts.diff`), `src/online/clientSkills.test.ts` und `testdata/protocol/c2s-hello.json`, `s2c-welcome.json` (Version 10). Sprint nach `aktiv/` verschoben (Status `in Arbeit` verletzte die Vorlage).
- `task check:diff`: Typecheck, Vitest (122 Dateien) und Go-Tests von engine/cmd grün, `golangci-lint` engine/cmd 0 Befunde, oxlint `src` ohne Fehler; rot nur durch fremde, nicht committete Studio-Chat-Änderungen im Arbeitsbaum (worker-2) und `tools/cleanup` (Git-sh im Lauf defekt). Manuelle Prüfung: keine (AC-04 bei 🧑, KB5.6).