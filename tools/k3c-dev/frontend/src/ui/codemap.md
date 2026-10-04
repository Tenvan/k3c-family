# tools/k3c-dev/frontend/src/ui/

## Responsibility

Gemeinsame, fachlich neutrale Präsentationsbausteine, die mehrere Seiten teilen.

## Design

- `parts.tsx`: dünne Wrapper über Radix Themes: `Tone` (`ok|warn|error|info|neutral`), `StatusBadge`, `NoticeCard`, `ActionButton` (zeigt während eines asynchronen `onClick` Ladezustand und sperrt Doppelklicks), `Tip` (Tooltip).
- `MarkdownView.tsx`: rendert den Block-/Inline-Baum aus `mcp/markdown.ts` als React-Elemente (nie als HTML-String); `headingOffset` verschiebt Überschriftsebenen.

## Flow

1. Eine Seite leitet aus ihrer Logik einen `Tone` ab (z. B. `badgeFor`, `describeRun`, `dotTone`) und übergibt ihn an `StatusBadge`/`NoticeCard`.
2. `ActionButton` ruft den Handler auf und deaktiviert sich bis zum Abschluss des Promise.
3. `MarkdownView source=...` -> `parseMarkdown` -> React-Baum.

## Integration

- Konsumenten: `Header.tsx`, `logs/`, `mcp/`, `planning/`, `services/`, `tasks/`; die Logik-Dateien dieser Ordner importieren zudem den Typ `Tone`.
- Abhängigkeiten: `@radix-ui/themes`, `react`, `mcp/markdown.ts`.
