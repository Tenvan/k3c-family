# B-727 · Der Styleguide hat Prompt-Buttons für Materialien und Töne

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ45
- **Projekt:** WZG
- **Erstellt:** 2026-11-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: alle offenen angehen)

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

Neue Orte der Vorlagen (`material`, `ton`, `papier`) im Vertrag `docs/standards/paper-mcp.md` und in den Fixtures `tools/paper/fixtures/ui/AssetsPrompts.json` nachziehen.

- **Bereich:** `tools/k3c-workbench/frontend/src/assets/style/**`, `tools/k3c-workbench/frontend/src/assets/prompts/**`, `data/asset-prompts.json`, `docs/standards/paper-mcp.md`, `tools/paper/fixtures/ui/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Klick auf „koenigsrot“ → Buttons „Material ändern“, „Nutzer zeigen“; der Prompt nennt Material und Stil-Datei.

## Ausnahme- und Fehlerfälle

Ohne Vorlagen für einen Ort keine Buttons, ein Hinweis statt leerer Fläche.

## Akzeptanzkriterien

- **AC-01** Material, Ton und Papier haben Prompt-Buttons mit Vorlagen aus `assets_prompts` (Test).
- **AC-02** Die Prompts setzen Material-, Ton- bzw. Papier-Namen ein (Test).

## Offene Fragen

keine. Entschieden (Default, autonom Claude, 2026-11-14): Aktionen je Material, Ton und Papier sind „ändern“, „neu anlegen“ und „Nutzer zeigen“.

## Notizen

Kein vorhandenes Ticket gefunden (Backlog, aktive und geplante Sprints; nur erledigte zum alten Studio wie B-677).
