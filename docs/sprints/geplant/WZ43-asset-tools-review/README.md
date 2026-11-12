# WZ43 · DEV · Asset-MCP-Tools und Review: Restbefunde, Teil-Patch, Basis-Stil, Katalog belegt

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-706, B-710, B-703, B-705, B-709, B-732, B-733, B-734
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12 (Gruppe von Ralf zur Umsetzung freigegeben; B-703, B-705, B-706, B-710 Klasse A)

## Ausgangslage

Aus den Reviews MC1, MG1 und RV1 sind Befunde an den Asset-MCP-Tools und der Review-Seite offen: Restbefunde (B-706), `assets_set` ohne Teil-Patch (B-710), eigener Transport der Review-Seite statt WB1-Backend (B-703), Vorher-Bild mit falschem Stil (B-705), fehlende Reittier-Boxen und ungerundete Prozente (B-709). Katalog-Tempo, Größenvergleich und Teil-Ausblenden (B-732 bis B-734) sind mit #514 umgesetzt, aber nicht belegt. PT2 liefert die CLI-Teile (`--stil`, Boxen `reittier:`).

## Ziel

Die Asset-Tools halten den Vertrag `docs/standards/paper-mcp.md` auch in Randfällen, ändern einzelne Teile gezielt, und das Review zeigt Vorher-Bilder im Stil der Basis; die Katalog-Tickets sind belegt.

Am Ende sichtbar: `assets_set` mit `{teile: {kopf: {…}}}` ändert nur `kopf`; Review mit reinem Stilwechsel zeigt `pixelAnteil > 0`; `assets_guide` zeigt `55 %`.

## Beteiligte und Zielgruppen

Claude (Umsetzung WZ43.1–WZ43.3, Review WZ43.5); 🧑 Ralf prüft Katalog-Vorschau und Teil-Ausblenden im Web-Client (WZ43.4, Gerät PC).

## Anforderungen

`B-706 › Anforderungen`, `B-710 › Ziel`, `B-703 › Ziel`, `B-705 › Ziel`, `B-709 › Ziel`, `B-732 › Akzeptanzkriterien`, `B-733 › Akzeptanzkriterien`, `B-734 › Akzeptanzkriterien`.

## Nicht-Ziele

Prompt-Buttons und Filter für Styleguide, Sound und Quellen (B-727 bis B-729, WZ45); Paper-CLI selbst (PT2); „Im Spiel ansehen“ auf das native Spiel umstellen (Teil-Ticket von B-823).

## Regeln und Einschränkungen

Domäne DEV; Oberfläche nur über `@webkit` (Atomic Design, `tests/atomicDesign.test.ts`); Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; Vertrag `docs/standards/paper-mcp.md` nur zusammen mit Code. Neubau der Workbench-EXE zum Nutzen der Tools (Sperre `exe`). Browser-Prüfungen nur durch 🧑 (WZ43.4) oder mit dessen Freigabe für den Lauf.

- **Bereich:** `tools/k3c-workbench/internal/paperassets/**`, `tools/k3c-workbench/internal/review/**`, `tools/k3c-workbench/internal/assetsui/**`, `tools/k3c-workbench/internal/mcpsrv/tools_assets*.go`, `tools/k3c-workbench/internal/mcpsrv/testdata/**`, `tools/k3c-workbench/frontend/src/assets/**`, `docs/standards/paper-mcp.md`
- **Abhängig von:** WZ42, PT2
- **Braucht 🧑:** Abnahme
- **Sperre:** exe

## Beispiele

Nase des Monarchen auf `haut`: `assets_set monarch {teile: {kopf: {lagen: […]}}}` → nur `kopf` geändert, Reihenfolge bleibt. Stilwechsel `koenigsrot` → Review zeigt Unterschied im Vorher/Nachher.

## Ausnahme- und Fehlerfälle

Teil-Patch auf unbekannte `id` → Ablehnung mit Feld und gültigen IDs. bun fehlt → Hinweis auf `requirements.md`. Paralleles `assets_create` mit gleicher ID in zwei Gruppen → eine Anlage scheitert mit klarer Meldung.

## Akzeptanzkriterien

- **AC-01** Je Punkt aus `B-706 › Anforderungen` ein Test oder eine Vertragsänderung (`B-706/AC-01`).
- **AC-02** Teil-Patch ändert genau ein Teil (`B-710/AC-01`).
- **AC-03** Review-Seite ohne eigenen Transport, eine Prompt-Füllfunktion, Vitest und `tsc` grün (`B-703/AC-01` bis `B-703/AC-03`).
- **AC-04** Vorher-Bild mit Stil der Basis (`B-705/AC-02`; `B-705/AC-01` aus PT2).
- **AC-05** `assets_preview` mit Reittier-Boxen, `assets_guide` rundet (`B-709/AC-02`; `B-709/AC-01` aus PT2).
- **AC-06** Katalog-Kriterien mit Tests belegt (`B-732/AC-01` bis `AC-03`, `B-732/AC-05`, `B-733/AC-01` bis `AC-03`, `B-734/AC-01`).
- **AC-07** Sichtprüfung im Web-Client durch 🧑 (`B-732/AC-04`, `B-734/AC-02`).

## Offene Fragen

keine blockierenden. Defaults (Claude, 2026-11-12, Meldung im Nachgang): `assets_style_set` rendert nur die betroffenen Assets, der Vertragstext folgt (B-706); Teil-Patch als Objekt `teile` nach `id` (B-710); Größenvergleich wie mit #514 umgesetzt (B-733 › Offene Fragen).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ43.1 | `WZ43.1-asset-tools-restbefunde.md` | Umsetzung | autonom | offen |
| WZ43.2 | `WZ43.2-review-backend-stil.md` | Umsetzung | autonom | offen |
| WZ43.3 | `WZ43.3-katalog-belegen.md` | Umsetzung | autonom | offen |
| WZ43.4 | `WZ43.4-sichtpruefung-pc.md` | Workshop | Mensch | offen |
| WZ43.5 | `WZ43.5-review.md` | Review | autonom | offen |

Keine eigenen Sim- oder Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
