import { useLayoutEffect, useRef } from 'react';
import type { Figur, Hintergrund } from './sizeData';

/** Szene des Größenvergleichs (B-733) in Spiel-Pixeln wie `src/core/constants.ts`: 1080 hoch, Boden bei 900, 1 Unit =
 *  32 px; Hintergrund-Kacheln 768 × 480 (`worldSprites.ts`). Das SVG skaliert die ganze Szene auf die Spaltenbreite. */
export const SZENE_H = 1080;
export const BODEN = 900;
export const UNIT = 32;
const KACHEL = 768;
const KACHEL_H = 480;
const MIN_BREITE = 2 * KACHEL;
/** Platz für den Maßstab links und Abstand zwischen den Figuren in Spiel-Pixeln. */
const RAND = 96;
const ABSTAND = 64;
const SCHRIFT = 24;

interface Props {
  asset: Figur;
  monarch: Figur;
  hintergrund?: Hintergrund;
  /** Bildschirm-Pixel je Spiel-Pixel (Zoom-Stufe, zoom.tsx). */
  zoom: number;
}

/** Ausdehnung einer Figur in Spiel-Pixeln: links und rechts vom Fuß, Höhe über dem Fuß. */
function masse(f: Figur) {
  const [x, y, b] = f.rahmen;
  const u = f.pxProEinheit;
  return { links: -x * u, rechts: (x + b) * u, oben: -y * u };
}

/** Szene: Asset links mit Blick nach rechts, Monarch rechts gespiegelt (zugewandt), Maßstab hoch und breit. */
export function SizeScene({ asset, monarch, hintergrund, zoom }: Props) {
  const a = masse(asset);
  const m = masse(monarch);
  const noetig = RAND + a.links + a.rechts + ABSTAND + m.links + m.rechts + RAND;
  const breite = Math.max(MIN_BREITE, Math.ceil(noetig / KACHEL) * KACHEL);
  const mitte = RAND + (breite - RAND) / 2;
  const fussAsset = mitte - ABSTAND / 2 - a.rechts;
  // gespiegelt: was beim Monarchen rechts vom Fuß liegt, liegt jetzt links
  const fussMonarch = mitte + ABSTAND / 2 + m.rechts;
  const rahmen = useZumBoden(mitte * zoom, zoom);
  return (
    <div className="as-zoom" ref={rahmen}>
      <svg viewBox={`0 0 ${breite} ${SZENE_H}`} className="sz-scene" role="img" data-size-scene data-breite={breite} data-zoom={zoom}
        width={breite * zoom} height={SZENE_H * zoom}
        aria-label={`Größenvergleich ${asset.id} neben ${monarch.id}`}>
        <rect x={0} y={0} width={breite} height={BODEN} className="sz-himmel" />
        {hintergrund && <Ebenen hintergrund={hintergrund} breite={breite} />}
        <rect x={0} y={BODEN} width={breite} height={SZENE_H - BODEN} className="sz-boden" />
        <Bild f={asset} fuss={fussAsset} data="asset" />
        <Bild f={monarch} fuss={fussMonarch} data="monarch" spiegeln />
        <Hoehe x={fussAsset - a.links} breite={a.links + a.rechts} oben={a.oben} />
        <MassstabHoch />
        <MassstabBreit breite={breite} />
      </svg>
    </div>
  );
}

/** Scrollt den Rahmen beim Zoomen so, dass die Figuren in der Mitte und der Boden unten zu sehen sind. */
function useZumBoden(mitteX: number, zoom: number) {
  const ref = useRef<HTMLDivElement>(null);
  useLayoutEffect(() => {
    const el = ref.current;
    if (!el) return;
    el.scrollLeft = Math.max(0, mitteX - el.clientWidth / 2);
    el.scrollTop = el.scrollHeight;
  }, [mitteX, zoom]);
  return ref;
}

