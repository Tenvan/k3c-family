# SK3.1 · Dieb: Backstab und Smoke Bomb in der Sim

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** sk3/1-dieb-skills-sim
- **Abhängig von:** –
- **Tickets:** B-577
- **Kriterien:** AC-01

## Ziel

Der Dieb hat Backstab und Smoke Bomb als lernbare aktive Skills.

## Kontext

B-577; `docs/rules/monarch.md`; Entscheidung KR5 (Variante b).

## Erlaubte Dateien

- `data/monarch.json`, `engine/sim/skills*.go`, `monarch*.go`, `common.go`, `types.go`, `docs/rules/monarch.md`, `docs/protocol.md`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Daten. 2. Wirkungstypen `backstab` und `stealth`. 3. Tests, Regeltext.

## Fertig, wenn

- [x] AC-01.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-01 umgesetzt: `presets.thief.activeSkills` nennt beide Skills; `skills_thief_test.go` (vorn/hinten/fliehend, Unsichtbarkeit, zwei Spieler im selben Tick, Taunt, Abbruch durch Angriff). Golden unverändert. `go test ./sim/... ./net/...` und golangci-lint grün; Review (Sonnet): keine schweren Befunde. Annahmen (ungeprüft, 🧑 bestätigt): Unsichtbare werden von keinem Gegner und keiner Wache als Ziel gewählt und nehmen keinen Schaden; jeder erfolgreiche eigene Schlag oder Skill beendet die Unsichtbarkeit; Backstab-Reichweite 2 Units; BAL-Lauf nicht gefahren (Balancing zurückgestellt).
