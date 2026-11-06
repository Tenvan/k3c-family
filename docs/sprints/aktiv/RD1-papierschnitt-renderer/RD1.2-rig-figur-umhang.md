# RD1.2 · Rig figur: Posen Idle und Lauf, Keyframes, Umhang-Physik

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** renderer
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** rd1/2-rig-figur-umhang
- **Abhängig von:** RD1.1
- **Tickets:** B-699
- **Kriterien:** AC-03

## Ziel

Der Renderer zeichnet `figur`-Specs bewegt: Posen `idle` und `lauf` wie im Prototyp, Keyframes der Spec obendrauf und
den Umhang als Verlet-Kette mit festem Zeitschritt. Monarch `idle` und `lauf` (mit Variante `spieler2`) haben feste
Golden-Hashes; das Reiten folgt in RD1.3.

## Kontext

- Stand nach RD1.1: `tools/paper/bin/paper.ts` (`run(args, {stil?})`), `render/{stil,matrix,baum,svg,raster,stufen,
  pose,ausgabe}.ts`, `golden/{regel,schreibe}.ts` und `golden/fixtures.golden.json` (Mauer). `posen(spec, animation)`
  wirft für `figur` noch „folgt in RD1.2“.
- Referenz-Verhalten: Prototyp `docs/decisions/005/prototyp-monarch-rig.html`
  - Umhang `cloth.step` (Zeilen 105–121), Zeichnung `cloth.draw` (122–135), Catmull-Rom `crPath` (137 ff.);
  - Posen `POSES.idle` (freq 0.5) und `POSES.walk` (freq 1.6) (Zeilen 339–349); Winkel in Grad, positiv = Uhrzeigersinn.
- Namen Prototyp → Spec-Teil: `thighF/B` → `oberschenkel-v/-h`, `shinF/B` → `unterschenkel-v/-h`, `upperF/B` →
  `oberarm-v/-h`, `foreF/B` → `unterarm-v/-h`, `head` → `kopf`, `crown` → `krone` (optional; Winkel für fehlende Teile
  entfallen). `mon` = Verschiebung der ganzen Figur (Basis-Matrix). `cape`/`capeW` des Prototyps entfallen (Altlast).
- Monarch-Fixture `tools/paper/fixtures/specs/monarch.paper.json`: `animationen {idle: 8, lauf: 8, reiten: 8 (reittier
  pferd, gangart galopp)}`, `umhang {anker: rumpf, punkt: [-4, -91], material: koenigsrot, laenge: 53.2, breite: [4,
  17.3], segmente: 8}`, `varianten.spieler2.tausch {koenigsblau: smaragd}`, Rahmen `[-60, -145, 85, 151]`, 192 px →
  Frame 108 × 192. Die Fixture nur lesen; Tests kopieren sie in einen Temp-Ordner.
- Zeit (Sprint-Festlegung): Phase von Frame `n` = `n / frames`; Pose-Zeit `t = phase / freq`; Physik-Schritt
  `dt = 1/120 s`, 240 Schritte Einschwingen vor Frame 0, Frame `n` liegt bei Schritt `round(n · 120 / (freq · frames))`.

## Erlaubte Dateien

- `tools/paper/rig/` (neu)
- `tools/paper/render/` (nur `pose.ts`, `svg.ts` und Tests anpassen)
- `tools/paper/golden/`
- **Nicht:** `tools/paper/schema/`, `tools/paper/fixtures/`

## Nicht-Ziele

Kein `vierbeiner`, kein Reiten (`reiten` antwortet bis RD1.3 mit Exit 2), keine Übergänge zwischen Posen (`blend` des
Prototyps), kein Seitenwechsel (`flip`), kein Boden, kein Schattenwurf am Boden.

## Schritte

Arbeitsweise: `superpowers:test-driven-development`, je Schritt erst der rote Test.

1. **Kurven** – Test `tools/paper/rig/kurve.test.ts`:
   - `it('crPath durch 3 Punkte: M plus 2 C-Segmente, Koordinaten über num()')`;
   - `it('crPath mit cont = true beginnt mit L statt M')`.
   Umsetzung `tools/paper/rig/kurve.ts`: `crPath(pts: Punkt[], cont = false): string` (Catmull-Rom als kubische Bézier wie
   Prototyp `crPath`), `spline(P: Punkt[], k: number): Punkt` (Zeile 283).
