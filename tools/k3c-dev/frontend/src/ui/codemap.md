# tools/k3c-dev/frontend/src/ui/

## Responsibility

Gemeinsame, fachlich neutrale Präsentationsbausteine, die mehrere Seiten teilen, sowie die Hinweis-Toasts der Agenten.

## Design

- `parts.tsx`: dünne Wrapper über Radix Themes: `Tone` (`ok|warn|error|info|neutral`), `StatusBadge`, `NoticeCard`, `ActionButton` (zeigt während eines asynchronen `onClick` Ladezustand und sperrt Doppelklicks), `Tip` (Tooltip).
- `MarkdownView.tsx`: rendert den Block-/Inline-Baum aus `mcp/markdown.ts` als React-Elemente (nie als HTML-String); `headingOffset` verschiebt Überschriftsebenen.
- `Notices.tsx`: `Notices` abonniert `ui:notify` (Tool `notify_ui`) und zeigt die Hinweise als Toasts unten rechts: höchstens vier, neueste unten, Ton aus `level` (`info|success|warn|error`); Fehler bleiben bis zum Klick, alle anderen schließen nach 8 s.

## Flow

1. Eine Seite leitet aus ihrer Logik einen `Tone` ab (z. B. `badgeFor`, `describeRun`, `dotTone`) und übergibt ihn an `StatusBadge`/`NoticeCard`.
2. `ActionButton` ruft den Handler auf und deaktiviert sich bis zum Abschluss des Promise.
3. `MarkdownView source=...` -> `parseMarkdown` -> React-Baum.
4. Go emittiert `ui:notify` -> `Notices` hängt den Hinweis an die Liste und entfernt ihn per Timer oder Klick.

## Integration

- Konsumenten: `App.tsx` (`Notices`), `Header.tsx`, `StudioLink.tsx`, `logs/`, `mcp/`, `planning/`, `services/`, `tasks/`; die Logik-Dateien dieser Ordner importieren zudem den Typ `Tone`.
- Abhängigkeiten: `@radix-ui/themes`, `react`, `api` (`backend`, `Notice`), `mcp/markdown.ts`.
