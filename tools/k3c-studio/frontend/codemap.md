# tools/k3c-studio/frontend/

## Responsibility
Vite/React-Frontend des Asset-Studios; Wails bettet es als `dist/` in die EXE ein, im Dev-Modus lädt es Vite (Port 5192).

## Design
- `package.json`: React 19, Radix Themes, Vite; Skripte `dev` und `build` (`tsc --noEmit && vite build`); `index.html` lädt `src/main.tsx`.
- `paths.ts`: `REPO`, `ALIAS` (`@game` → `src/`, `@data` → `data/`) und `PUBLIC`, gemeinsam für `vite.config.ts` und `vitest.config.ts`; `tsconfig.json` mit denselben `paths`.
- `vite.config.ts`: `appType: 'mpa'` (unbekannte Pfade 404, damit Wails auf den Asset-Handler zurückfällt), `publicDir` = `public/` nur im Dev-Modus, `server.fs.allow` auf die Repo-Wurzel.
- `wailsjs/`: generierte Bindings (die Oberfläche bindet sie bewusst nicht ein, siehe `src/api/wails.ts`); `src/`: siehe dort.

## Flow
`wails dev` startet Vite; `wails build` bettet `dist/` ein; im Browser ohne Wails läuft der Mock.

## Integration
- Backend und Einbettung: `../main.go` (`go:embed frontend/dist`), `../public.go`.
- Tasks: `task studio:frontend`, `task check:studio`.
