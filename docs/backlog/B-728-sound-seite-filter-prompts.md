# B-728 · Die Sound-Seite hat Filter und Prompt-Buttons

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ45
- **Projekt:** WZG
- **Erstellt:** 2026-11-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: alle offenen angehen)

## Ausgangslage

Die Seite Sound im Bereich Assets (`tools/k3c-workbench/frontend/src/assets/sound/SoundPage.tsx`) listet die Sound-Slots nach Kategorie mit Status und Hörprobe, ohne Filter und ohne Prompt-Buttons. CU1.4 stellt die Quelle auf `data/sounds.json` um.

## Ziel

Die Sound-Seite lässt sich filtern (Text, Kategorie, Status) und bietet je Slot Prompt-Buttons (z. B. Sound suchen, anderen Kandidaten zuordnen, Lautstärke prüfen).

## Beteiligte und Zielgruppen

🧑 Ralf hört und wählt Sounds; Claude setzt die Prompts über MCP um.

## Anforderungen

Auftrag Ralf (2026-11-07, über Sprint WB2). Filter wie im Katalog bzw. `@webkit/molecules` › `ListFilter`, Prompt-Buttons wie im Katalog, kurze Prompts (B-639); Layout gern mit `@webkit/templates` › `ListDetail` (WB2).

## Nicht-Ziele

Sounds in der Workbench zuordnen oder bearbeiten.

## Regeln und Einschränkungen

Datenquelle `data/sounds.json`. Neue Vorlagen-Orte im Vertrag `docs/standards/paper-mcp.md` und in den Fixtures nachziehen.

- **Bereich:** `tools/k3c-workbench/frontend/src/assets/sound/**`, `tools/k3c-workbench/frontend/src/assets/prompts/**`, `data/asset-prompts.json`, `docs/standards/paper-mcp.md`, `tools/paper/fixtures/ui/**`
- **Abhängig von:** B-727
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Filter „huf“ zeigt nur die Hufschlag-Slots; Button „Anderen Sound suchen“ auf `ride.loop` schickt Slot-Id und aktuelle Datei.

## Ausnahme- und Fehlerfälle

Kein Treffer: „Keine Sounds“, Filter bleibt sichtbar.

## Akzeptanzkriterien

- **AC-01** Filter nach Text, Kategorie und Status (Test).
- **AC-02** Je Slot Prompt-Buttons mit Slot-Id und Datei im Prompt (Test).

## Offene Fragen

keine. Entschieden (Default, autonom Claude, 2026-11-14): Aktionen je Slot sind „anderen Sound suchen“, „Kandidaten vergleichen“ und „Lautstärke prüfen“.

## Notizen

Kein offenes Ticket gefunden; verwandt: B-599 (erledigt, altes Studio), CU1.4 (Sound-Seite liest `data/sounds.json`).
