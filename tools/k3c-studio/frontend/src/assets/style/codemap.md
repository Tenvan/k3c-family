# tools/k3c-workbench/frontend/src/assets/style/

## Responsibility

Seite **Styleguide** im Bereich Studio (WB1.2): Materialtafel aus der Stil-Datei (`licht`, `basis`, `schatten` mit Hex),
Einzeltöne und Papier; ein Klick auf ein Element zeigt Prompt-Buttons (B-727). Geändert wird nur über das MCP-Tool
`assets_style_set` (Entscheidung 005 › 2, 4).

## Design

- `StylePage.tsx`: lädt den Stil einmal (`assetsStyle`), rendert Materialien (Typ `MaterialToene`), Einzeltöne und
  Papier als Kacheln. Die Auswahl (`Wahl`: Material, Ton oder Papier mit Vorlagen-Ort) zeigt in `Auswahl` die
  `PromptButtons` mit `jeOrt(wahl.ort)` aus `useVorlagen`. Fehler über `errorText` (`@webkit/lib/errors`).

## Flow

1. Mount -> `assetsStyle()` (`../api.ts` -> `backend.assetsStyle`) -> Tafel.
2. Klick auf Kachel -> `Wahl` -> `Auswahl` -> `PromptButtons` (`../prompts/PromptButtons.tsx`) mit `material`/`ton`.

## Integration

- Eingehängt von `../AssetsArea.tsx` (Reiter `styleguide`).
- Abhängigkeiten: `../api.ts` (Typen `Stil`, `MaterialToene`), `../prompts/`, `@webkit/atoms`.
