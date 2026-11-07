# B-729 · Die Quellen-Seite hat Filter und Prompt-Buttons

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Seite Quellen im Bereich Assets (`tools/k3c-workbench/frontend/src/assets/sources/SourcesPage.tsx`) zeigt den Katalog unter `public/` mit Lizenz-Kennzeichen und die Recherche-Treffer. Es gibt nur einen Freitext-Filter für den Katalog, keine Filter nach Lizenz, Pack oder für die Treffer, und keine Prompt-Buttons.

## Ziel

Die Quellen-Seite lässt sich nach Lizenz, Pack und Art filtern (Katalog und Treffer) und bietet Prompt-Buttons (z. B. Lizenz prüfen, Credit nachtragen, Treffer übernehmen).

## Beteiligte und Zielgruppen

🧑 Ralf prüft Lizenzen und Quellen; Claude setzt die Prompts über MCP um (`assets_research`).

## Anforderungen

Auftrag Ralf (2026-11-07, über Sprint WB2). Filter über `@webkit` (`ListFilter`, `ChipSwitch`), Prompt-Buttons wie im Katalog, kurze Prompts (B-639).

## Nicht-Ziele

Dateien in der Workbench importieren oder löschen.

## Regeln und Einschränkungen

Neue Vorlagen-Orte im Vertrag `docs/standards/paper-mcp.md` und in den Fixtures nachziehen.

## Beispiele

Chip „CC-BY“ zeigt nur Dateien mit Namensnennung; Button „Credit nachtragen“ nennt Datei und Pack.

## Ausnahme- und Fehlerfälle

Unbekannte Lizenz bleibt als eigener Chip „unbekannt“ sichtbar.

## Akzeptanzkriterien

- **AC-01** Filter nach Lizenz, Pack und Art für Katalog und Treffer (Test).
- **AC-02** Prompt-Buttons je Datei und je Treffer mit Datei, Pack und Lizenz im Prompt (Test).

## Offene Fragen

Welche Aktionen? Default: Lizenz prüfen, Credit nachtragen, Treffer übernehmen.

## Notizen

Kein offenes Ticket gefunden; verwandt: B-673 (erledigt, altes Studio).
