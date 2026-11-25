# WE6.4 · Welten-Editor: Endboss-Auswahl, Events, Plätze und Skin-Tabelle (B-987)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/we6
- **Abhängig von:** WE6.3
- **Tickets:** B-987
- **Kriterien:** AC-04

## Ziel

Der Welten-Editor hat Endboss-Auswahl je Insel, Events und Plätze in der Werte-Ansicht und eine Skin-Tabelle mit Vorschau (B-987, Frontend-Anteil).

## Kontext

Der Welten-Editor liegt im Studio (`tools/k3c-studio`, nicht in der Workbench). Frontend `tools/k3c-studio/frontend/src/world/`: `WorldPage.tsx`, `Overview.tsx`, `CrossSection.tsx`, `ChunkInspector.tsx`, `WorldValues.tsx`, `worldOverrides.ts`, `useWorld.ts`, Tests daneben (`*.test.tsx`, Vitest). Backend `tools/k3c-studio/internal/world/` und MCP-Server `tools/k3c-studio/internal/mcpsrv/`. Bausteine nur aus `@webkit` (`atoms`, `molecules`, `organisms`, `templates`), Radix nur über `@webkit`, rohe Controls nur in `atoms/` (`tests/atomicDesign.test.ts`). Daten kommen über die Backend-API aus WE6.3 (`endBoss`, `skins`, Overrides `events` und `hub › sites`). Asset-Vorschau: Standbilder wie im Bereich Assets des Studios.

## Erlaubte Dateien

- `tools/k3c-studio/frontend/src/world/**`
- `tools/k3c-studio/internal/world/**`
- `tools/k3c-studio/internal/mcpsrv/**`

## Nicht-Ziele

Neue Bausteine in `tools/webkit` (fehlt einer → Ticket), Drag & Drop, Assets anlegen.

## Schritte

1. Tests zuerst (Vitest): Endboss-Auswahl schreibt `endBoss` bzw. entfernt es; Skin-Tabelle fügt hinzu und entfernt.
2. Endboss-Auswahl in der Insel-Ansicht; Events und Plätze in `WorldValues`/`worldOverrides.ts`.
3. Skin-Tabelle mit Vorschau, Warnung bei fehlendem Asset.
4. `task check:dev` (bzw. `task check:diff`); Review durch den Agenten `k3c-workbench` (zuständig auch für das Studio) vor dem Commit.

## Fertig, wenn

- [ ] AC-04: B-987/AC-03 Frontend-Tests grün.
- [ ] AC-04: `check_run task:check:dev` grün (B-987/AC-04); Sichtprüfung 🧑 steht in der Ralf-Queue.

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
