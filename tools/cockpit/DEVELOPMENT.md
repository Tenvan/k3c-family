# tenvan-cc – Arbeitsanleitung für Agenten

Mod für Claude Desktop und die Claude-Code-CLI (Claude Code Function Hooks, Early Access, Engine ≥ 2.1.295). Was gebaut wird und warum: [SPEC.md](SPEC.md). Diese Datei regelt, wie daran gearbeitet wird.

## Session-Start

1. Ort prüfen: `git status --short --branch`. Entwickelt wird in `plugins/tenvan-cc` des Hauptcheckouts; die globale Junction `~/.claude/skills/tenvan-cc` lädt genau diesen Ordner in jeder Claude-Code-Session (`/reload-plugins` nach Änderungen). Keine zweite Junction im Projekt und keine Marketplace-Installation daneben, sonst lädt das Mod doppelt und das Panel bleibt leer. Prüfen: `claude plugin list` zeigt `tenvan-cc@skills-dir` genau einmal mit Scope `user`.
2. [SPEC.md](SPEC.md) lesen: Leitplanken, offene Akzeptanzkriterien, Backlog.
3. Skill `plugin-authoring` laden. Er nennt den Pfad zur aktuellen API-Typdatei `claude-code.d.ts` (rund 21.000 Zeilen). Gezielt danach greppen, nie ganz lesen, z. B. `'tool.check'`, `export type SessionUsage =`, `AbovePrompt: {`.

## Dateien

| Datei | Inhalt | Regel |
| --- | --- | --- |
| `hooks/register.tsx` | alle Hooks, Band, Panel | Einstiegspunkt laut `hooks/hooks.json` |
| `hooks/ui.tsx` | Design-System: Tokens je Bereich (`MODES`: pane, skin, sidebar, band), Atome `pill`, `chip`, Moleküle `labelRow`, `tile` | Knöpfe, Chips, Raster-Zeilen und Karten nur hierüber, mit dem Bereich als `mode`; Style-Änderungen in `MODES`, nie an der Aufrufstelle. Kein `$` (Engine 2.1.295 lädt keine `$`-Funktionen aus anderen Dateien) |
| `hooks/core.ts` | reine Funktionen ohne `$` | neue Rechen-/Formatlogik hierher, damit sie direkt testbar ist |
| `hooks/mr.ts`, `hooks/forge.ts` | MR/PR: Übersetzung der Plattform-Antworten (rein) bzw. Aufrufe von glab, gh, Bitbucket-REST (über `io`, ohne `$`) | neue Plattform: Konverter in `mr.ts`, Aufrufe in `forge.ts` |
| `hooks/mock.ts` | Mockdaten für den Dev-Mode (`/cockpit-dev`) | neue Panel-Werte hier mitpflegen |
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
- **Surface-Gate zuerst**: jeder `ui.render`-Hook beginnt mit `if (!drawsOn(e.surface)) return next(e)` (Desktop und Terminal).
- **Elementtabelle nur über `elementsOf($, e)`**, nie direkt `$.ui.resolve(e)`: die Engine ergänzt jede Tabelle um alle Namen, im Terminal zeichnet `Svg` ein leeres Fragment. `elementsOf` nimmt es dort heraus, erst dann ist `'Svg' in ui` die richtige Weiche zwischen Grafik und Text-Fallback.
- **Desktop First, Terminal mit Fallback**: jede neue Anzeige mit `Svg` braucht einen Text-Fallback (Blockzeichen, Unicode) oder lässt die Engine zeichnen; im Test je Anzeige einmal mit `surface: 'terminal'` mounten (`terminal.test.ts`).
- **Hintergrundarbeit** (z. B. Model-Call nach `turn.complete`) mit `void …catch(() => undefined)` starten. Sonst entsteht nach Session-Ende oder im Test eine unbehandelte Rejection. Ergebnisse gegen Veralten absichern (siehe `asked === lastPrompt`).
- **Modul-Variablen** (`lastPrompt`) überleben keinen Reload. Was über einen Reload bleiben muss, gehört in ein Atom.
- **`$` verlässt nie das Modul**: auch nicht an importierte Funktionen (`forgeFind($, …)` lädt nicht). Andere Module bekommen ein `io`-Objekt aus `$.process.run`/`$.http.fetch` (siehe `forgeIo`).
- **`Button` ohne Kinder-Elemente**: Engine 2.1.295 nimmt nur `label` oder ein String-Kind, keinen farbigen `Text` (entgegen den Typen). Farbe über eine getönte Box drumherum.
- **Op-Stubs im Test** antworten immer mit `{ value: … }`, auch ohne Ergebnis: `on('store.set', () => ({ value: undefined }) as never)`. `{}` lehnt Engine 2.1.295 ab.
- **CLI-Version**: `claude` im PATH kann älter sein als die Engine der Desktop-App. Prüfen mit der Desktop-Engine: `"$APPDATA/Claude/claude-code/<version>/<hash>/claude.exe" plugin test plugins/tenvan-cc`.
- **Elemente**: `Box`, `Text`, `Button`, `Input`, `Select`, `Code`, `Markdown` auf Desktop und Terminal, `Svg` nur auf dem Desktop. Farben über Theme-Keys (`success`, `warning`, `error`, `claude`).

