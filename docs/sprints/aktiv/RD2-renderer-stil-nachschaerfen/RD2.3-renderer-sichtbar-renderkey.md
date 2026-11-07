# RD2.3 · Renderer: renderKey mit Reittier, Keyframe sichtbar

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
