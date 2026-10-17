import { BIOMES } from '@game/model/biome';
import { Pixel, ladeBild, pixelContext } from './pixel';
import { hintergrundStreifen, type GebaeudeEintrag } from './szenenLogic';

// Streifen „Hintergründe“ (GEB7, umgezogen aus src/tools/assetvorschau.ts, laut B-459/AC-06 in der Szene Umgebung):
// je Biom die drei Schichten fern → nah auf der Himmelsfarbe, zwei Kacheln nebeneinander (Naht sichtbar),
// 32 px ×1 und 16 px ×2.

const DICHTEN = [[32, 1], [16, 2]] as const;

async function zeichnen(c: HTMLCanvasElement, himmel: string, dateien: string[], scale: number): Promise<void> {
  const imgs = await Promise.all(dateien.map(ladeBild));
  const w = imgs[0]!.width * scale;
  const g = pixelContext(c, w * 2, imgs[0]!.height * scale);
  if (!g) return;
  g.fillStyle = himmel;
  g.fillRect(0, 0, c.width, c.height);
  for (const img of imgs) for (let i = 0; i < 2; i++) g.drawImage(img, i * w, 0, w, c.height);
}

export function Hintergruende({ index }: { index: readonly GebaeudeEintrag[] }) {
  const streifen = hintergrundStreifen(index, BIOMES);
  if (streifen.length === 0) return null;
  return (
    <>
      <h3 className="sz-title">Hintergründe</h3>
      <div className="sz-strip sz-column">
        {streifen.map((s) => (
          <div key={s.biom} className="sz-row">
            {DICHTEN.map(([dichte, scale]) => {
              const dateien = s.schichten.map((l) => l.dateien[dichte]!);
              return (
                <figure key={dichte} className="sz-fig">
                  <Pixel draw={(c) => zeichnen(c, s.himmel, dateien, scale)} drawKey={`${s.himmel}:${dateien.join(',')}:${scale}`} />
                  <figcaption>{s.biom} · {s.schichten.map((l) => l.id).join(' + ')} · {dichte} px ×{scale}</figcaption>
                </figure>
              );
            })}
          </div>
        ))}
      </div>
    </>
  );
}
