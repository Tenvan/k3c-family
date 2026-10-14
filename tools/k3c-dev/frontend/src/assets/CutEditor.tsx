import { Button, Flex, Text, TextField } from '@radix-ui/themes';
import { useEffect, useRef, useState } from 'react';
import { backend, type AssetChoice, type AssetRect, type CatalogAsset } from '../api';
import { ActionButton } from '../ui/parts';
import { fileUrl, GAME_SCALE, previewSize } from './assets';

interface Props {
  choice: AssetChoice;
  preview: string;
  /** Speichert den Ausschnitt (undefined = ganzes Bild); false bei Ablehnung aus Go */
  onCut: (cut: AssetRect | undefined) => Promise<boolean>;
}

const FIELDS = ['x', 'y', 'w', 'h'] as const;

/** Ausschnitt einer Grafik festlegen; Vorschau im Spielmaßstab (×2, B-298/AC-03). Warnt bei CC-BY-SA (Q70). */
export function CutEditor({ choice, preview, onCut }: Props) {
  const asset = useCatalogEntry(choice.datei);
  const [cut, setCut] = useState<AssetRect>(choice.ausschnitt ?? { x: 0, y: 0, w: asset?.w ?? 16, h: asset?.h ?? 16 });
  useEffect(() => {
    if (!choice.ausschnitt && asset?.w && asset.h) setCut({ x: 0, y: 0, w: asset.w, h: asset.h });
  }, [asset, choice.ausschnitt]);
  const size = previewSize(cut);
  const set = (k: (typeof FIELDS)[number], v: string) => setCut((c) => ({ ...c, [k]: Math.max(0, Math.floor(Number(v)) || 0) }));
  return (
    <div className="as-cut">
      <Flex gap="2" align="center" wrap="wrap">
        <Text size="2" weight="bold">Ausschnitt</Text>
        {FIELDS.map((k) => (
          <label key={k} className="as-num">{k}
            <TextField.Root size="1" type="number" min="0" value={String(cut[k])} onChange={(e) => set(k, e.target.value)} />
          </label>
        ))}
        <ActionButton size="1" onClick={() => onCut(cut)}>Ausschnitt speichern</ActionButton>
        <Button size="1" variant="soft" disabled={!choice.ausschnitt} onClick={() => void onCut(undefined)}>Ganzes Bild</Button>
      </Flex>
      {asset && <Text size="1" color="gray">{asset.w}×{asset.h} px · {asset.lizenz}{asset.urheber ? ` · ${asset.urheber}` : ''}</Text>}
      {asset?.kennzeichen === 'SA' && <p className="as-warn">CC-BY-SA: ein Ausschnitt bleibt unter CC-BY-SA, Credit nennt den Ausschnitt (Q70).</p>}
      {asset?.kennzeichen === 'NC' && <p className="as-warn">CC-BY-NC: nur als markierter Lückenfüller (Q70).</p>}
      <Text size="1" color="gray">Spielmaßstab ×{GAME_SCALE}: {size.w}×{size.h} px = {size.unitsW}×{size.unitsH} Units</Text>
      <CutCanvas src={fileUrl(preview, choice.datei)} cut={cut} />
    </div>
  );
}

/** Katalog-Eintrag einer Datei (Maße, Lizenz) */
function useCatalogEntry(datei: string): CatalogAsset | undefined {
  const [asset, setAsset] = useState<CatalogAsset>();
  useEffect(() => {
    let live = true;
    backend.assetsCatalog(datei, '', 5).then((list) => live && setAsset(list.find((a) => a.datei === datei)), () => undefined);
    return () => {
      live = false;
    };
  }, [datei]);
  return asset;
}

function CutCanvas({ src, cut }: { src: string; cut: AssetRect }) {
  const ref = useRef<HTMLCanvasElement>(null);
  const [failed, setFailed] = useState(false);
  useEffect(() => {
    if (!src) return;
    const img = new Image();
    img.onload = () => {
      const c = ref.current;
      if (!c || cut.w <= 0 || cut.h <= 0) return;
      c.width = cut.w * GAME_SCALE;
      c.height = cut.h * GAME_SCALE;
      const g = c.getContext('2d');
      if (!g) return;
      g.imageSmoothingEnabled = false;
      g.drawImage(img, cut.x, cut.y, cut.w, cut.h, 0, 0, c.width, c.height);
      setFailed(false);
    };
    img.onerror = () => setFailed(true);
    img.src = src;
  }, [src, cut]);
  if (!src) return <Text size="2" color="gray">Vorschau braucht den Vite-Dienst.</Text>;
  if (failed) return <p className="as-warn">Bild lädt nicht: {src}</p>;
  return <div className="as-canvas"><canvas ref={ref} /></div>;
}
