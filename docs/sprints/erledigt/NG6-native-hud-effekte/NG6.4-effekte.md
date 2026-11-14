# NG6.4 · Effekte: Schaden, Skill-Wirkungen und Status

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** hud
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng6/4-effekte
- **Abhängig von:** NG6.3
- **Tickets:** B-799
- **Kriterien:** AC-04

## Ziel

Paket `fx` zeigt Schadenszahlen, Skill-Wirkungen und Status an Figuren zu den Ereignissen des Frames.

## Kontext

Spec: B-799. Vorbild: `src/scenes/effects/effectRules.ts`, `effects.ts`, `effectsView.ts`, `skillCasts.ts`, `skillLooks.ts`, `statusLooks.ts`, `statusView.ts`, `src/scenes/render/damageRules.ts` samt Tests; Ereignisse aus `engine/sim/events*.go` über den Frame. Vertrag und Importregel wie NG6.1 (`fx` importiert `hud` nicht).

## Erlaubte Dateien

- `native/k3c-game/fx/**`
- Planung NG6, B-799 (über MCP)

## Nicht-Ziele

Shader-Effekte; neue Effekte.

## Schritte

1. Fälle aus den TS-Tests als Go-Tests (rot).
2. Schadenszahlen.
3. Skill-Wirkungen und Status.

## Fertig, wenn

- [ ] AC-04: Tests grün; Screenshot eines Kampfs mit Schadenszahlen im Ergebnis.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-04 umgesetzt: Effekte (Treffer, Tod, Münzen, Bau, Schlag, Skill-Ringe, Status-Marken, Warnkreise, Geschosse), aufsteigende Schadenszahlen, Blitzgrenze, deterministisch aus Ereignis und `Frame.Clock`. Geprüft mit `effects_test.go`, `status_test.go` und an Bildern `ng6-kampf`, `ng6-fx`.
Bildgleichheit: Szene Takt 300 byte-gleich; Szene Takt 16500 nicht, weil „gesichert“ vom Zeitpunkt des Hintergrund-Speicherns abhängt (Ursache in `local`, Zusammenführung).
