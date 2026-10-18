# tools/cockpit/

## Responsibility
Claude-Code-Plugin `tenvan-cc` („Tenvan CC“, Version laut `.claude-plugin/plugin.json`): Mod für Claude Desktop und die Claude-Code-CLI mit Band über der Prompt-Eingabe (Kontext, 5h/7d-Limits, Prompt-Cache, Kosten, Subagents, Folge-Schritte, Checkliste) und Panels rechts (`/cockpit`, Zusammenfassung, Einstellungen, Verlauf), dazu Chat-Skins, MR/PR-Anzeige und Atemübungen. TypeScript, kein Netzwerk und keine Telemetrie (Ausnahme: Plattform-Aufrufe für MR/PR und ein Haiku-Call für Folge-Schritte).

## Design
- `.claude-plugin/plugin.json`: Manifest mit `userConfig` (`chatInfo`, `nextSteps`); `.claude-plugin/types/` enthält die Engine-Typen (`claude-code`, `claude-code-mcp`, `claude-code-tools`) und die gemeinsame `tsconfig.json`; `tsconfig.json` erweitert sie.
- `hooks/hooks.json`: Modul-Liste mit dem Einstieg `register.tsx`.
- `hooks/register.tsx`: alle Hooks, Atome (`$.state`), Band, Panel und Befehle; einziger Ort mit `$`. Reicht Prozess und HTTP als `io` an `forge.ts` weiter.
- Reine Module ohne `$`: `core.ts` (Rechen-/Formatfunktionen, Farbtabelle `THEME`), `i18n.ts` (alle Texte Deutsch/Englisch, gleiche Schlüssel), `glance.ts` (Checkliste: Job, Schritte, Restzeit, Verlauf, Tages- und Wochenbericht), `mr.ts` (Remote erkennen, API-Antworten von GitLab/GitHub/Bitbucket Server in `Mr` übersetzen, Ereignisse, Prompts), `skins.ts` und `mermaid.ts` (Chat-Skins, Mermaid-Parser und Layout), `breath.ts` (Atemübungen), `mock.ts` (Dev-Mode-Mockdaten für `/cockpit-dev`).
- `hooks/forge.ts`: Aufrufe von `glab`, `gh` und Bitbucket-REST über das hereingereichte `io`.
- `hooks/ui.tsx`: Design-System (Tokens je Bereich `pane`, `skin`, `sidebar`, `band`; Atome und Moleküle), bekommt die Elementtabelle statt `$`.
- `types/index.d.ts`: Vertrag des `$.state` (`PluginState['tenvan-cc']`).
- Doku: `README.MD`, `SPEC.md`, `DEVELOPMENT.md`, `RELEASE_NOTES.md`, `THIRD_PARTY.md`; `assets/` mit Logo und Texten für Features, Catch-up und Skin-Demo (de/en).

## Flow
1. Die Engine lädt `register.tsx` laut `hooks.json`; Hooks beobachten Mod-Events (Tool-Aufrufe, Turns, Model-Requests, Usage).
2. Events füllen Atome; die reinen Module rechnen und formatieren, `register.tsx` rendert Band, Panel und Skins daraus über `ui.tsx`.
3. Für MR/PR fragt `register.tsx` per `forge.ts` die Plattform ab, `mr.ts` übersetzt die Antwort und meldet Änderungen.

## Integration
- Geladen zum Entwickeln per `claude --plugin-dir tools/cockpit`; Prüfung mit `claude plugin validate tools/cockpit`.
- Version synchron mit `.claude-plugin/marketplace.json` im Plugin-Ordner `tools/cockpit/`.
- Tests liegen neben den Modulen als `hooks/*.test.ts` (Testrahmen `claude-code/testing`).
- Eigenständig; keine Verbindung zu Spiel-Code oder Go-Modulen des Repos.