function Ebenen({ hintergrund, breite }: { hintergrund: Hintergrund; breite: number }) {
  const kacheln = Array.from({ length: Math.ceil(breite / KACHEL) }, (_, k) => k * KACHEL);
  return (
    <g data-hintergrund>
      {[hintergrund.fern, hintergrund.mitte, hintergrund.nah].map((bild, i) =>
        kacheln.map((x) => <image key={`${i}-${x}`} href={bild} x={x} y={BODEN - KACHEL_H} width={KACHEL} height={KACHEL_H} />))}
    </g>
  );
}

/** Bild einer Figur mit dem Fuß bei (fuss, BODEN); `spiegeln` dreht die Blickrichtung. */
function Bild({ f, fuss, spiegeln = false, data }: { f: Figur; fuss: number; spiegeln?: boolean; data: string }) {
  const [x, y, b, h] = f.rahmen;
  const u = f.pxProEinheit;
  return (
    <g transform={`translate(${fuss} ${BODEN}) scale(${spiegeln ? -1 : 1} 1)`} data-figur={data} data-px-pro-einheit={u}>
      <image href={f.bild} x={x * u} y={y * u} width={b * u} height={h * u} preserveAspectRatio="none" />
    </g>
  );
}

const units = (px: number) => (px / UNIT).toLocaleString('de-DE', { maximumFractionDigits: 1 });

/** Höhe des Assets: Linie an der Oberkante mit Beschriftung in Units und Spiel-Pixeln. */
function Hoehe({ x, breite, oben }: { x: number; breite: number; oben: number }) {
  const y = BODEN - oben;
  return (
    <g className="sz-hoehe" data-hoehe-px={Math.round(oben)}>
      <line x1={x - 16} x2={x + breite + 16} y1={y} y2={y} />
      <text x={x} y={y - 10} fontSize={SCHRIFT}>{units(oben)} Units · {Math.round(oben)} px · {units(breite)} breit</text>
    </g>
  );
}

/** Maßstab hoch am linken Rand: Strich je Unit, Zahl alle 5 Units, vom Boden bis zur Oberkante der Szene. */
function MassstabHoch() {
  const n = Math.floor(BODEN / UNIT);
  return (
    <g className="sz-massstab" data-massstab="hoch">
      <line x1={RAND / 2} x2={RAND / 2} y1={BODEN} y2={BODEN - n * UNIT} />
      {Array.from({ length: n + 1 }, (_, k) => (
        <g key={k}>
          <line x1={RAND / 2} x2={RAND / 2 + (k % 5 ? 10 : 22)} y1={BODEN - k * UNIT} y2={BODEN - k * UNIT} />
          {k % 5 === 0 && k > 0 && <text x={RAND / 2 - 6} y={BODEN - k * UNIT + 8} fontSize={SCHRIFT} textAnchor="end">{k}</text>}
        </g>
      ))}
    </g>
  );
}

/** Maßstab breit unter dem Boden: Strich je Unit, Zahl alle 5 Units. */
function MassstabBreit({ breite }: { breite: number }) {
  const n = Math.floor((breite - RAND / 2) / UNIT);
  const y = BODEN + 24;
  return (
    <g className="sz-massstab" data-massstab="breit">
      <line x1={RAND / 2} x2={RAND / 2 + n * UNIT} y1={y} y2={y} />
      {Array.from({ length: n + 1 }, (_, k) => (
        <g key={k}>
          <line x1={RAND / 2 + k * UNIT} x2={RAND / 2 + k * UNIT} y1={y} y2={y + (k % 5 ? 10 : 22)} />
          {k % 5 === 0 && <text x={RAND / 2 + k * UNIT} y={y + 50} fontSize={SCHRIFT} textAnchor="middle">{k}</text>}
        </g>
      ))}
      <text x={breite - 16} y={y + 100} fontSize={SCHRIFT} textAnchor="end">Units (1 Unit = 32 px)</text>
    </g>
  );
}
