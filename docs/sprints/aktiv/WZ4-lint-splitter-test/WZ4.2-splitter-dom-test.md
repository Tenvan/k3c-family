# WZ4.2 · Splitter: Doppelklick und Ziehen in einer DOM-Umgebung getestet

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz4/2-splitter-dom-test
- **Abhängig von:** –
- **Tickets:** B-406
- **Kriterien:** AC-01

## Ziel

Doppelklick und Ziehen des Splitters (`Splitter` in `tools/k3c-dev/frontend/src/tasks/TaskBars.tsx`) sind in einer DOM-Umgebung (`happy-dom`) getestet; der Test läuft in `task check`.

## Kontext

- `Splitter` setzt `onPointerDown` (registriert `pointermove`/`pointerup` auf `window`, ruft `onChange(clampSplit(splitShare(...)))`) und `onDoubleClick={onReset}`. Die Rechenhilfen `splitShare` und `clampSplit` sind schon getestet (`tools/k3c-dev/frontend/src/planning/Splitter.test.tsx`, nur Markup über `renderToStaticMarkup`).
- Der Root-Vitest (`npx vitest run`, Taskfile `test`) sammelt auch die Frontend-Tests; es gibt keine DOM-Umgebung (kein jsdom, kein happy-dom). `Splitter.test.tsx` mockt `../api` mit `vi.mock('../api', () => ({ backend: {} }))`, weil `TaskBars.tsx` die API importiert; das gleiche hier tun.
- Vitest wählt die Umgebung je Datei über den Docblock `// @vitest-environment happy-dom` in der ersten Zeile; alle anderen Tests bleiben in Node.
- happy-dom liefert `getBoundingClientRect()` mit Nullen: die Box des Elternelements per `vi.spyOn(parent, 'getBoundingClientRect')` setzen.
- React 19: `act` aus `react` importieren, Wurzel mit `createRoot` aus `react-dom/client`; `globalThis.IS_REACT_ACT_ENVIRONMENT = true`.
- Die Abhängigkeit ist in B-406 › Offene Fragen mit der Spec-Freigabe von 🧑 zu bestätigen; ohne Freigabe wird diese Session nicht gestartet.

## Erlaubte Dateien

- `tools/k3c-dev/frontend/src/planning/SplitterDom.test.tsx` (neu)
- `package.json`, `package-lock.json` (Root: nur `happy-dom` als devDependency)
- `docs/backlog/B-406-splitter-dom-test.md`, `docs/backlog/README.md` (Ticket erledigen)
- `tools/k3c-dev/frontend/codemap.md` oder `tools/k3c-dev/frontend/src/planning/codemap.md`, falls die Testdatei dort aufgeführt wird

## Nicht-Ziele

Keine Änderung an `TaskBars.tsx` oder der Planungsseite; keine weiteren DOM-Tests; keine globale `environment`-Einstellung in der Vitest-Konfiguration; keine Änderung an Lint-Regeln; keine zweite neue Abhängigkeit (kein `@testing-library`).

## Schritte

1. `happy-dom` als devDependency im Root eintragen (`npm install -D happy-dom`), Lockfile mitcommitten.
2. `SplitterDom.test.tsx` anlegen: erste Zeile `// @vitest-environment happy-dom`, `vi.mock('../api', …)`, ein Helfer rendert `<Splitter>` in ein `<div>` mit gesetzter Box (`left 100, top 0, width 800, height 400`) per `createRoot` und `act`.
3. Test „Doppelklick“: `dblclick` auf `[role=separator]` → `onReset` einmal gerufen, `onChange` nie.
4. Test „Ziehen senkrecht“: `pointerdown` auf den Balken, `pointermove` auf `window` mit `clientX: 300` → `onChange(25)`; `clientX: 1000` → `onChange(80)`, `clientX: 100` → `onChange(20)`.
5. Test „Ziehen waagerecht“ (`horizontal`): `clientY` von der unteren Kante gemessen (`clientY: 200` bei Boxunterkante 400 und Höhe 400 → `onChange(50)`).
6. Test „Loslassen“: nach `pointerup` ruft `pointermove` `onChange` nicht mehr auf.
7. Test „Box fehlt“: Elternelement ohne messbare Box (`getBoundingClientRect` gibt `undefined` zurück bzw. kein Elternelement) → `pointerdown` ruft `onChange` nicht auf.
8. `task test -- SplitterDom` und danach `task check` ausführen; B-406 mit `plan_set` auf `erledigt` setzen (Archiv zieht das Tool nach).

## Fertig, wenn

- [x] AC-01: `SplitterDom.test.tsx` läuft in `happy-dom` und prüft Doppelklick (`onReset`), Ziehen senkrecht und waagerecht mit Klemmung 20–80 %, Loslassen und fehlende Box; `task test -- SplitterDom` ist grün.
- [x] `happy-dom` ist die einzige neue Abhängigkeit, nur als devDependency im Root; kein anderer Test wechselt die Umgebung.
- [x] `task check` ist grün.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 geprüft: `SplitterDom.test.tsx` (happy-dom, fünf Tests: Doppelklick, Ziehen senkrecht mit Klemmung 20–80 %, Ziehen waagerecht, Loslassen, fehlende Box); `npx vitest run SplitterDom` 5/5 grün, `task check` grün.
- `happy-dom` ist die einzige neue Abhängigkeit (Root-devDependency); die Umgebung gilt nur für diese Datei per Docblock.
- Abweichung: Der Fall „Box fehlt“ wird über ein `getBoundingClientRect`, das `undefined` liefert, ausgelöst, da das Elternelement im DOM immer existiert.
- Keine neuen Tickets. Geprüft hat der Agent, keine manuelle Prüfung.
