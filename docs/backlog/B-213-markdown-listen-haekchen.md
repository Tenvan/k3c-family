# B-213 · MarkdownView in k3c-dev zeigt nummerierte Listen und Häkchen wie die alte Planungsseite

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** M9
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`tools/k3c-dev/frontend/src/mcp/markdown.ts` (`parseMarkdown`, genutzt von `ui/MarkdownView.tsx`) kennt Überschriften, `-`-Listen, Tabellen, Code und Absätze, aber keine nummerierten Listen (`1.`) und keine Häkchen (`- [ ]`, `- [x]`). Seit M8.2 zeigt die Planungsseite Session-Dateien damit: „Schritte“ erscheinen als ein Fließtext-Absatz, „Fertig, wenn“ mit rohem `[ ]`. Die alte `page.html` konnte beides.

## Ziel

Session-Dateien sind im Detail-Panel der Planung und auf der MCP-Seite so lesbar wie im Editor: nummerierte Schritte als Liste, Häkchen als ☐/☑.

## Beteiligte und Zielgruppen

🧑 und Entwickler, die Sessions in k3c-dev lesen.

## Anforderungen

- `parseMarkdown` erkennt Zeilen `1. …` als geordnete Liste (`ol`), aufeinanderfolgende Punkte bilden eine Liste.
- Listenpunkte mit `[ ] ` bzw. `[x] ` am Anfang zeigen ☐ bzw. ☑.
- Weiterhin React-Elemente, nie HTML-String.

## Nicht-Ziele

Vollständiges CommonMark, verschachtelte Listen, neue Abhängigkeit (z. B. eine Markdown-Bibliothek).

## Regeln und Einschränkungen

Domäne SRV, nur `tools/k3c-dev/frontend/src/mcp/markdown.ts`, `ui/MarkdownView.tsx` und Tests. Komplexitäts-Budget.

## Beispiele

`1. Status setzen\n2. Prüfen` → zwei Punkte einer nummerierten Liste; `- [x] AC-01: …` → „☑ AC-01: …“.

## Ausnahme- und Fehlerfälle

`2026. war ein Jahr` mitten im Absatz bleibt Text (nur am Zeilenanfang mit Leerzeichen nach dem Punkt ist es eine Liste).

## Akzeptanzkriterien

- **AC-01** Vitest in `mcp/markdown.test.ts`: nummerierte Liste ergibt einen `ol`-Block mit allen Punkten.
- **AC-02** Vitest: `[ ]`/`[x]` am Listenanfang ergeben ☐/☑, im Fließtext bleiben sie unverändert.

## Offene Fragen

keine

## Notizen

Gefunden in M8.2 beim Prüfen der React-Planungsseite im Mock.
