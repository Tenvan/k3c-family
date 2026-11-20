import type { Stufen } from '../api';
import type { Figur, Hintergrund } from './sizeData';

/** Szene des Größenvergleichs (B-733, B-1170) in Spiel-Pixeln wie `src/core/constants.ts`: Boden bei 900, 1 Unit = 32 px;
 *  Hintergrund-Kacheln 768 × 480 (`worldSprites.ts`). Die Szene füllt das Fenster: das SVG skaliert über viewBox auf die
 *  verfügbare Breite und Höhe (keine Zoomstufen). Links steht das Lineal in M (1 M = berittener Monarch, `bezugPx` aus
 *  data/sizes.json), die Größenstufen sind als Marken und Linien eingezeichnet. */
export const BODEN = 900;
export const UNIT = 32;
const KACHEL = 768;
const KACHEL_H = 480;
/** Mindestmaße der Szene in Spiel-Pixeln: klein genug, dass die Figuren das Fenster füllen, groß genug für Lineal und Beschriftung. */
const MIN_BREITE = 900;
const MIN_HOEHE = 300;
/** Platz für das Lineal links und Abstand zwischen den Figuren in Spiel-Pixeln. */
const RAND = 120;
const ABSTAND = 64;
const SCHRIFT = 18;
/** Streifen unter dem Boden für den Maßstab in Units. */
const UNTEN = 84;
/** Luft über der höchsten Marke, damit Beschriftungen nicht am Rand kleben. */
const LUFT = 1.2;

interface Props {
  asset: Figur;
  monarch: Figur;
  hintergrund?: Hintergrund;
  /** Größenstufen (data/sizes.json); ohne sie fehlt das Lineal in M, die Maßstäbe in Units bleiben. */
  stufen?: Stufen;
  /** Stufe des Assets laut sizes.json: ihre Höhe bekommt eine Linie. */
  assetStufe?: string;
  /** Vom Nutzer gewählte Stufe: Linie über die ganze Szene. */
  markiert?: string;
  /** Maße der Szene (viewBox) von außen: Vorher und Nachher teilen sie, damit gleiche M gleich groß aussehen. */
  mass?: SzenenMass;
}

/** Breite und Höhe der Szene in Spiel-Pixeln über dem Boden (ohne den Streifen für die Units). */
export interface SzenenMass {
  breite: number;
  hoehe: number;
}

/** Ausdehnung einer Figur in Spiel-Pixeln: links und rechts vom Fuß, Höhe über dem Fuß. */
function masse(f: Figur) {
  const [x, y, b] = f.rahmen;
  const u = f.pxProEinheit;
  return { links: -x * u, rechts: (x + b) * u, oben: -y * u };
}

/** Höhe einer Stufe in Spiel-Pixeln (0, wenn es sie nicht gibt). */
export const stufenPx = (s: Stufen | undefined, id: string | undefined): number => {
  const m = s?.stufen.find((x) => x.id === id)?.m;
  return m === undefined || !s ? 0 : m * s.bezugPx;
};

const zahl = (n: number) => n.toLocaleString('de-DE', { maximumFractionDigits: 2 });

/** Maße der Szene aus Asset, Monarch, Stufen und gewählter Stufe. */
export function szenenMass(asset: Figur, monarch: Figur, stufen?: Stufen, assetStufe?: string, markiert?: string): SzenenMass {
  const a = masse(asset);
  const m = masse(monarch);
  const bezug = stufen?.bezugPx ?? m.oben;
  const noetig = RAND + a.links + a.rechts + ABSTAND + m.links + m.rechts + RAND;
  const hoch = Math.max(a.oben, m.oben, bezug * 1.3, stufenPx(stufen, assetStufe), stufenPx(stufen, markiert));
  return { breite: Math.max(MIN_BREITE, Math.ceil(noetig)), hoehe: Math.max(MIN_HOEHE, Math.ceil(hoch * LUFT)) };
}

/** Das größere von zwei Maßen je Richtung (für Vorher und Nachher). */
export const groesseresMass = (x: SzenenMass, y: SzenenMass): SzenenMass => ({ breite: Math.max(x.breite, y.breite), hoehe: Math.max(x.hoehe, y.hoehe) });

