# tools/k3c-workbench/frontend/src/ui/organisms/

## Responsibility

Organismus-Ebene (Atomic Design) nur von k3c-workbench: Hinweis-Toasts der Agenten (MCP-Tool `notify_ui`) unten rechts.

## Design

- `Notices.tsx`: `Notices` abonniert `ui:notify` und hält höchstens vier Hinweise (`MAX`), neueste unten. Jeder Hinweis
  ist ein `CardButton` mit Titel in der Farbe des Tons (`level` `info|success|warn|error` -> `Tone` über `TONE_COLOR`)
  und optionalem Text. Fehler bleiben bis zum Klick, alle anderen schließen nach `AUTO_CLOSE_MS = 8000`.
  Container mit `role="status"` und `aria-live="polite"`.
- Importiert nur nach unten: `api`, webkit (geprüft von `tests/atomicDesign.test.ts`).

## Flow

1. Go emittiert `ui:notify` (Tool `notify_ui`) -> `backend.on` -> Hinweis wird angehängt, Liste auf vier gekürzt.
2. Timer oder Klick entfernt den Hinweis.

## Integration

- Konsument: `src/App.tsx` (`Notices` als `overlay` von `AppFrame`).
- Abhängigkeiten: `react`, `backend` und Typ `Notice` aus `../../api`, `@webkit/atoms` (`CardButton`, `Flex`, `Text`,
  `TONE_COLOR`). Stile: Klassen `notices`, `notice-toast` in `src/styles/app.css`.
