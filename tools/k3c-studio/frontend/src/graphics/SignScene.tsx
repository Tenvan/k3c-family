import { Text } from '@webkit/atoms';
import { drawBuildSign, SIGN } from '@game/scenes/render/buildSignView';
import { canvasGraphics } from './canvasGraphics';
import { zeichneHintergrund } from './backdrop';
import { Pixel, ladeBild, pixelContext } from './pixel';
import { GERUEST_ID, SIGN_KINDS, SIGN_SAMPLES, type Licht } from './gameScenesLogic';
import type { GebaeudeEintrag } from './scenesLogic';

// Szene „Schilder und Gerüste“ (B-598, B-584/B-586): die drei Bauschilder über `drawBuildSign` des Spiels, das Baugerüst der Burg
// aus dem Gebäude-Index; Maßstab ×3.

const SCALE = 3;
const W = 60;
const H = 70;
const GROUND = 60;

function schild(c: HTMLCanvasElement, kind: (typeof SIGN_SAMPLES)[number]['kind'], biom: string, licht: Licht): Promise<void> {
  const g = pixelContext(c, W * SCALE, H * SCALE);
  if (g) {
    g.scale(SCALE, SCALE);
    zeichneHintergrund(g, W, H, GROUND, biom, licht);
    g.translate(W / 2, GROUND);
    drawBuildSign(canvasGraphics(g), kind);
  }
  return Promise.resolve();
}

async function geruest(c: HTMLCanvasElement, datei: string, biom: string, licht: Licht): Promise<void> {
  const img = await ladeBild(datei);
  const g = pixelContext(c, img.width * 2, (img.height + 10) * 2);
  if (!g) return;
  g.scale(2, 2);
  zeichneHintergrund(g, img.width, img.height + 10, img.height, biom, licht);
  g.drawImage(img, 0, 0);
}

export function SignScene({ biom, licht, index }: { biom: string; licht: Licht; index: readonly GebaeudeEintrag[] }) {
  const datei = index.find((e) => e.id === GERUEST_ID)?.dateien[32];
  return (
    <>
      <Text size="2" color="gray">Bauschilder wie im Spiel (Brett {SIGN.w}×{SIGN.h} px am Stock), Maßstab ×{SCALE}.</Text>
      <div className="sz-strip">
        {SIGN_SAMPLES.map((s) => (
          <figure key={s.variant} className="sz-fig">
            <Pixel draw={(c) => schild(c, s.kind, biom, licht)} drawKey={`${s.kind}:${biom}:${licht}`} />
            <figcaption>{s.variant}<br /><span className="sz-dim">{SIGN_KINDS(s.variant)}</span></figcaption>
          </figure>
        ))}
        <figure className="sz-fig">
          {datei ? <Pixel draw={(c) => geruest(c, datei, biom, licht)} drawKey={`${datei}:${biom}:${licht}`} /> : <div className="sz-gap">Lücke: {GERUEST_ID} (Gebäude-Index fehlt)</div>}
          <figcaption>Baugerüst der Burg · {GERUEST_ID}</figcaption>
        </figure>
      </div>
    </>
  );
}