/** Szene: Asset links mit Blick nach rechts, Monarch rechts gespiegelt (zugewandt), Lineal in M links, Units unten. */
export function SizeScene({ asset, monarch, hintergrund, stufen, assetStufe, markiert, mass }: Props) {
  const a = masse(asset);
  const m = masse(monarch);
  const bezug = stufen?.bezugPx ?? m.oben;
  const { breite, hoehe } = mass ?? szenenMass(asset, monarch, stufen, assetStufe, markiert);
  const mitte = RAND + (breite - RAND) / 2;
  const fussAsset = mitte - ABSTAND / 2 - a.rechts;
  // gespiegelt: was beim Monarchen rechts vom Fuß liegt, liegt jetzt links
  const fussMonarch = mitte + ABSTAND / 2 + m.rechts;
  const oben = BODEN - hoehe;
  const stufeAsset = stufen?.stufen.find((s) => s.id === assetStufe);
  return (
    <div className="sz-fill" data-size-fill style={{ aspectRatio: `${breite} / ${hoehe + UNTEN}`, ['--sz-ratio' as string]: breite / (hoehe + UNTEN) }}>
      <svg viewBox={`0 ${oben} ${breite} ${hoehe + UNTEN}`} className="sz-scene" role="img" data-size-scene data-breite={breite} data-hoehe={hoehe}
        preserveAspectRatio="xMidYMax meet" aria-label={`Größenvergleich ${asset.id} neben ${monarch.id}`}>
        <rect x={0} y={oben} width={breite} height={hoehe} className="sz-himmel" />
        {hintergrund && <Ebenen hintergrund={hintergrund} breite={breite} />}
        <rect x={0} y={BODEN} width={breite} height={UNTEN} className="sz-boden" />
        <Bild f={asset} fuss={fussAsset} data="asset" />
        <Bild f={monarch} fuss={fussMonarch} data="monarch" spiegeln />
        {stufen && <Lineal stufen={stufen} oben={oben} breite={breite} markiert={markiert} />}
        {stufen && <Linie px={bezug} breite={breite} text="1 M = Monarch" klasse="sz-eins" data="eins" />}
        {stufen && stufeAsset && <Linie px={stufeAsset.m * bezug} breite={breite} text={`Stufe ${stufeAsset.id}: ${zahl(stufeAsset.m)} M`} klasse="sz-stufe" data="stufe" unten={Math.abs(stufeAsset.m * bezug - bezug) < 3 * SCHRIFT} />}
        <Hoehe x={fussAsset - a.links} breite={a.links + a.rechts} oben={a.oben} bezug={stufen?.bezugPx} />
        <MassstabBreit breite={breite} />
      </svg>
    </div>
  );
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

/** Höhe des Assets: Linie an der Oberkante mit Beschriftung in M (wenn der Bezug bekannt ist), Units und Spiel-Pixeln. */
function Hoehe({ x, breite, oben, bezug }: { x: number; breite: number; oben: number; bezug?: number }) {
  const y = BODEN - oben;
  const m = bezug ? `${zahl(oben / bezug)} M · ` : '';
  return (
    <g className="sz-hoehe" data-hoehe-px={Math.round(oben)} data-hoehe-m={bezug ? zahl(oben / bezug) : undefined}>
      <line x1={x - 16} x2={x + breite + 16} y1={y} y2={y} />
      <text x={x} y={y - 8} fontSize={SCHRIFT}>{m}{units(oben)} Units · {Math.round(oben)} px · {units(breite)} breit</text>
    </g>
  );
}

/** Waagerechte Linie über die Szene in `px` Höhe über dem Boden, beschriftet rechts. */
function Linie({ px, breite, text, klasse, data, unten = false }: { px: number; breite: number; text: string; klasse: string; data: string; unten?: boolean }) {
  const y = BODEN - px;
  return (
    <g className={klasse} data-linie={data} data-linie-px={Math.round(px)}>
      <line x1={RAND - 14} x2={breite} y1={y} y2={y} />
      <text x={RAND + 4} y={unten ? y + SCHRIFT + 6 : y - 8} fontSize={SCHRIFT}>{text}</text>
    </g>
  );
}

/** Schrittweite der Zahlen am Lineal in M: je nach Höhe 0,5 oder 1. */
const schritt = (hoeheM: number) => (hoeheM > 4.5 ? 1 : 0.5);

/** Lineal in M links: Strich je 0,1 M, Zahl je Schritt; die Größenstufen sind als Marken rechts daneben, die gewählte hat
 *  eine Linie über die Szene. */
function Lineal({ stufen, oben, breite, markiert }: { stufen: Stufen; oben: number; breite: number; markiert?: string }) {
  const x = RAND - 14;
  const hoeheM = (BODEN - oben) / stufen.bezugPx;
  const n = Math.floor(hoeheM * 10);
  const s = schritt(hoeheM);
  const sel = stufen.stufen.find((t) => t.id === markiert);
  return (
    <g className="sz-massstab" data-lineal>
      <line x1={x} x2={x} y1={BODEN} y2={BODEN - n * 0.1 * stufen.bezugPx} />
      {Array.from({ length: n + 1 }, (_, k) => {
        const y = BODEN - k * 0.1 * stufen.bezugPx;
        const ganz = k % 10 === 0;
        const beschriftet = k > 0 && Math.abs((k / 10) / s - Math.round((k / 10) / s)) < 1e-9;
        return (
          <g key={k}>
            <line x1={x} x2={x - (ganz ? 22 : beschriftet ? 16 : 8)} y1={y} y2={y} />
            {beschriftet && <text x={x - 26} y={y + 6} fontSize={SCHRIFT} textAnchor="end">{zahl(k / 10)} M</text>}
          </g>
        );
      })}
      {stufen.stufen.filter((t) => t.m * stufen.bezugPx <= BODEN - oben).map((t) => (
        <line key={t.id} className="sz-marke" data-stufe-marke={t.id} x1={x} x2={x + 12} y1={BODEN - t.m * stufen.bezugPx} y2={BODEN - t.m * stufen.bezugPx} />
      ))}
      {sel && <Linie px={sel.m * stufen.bezugPx} breite={breite} text={`${sel.id}: ${zahl(sel.m)} M`} klasse="sz-gewaehlt" data="gewaehlt" unten />}
    </g>
  );
}

/** Maßstab breit unter dem Boden: Strich je Unit, Zahl alle 5 Units. */
function MassstabBreit({ breite }: { breite: number }) {
  const n = Math.floor((breite - RAND) / UNIT);
  const y = BODEN + 14;
  return (
    <g className="sz-massstab" data-massstab="breit">
      <line x1={RAND} x2={RAND + n * UNIT} y1={y} y2={y} />
      {Array.from({ length: n + 1 }, (_, k) => (
        <g key={k}>
          <line x1={RAND + k * UNIT} x2={RAND + k * UNIT} y1={y} y2={y + (k % 5 ? 8 : 16)} />
          {k % 5 === 0 && <text x={RAND + k * UNIT} y={y + 38} fontSize={SCHRIFT} textAnchor="middle">{k}</text>}
        </g>
      ))}
      <text x={breite - 12} y={y + 62} fontSize={SCHRIFT} textAnchor="end">Units (1 Unit = 32 px)</text>
    </g>
  );
}
