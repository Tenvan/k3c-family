# GEB12.3 · Baugerüst nur ohne Gefahr (D8)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb12/3-geruest-ohne-gefahr
- **Abhängig von:** GEB12.2
- **Tickets:** B-545
- **Kriterien:** AC-04

## Ziel

Das Baugerüst vor der Burg steht nur, wenn ein Bauer den Hub-Ausbau bauen kann (`waitingWorker` ohne Gefahr).

## Kontext

`src/scenes/render/hubGrowth.ts` › `hubLookOf` setzt `scaffold` heute bei `hubUpgrade.state === 'waitingWorker'`. `World.danger` (`src/model/types.ts`) heißt: Nacht oder Gegner, Bau wartet. `src/scenes/render/siteView.ts` › `updateCastleView` übergibt die ganze Welt. Entscheidung D8, Frage-Ticket B-545.

## Erlaubte Dateien

- `src/scenes/render/hubGrowth.ts`, `src/scenes/render/hubGrowth.test.ts`
- `src/scenes/render/siteView.ts` (nur Typ der Welt)

## Nicht-Ziele

Server, Protokoll, Studio-Vorschau (zeigt das Gerüst als Beispiel weiter).

## Schritte

1. Test: `waitingWorker` mit `danger: true` → `scaffold: false`.
2. `hubLookOf` nimmt `danger` dazu.
3. `task test -- hubGrowth`, dann `task check:diff`.

## Fertig, wenn

- [ ] AC-04: Vitest `hubGrowth` grün mit Gefahr-Fall.

## Prüfen

```bash
task check:diff
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
