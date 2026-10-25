import { BIOMES } from '@game/model/biome';
import { Pixel, ladeBild, pixelContext } from './pixel';
import { hintergrundStreifen, type GebaeudeEintrag } from './scenesLogic';

// Streifen „Hintergründe“ (GEB7, umgezogen aus der früheren Asset-Vorschau, laut B-459/AC-06 in der Szene Umgebung):
// je Biom die drei Schichten fern → nah auf der Himmelsfarbe, zwei Kacheln nebeneinander (Naht sichtbar). Bisherige Fassung
// in 32 px ×1 und 16 px ×2, die Varianten aus GEB18 in 128 px ×0,5 (Details sichtbar) und 32 px ×1 (Spielgröße).

const DICHTEN = [[128, 0.5], [32, 1], [16, 2]] as const;

async function zeichnen(c: HTMLCanvasElement, himmel: string, dateien: string[], scale: number): Promise<void> {
  const imgs = await Promise.all(dateien.map(ladeBild));
  const w = imgs[0]!.width * scale;
  const g = pixelContext(c, w * 2, imgs[0]!.height * scale);
  if (!g) return;
  g.fillStyle = himmel;
  g.fillRect(0, 0, c.width, c.height);
  for (const img of imgs) for (let i = 0; i < 2; i++) g.drawImage(img, i * w, 0, w, c.height);
}

export function Backgrounds({ index }: { index: readonly GebaeudeEintrag[] }) {
  const streifen = hintergrundStreifen(index, BIOMES);
  if (streifen.length === 0) return null;
  return (
    <>
      <h3 className="sz-title">Hintergründe</h3>
      <div className="sz-strip sz-column">
        {streifen.map((s) => (
          <div key={`${s.biom}:${s.variante}`} className="sz-row sz-hg-row">
            {DICHTEN.map(([dichte, scale]) => {
              const dateien = s.schichten.flatMap((l) => l.dateien[dichte] ?? []);
              if (dateien.length !== s.schichten.length) return null;
              return (
                <figure key={dichte} className="sz-fig sz-hg">
                  <Pixel draw={(c) => zeichnen(c, s.himmel, dateien, scale)} drawKey={`${s.himmel}:${dateien.join(',')}:${scale}`} />
                  <figcaption>{s.biom} · {s.variante ? `Variante ${s.variante.slice(1)}` : 'bisher (GEB7)'} · {dichte} px ×{scale}</figcaption>
                </figure>
              );
            })}
          </div>
        ))}
      </div>
    </>
  );
}
