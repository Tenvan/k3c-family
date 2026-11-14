# B-706 · Asset-MCP-Tools: Restbefunde aus dem Review MC1 nachziehen

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** WZ43
- **Projekt:** WZG
- **Erstellt:** 2026-11-06
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

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

Domäne DEV, Klasse A (Schuld). Vertrag `docs/standards/paper-mcp.md` nur zusammen mit Code; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

- **Bereich:** `tools/k3c-workbench/internal/paperassets/**`, `tools/k3c-workbench/internal/review/**`, `tools/k3c-workbench/internal/mcpsrv/tools_assets*.go`, `tools/k3c-workbench/internal/mcpsrv/testdata/**`, `docs/standards/paper-mcp.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** exe

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Je Punkt der Anforderungen ein Test oder eine Vertragsänderung.

## Offene Fragen

Umfang von `assets_style_set`: Default (Claude, 2026-11-12, Meldung im Nachgang) nur die betroffenen Assets neu rendern, Vertragstext folgt.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
