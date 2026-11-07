# RD2.1 · Stil: Schatten und Papierstruktur wie der Prototyp

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** stil
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** rd2/1-stil-schatten
- **Abhängig von:** –
- **Tickets:** B-712
- **Kriterien:** AC-01

## Ziel

Schatten und Papierstruktur der resvg-Ausgabe wirken bei gleicher Anzeigegröße wie im Prototyp; alle Gruppen sind neu gerendert, Golden und Tests grün.

## Kontext

Ticket B-712 (Sprint RD2). Messung beim Planen: Die Stil-Werte `papier.lage`, `auflage`, `struktur` in `public/grafik/stil.paper.json` sind schon identisch mit dem Prototyp `docs/decisions/005/prototyp-monarch-rig.html` (Zeilen 321–327). Die Ursache der schwächeren Wirkung ist der Maßstab: Der Prototyp zeigt 240 Einheiten auf 720 px (3 px je Einheit), der Monarch-Frame hat Rahmen 151 Einheiten auf 192 px (1,27 px je Einheit). Schatten (Einheiten) und Rausch-Frequenz (je Einheit) stehen aber in Einheiten, die Schatten werden also unter 1 px dünn und das Rauschen liegt im Pixelraster. Lösung: Die Filter sind in Prototyp-Pixeln definiert (neues Stil-Feld `papier.bezug_px_je_einheit`, Wert 3) und der Renderer rechnet sie je Ausgabe auf die tatsächliche Pixeldichte um (Schatten × bezug/dichte, Rausch-Frequenz × dichte/bezug; Faktor geklemmt auf 1…4). Relevante Dateien: `tools/paper/render/svg.ts` (`defs`), `tools/paper/render/ausgabe.ts` (Aufruf, `FrameGroesse` aus `raster.ts`), `tools/paper/schema/types.ts` und `stil.schema.json` (neues Feld), `docs/standards/paper-spec.md` (Abschnitte 2, 3, 6), `tools/paper/golden/` (Golden aller Gruppen). Alle Assets rendern neu (`task gen:paper`).

## Erlaubte Dateien

- `tools/paper/render/svg.ts`, `tools/paper/render/ausgabe.ts`, `tools/paper/render/*.test.ts`
- `tools/paper/schema/types.ts`, `tools/paper/schema/stil.schema.json`, `tools/paper/schema/*.test.ts`
- `public/grafik/stil.paper.json`
- `docs/standards/paper-spec.md`, `docs/backlog/B-712-*.md` (Notiz zur Ursache über `plan_sections`)
- `tools/paper/golden/**`, `tools/paper/fixtures/**`, alle von `gen:paper` erzeugten Dateien unter `public/grafik/<gruppe>/`

## Nicht-Ziele

Neue Materialien und Umstellung der Assets (RD2.2); Formänderungen an Assets; Änderungen an `lage.farbe`.

## Schritte

1. Test zuerst: `defs` mit Rahmenhöhe/Pixelhöhe liefert bei 3 px je Einheit die Prototyp-Werte, bei 1,27 px je Einheit die umgerechneten; Faktor geklemmt.
2. Stil-Feld `bezug_px_je_einheit` in Schema, Typ und `stil.paper.json` ergänzen; `paper-spec.md` anpassen.
3. `defs` und Aufruf in `ausgabe.ts` umstellen.
4. `task gen:paper` (alle Gruppen), Golden neu schreiben (`schreibe.ts`).
5. Monarch idle 4 neben dem Prototyp-Screenshot ablegen (Datei im Ergebnis nennen) für Ralfs Sichtprüfung.

## Fertig, wenn

- [ ] AC-01: Schatten und Struktur bei gleicher Anzeigegröße wie der Prototyp (Agent: Vergleichsbild erzeugt und unter `.omc/artifacts/` abgelegt; Sichtprüfung durch Ralf steht offen).
- [ ] `bun test` in `tools/paper` grün, Golden aller Gruppen neu.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

2026-11-07, Claude.

- AC-01 umgesetzt, Sichtprüfung offen bei Ralf: Ursache war nicht der Wert (identisch mit dem Prototyp), sondern der Maßstab (Prototyp 3 px je Einheit, Monarch-Frame 1,27). Neues Stil-Feld `papier.bezug_px_je_einheit` = 3; `defs(stil, id, dichte)` rechnet Schatten (× bezug/dichte, Faktor 1…4) und Rausch-Frequenz (÷ Faktor) um, Aufrufer `ausgabe.ts` und `cli/frame.ts` übergeben `frameH / rahmen[3]`. Vergleichsbild vorher/nachher: `.omc/artifacts/rd2-1/vergleich.png` (Monarch idle 1; nachher sichtbarer Schatten und Papierkörnung). Der Vergleich neben dem Prototyp im Browser ist Ralfs Abnahme.
- Getestet: `svg.test.ts` (Bezugsdichte = Prototyp-Werte, Umrechnung bei 1,27 px, Klemmung 1…4), Schema-Tests, `task check:diff` grün.
- Alle neun Gruppen neu gerendert (`task gen:paper`), `fixtures.golden.json` neu (gewollte Änderung: alle Hashes, weil sich die Filter ändern).
- Abweichung vom Tickettext: B-712 sagt „Stil-Werte angleichen“; die Werte waren schon gleich, angeglichen wurde deshalb die Wirkung über das neue Feld.
- Neue Tickets: keine.
