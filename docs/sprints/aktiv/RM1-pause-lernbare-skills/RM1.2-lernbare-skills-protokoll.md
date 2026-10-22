# RM1.2 · Lernbare Skills je Spieler im Protokoll

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** rm1/2-lernbare-skills
- **Abhängig von:** –
- **Tickets:** B-285
- **Kriterien:** AC-02

## Ziel

Der Eintrag `learn` der Aktionsliste nennt die gerade lernbaren Skill-IDs (`skills`, berechnet mit `sim.CanLearn`), und das Skill-Menü markiert nur diese als lernbar.

## Kontext

`engine/net/actions.go` (`actionsOf`) sendet heute `{ "action": "learn" }` pauschal bei `points` > 0. `sim.CanLearn(w, p, id)` (`engine/sim/monarch.go`) prüft ohne Seiteneffekt inkl. Tier-Gating. Katalog in fester Reihenfolge: `skillCatalog` (`engine/sim/monarch_data.go`, nach ID sortiert); für `engine/net` braucht es eine exportierte Liste `sim.LearnableSkills(w, p)`. Client: `src/scenes/hud/skillMenuLogic.ts` (`menuEntries`), Typ `PlayerAction` in `src/model/types.ts`. Protokoll-Doku `docs/protocol.md` › Skills und Aktionen, Beispiele `testdata/protocol/s2c-snapshot-*.json` (Form-Test `TestFormWieBeispiele` vergleicht die Schlüssel der ersten Listeneinträge). Entscheidung zur Form: Sprint-README › Offene Fragen. Das Feld ist zusätzlich, keine neue Protokoll-Version.

## Erlaubte Dateien

- `engine/sim/monarch.go` (nur `LearnableSkills`), `engine/sim/monarch_*_test.go`
- `engine/net/actions.go`, `engine/net/*_test.go`
- `docs/protocol.md`, `testdata/protocol/s2c-snapshot-*.json`
- `src/model/types.ts`, `src/scenes/hud/skillMenuLogic.ts`, `src/scenes/hud/skillMenuLogic.test.ts`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Neue Lern-Regeln, Gestaltung des Menüs, `respec` (RM1.4).

## Schritte

1. `sim.LearnableSkills(w, p) []string` über `skillCatalog` mit `CanLearn`, Go-Test (B-285/AC-01).
2. `actionsOf`: `learn` nur, wenn die Liste nicht leer ist, mit Feld `skills`.
3. Doku und Beispiele nachziehen.
4. Client: `PlayerAction` mit `skills?`, `menuEntries` nutzt `skills`, ohne Feld pauschal (B-285/AC-02), Test.
5. `task check:diff`.

## Fertig, wenn

- [ ] AC-02 (B-285/AC-01): Go-Test: 1 Punkt, nichts gelernt → `taunt` lernbar, `ironWall` nicht.
- [ ] AC-02 (B-285/AC-02): Vitest: `menuEntries` markiert nur die genannten Skills.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

2026-10-23, autonom (Claude).

- **AC-02 (B-285/AC-01) umgesetzt, geprüft:** `sim.LearnableSkills` (über den sortierten `skillCatalog` mit `CanLearn`); `actionsOf` sendet `{ action: learn, skills: [...] }` nur, wenn etwas lernbar ist. Go-Tests `TestLearnableSkillsTierGating` (1 Punkt: `taunt` ja, `ironWall` nein, nur Tier 1, sortiert) und `TestSkillsAktionsliste`.
- **AC-02 (B-285/AC-02) umgesetzt, geprüft:** `menuEntries` markiert nur `learn.skills`; ohne Feld pauschal (älterer Server). Vitest `skillMenuLogic.test.ts`.
- Doku `docs/protocol.md`, Beispiele `testdata/protocol/s2c-snapshot-*.json` mit `skills`; `src/online/clientSkills.test.ts` nachgezogen (außerhalb der Liste, erzwungen durch die Beispiele). Keine neue Protokoll-Version (Feld zusätzlich). Form-Entscheidung zur Bestätigung in KR5.
- Reviewer (Sonnet): keine schweren Befunde. `task check:diff` grün (erster Lauf rot: Planungs-Test, AC-01 ohne Session → RM1.1 als verworfen angelegt).
