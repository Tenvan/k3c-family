# tools/k3c-workbench/frontend/src/ui/molecules/

## Responsibility

Molekül-Ebene (Atomic Design) nur von k3c-workbench: Markdown-Anzeige als React-Elemente. Atome und allgemeine
Moleküle liegen in `tools/webkit` (`@webkit/atoms`, `@webkit/molecules`).

## Design

- `MarkdownView.tsx`: `MarkdownView({ source, headingOffset })` rendert den Block-/Inline-Baum aus `lib/markdown.ts`
  (Überschriften, Listen mit ☐/☑, Tabellen, Absätze, Codeblöcke, `code`, **fett**, Links nur als Text) als React-Elemente,
  nie als HTML-String. `headingOffset` verschiebt die Überschriftsebenen, wenn die Seite selbst eine h1 hat.
- Importiert nur nach unten: `lib/`, webkit (geprüft von `tests/atomicDesign.test.ts`).

## Flow

1. Konsument übergibt `source` -> `parseMarkdown` -> `Block[]` -> `BlockView` je Block -> Inline-Teile als Elemente.

## Integration

- Konsumenten: `planning/` (`Detail.tsx`, `PlanningPage.tsx`, `Reader.tsx`), `mcp/InstructionsDialog.tsx`,
  `balancing/` (`DataFiles.tsx`, `HelpButton.tsx`).
- Abhängigkeiten: `react`, `../../lib/markdown.ts`.
