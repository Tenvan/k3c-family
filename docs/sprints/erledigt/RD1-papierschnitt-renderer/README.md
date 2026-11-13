# RD1 · PAP · Papierschnitt-Renderer: Rig-Typen, IK, Physik, resvg-Export, Golden-Tests

- **Status:** erledigt
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-699
- **Start-Commit:** 8b0e2f18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-06, Chat (🧑: „alles bis zur Abnahme abarbeiten“; Defaults in B-701)

## Ausgangslage

Welle 1, Lane **renderer** (Domäne PAP, läuft parallel zu MC1, WB1, RV1). Verträge aus PV1: `docs/standards/paper-spec.md` (Schema, Pflicht-Teile, Pivots, Hash- und Golden-Regel, Ausgabe-Layout, CLI), `tools/paper/schema/` (Typen, Regeln `check.ts`, `hash.ts`, `form.ts`), Fixtures und CLI-Attrappe `tools/paper/fixtures/` (`cli-stub.ts`, Specs `monarch`, `pferd`, `mauer`, Prototyp-PNGs). Referenz-Verhalten im Prototyp `docs/decisions/005/prototyp-monarch-rig.html`. Bis RD1.4 rufen MC1 und RV1 die Attrappe; danach zeigt `K3C_PAPER_CLI` (Standard `tools/paper/bin/paper.ts`) auf den echten Renderer.

## Ziel

`bun tools/paper/bin/paper.ts` erfüllt den CLI-Vertrag vollständig (`validate`, `render`, `frame`, `diff`, `pixeldiff`) und rendert Monarch und Pferd aus ihren Specs wie der Prototyp; `task gen:paper` baut alle Gruppen unter `public/grafik/`.

Am Ende sichtbar: `public/grafik/figuren/monarch/galopp.png` aus der Spec, Golden-Tests grün in der CI.

## Beteiligte und Zielgruppen

Agenten der Lane `renderer` setzen um (Opus für Kern und Rigs, Sonnet für CLI-Rest und `gen:paper`). MC1 und RV1 nutzen die CLI ab RD1.4 statt der Attrappe; die Migration (MG*) rendert damit alle Assets. 🧑 gibt den Sprint frei und beurteilt den Sichtvergleich mit dem Prototyp.

## Anforderungen

Entscheidung 005 › 1–3, 7; `docs/standards/paper-spec.md`.

## Nicht-Ziele

MCP-Tools (MC1), Oberfläche (WB1, RV1), Migration weiterer Assets (MG*).

## Regeln und Einschränkungen

Nur `tools/paper/` (außer `schema/` und `fixtures/`, die Vertrag bleiben); RD1.4 zusätzlich der Task `paper` in `tools/Taskfile.yml` und die Specs samt Ausgabe unter `public/grafik/figuren/` und `public/grafik/reittiere/`. Laufzeit bun (B-701 D-03), Code nur mit `node:`-Modulen, damit Vitest die CLI in-process (`run(args, {stil?})`) prüft. Determinismus: fester Zeitschritt 1/120 s, 240 Schritte (2 s) Einschwingen, Struktur-Seed aus der Asset-ID, Koordinaten über `num()` (2 Nachkommastellen), `renderKey` aus `hash.ts` (PV1.3); Golden-Regel aus `paper-spec.md` › 6 (exakt oder Toleranz, Stand nach PV1.5). `gen:paper` schreibt je Gruppe `index.paper.json` und `golden.json`, keinen gemeinsamen Index. Tests rendern Fixture-Specs nur in Temp-Ordnern (`render` schreibt neben die Spec). Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

### Festlegungen (Default dieser Planung, Meldung an B-701 durch die Haupt-Session)

