# src/core/

## Responsibility

Infrastructure-Schicht des Browser-Clients: Shell-Protokoll zur Landingpage, Vollbild, Client-Logging, Persistenz (Settings, Spielstand), i18n-Texte, Build-Version und gemeinsame Konstanten. Enthält keine Spiel-Logik und kein Phaser.

## Design

- Shell/Facade: `shell.ts` kapselt das `postMessage`-Protokoll (`SHELL_MESSAGE`, `ShellMessage`, `postToShell`, `isEmbedded`) und `installPageChrome()` (Home-Button, View+Menu-Kombi, Pos1, Zurück-Falle). `openPage()`/`goHome()`/`isOpenable()` sind die einzigen erlaubten Seitenwechsel.
- Delegation: `fullscreen.ts` – `toggleFullscreen()` fragt eingebettet die Shell (`askShell`, Request-ID + Timeout), sonst `toggleLocal()`; `onFullscreenChange()`.
- Fail-soft-Persistenz: `settings.ts` (`loadSettings`/`saveSettings`/`normalizeSettings`, `localStorage` `k3c-settings`, nie Exception), `saveStore.ts` (`storeSave`/`fetchSave`: Server `api/save` plus `localStorage` als Rückfall, neuerer Stand gewinnt).
- Typisierte i18n: `texts.de.ts` (Quelle der `TextKey`s), `texts.en.ts` (`Record<TextKey, string>`), `texts.ts` mit `t()`, `nameOf()`, `setLanguage()`; Fallback Deutsch.
- Batching Logger: `clientLog.ts` – Queue, Dedup gleicher Meldungen, Flush nach `FLUSH_MS`, Fehler sofort; `installClientLog()` hängt `error`/`unhandledrejection`/`console.warn|error` ein.
- `version.ts`: `CLIENT` (Vite-`define`), `formatBuild`, `versionLine`, `versionMismatch`, `fetchServerBuild`. `constants.ts`: `GAME_WIDTH/HEIGHT`, `UNIT_PX`, `GROUND_Y`, `MAX_PLAYERS`, `PLAYER_COLORS`.

## Flow

1. Seite startet: `installPageChrome()` → Home-Button, rAF-Poll der Gamepads (View+Menu > `HOLD_MS` → `goHome()`), Home-Taste.
2. Vollbild: Aktion → `toggleFullscreen()` → `postToShell({type: fullscreen, id})` → Landingpage ruft `toggleLocal()` → Antwort `fullscreenResult` mit gleicher `id` → Promise `string | null`.
3. Log: `clientLog(level, msg, ctx)` → Queue → `POST /api/clientlog` (Server schreibt `logs/k3c-client.jsonl`).
4. Speichern: `storeSave()` → `localStorage` und `POST api/save?slot=`; Laden: `fetchSave()` vergleicht Server- und lokalen Stand.
5. Texte: `t(key, params)` liest beim ersten Aufruf die Sprache aus `loadSettings()`, ersetzt `{name}`-Platzhalter.
6. Version: `fetchServerBuild()` → `GET api/health`; ohne Go-Server `null`.

## Integration

- Konsumenten: `src/landing/landing.ts` (shell, fullscreen, version), `src/main.ts` (shell, clientLog, version, constants), `src/input/touchInput.ts` (fullscreen), `src/tools/*` (`installPageChrome`, `toggleFullscreen`), `src/scenes/*` (texts, constants, settings, saveStore, clientLog, Version im Debug-Overlay), `src/online/clientConnection.ts` (clientLog, texts), `src/audio/*` (settings, clientLog).
- Abhängigkeiten: `src/model/types` (`isSaveGame`); Endpoints `/api/clientlog`, `api/save`, `api/health`; Browser-APIs `localStorage`, Fullscreen API, `postMessage`, Gamepad API.
- Wird von `src/scenes/noSim.test.ts` auf Importgrenzen geprüft (nur `src/model`).
