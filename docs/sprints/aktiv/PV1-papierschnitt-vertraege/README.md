# PV1 · INF, DEV, PAP · Papierschnitt-Verträge: Domäne, Spec-Schema, Styleguide, Render-, MCP- und Review-Vertrag

- **Status:** aktiv
- **Projekt:** PAP
- **Domäne:** INF, DEV, PAP
- **Reife:** bereit
- **Tickets:** B-699
- **Start-Commit:** c81160d4
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-06, Chat (🧑: „Committen und starten“; Vorab-Entscheidungen in B-701, Bestätigung in PS1)

## Ausgangslage

Entscheidung 005 (freigegeben 2026-11-06) legt Papierschnitt-Grafik aus Specs, Asset-Änderungen nur über MCP und ein Review in der Workbench fest. Umgesetzt wird in Wellen: Dieser Sprint ist **Welle 0** und legt die Verträge fest, gegen die danach vier Lanes parallel arbeiten (RD1 Renderer, MC1 MCP, WB1 Workbench-Hülle, RV1 Review). Heute sperrt eine Session ihre ganze Domäne; MC1, WB1 und RV1 liegen alle in DEV und könnten nicht parallel laufen.

## Ziel

Nach PV1 kann jede Lane ohne Rückfrage gegen feste, getestete Verträge und Fixtures arbeiten, und drei DEV-Lanes dürfen gleichzeitig laufen.

Am Ende sichtbar: `docs/standards/paper-spec.md` und `docs/standards/paper-mcp.md`, Schemas und Fixtures unter `tools/paper/`, eine resvg-gerenderte Fixture-PNG im Papierschnitt-Look, `plan_next` meldet parallele Sessions je Lane.

## Beteiligte und Zielgruppen

Agenten der Lanes RD1, MC1, WB1, RV1 und der Migration; 🧑 gibt den Sprint frei.

## Anforderungen

`B-699 › Anforderungen`, `docs/decisions/005-papierschnitt-assets-mcp.md` › Abschnitte 1–7 und › Umsetzung.

## Nicht-Ziele

Kein Renderer-Code außer der resvg-Probe (RD1), keine MCP-Tools (MC1), keine Oberfläche (WB1, RV1).

## Regeln und Einschränkungen

Verträge sind nach PV1 nur über ein Ticket änderbar. Feldnamen deutsch und ohne Umlaute in JSON-Schlüsseln (`hoehe_px`, `geaendert`). Domäne PAP gibt es seit PV1.1; PV1.6, PV1.3 und PV1.4 laufen als PAP.

## Beispiele

Agent der Lane MC1 liest `docs/standards/paper-mcp.md`, ruft im In-Process-Test die Attrappe `tools/paper/fixtures/cli-stub.ts` und bekommt die festgelegten JSON-Antworten; WB1 ruft die Stub-Methode `AssetsList` und bekommt `tools/paper/fixtures/ui/AssetsList.json`.

## Ausnahme- und Fehlerfälle

resvg rendert `feTurbulence` oder `feDropShadow` sichtbar falsch → PV1.3 hält das im Ergebnis fest und legt ein Ticket an; Fallback im Vertrag: Struktur als vorgerechnete Rauschtextur (`<image>`), Schatten als versetzte, unscharfe Kopie.

## Akzeptanzkriterien

- **AC-01** `docs/arbeitsweise.md` kennt die Domäne PAP, die Session-Felder `Lane` und `Modell` und die Modell-Wahl; die Planungstests (`tests/planningDocs.ts`, `tests/planningProjects.test.ts`) prüfen sie, bestehende Sessions ohne die Felder bleiben gültig.
- **AC-02** `plan_next` liefert zwei offene autonome Sessions derselben Domäne mit verschiedenen `Lane` als parallel möglich, sperrt gleiche Lanes, nennt je Session das Modell und schlägt höchstens vier Agenten gleichzeitig vor.
- **AC-03** `tools/paper/schema/paper-spec.schema.json`, `stil.schema.json` und `check.ts` validieren die Fixtures `monarch`, `pferd`, `mauer` und lehnen die Negativ-Fixtures ab (Hex-Farbe, unbekanntes Feld, fremdes Material, fehlendes Pflicht-Teil, Proportion) – Herkunft 005/AC-01.
- **AC-04** `docs/standards/paper-spec.md` beschreibt Spec, Stil-Datei, Lagen-Arten, Rig-Typen mit Pflicht-Teilen und Pivots, Hash- und Golden-Regel, Ausgabe-Layout und die CLI `paper validate|render|frame|diff|pixeldiff` mit JSON-Antworten.
- **AC-05** Die Fixture-SVGs aus dem Prototyp rendern mit `@resvg/resvg-js` zweimal byte-gleich; die Golden-Regel (exakt oder Toleranz) ist nach dem Linux-Lauf festgelegt – Herkunft 005/AC-02 (Machbarkeit). Sichtprüfung gegen den Browser folgt in PS1.
- **AC-06** `docs/standards/paper-mcp.md` legt alle Asset-Tools (alle mit `checkout`), die RPC-Methoden der Oberfläche und das Format von `data/asset-prompts.json` fest; `tools/paper/fixtures/` enthält CLI-Attrappe, RPC-Beispielantworten und ein Review-Test-Repo.
- **AC-07** Die RPC-Methoden der Assets, der Platzhalter `ReviewPage` und das Slider-Atom liegen als Stubs auf `develop`; `task check:dev` grün.

## Offene Fragen

keine blockierende; vorab getroffene Entscheidungen (u. a. neue Abhängigkeiten `@resvg/resvg-js`, `ajv`, Lanes/Modell-Feld) stehen in B-701 und werden in PS1.1 von 🧑 bestätigt.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| PV1.1 | `PV1.1-domaene-pap-und-lanes.md` | Umsetzung | autonom | fertig |
| PV1.2 | `PV1.2-plan-next-kennt-lanes.md` | Umsetzung | autonom | fertig |
| PV1.6 | `PV1.6-fixture-svg-export.md` | Umsetzung | autonom | fertig |
| PV1.3 | `PV1.3-spec-schema-stil-fixtures.md` | Umsetzung | autonom | fertig |
| PV1.4 | `PV1.4-mcp-und-review-vertrag.md` | Umsetzung | autonom | fertig |
| PV1.8 | `PV1.8-workbench-stubs.md` | Umsetzung | autonom | offen |
| PV1.5 | `PV1.5-review.md` | Review | autonom | offen |

Reihenfolge auf dem Sprint-Branch (nacheinander): PV1.1 → PV1.2 → PV1.6 → PV1.3 → PV1.4 → PV1.8 → PV1.5. Modelle: PV1.1 Sonnet, PV1.2 Sonnet, PV1.6 Sonnet (Browser), PV1.3 Opus, PV1.4 Opus, PV1.8 Sonnet, PV1.5 Opus.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
