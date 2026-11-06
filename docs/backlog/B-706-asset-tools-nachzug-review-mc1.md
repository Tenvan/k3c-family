# B-706 · Asset-MCP-Tools: Restbefunde aus dem Review MC1 nachziehen

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Sprint-Review MC1 (MC1.5, 2026-11-06) hat Befunde an den Asset-MCP-Tools (`tools/k3c-workbench/internal/paperassets`, `internal/mcpsrv/tools_assets*.go`) als Minor zurückgestellt.

## Ziel

Die Tools verhalten sich auch in Randfällen wie der Vertrag `docs/standards/paper-mcp.md`, ohne doppelte CLI-Anbindung.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `golden.json` der Gruppe bei `assets_create`/`assets_delete` nachziehen (oder über `gen:paper`).
- Gültigkeits-Cache von `assets_list` berücksichtigt Abhängigkeiten (Reittier); `assets_set` auf ein Reittier nennt bzw. rendert die Reiter neu.
- Klären: `assets_style_set` rendert alle Assets (Vertragstext) oder nur die betroffenen (Fixture).
- Eine gemeinsame CLI-Anbindung und Basis-Prüfung für `paperassets` und `internal/review` (Zeitlimit, Attrappen-Rückfall, `K3C_PAPER_CLI`).
- Overlay-Namen von `assets_set` und `assets_review` getrennt; Hinweis auf `requirements.md`, wenn bun fehlt; doppelte ID über Gruppen bei parallelem `assets_create` ausschließen; beiseitegelegte Ausgabe nach `.cache/` statt neben die Ausgabe.
- Fixtures `assets_set.txt` (Pixel je Animation) und `assets_review.txt` (Stil-Pfad, vorher/nachher) mit dem Stand abgleichen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Je Punkt der Anforderungen ein Test oder eine Vertragsänderung.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
