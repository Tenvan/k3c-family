import { Text } from '@webkit/atoms';
import { useEffect, useRef, useState } from 'react';
import type { AssetChoice } from '../api';
import { fileUrl, GAME_SCALE, sceneLayout, SCENE, UNIT_PX } from './assets';

interface Props {
  wahl: readonly AssetChoice[];
  /** Index der gewählten Präferenz (hervorgehoben) */
  active: number;
}

const load = (src: string) => new Promise<HTMLImageElement | null>((ok) => {
  const img = new Image();
  img.onload = () => ok(img);
  img.onerror = () => ok(null);
  img.src = src;
});

/** Mini-Szene: alle Zuordnungen eines Slots im Spielmaßstab (×2) nebeneinander auf einer Bodenlinie mit Unit-Raster; Präferenz 1 ist die im Spiel aktive. */
export function MiniScene({ wahl, active }: Props) {
  const ref = useRef<HTMLCanvasElement>(null);
  const [missing, setMissing] = useState<string[]>([]);
  useEffect(() => {
    let live = true;
    void Promise.all(wahl.map((c) => load(fileUrl(c.datei)))).then((imgs) => {
      const c = ref.current;
      const g = c?.getContext('2d');
      if (!live || !c || !g) return;
      const cuts = wahl.map((w, i) => w.ausschnitt ?? { x: 0, y: 0, w: imgs[i]?.width ?? 16, h: imgs[i]?.height ?? 16 });
      const lay = sceneLayout(cuts);
      c.width = lay.w;
      c.height = lay.h;
      const sky = g.createLinearGradient(0, 0, 0, lay.floor);
      sky.addColorStop(0, '#2c3350');
      sky.addColorStop(1, '#4a3a5c');
      g.fillStyle = sky;
      g.fillRect(0, 0, lay.w, lay.floor);
      g.fillStyle = '#5b4a3a';
      g.fillRect(0, lay.floor, lay.w, lay.h - lay.floor);
      g.fillStyle = 'rgba(255,255,255,0.14)';
      for (let x = 0; x < lay.w; x += UNIT_PX) g.fillRect(x, lay.floor, 1, 6); // Unit-Raster auf dem Boden
      g.imageSmoothingEnabled = false;
      g.font = '12px system-ui, sans-serif';
      imgs.forEach((img, i) => {
        const it = lay.items[i]!;
        const cut = cuts[i]!;
        if (img) g.drawImage(img, cut.x, cut.y, cut.w, cut.h, it.x, it.y, it.w, it.h);
        else {
          g.strokeStyle = '#ff8fa3';
          g.strokeRect(it.x + 0.5, it.y + 0.5, it.w - 1, it.h - 1);
        }
        g.fillStyle = i === active ? '#ffd166' : '#8a97aa';
        g.fillText(`${i + 1}${i === 0 ? ' · aktiv' : ''} · ${(it.w / UNIT_PX).toFixed(1)}×${(it.h / UNIT_PX).toFixed(1)} U`, it.x, SCENE.label - 6);
        if (i === active) {
          g.strokeStyle = '#ffd166';
          g.strokeRect(it.x - 2.5, it.y - 2.5, it.w + 5, it.h + 5);
        }
      });
      setMissing(wahl.filter((_, i) => !imgs[i]).map((w) => w.datei));
    });
    return () => {
      live = false;
    };
  }, [wahl, active]);
  if (wahl.length === 0) return <Text size="2" color="gray">Mini-Szene: noch keine Zuordnung.</Text>;
  return (
    <div className="as-scene">
      <Text size="2" color="gray">Mini-Szene im Spielmaßstab ×{GAME_SCALE}: alle Präferenzen auf einer Bodenlinie, 1 Unit = {UNIT_PX} px</Text>
      <div className="as-scene-view"><canvas ref={ref} /></div>
      {missing.length > 0 && <p className="as-warn">Bild lädt nicht: {missing.join(', ')}</p>}
    </div>
  );
}