- **D-01 CLI-Pfad:** `bun tools/paper/bin/paper.ts` laut Vertrag; der frühere Entwurf `node tools/paper/bin/paper.mjs` entfällt.
- **D-02 Öffentliche Specs:** RD1.4 legt `public/grafik/figuren/monarch.paper.json` (Animationen `idle`, `lauf`, `schritt`, `trab`, `galopp`, die drei letzten mit `reittier: pferd`) und `public/grafik/reittiere/pferd.paper.json` an. Die Mauer bleibt bei den Fixtures (Golden in `tools/paper/golden/fixtures.golden.json`), weil `gen:gebaeude` alle `public/grafik/gebaeude/*.json` liest; ihr Umzug gehört zur Migration (MG*).
- **D-03 Task:** `gen:paper` als Task `paper` in `tools/Taskfile.yml` (Namensraum `gen`), kein eigenes `tools/paper/Taskfile.yml`; Einstieg `bun tools/paper/bin/gen.ts public/grafik`.
- **D-04 Zeit:** Phase von Frame `n` = `n / frames`; Pose-Zeit = Phase / `freq` der Pose (Prototyp: idle 0.5, lauf 1.6, schritt 0.9, trab 1.5, galopp 2.1); Frame `n` bei Physik-Schritt `round(n · 120 / (freq · frames))`; `fps` gilt nur fürs Abspielen.
- **D-05 Papier-Filter:** `seed` = erste 4 Byte von SHA-256(Asset-ID) als uint32 `% 1000 + 1`; `feComponentTransfer` mit `slope = struktur.staerke`, `intercept = 0.86` (Wert des Prototyps, kein Feld in der Stil-Datei).
- **D-06 Frame-Streifen:** ein SVG je Datei, je Frame ein verschachteltes `<svg>` mit `viewBox = rahmen` (Schatten bleiben im Frame); Frame-Hashes aus den RGBA-Ausschnitten des Streifens, kein eigener PNG-Encoder.
- **D-07 Keyframes:** addieren sich zu den Rig-Winkeln; `figur` kennt ohne Keyframes nur `idle` und `lauf`, sonst Exit 1 `animation-unbekannt`.
- **D-08 Umhang:** gezeichnet vor allen Teilen der Figur; Licht-Kante als `falz`; der Hermelin-Saum des Prototyps entfällt, weil die Spec kein Saum-Material hat (Vorschlag: Feld `umhang.saum`, Vertrags-Nachtrag).
- **D-09 Reiten:** Reittier-Suche im Ordner der Spec, dann `../*/<id>.paper.json`; fehlt es oder ist es kein `vierbeiner` → Exit 1 `reittier-unbekannt` (neue CLI-Regel wie `animation-unbekannt`). Reihenfolge: Reittier-Teile bis vor `hals`, Umhang, Reiter ohne `oberschenkel-h`-Kette, `hals` und Rest, Zügel (`falz`, `leder`/`schatten`, Breite 1.3).
- **D-10 Reit-Rahmen (Vertrags-Lücke):** „Rahmen des Reittiers“ fasst den Reiter nicht. Default: Hülle aus `reittier.rahmen · massstab` und `reiter.rahmen + [−2, −54]`, nach außen gerundet, Pixel je Einheit wie beim Reiter (Monarch: `[-89, -199, 190, 205]`, 242 × 261 px); das Manifest trägt die Abweichung zusätzlich je Animation (`animationen.<name>.{rahmen, frameW, frameH}`). Nachtrag in `paper-spec.md` › 7 nötig.
- **D-11 Beinpfad:** Material = erste `flaeche` von `rumpf-vorn` (Vorderbeine) bzw. `rumpf-hinten` (Hinterbeine); gezeichnet an der Stelle von `oberbein-<bein>`.
- **D-12 renderKey beim Reiten (Vertrags-Lücke):** laut Vertrag nur Spec und Stil; eine Änderung am Reittier ändert den `renderKey` des Reiters nicht. RD1 hält sich an den Vertrag; Nachtrag (Reittier-Hash in den Schlüssel) für den Cache von MC1/RV1 nötig. `gen:paper` rendert immer alles und ist nicht betroffen.

## Beispiele

