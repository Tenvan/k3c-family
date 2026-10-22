# BD2.1 · castFailed einmal je Tastendruck statt je Tick

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** bd2/1-castfailed-drosseln
- **Abhängig von:** –
- **Tickets:** B-378
- **Kriterien:** AC-01

## Ziel

Eine gehaltene Skill-Taste ohne Ziel erzeugt genau ein `castFailed`; ein neuer Druck meldet wieder. Skills mit Ziel und Abklingzeit bleiben unverändert (B-378).

## Kontext

- `engine/sim/skills.go` › `stepSkills` ruft jeden Tick `castSkill(w, p, cmd.Skill-1)`, solange `cmd.Skill` in 1..4 steht. `PlayerCommand.Skill` ist ein **gehaltener Wert** ohne Flanke (geprüft 2026-10-22), die Sim muss den Slot des Vor-Ticks selbst merken.
- `castSkill` sendet `castFailed {from, slot, x}`, wenn `cast(...)` false liefert (kein Ziel, keine Abklingzeit). Gehalten sind das bis zu 30 Ereignisse/s und Spieler; Netz-Budget Q08 (≤ 200 Byte je Tick und Client), siehe B-386.
- Entscheidung (Default, Specs autonom bzw. von Ralf zu prüfen): nur das **Ereignis** wird gedrosselt, der Wirkungsversuch läuft weiter jeden Tick. Findet der Skill beim Halten ein Ziel, wirkt er wie heute (Abklingzeit danach). Dafür merkt sich `Player` den zuletzt gedrückten Slot (`SkillHeld`, 0 = keiner); `castFailed` nur, wenn `SkillHeld != cmd.Skill`. `SkillHeld` wird in `stepSkills` jeden Tick auf `cmd.Skill` gesetzt (auch auf 0) und gehört **nicht** in den Spielstand.
- Tote Monarchen: `stepSkills` läuft nur für lebende, `SkillHeld` beim Tod/Wiederbeleben auf 0 zurücksetzen oder beim ersten Tick neu setzen.
- Tests: `engine/sim/feedback_test.go` (Helfer `eventsOfType`, Fälle für leeren Slot, nicht gelernt, Abklingzeit). Golden-Läufe (`task go:golden`) ändern sich höchstens um weggefallene `castFailed`-Wiederholungen; Begründung im Commit.
- Regeln: deterministisch (`engine/rng`), Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Erlaubte Dateien

- `engine/sim/skills.go`, `engine/sim/types.go` (Feld an `Player`), `engine/sim/feedback_test.go`
- `engine/sim/events.go` (nur Kommentar zu `castFailed`)
- `testdata/golden/` nur über `task go:golden`, falls sich Läufe ändern
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Abklingzeit für fehlgeschlagene Skills, Client-Darstellung (S9.3), Protokoll, Spielstand-Format, `PlayerCommand` ändern.

## Schritte

1. Branch `bd2/1-castfailed-drosseln` von `sprint/bd2`, `Status: in Arbeit`. `skills.go` und `feedback_test.go` lesen.
2. Test zuerst (rot): Taste 30 Ticks gehalten ohne Ziel → ein `castFailed`; Loslassen (Skill 0), neu drücken → zweites; Spieler 2 gleichzeitig unabhängig; Ziel erscheint beim Halten → Skill wirkt, kein `castFailed`.
3. `Player.SkillHeld` einführen, in `stepSkills` pflegen, `castSkill` meldet nur bei Flanke (Parameter oder Rückgabewert, Funktion ≤ 60 Zeilen).
4. Prüfen, ob `TestEventsBudgetJeTick` gehaltene Tasten abdeckt; sonst Fall ergänzen (AC-02 des Tickets).
5. `task check:diff`; ändern sich Golden-Läufe, nur weggefallene Wiederholungen, dann `task go:golden` mit Begründung im Commit.
6. Ergebnis, `Status: fertig`, Tabelle der Sprint-README. Review bei SIM Pflicht (Reviewer-Agent parallel zu `check:diff`).

## Fertig, wenn

- [ ] AC-01: Sim-Test: 30 Ticks gehalten ohne Ziel → genau ein `castFailed`; nach Loslassen und neuem Druck ein zweites; zwei Spieler unabhängig; `task check:go` grün.
- [ ] B-378/AC-02: `TestEventsBudgetJeTick` oder ein neuer Test deckt gehaltene Skill-Tasten ohne Ziel ab.
- [ ] Golden-Läufe unverändert oder nur um weggefallene `castFailed` geändert (Begründung im Ergebnis).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; währenddessen nur gezielte Tests (`task test` bzw. `check_run` mit Filter `Skill`/`Feedback`). Keine manuellen Prüfungen.

## Ergebnis

- AC-01 geprüft: `TestGehalteneSkillTasteMeldetCastFailedEinmal` (30 Ticks gehalten → 1 castFailed, neuer Druck → 2., Spieler 2 unabhängig, mit Ziel kein castFailed); `go test ./...` grün, `golangci-lint ./sim/...` 0 Issues.
- B-378/AC-02 geprüft: `TestGehalteneSkillTastenImEreignisBudget` (1 min, alle Spieler halten Skill 1; ≤ 1 castFailed je Spieler, Budget eingehalten). Golden-Läufe unverändert.
- Abweichung: `task check:diff` rot nur wegen fehlender Frontend-Abhängigkeiten im Worktree (react/radix nicht installiert, nicht von diesem Diff); Go-Teil grün. `castSkill` bekam den Parameter `announce`.
- Review (Sonnet): kein schwerer Befund; mittlerer Befund (skillHeld beim Tod nicht zurückgesetzt, `economy.go` nicht erlaubt) → B-524.
