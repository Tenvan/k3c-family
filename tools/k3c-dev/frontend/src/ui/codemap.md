# tools/k3c-dev/frontend/src/ui/

## Responsibility

Präsentationsbausteine nur von k3c-dev nach Atomic Design (Markdown-Ansicht, Hinweis-Toasts der Agenten). Atome und die Radix-Bausteine (`Tone`, `StatusBadge`, `CardButton`, `NoticeCard` …) liegen in `tools/webkit` (`@webkit/atoms`, `@webkit/molecules`); `ui/` hat nur `molecules/`, `organisms/`, `templates/` und importiert nur nach unten, aus `api/`, `lib/` und webkit (`tests/atomicDesign.test.ts`).

## Design

- `molecules/MarkdownView.tsx`: rendert den Block-/Inline-Baum aus `lib/markdown.ts` als React-Elemente (nie als HTML-String); `headingOffset` verschiebt Überschriftsebenen.
- `organisms/Notices.tsx`: `Notices` abonniert `ui:notify` (Tool `notify_ui`) und zeigt die Hinweise als Toasts (`CardButton`) unten rechts: höchstens vier, neueste unten, Titel in der Farbe des Tons aus `level` (`info|success|warn|error`); Fehler bleiben bis zum Klick, alle anderen schließen nach 8 s.

## Flow

1. `MarkdownView source=...` -> `parseMarkdown` -> React-Baum.
2. Go emittiert `ui:notify` -> `Notices` hängt den Hinweis an die Liste und entfernt ihn per Timer oder Klick.

## Integration

- Konsumenten: `App.tsx` (`Notices` als `overlay` von `AppFrame`), `mcp/InstructionsDialog.tsx`, `planning/` (`Detail`, `PlanningPage`, `Reader`) für `MarkdownView`.
- Abhängigkeiten: `react`, `@webkit/atoms`, `api` (`backend`, `Notice`), `lib/markdown.ts`.