2. **Posen der Figur** – Test `tools/paper/rig/figur.test.ts`:
   - `it('idle bei Phase 0: oberarm-v 5, unterarm-v -6, oberarm-h -3, unterarm-h -4, kopf 0')`;
   - `it('lauf bei Phase 0.25 (sin = 1): oberschenkel-v -30, oberschenkel-h 30, unterschenkel-v 0')`;
   - `it('lauf verschiebt die Figur um mon = [0, 2.5·|sin| − 2]')`;
   - `it('Keyframes der Spec addieren sich zum Rig-Winkel')`;
   - `it('unbekannte Animation einer figur ohne Keyframes: Exit 1, Regel animation-unbekannt')` – Namen außer `idle`
     und `lauf` brauchen Keyframes (Regel wie in der Attrappe).
   Umsetzung `tools/paper/rig/figur.ts`: `FIGUR_POSEN: Record<'idle' | 'lauf', {freq: number; f(p: number): {winkel:
   Record<string, number>; versatz: Punkt; wind: number}}>` (`wind` = `scroll` des Prototyps: idle 0, lauf 30),
   `figurPose(spec, animation, t): Pose & {wind: number}`; `render/pose.ts` ruft sie für `rig: figur`.
3. **Umhang-Physik** – Test `tools/paper/rig/umhang.test.ts`:
   - `it('Startkette: segmente Punkte, Abstand laenge / (segmente − 1) = 7.6')`;
   - `it('schritt: dt = 1/120, Dämpfung 0.99, ax = 2.4·(luft − vx), ay = 260 − 1.2·vy, 8 Iterationen')` – nach einem
     Schritt ab Ruhelage gleich den im Test gerechneten Werten;
   - `it('kein Punkt vor anker.x + 2 und keiner unter dem Boden (y > 0)')`;
   - `it('Böe nur ohne Fahrtwind: luft = −wind·0.8 − 8 − 6·sin(1.7·zeit) − böe')`;
   - `it('deterministisch: zwei Simulationen über 240 + 60 Schritte gleich bis aufs Bit')`.
   Umsetzung `tools/paper/rig/umhang.ts`: `class Umhang { constructor(u: Spec['umhang'], anker: Punkt); schritt(anker:
   Punkt, wind: number, boden: number): void; punkte(): Punkt[] }` (Konstanten wie Prototyp `cloth.step`; der
   Anker ist `umhang.punkt` durch die Welt-Matrix von `umhang.anker`).
4. **Umhang zeichnen** – Test im selben `umhang.test.ts`:
   - `it('umhangSvg: Fläche basis mit ds, Innenseite schatten mit ds2, Licht-Kante als falz')`;
   - `it('Breite wächst linear von breite[0] bis breite[1]')` – Punkt 0 → 4, Punkt 7 → 17.3;
   - `it('Varianten-Tausch gilt auch für das Umhang-Material')`.
   Umsetzung `umhangSvg(punkte, u, stil): string` wie `cloth.draw` (L-, R-, I-Kanten; R mit Faktor 0.6, I mit 0.35);
   Licht-Kante (`capeHi`) als `falz` mit `breite 1.8` und Ton `licht`. Der Saum (`capeHem`, Hermelin) entfällt in v1,
   weil die Spec kein Saum-Material kennt (Sprint-Festlegung). Reihenfolge: Umhang vor allen Teilen der Figur.
5. **Animation rendern** – Test `tools/paper/rig/animation.test.ts`:
   - `it('Frame-Schritte: idle 8 Frames → Schritt 0, 30, 60 …; lauf → round(n · 9.375)')`;
   - `it('Einschwingen: 240 Schritte vor Frame 0, Pose läuft rückwärts periodisch weiter')`;
   - `it('render monarch ohne reiten: idle.png, lauf.png, idle@spieler2.png, lauf@spieler2.png')` – Hilfs-Spec = Fixture
     ohne `reiten` im Temp-Ordner.
   Umsetzung `tools/paper/rig/animation.ts`: `frameSchritte(freq, frames): number[]`, `simuliere(spec, animation,
   stil): {pose: Pose; umhang?: Punkt[]}[]`; `render/svg.ts` fügt den Umhang ein.
6. **Golden Monarch** – `golden.test.ts` erweitern:
   `it('monarch idle und lauf (mit spieler2): zweimal byte-gleich und gleich fixtures.golden.json')`;
   `golden/schreibe.ts` rendert dafür die Monarch-Fixture ohne `reiten` (bis RD1.3); Golden neu schreiben, Hashes im
   Ergebnis.
7. **Sichtvergleich** (AC-03 teilweise): Frame 0 von `idle.png` (aus dem Temp-Render) und
   `tools/paper/fixtures/render/monarch/monarch-idle-0.png` mit dem Read-Tool ansehen; Abweichungen (Umhang-Lage,
   fehlender Saum, Farben) im Ergebnis beschreiben. Keine Overlay-PNGs committen.
8. Refactor, `superpowers:requesting-code-review` (Rig-Mathematik), `superpowers:verification-before-completion`.

## Fertig, wenn

- [ ] AC-03 (teilweise): `figur.test.ts`, `umhang.test.ts`, `animation.test.ts` grün; Sichtvergleich Idle im Ergebnis.
- [ ] `golden.test.ts` grün mit Monarch `idle.png`, `lauf.png`, `idle@spieler2.png`, `lauf@spieler2.png`.
- [ ] `task test -- tools/paper` grün, `task check:diff` grün.

## Prüfen

```bash
task test -- tools/paper
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
