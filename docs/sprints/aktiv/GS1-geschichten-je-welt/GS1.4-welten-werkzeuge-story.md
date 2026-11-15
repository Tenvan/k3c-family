# GS1.4 · Workbench: world_story_set, Prüfung und Editor-Hinweis (B-975)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/gs1
- **Abhängig von:** GS1.2
- **Tickets:** B-975
- **Kriterien:** AC-05

## Ziel

Die Welten-Werkzeuge der Workbench schreiben, zeigen und prüfen die Geschichte; „Welt anlegen“ nimmt sie mit auf bzw. weist auf die fehlende Geschichte hin.

## Kontext

Spec: `docs/backlog/B-975-geschichte-welten-werkzeuge.md` (R1–R6, AC-01 bis AC-05). MCP-Tools: `tools/k3c-workbench/internal/mcpsrv/tools_world.go` (`world_check`, `world_list`, `world_get`, `world_create`, `world_set`, `world_override_set`, `world_chunks_set`), Tests `tools_world_test.go`. Logik: `tools/k3c-workbench/internal/world/` (`edit.go`, `edit_island.go`, `patch.go`, `world.go`). Editor: `tools/k3c-workbench/frontend/src/world/` (`NewWorld.tsx`, `useWorld.ts`, `WorldPage.test.tsx`), Typen `frontend/src/api/typesWorld.ts`. Format und Prüfung liefert die Engine (GS1.2, `engine/sim/story*.go`); die Workbench ruft diese Prüfung, statt sie nachzubauen. Tool-Beschreibungen enden mit „Nutze es bei: … Statt: …“ (`docs/standards/workbench-seiten.md`). Oberfläche nach Atomic Design nur über `@webkit`. Das neue Tool ist erst nach Neubau der Workbench nutzbar (🧑 baut neu, Sperre `exe`).

## Erlaubte Dateien

- `tools/k3c-workbench/internal/world/**`
- `tools/k3c-workbench/internal/mcpsrv/**`
- `tools/k3c-workbench/frontend/src/world/**`
- `tools/k3c-workbench/frontend/src/api/typesWorld.ts`
- `docs/standards/workbench-seiten.md`

## Nicht-Ziele

Texte schreiben (GS1.5), Texteditor für Geschichten, eigene Prüfregeln, Neubau oder Neustart der laufenden Workbench.

## Schritte

1. Go-Tests zuerst: `world_story_set` schreibt und lehnt ab, übrige Felder byte-gleich; `world_get`, `world_check`, `world_create` mit und ohne Geschichte.
2. Schreiblogik in `internal/world/` (neue Datei `story.go`), Prüfung über die Engine.
3. Tool `world_story_set` registrieren; `world_get`, `world_check`, `world_create` erweitern.
4. Editor: Stand der Geschichte und Prompt-Button „Geschichte schreiben“; Vitest in `WorldPage.test.tsx`.
5. `docs/standards/workbench-seiten.md` ergänzen.
6. Fachprüfung `k3c-workbench`, `check_run task:check:dev`, `check_run task:check:diff`.

## Fertig, wenn

- [ ] AC-05: Tools und Editor kennen die Geschichte, Tests grün, `check_run task:check:dev` grün (B-975/AC-01 bis AC-05).
- [ ] Im Ergebnis steht der Hinweis an 🧑: Workbench neu bauen und neu starten, bevor GS1.5 beginnt.

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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
