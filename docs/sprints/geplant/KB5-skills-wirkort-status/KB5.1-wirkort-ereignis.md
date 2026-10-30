# KB5.1 · Wirk-Ereignis mit Ort und Radius, Client zeichnet am Wirkort (B-674)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** kb5/1-wirkort-ereignis
- **Abhängig von:** –
- **Tickets:** B-674
- **Kriterien:** B-674 AC-01, AC-02, AC-03

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

- [ ] B-674 AC-01: Go-Test Ereignis mit Ort und Radius, ohne Ziel keines.
- [ ] B-674 AC-02: Protokoll und Golden nachgezogen.
- [ ] B-674 AC-03: Vitest Effekt am gemeldeten Ort mit Radius.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
