# B-727 · Der Styleguide hat Prompt-Buttons für Materialien und Töne

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ45
- **Projekt:** WZG
- **Erstellt:** 2026-11-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Seite Styleguide im Bereich Assets der Workbench (`tools/k3c-workbench/frontend/src/assets/style/StylePage.tsx`) zeigt Materialtafel, Einzeltöne und Papier aus der Stil-Datei (`public/grafik/stil.paper.json`), aber anders als Katalog und Review keine Prompt-Buttons.

## Ziel

Auf dem Styleguide schickt ein Klick auf ein Material, einen Ton oder das Papier einen kurzen Prompt an Claude (z. B. Material ändern, neues Material anlegen, alle Nutzer eines Materials zeigen).

## Beteiligte und Zielgruppen

🧑 Ralf pflegt den Stil; Claude setzt die Prompts über `assets_style_get`/`assets_style_set` um.

## Anforderungen

Auftrag Ralf (2026-11-07, über Sprint WB2). Prompt-Vorlagen wie die vorhandenen (`assets_prompts`, Feld `ort`), Buttons über `@webkit` (`assets/prompts/PromptButtons.tsx`), kurze Prompts ohne Anweisungen (B-639).

## Nicht-Ziele

Die Stil-Datei in der Workbench direkt bearbeiten (Änderungen nur über MCP, Entscheidung 005).

## Regeln und Einschränkungen

Neue Orte der Vorlagen (z. B. `material`, `ton`, `papier`) im Vertrag `docs/standards/paper-mcp.md` und in den Fixtures `tools/paper/fixtures/ui/AssetsPrompts.json` nachziehen.

## Beispiele

Klick auf „koenigsrot“ → Buttons „Material ändern“, „Nutzer zeigen“; der Prompt nennt Material und Stil-Datei.

## Ausnahme- und Fehlerfälle

Ohne Vorlagen für einen Ort keine Buttons, ein Hinweis statt leerer Fläche.

## Akzeptanzkriterien

- **AC-01** Material, Ton und Papier haben Prompt-Buttons mit Vorlagen aus `assets_prompts` (Test).
- **AC-02** Die Prompts setzen Material-, Ton- bzw. Papier-Namen ein (Test).

## Offene Fragen

Welche Aktionen je Material/Ton/Papier? Default: ändern, neu anlegen, Nutzer zeigen.

## Notizen

Kein vorhandenes Ticket gefunden (Backlog, aktive und geplante Sprints; nur erledigte zum alten Studio wie B-677).
