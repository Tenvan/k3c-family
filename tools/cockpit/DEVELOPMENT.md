# tenvan-cc – Arbeitsanleitung für Agenten

Claude-Desktop-Mod (Claude Code Function Hooks, Early Access, Engine ≥ 2.1.295). Was gebaut wird und warum: [SPEC.md](SPEC.md). Diese Datei regelt, wie daran gearbeitet wird.

## Session-Start

1. Ort prüfen: `git status --short --branch`. Entwickelt wird in `plugins/tenvan-cc` des Hauptcheckouts; die globale Junction `~/.claude/skills/tenvan-cc` lädt genau diesen Ordner in jeder Claude-Code-Session (`/reload-plugins` nach Änderungen). Keine zweite Junction im Projekt und keine Marketplace-Installation daneben, sonst lädt das Mod doppelt und das Panel bleibt leer. Prüfen: `claude plugin list` zeigt `tenvan-cc@skills-dir` genau einmal mit Scope `user`.
2. [SPEC.md](SPEC.md) lesen: Leitplanken, offene Akzeptanzkriterien, Backlog.
3. Skill `plugin-authoring` laden. Er nennt den Pfad zur aktuellen API-Typdatei `claude-code.d.ts` (rund 21.000 Zeilen). Gezielt danach greppen, nie ganz lesen, z. B. `'tool.check'`, `export type SessionUsage =`, `AbovePrompt: {`.

## Dateien

| Datei | Inhalt | Regel |
| --- | --- | --- |
| `hooks/register.tsx` | alle Hooks, Band, Panel | Einstiegspunkt laut `hooks/hooks.json` |
| `hooks/core.ts` | reine Funktionen ohne `$` | neue Rechen-/Formatlogik hierher, damit sie direkt testbar ist |
| `types/index.d.ts` | `$.state`-Vertrag `PluginState['tenvan-cc']` | jedes neue Atom hier deklarieren, sonst schlägt `validate` fehl |
| `hooks/register.test.ts` | Tests (`claude-code/testing`) | pro Akzeptanzkriterium ein Test |
| `.claude-plugin/plugin.json` | Manifest, `userConfig` | Version synchron mit `.claude-plugin/marketplace.json` im Repo-Root |
| `README.MD` | Nutzerdoku | bei sichtbaren Änderungen nachziehen |

## Prüfen (vor jedem Abschluss alle drei)

```bash
claude plugin validate plugins/tenvan-cc
```

```bash
claude plugin test plugins/tenvan-cc
```

Typecheck: Die Engine legt die Typen nur unter `.claude-plugin/types/` ab, wenn sie das Mod per `--plugin-dir` lädt. Bei der Junction tut sie das nicht. Deshalb eine tsconfig außerhalb des Plugins (Scratchpad) mit diesen Optionen anlegen; `include` nennt die `claude-code.d.ts` aus dem Skill, `plugins/tenvan-cc/hooks` und `plugins/tenvan-cc/types`:

```json
{ "compilerOptions": { "target": "es2023", "lib": ["es2023"], "types": [], "module": "esnext", "moduleResolution": "bundler", "strict": true, "noUncheckedIndexedAccess": true, "noEmit": true, "skipLibCheck": true, "jsx": "react", "jsxFactory": "h", "jsxFragmentFactory": "Fragment" } }
```

```bash
npx -y -p typescript@5.6.3 tsc -p <scratchpad>/tsconfig.json
```

Nach Änderungen am Manifest zusätzlich im Repo-Root `npm run validate` und `npm run registry:build`. Achtung: `registry:build -- --fix` sortiert alle Marketplace-Einträge um. Neue Einträge deshalb von Hand ans Ende setzen und nur ohne `--fix` prüfen.

## Fallstricke der Mod-API (alle schon einmal aufgetreten)

- **Atom-Refs nur mit Literalen**: `atom({ plugin: 'tenvan-cc', key: '…' } as const, init)`. Eine Konstante wie `PLUGIN` statt `'tenvan-cc'` lehnt `validate` ab.
- **`$` nur an Top-Level-Funktionen übergeben**: Hilfsfunktionen, die `$` bekommen, müssen auf Modulebene deklariert sein, nicht innerhalb von `register`. Closures in Hooks und `onPress` dürfen `$` direkt nutzen.
- **Matcher mit Literalen**: `{ component: 'Pane', requestId: 'tenvan-cc' }` statt `PANE.id`, sonst kann `validate` den Matcher nicht auflösen.
- **Surface-Gate zuerst**: jeder `ui.render`-Hook beginnt mit `if (e.surface !== 'desktop') return next(e)`.
- **Hintergrundarbeit** (z. B. Model-Call nach `turn.complete`) mit `void …catch(() => undefined)` starten. Sonst entsteht nach Session-Ende oder im Test eine unbehandelte Rejection. Ergebnisse gegen Veralten absichern (siehe `asked === lastPrompt`).
- **Modul-Variablen** (`lastPrompt`) überleben keinen Reload. Was über einen Reload bleiben muss, gehört in ein Atom.
- **Elemente** auf Desktop: `Box`, `Text`, `Button`, `Input`, `Select`, `Svg` aus `$.ui.resolve(e)`. Farben über Theme-Keys (`success`, `warning`, `error`, `claude`).

## Tests schreiben

- Unter dem Plugin steht im Test nichts. Jedes Event, das der Code durchreicht, braucht einen Stub der Test-`on`:
  - `session.measure` → `{ changed: [] }`
  - `ui.render` (AbovePrompt) → `{ type: 'Box' }`
  - `turn.complete` → `{ text }`
  - `prompt.submit` → `{ text }`
- Op-Events antworten mit `{ value: … }`, z. B. `ui.open` → `{ value: { isPlaced: true } }`, `model.complete`, `session.usage`.
- `mock.clock(on, { now })` für feste Zeit. Mount-Props brauchen alle Pflichtfelder (`bodyColumns`, `scroll`, `view` …); der Typecheck sagt, welche.
- Asynchrones Nachzeichnen abwarten: `for (let i = 0; i < 100 && !(await ui.find({ key })); i++) await Promise.resolve()`.
- Optionen testen: `test(name, { options: { nextSteps: false } }, body)`.

## Ein neues Community-Mod aufnehmen

1. Nur den Funktionsumfang erfassen (README, Manifest, Event-/API-Namen). Keinen Code kopieren, keine Struktur nachbauen.
2. Gegen die Leitplanken in SPEC.md prüfen; über weggelassene Teile mit Ralf entscheiden.
3. In SPEC.md eintragen: Quellen-Tabelle, Features, Akzeptanzkriterien (zunächst offen).
4. Umsetzen: Logik nach `core.ts`, State als Atom plus Eintrag in `types/index.d.ts`, Anzeige als Abschnitt in Band oder Panel.
5. Tests je Kriterium, dann alle Prüfungen.
6. Bei Bedarf eine Option in `userConfig`, README-Tabellen ergänzen, `version` in `plugin.json` und `marketplace.json` gleich anheben (Minor bei neuem Feature).

## Konventionen

- Bewusste Vereinfachungen mit `// ponytail: <Grenze>, <Ausbaupfad>` markieren.
- Commits über Skill `orga-core:commit`, Scope `mods`.
- Kein Auto-Open, keine Toasts, keine Statuszeile ohne Rücksprache.
