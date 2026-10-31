import { Button, Flex, IconButton, Text } from '@webkit/atoms';
import { useEffect, useRef, useState } from 'react';
import type { AssetSlot } from '../api';
import { GAME_SCALE, UNIT_PX } from '../assets/assets';
import { MiniScene } from '../assets/MiniScene';
import * as prefs from '../lib/prefs';
import { fitZoom, parseZoom, stepZoom, ZOOM_MAX, ZOOM_MIN, zoomLabel } from './stageLogic';

/** Gewählte Präferenz des gewählten Slots: Belegung setzt sie, die Bühne hebt sie hervor; ein Slot-Wechsel beginnt bei 1. */
export function useChoiceIndex(slotId: string): [number, (i: number) => void] {
  const [pick, setPick] = useState({ slot: slotId, i: 0 });
  return [pick.slot === slotId ? pick.i : 0, (i) => setPick({ slot: slotId, i })];
}

interface Props {
  slot: AssetSlot | undefined;
  /** gewählte Präferenz (hervorgehoben) */
  active: number;
}

/** Inhalt der Bühne: Mini-Szene eines Grafik-Slots, sonst ein Hinweis. */
function StageBody({ slot, active, zoom, onSize }: Props & { zoom: number; onSize: (w: number, h: number) => void }) {
  if (!slot) return <Text size="2" color="gray">Ein Grafik-Slot zeigt hier seine Präferenzen im Spielmaßstab.</Text>;
  if (slot.art !== 'grafik') return <Text size="2" color="gray">Sound-Slot: keine Bühne. ▶ in der Belegung oder in der Abspielleiste.</Text>;
  return <MiniScene wahl={slot.wahl} active={active} zoom={zoom} caption={false} ziel={slot.ziel} onSize={onSize} />;
}

/** Bühne der Assets-Seite (Variante „Bühne“): die Mini-Szene des gewählten Slots unter dem Raster über die volle Breite, mit Zoom
 *  (− / + / Anpassen, gemerkt als `assets.stagezoom`) und Vollbild als reiner Layout-Umschalter (Klasse `ap-stage-full`, Esc verlässt ihn). */
export function Stage({ slot, active }: Props) {
  const [zoom, setZoomState] = useState(() => parseZoom(prefs.loadText('assets.stagezoom', '')));
  const [full, setFull] = useState(false);
  const body = useRef<HTMLDivElement>(null);
  const scene = useRef({ w: 0, h: 0 });
  const setZoom = (z: number) => { setZoomState(z); prefs.savePref('assets.stagezoom', String(z)); };
  const fit = () => {
    const b = body.current;
    if (b) setZoom(fitZoom(scene.current, { w: b.clientWidth, h: b.clientHeight }));
  };
  useEffect(() => {
    if (!full) return;
    const onKey = (e: KeyboardEvent) => { if (e.key === 'Escape') { e.preventDefault(); setFull(false); } };
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  }, [full]);
  const graphic = slot?.art === 'grafik' && slot.wahl.length > 0;
  return (
    <section className={`ap-stage${full ? ' ap-stage-full' : ''}`} aria-label="Bühne" data-testid="stage" data-slot={slot?.id ?? ''}>
      <Flex gap="2" align="center" className="ap-stage-bar" role="toolbar" aria-label="Bühne">
        <Text size="2" weight="bold">Bühne</Text>
        <Text size="1" color="gray" className="ap-stage-caption">{slot ? `${slot.id} · ` : ''}Spielmaßstab ×{GAME_SCALE}, 1 Unit = {UNIT_PX} px</Text>
        <IconButton size="1" variant="soft" color="gray" aria-label="Verkleinern" disabled={!graphic || zoom <= ZOOM_MIN} onClick={() => setZoom(stepZoom(zoom, -1))}>−</IconButton>
        <Text size="1" className="ap-stage-zoom" aria-live="polite" data-testid="stage-zoom">{zoomLabel(zoom)}</Text>
        <IconButton size="1" variant="soft" color="gray" aria-label="Vergrößern" disabled={!graphic || zoom >= ZOOM_MAX} onClick={() => setZoom(stepZoom(zoom, 1))}>+</IconButton>
        <Button size="1" variant="soft" color="gray" disabled={!graphic} onClick={fit}>Anpassen</Button>
        <Button size="1" variant={full ? 'solid' : 'soft'} color="gray" aria-pressed={full} onClick={() => setFull((f) => !f)}>{full ? 'Vollbild verlassen' : 'Vollbild'}</Button>
      </Flex>
      <div className="ap-stage-body" ref={body}>
        <StageBody slot={slot} active={active} zoom={zoom} onSize={(w, h) => { scene.current = { w, h }; }} />
      </div>
    </section>
  );
}
