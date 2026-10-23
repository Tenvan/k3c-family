# GEB14.3 · Gerüst bei jedem wartenden Arbeiter, Wimpel nach Markierer

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb14/3-geruest
- **Abhängig von:** –
- **Tickets:** B-572, B-532
- **Kriterien:** AC-02

## Ziel

Das Gerüst steht bei jedem `waitingWorker`, der Wimpel folgt dem Markierer aus dem Snapshot.

## Kontext

B-572, B-532 (Client-Teil).

## Erlaubte Dateien

- `src/scenes/render/`, `src/model/types.ts`, `docs/rules/materialien-gebaeude.md`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. `hubLookOf` ohne `danger`. 2. `markerOf`.

## Fertig, wenn

- [x] AC-02.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-02 umgesetzt: `scaffold = state === 'waitingWorker'`, Test von GEB12.3 umgedreht, Regeltabelle angepasst; `pennant.ts` nutzt `markedBy` mit Ereignis-Rückfall. Geprüft: Vitest (916), `tsc`.