- `bun tools/paper/bin/paper.ts validate tools/paper/fixtures/specs/monarch.paper.json` → Exit 0, `{"ok":true,"fehler":[]}`.
- `bun tools/paper/bin/paper.ts render public/grafik/figuren` → Exit 0, schreibt `monarch/{idle,lauf,schritt,trab,galopp}.png`, dieselben mit `@spieler2`, `manifest.json` und `index.paper.json`.
- `task gen:paper` direkt zweimal → beim zweiten Lauf ändert sich keine Datei.
- `koenigsrot` in `stil.paper.json` geändert → Monarch-Hashes ändern sich, Pferd-Hashes nicht; `renderKey` ändert sich bei allen.

## Ausnahme- und Fehlerfälle

- Ungültige Spec → Exit 1 mit `{feld, regel, text}`, keine Datei geändert; beim Ordner-Render bleibt der alte Stand aller Assets.
- Unbekannte Animation oder unbekanntes Reittier → Exit 1 (`animation-unbekannt`, `reittier-unbekannt`).
- Fehlende Datei, Option oder unbekannter Befehl → Exit 2, Text auf stderr, nichts auf stdout.
- Golden weicht auf Linux ab → RD1.5 meldet den Pixel-Anteil; `GOLDEN_REGEL` folgt nur dem Stand in `paper-spec.md` › 6, sonst Ticket.

## Akzeptanzkriterien

- **AC-01** `paper validate` erfüllt alle Regeln aus `paper-spec.md` (005/AC-01).
- **AC-02** Golden-Tests: Spec → PNG-Hash fest für Monarch (5 Animationen), Pferd (3 Gangarten), Mauer (2 Stufen); zwei Läufe byte-gleich (005/AC-02).
- **AC-03** Monarch rendert Idle, Lauf, Schritt, Trab, Galopp mit IK-Hufen, Rückengelenk und Umhang-Physik (005/AC-03); Sichtvergleich mit dem Prototyp im Ergebnis.
- **AC-04** Materialänderung in `stil.paper.json` ändert die Hashes aller betroffenen Assets ohne Spec-Änderung (005/AC-04).
- **AC-05** `frame`, `diff`, `pixeldiff` liefern die Antworten aus dem Vertrag; die Attrappe aus PV1.4 und der echte Renderer bestehen denselben Vertragstest.

## Offene Fragen

keine (Defaults unter Regeln › Festlegungen; D-08, D-10, D-12 brauchen einen Vertrags-Nachtrag)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RD1.1 | `RD1.1-kern-validate-render.md` | Umsetzung | autonom | fertig |
| RD1.2 | `RD1.2-rig-figur-umhang.md` | Umsetzung | autonom | fertig |
| RD1.3 | `RD1.3-rig-vierbeiner-reiten.md` | Umsetzung | autonom | fertig |
| RD1.4 | `RD1.4-frame-diff-gen-vertrag.md` | Umsetzung | autonom | fertig |
| RD1.5 | `RD1.5-review.md` | Review | autonom | fertig |

Reihenfolge auf dem Sprint-Branch (nacheinander, Lane `renderer`): RD1.1 → RD1.2 → RD1.3 → RD1.4 → RD1.5. Modelle: RD1.1 Opus, RD1.2 Opus, RD1.3 Opus, RD1.4 Sonnet, RD1.5 Opus.

## Abnahme

2026-11-06 (RD1.5, Claude): AC-01–AC-05 erfüllt, Nachweise in RD1.1–RD1.4 (Golden auf Linux bestätigt, CI-Run 42279806963 grün).
Sprint-Review (Opus): drei wichtige Befunde behoben (Golden-Regel im Vergleich, `validate` mit Render-Regeln, Aufräumen nur toter Prozesse); leichte Befunde als offene Punkte im Ergebnis von RD1.5.
Entscheidungen: B-701 D-RD1-01 bis D-RD1-08; Vertrags-Nachträge nötig für Reit-Rahmen, renderKey beim Reiten, Saum, neue CLI-Regeln.
Offen für 🧑: Sichtvergleich mit dem Prototyp (Monarch idle/lauf/galopp, Pferd) beurteilen.