## Offizielle Quelle: anthropics/claude-code

Im Cache: `~/.deltra/repo-cache/github.com/anthropics/claude-code` (Skill `repo-cache`). Relevant: `CHANGELOG.md` (Mod-Änderungen je Version, vor der Desktop-Engine), `mods/` (offizielle Referenz-Mods `diff`, `agents-md`, `sec-default`, `telemetry` mit Tests). `mods/types/claude-code.d.ts` ist älter als die Engine – maßgeblich bleibt die Typdatei aus dem Skill `plugin-authoring`.

## Muster aus ray-amjad/claude-code-mods

Referenz-Mods (MIT) im Cache: `~/.deltra/repo-cache/github.com/ray-amjad/claude-code-mods` (per Skill `repo-cache`, Stand `56a266b`). Nur als Vorlage, kein Code übernehmen. Vor neuen Features dort nach einem ähnlichen Mod suchen.

| Muster | Quelle | Für das Cockpit |
| --- | --- | --- |
| Echte Cache-TTL aus dem Transcript: neueste Hauptthread-Zeile mit `usage.cache_creation.ephemeral_1h_input_tokens` bzw. `_5m_` (Pfad aus `classic.Stop`/`classic.SessionStart` → `transcript_path`) | `cache-clock` | statt Schätzung „1 h mit Subscription“ |
| Resume: `classic.SessionStart` liefert `seconds_since_last_response`, damit läuft der Cache-Countdown nach `--resume` richtig weiter | `cache-clock` | Cache-Wächter |
| Ein Command kann keinen Prompt absetzen (er wartet auf seinen eigenen Turn): `$.clock.after(50, () => $.prompt.submit({ text, asUser: true }))` | `prompt-bouncer` | Alternative zu `prompt.fill`, wenn Knopf direkt senden soll |
| Band einzeilig halten: Teile nach Wichtigkeit, von hinten weglassen, bis `bodyColumns` reicht | `cache-clock` | Band bei schmalem Fenster |
| Pane unaufgefordert nur ab 144 Spalten platziert; `isPlaced` prüfen, sonst Toast mit dem Command. `focus` nur `true` oder weglassen; `closeOnEscape`, `rows` beim gewollten Öffnen | `decision-tree`, `diagram-pane` | `$.ui.open` |
| Hook-Laufzeit max. 10 s, Zeit in Mod-API-Aufrufen zählt nicht mit | `secret-box` | lange Waits nur über API-Aufrufe |
| Schwere Auswertung über einen Fork der Session (voller Kontext, keine Tools, Antwort als JSON). Vorher ein billiges Gate (Regex, dann Haiku), JSON aus Prosa über die äußersten Klammern holen | `diagram-pane`, `decision-tree`, `dream-pass` | Zusammenfassung, Handoff |
| Fail open: fällt Haiku aus oder antwortet Unsinn, nichts blockieren | `prompt-bouncer` | Auto-Effort, Gates |
| Pro Session im `$.store` ablegen, nur die N neuesten Sessions behalten | `diagram-pane` | Store-Hygiene |
| Reine Logik ohne `$` in eigene Datei, damit sie unter plain Node testbar ist | `vercel-deploy-status`, `secret-box` | `core.ts`, `skins.ts`, `mock.ts` |
| Fußzeile (`SessionMode`) trägt mehrere Knöpfe, wenn die Labels kurz sind (` ⏻ `, ` ⚙︎ `): Leerzeichen als Innenrand, `gap={1}`, `hover={{ scope, backgroundColor }}` als Pille; lange Labels drängen den Rest hinter „…“ | `HeyCubit/effortless` | MR-Knopf und „Tenvan“ unter dem Prompt |
| Fehler im eigenen Hook nie die Session blockieren lassen: `.catch(($, e, next) => (next.called ? undefined : next(e)))` am `on(...)` | `cache-clock` | `session.start`, `prompt.submit` |

## Dev-Mode (Mockdaten)

`/cockpit-dev` schaltet das Panel auf Mockdaten aus `hooks/mock.ts` um (Kennzeichen „DEV · Mockdaten“), erneut aufgerufen zurück. Gilt nur für die Session, die echten Atome bleiben unberührt. Neue Panel-Werte gleich in `mockData` ergänzen, damit jeder Zustand ohne echte Session sichtbar ist.

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
