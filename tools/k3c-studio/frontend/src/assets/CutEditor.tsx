import { ActionButton, Button, Flex, Text, TextField } from '@webkit/atoms';
import { useEffect, useState } from 'react';
import { backend, type AssetChoice, type AssetRect, type CatalogAsset } from '../api';
import { loadPref, savePref } from '../lib/prefs';
import { CUT_VIEWS, fileUrl, GAME_SCALE, previewSize, type CutView } from './assets';
import { CutStage } from './CutStage';

interface Props {
  choice: AssetChoice;
  /** Speichert den Ausschnitt (undefined = ganzes Bild); false bei Ablehnung aus Go */
  onCut: (cut: AssetRect | undefined) => Promise<boolean>;
}

const FIELDS = ['x', 'y', 'w', 'h'] as const;

/** Ausschnitt einer Grafik festlegen; Vorschau im Spielmaßstab (×2, B-298/AC-03). Warnt bei CC-BY-SA (Q70). */
export function CutEditor({ choice, onCut }: Props) {
  const asset = useCatalogEntry(choice.datei);
  const [cut, setCut] = useState<AssetRect>(choice.ausschnitt ?? { x: 0, y: 0, w: asset?.w ?? 16, h: asset?.h ?? 16 });
  useEffect(() => {
    if (!choice.ausschnitt && asset?.w && asset.h) setCut({ x: 0, y: 0, w: asset.w, h: asset.h });
  }, [asset, choice.ausschnitt]);
  const [view, setView] = useState<CutView>(() => loadPref('assets.cutview', CUT_VIEWS, 'rahmen'));
  const pick = (v: CutView) => { setView(v); savePref('assets.cutview', v); };
  const size = previewSize(cut);
  const set = (k: (typeof FIELDS)[number], v: string) => setCut((c) => ({ ...c, [k]: Math.max(0, Math.floor(Number(v)) || 0) }));
  return (
    <div className="as-cut">
      <Flex gap="2" align="center" wrap="wrap">
        <Text size="2" weight="bold">Ausschnitt</Text>
        <Flex gap="1" role="group" aria-label="Ansicht">
          {CUT_VIEWS.map((v) => (
            <Button key={v} size="1" variant={view === v ? 'solid' : 'soft'} aria-pressed={view === v} onClick={() => pick(v)}>{v === 'rahmen' ? 'Rahmen' : 'Ausschnitt'}</Button>
          ))}
        </Flex>
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
      <CutStage src={fileUrl(choice.datei)} cut={cut} view={view} size={asset?.w && asset.h ? { w: asset.w, h: asset.h } : undefined} onCut={setCut} />
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
