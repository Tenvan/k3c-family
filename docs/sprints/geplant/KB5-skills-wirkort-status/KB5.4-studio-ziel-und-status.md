# KB5.4 · Studio zeigt Skills am Ziel-Gegner und Status-Effekte (B-676)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** kb5/4-studio-ziel-und-status
- **Abhängig von:** KB5.1, KB5.3
- **Tickets:** B-676
- **Kriterien:** B-676 AC-01, AC-02

## Ziel

Die Monarch-Seite des Studios zeigt Flächenzauber am Ziel-Gegner und Status-Effekte über ihre Dauer, die Gegner-Seite den Modus „Status“.

## Kontext

Spec: `docs/backlog/B-676-studio-skills-ziel-und-status.md`. Studio: `tools/k3c-studio/frontend/src/monarch/` (`actions.ts` `skillPlay`, `useMonarch.ts`), `enemies/`, `playback/timeline.ts` und `effectView.ts` (Zeitachse mit Schleife, Tempo, Einzelbild). Darstellung aus dem Spiel importieren (`src/scenes/effects/`, KB5.1, KB5.3).

## Erlaubte Dateien

- `tools/k3c-studio/frontend/src/monarch/**`, `enemies/**`, `playback/**`, `styles/figures.css`

## Nicht-Ziele

- Eigene Darstellungen ohne Gegenstück im Spiel.

## Schritte

1. Option Ziel-Gegner auf der Monarch-Seite, Flächenzauber am Ziel mit Radius.
2. Status-Effekte als Zeitachsen-Effekte an Gegner und Monarch.
3. Modus „Status“ auf der Gegner-Seite.
4. Vitest, `task check:diff`.

## Fertig, wenn

- [ ] B-676 AC-01: Vitest Flächenzauber am Ziel.
- [ ] B-676 AC-02: Vitest Status-Effekte abspielbar oder Lücke.

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
