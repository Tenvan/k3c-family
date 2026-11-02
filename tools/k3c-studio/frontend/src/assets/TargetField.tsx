import { ActionButton, Button, Flex, Text, TextField } from '@webkit/atoms';
import { useEffect, useState } from 'react';
import { fitTarget } from '@game/scenes/render/targetFit';
import type { AssetSlot, AssetTarget } from '../api';
import { fileUrl, UNIT_PX } from './assets';

interface Props {
  slot: AssetSlot;
  /** Speichert das Zielmaß (null = heutiges Maß); false bei Ablehnung aus Go */
  onTarget: (id: string, ziel: AssetTarget | null) => Promise<boolean>;
}

/** Quellmaß der aktiven Zuordnung: Ausschnitt, sonst die Bildgröße (geladen) */
function useSourceSize(slot: AssetSlot): { w: number; h: number } | undefined {
  const first = slot.wahl[0];
  const [img, setImg] = useState<{ w: number; h: number }>();
  useEffect(() => {
    setImg(undefined);
    if (!first || first.ausschnitt) return;
    let live = true;
    const el = new Image();
    el.onload = () => live && setImg({ w: el.naturalWidth, h: el.naturalHeight });
    el.src = fileUrl(first.datei);
    return () => {
      live = false;
    };
  }, [first]);
  return first?.ausschnitt ? { w: first.ausschnitt.w, h: first.ausschnitt.h } : img;
}

/** Text zur Wirkung im Spiel: belegte Fläche und Maßstab der aktiven Grafik (gleiche Rechnung wie das Spiel, `fitTarget`) */
export function targetSummary(source: { w: number; h: number } | undefined, ziel: AssetTarget | null): string {
  if (!source) return 'Quelle lädt …';
  const fit = fitTarget(source, ziel);
  const w = Math.round(fit.w * 10) / 10;
  const h = Math.round(fit.h * 10) / 10;
  const scale = Math.round(fit.scale * 1000) / 1000;
  return `Quelle ${source.w}×${source.h} px → ${w}×${h} px im Spiel (×${scale}, ${fit.smooth ? 'weich' : 'scharf'})${ziel ? '' : ', heutiges Maß'}`;
}

/** Zielmaß eines Gebäude-Slots in Units (B-685): die aktive Grafik wird ohne Verzerrung eingepasst, unten am Boden. */
export function TargetField({ slot, onTarget }: Props) {
  const [w, setW] = useState(String(slot.ziel?.w ?? ''));
  const [h, setH] = useState(String(slot.ziel?.h ?? ''));
  useEffect(() => {
    setW(String(slot.ziel?.w ?? ''));
    setH(String(slot.ziel?.h ?? ''));
  }, [slot.ziel?.w, slot.ziel?.h]);
  const source = useSourceSize(slot);
  const nw = Number(w);
  const nh = Number(h);
  const valid = nw > 0 && nh > 0 && Number.isFinite(nw) && Number.isFinite(nh);
  const draft = valid ? { w: nw, h: nh } : null;
  return (
    <div className="as-target">
      <Flex gap="2" align="center" wrap="wrap">
        <Text size="2" weight="bold">Zielmaß</Text>
        <label className="as-num">Breite
          <TextField.Root size="1" type="number" min="0" step="0.5" aria-label="Zielmaß Breite in Units" value={w} onChange={(e) => setW(e.target.value)} />
        </label>
        <label className="as-num">Höhe
          <TextField.Root size="1" type="number" min="0" step="0.5" aria-label="Zielmaß Höhe in Units" value={h} onChange={(e) => setH(e.target.value)} />
        </label>
        <Text size="1" color="gray">Units{draft ? ` = ${draft.w * UNIT_PX}×${draft.h * UNIT_PX} px` : ''}</Text>
        <ActionButton size="1" disabled={!draft} onClick={() => onTarget(slot.id, draft)}>Zielmaß speichern</ActionButton>
        <Button size="1" variant="soft" disabled={!slot.ziel} onClick={() => void onTarget(slot.id, null)}>Heutiges Maß</Button>
      </Flex>
      <Text size="1" color="gray">{targetSummary(source, draft)}</Text>
    </div>
  );
}
