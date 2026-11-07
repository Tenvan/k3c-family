# RD2.3 · Renderer: renderKey mit Reittier, Keyframe sichtbar

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** renderer
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** rd2/3-renderer-sichtbar-renderkey
- **Abhängig von:** –
- **Tickets:** B-713, B-714
- **Kriterien:** AC-02, AC-03

## Ziel

Eine Reittier-Änderung rendert den Reiter neu, und Keyframes schalten Teile mit `sichtbar` ein und aus (Pfeil, Lavaschleim-Spritzer).

## Kontext

Tickets B-713 (renderKey) und B-714 (`sichtbar`). `renderKey(spec, stil)` in `tools/paper/schema/hash.ts`; der Aufruf in `tools/paper/render/ausgabe.ts` (`rendereAsset`) nutzt nur `specHash(spec)`. Für Assets mit `reittier` muss der Hash der Reittier-Spec einfließen (Reittier-Spec per `specPfad` laden, siehe `tools/paper/rig/reiten.ts`; `RENDERER_VERSION` anheben, falls das Hash-Format ändert). Keyframes: `keyframes?: { t, winkel }[]` in `tools/paper/schema/types.ts`; neues Feld `sichtbar` je Teil (Record<string, boolean>) im Keyframe, in `paper-spec.schema.json`, `docs/standards/paper-spec.md` und im Renderer (`tools/paper/render/pose.ts`, `rig/animation.ts`, `bild.ts`/`svg.ts` über `ausblenden`). Ohne Übergang: gilt der letzte Keyframe mit `t` ≤ aktuellem t. Nachziehen: Pfeil der Bogenschützen (Gruppe truppen, Goblin-Bogenschütze in gegner) nur in `angriff` bis zum Abschuss, Lavaschleim-Spritzer nur in `angriff`; Spec-Dateien unter `public/grafik/<gruppe>/`. Danach `task gen:paper`, Golden neu.

## Erlaubte Dateien

- `tools/paper/schema/**`, `tools/paper/render/**`, `tools/paper/rig/**`, `tools/paper/cli/gen*.ts`
- `docs/standards/paper-spec.md`
- `public/grafik/truppen/*.paper.json`, `public/grafik/gegner/*.paper.json` (nur die betroffenen Specs)
- erzeugte PNGs und Manifeste, `tools/paper/golden/**`, `tools/paper/fixtures/**`

## Nicht-Ziele

Ein- und Ausblenden mit Übergang; weitere Assets umstellen; Rig-Proportionen (RD2.4).

## Schritte

1. Tests zuerst: renderKey ändert sich bei Pferd-Änderung (mit Reittier) und nicht ohne; Teil mit `sichtbar: false` fehlt im Frame.
2. renderKey um den Reittier-Hash erweitern.
3. `sichtbar` in Schema, Typ, Spec-Doku und Renderer.
4. Bogenschützen und Lavaschleim nachziehen.
5. `task gen:paper`, Golden neu.

## Fertig, wenn

- [ ] AC-02: Test: Pferd-Spec ändern → renderKey des Monarchen ändert sich; `gen:paper` rendert auch `figuren/monarch` (B-713/AC-01, AC-02).
- [ ] AC-03: Schema und `paper-spec.md` kennen `sichtbar`, Test grün; Pfeil nur in `angriff` bis zum Abschuss, Lavaschleim-Spritzer nur in `angriff` (B-714/AC-01 bis AC-03).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

2026-11-07, Claude.

- AC-02 umgesetzt, geprüft (`tools/paper/rig/reiten.test.ts` neu: `reittierHash`, Pferd-Spec ändern → renderKey des Monarchen ändert sich, `gen` auf einer Test-Wurzel rendert `figuren/monarch` danach neu; `check:diff` grün): Der renderKey eines Assets mit `reittier` ist `renderKey(specHash + "+" + id=specHash(Reittier)…, stilHash)`; Assets ohne Reit-Animation behalten ihren Schlüssel. `manifest.specHash` bleibt der Hash der eigenen Spec. Nur der renderKey des Monarchen ändert sich.
- AC-03 umgesetzt, geprüft: Schema (`keyframes[].sichtbar`), Typ, `check.ts` (Teil unbekannt), `paper-spec.md` und Renderer (`keyframeAusgeblendet` in `render/pose.ts`; `einfachesBild` und `figurBild`, ausgeblendetes Teil fehlt im SVG und in den Teil-Boxen). Test `render/pose.test.ts`. Gilt nicht für `vierbeiner` und Reiten (kein Bedarf, in `paper-spec.md` vermerkt).
- AC-03 Assets: Pfeil der drei Bogenschützen (`bogenschuetze`, `elite-bogenschuetze`, `goblin-bogenschuetze`) nur in `angriff` bis t = 0,7 (Frames 0–5), in `idle` und `lauf` nie; Lavaschleim-Zunge (der „Spritzer“ des Tickets ist das Teil `zunge`) nur in `angriff` zwischen t = 0,35 und 0,8. Test `tools/paper/golden/sichtbar.test.ts`.
- Alle neun Gruppen neu gerendert, Golden neu (gewollt: Pfeil und Zunge fehlen in Frames, renderKey Monarch).
- Abweichung: Das Ticket nennt „Pfeil bis zum Abschuss“; Abschuss = t 0,7 (Keyframe mit Pfeil-Drehung 388°). Wenn Ralf einen anderen Zeitpunkt will, ändert er nur das `sichtbar` im Keyframe.
