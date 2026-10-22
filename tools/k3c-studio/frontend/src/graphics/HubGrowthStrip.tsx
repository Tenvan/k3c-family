import { UNIT_PX as UNIT } from '@game/core/constants';
import { Pixel, ladeBild, pixelContext } from './pixel';
import { hubWachstum, type GebaeudeEintrag, type WachstumKarte } from './scenesLogic';

// Streifen „Hub wächst (GEB11)“: die Wachstumsreihe der Burg vom Lagerfeuer bis zur Kristallburg in Spielreihenfolge,
// 32 px ×1 auf einer gemeinsamen Bodenlinie; das Baugerüst liegt als zweites Bild über der Burg
// (docs/rules/materialien-gebaeude.md § 2.1, docs/standards/gebaeude-grafik.md § 9).

export function HubGrowthStrip({ index }: { index: readonly GebaeudeEintrag[] }) {
  const karten = hubWachstum(index);
  if (karten.length === 0) return null;
  const boden = Math.max(...karten.map((k) => k.h)) * UNIT;
  return (
    <div className="sz-strip sz-top">
      {karten.map((k) => (
        <figure key={`${k.datei}:${k.geruest ?? ''}`} className="sz-fig">
          <Pixel draw={(c) => zeichne(c, k, boden)} drawKey={`${k.datei}:${k.geruest ?? ''}:${boden}`} />
          <figcaption>{k.titel} · 32 px ×1</figcaption>
          <p className="sz-change">{k.wann}</p>
        </figure>
      ))}
    </div>
  );
}

/** Look und Gerüst übereinander, unten mittig ausgerichtet wie im Spiel (Ursprung 0,5 / 1). */
async function zeichne(c: HTMLCanvasElement, k: WachstumKarte, boden: number): Promise<void> {
  const imgs = await Promise.all([k.datei, ...(k.geruest ? [k.geruest] : [])].map(ladeBild));
  const w = Math.max(...imgs.map((i) => i.width));
  const h = Math.max(...imgs.map((i) => i.height));
  const g = pixelContext(c, w, h);
  for (const img of imgs) g?.drawImage(img, Math.floor((w - img.width) / 2), h - img.height);
  c.style.marginTop = `${Math.max(0, boden - h)}px`;
}
