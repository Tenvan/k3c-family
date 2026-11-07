import { useCallback, useEffect, useRef, useState, type ReactNode } from 'react';
import { Flex, Heading, Text } from '@webkit/atoms';
import { ChipSwitch, FramePlayer, LoadingNote } from '@webkit/molecules';
import type { Frame, Spec } from '../api';
import { cachedFrame, useFrame, usePlayback, useSpec, type FrameCache } from './useCatalog';

interface Props {
  id: string;
  /** Unter dem Bild, z. B. Prompt-Buttons (WB1.3); bekommt Spec und Frame. */
  footer?: (spec: Spec, frame: Frame | null) => ReactNode;
  /** Über dem Bild liegende Ebene (Teil-Markierung, WB1.3). */
  overlay?: (frame: Frame, spec: Spec) => ReactNode;
}

/** Vorschau eines Assets: Animation wählen, Frame-Schieber, Abspielen (nur Ansehen); Laden mit Lade-Anzeige (WB2). */
export function Preview({ id, footer, overlay }: Props) {
  const { spec, error } = useSpec(id);
  if (error) return <Text color="red">{error}</Text>;
  if (!spec) return <LoadingNote id="asset" text={`Lädt ${id} …`} />;
  return <SpecPreview key={id} spec={spec} footer={footer} overlay={overlay} />;
}

/** Gewählte Animation, Frame und Abspielen einer Spec. */
function useAnimation(spec: Spec) {
  const names = Object.keys(spec.animationen ?? {});
  const [animation, setAnimation] = useState('');
  const [frame, setFrame] = useState(0);
  const anim = names.includes(animation) ? animation : (names[0] ?? '');
  const info = spec.animationen?.[anim];
  const frames = info?.frames ?? 1;
  // Lädt der gezeigte Frame noch, wartet das Abspielen (`busy`, von SpecPreview gesetzt): so fragt der Lauf nicht Frame um
  // Frame neu an, während das Backend noch rendert (Review-Befund 2)
  const busy = useRef(false);
  const advance = useCallback((next: (f: number) => number) => setFrame((f) => (busy.current ? f : next(f))), []);
  const playback = usePlayback(frames, info?.fps ?? 8, advance);
  const { setPlaying } = playback;
  useEffect(() => {
    setFrame(0);
    setPlaying(false);
  }, [anim, setPlaying]);
  return { names, anim, setAnimation, frame: Math.min(frame, frames - 1), setFrame, frames, busy, ...playback };
}

/** Beim Abspielen die Frames der Animation vorab laden, damit der Lauf nicht auf das Backend wartet (WB2): nacheinander ab
 *  dem nächsten Frame (neben dem gezeigten höchstens ein Render offen, das Backend rendert nur wenige zugleich), Abbruch
 *  bei Anhalten, Animations- oder Asset-Wechsel (Review-Befund 2). */
function usePrefetch(cache: FrameCache, id: string, anim: string, frames: number, playing: boolean, frame: number) {
  const from = useRef(frame);
  from.current = frame;
  useEffect(() => {
    if (!playing || !anim) return;
    let live = true;
    const next = (n: number, left: number) => {
      if (!live || left <= 0) return;
      void cachedFrame(cache, id, anim, n)
        .catch(() => undefined)
        .then(() => next((n + 1) % frames, left - 1));
    };
    next((from.current + 1) % frames, frames - 1);
    return () => {
      live = false;
    };
  }, [cache, id, anim, frames, playing]);
}

function SpecPreview({ spec, footer, overlay }: { spec: Spec } & Omit<Props, 'id'>) {
  const a = useAnimation(spec);
  const cache = useRef<FrameCache>(new Map()).current;
  usePrefetch(cache, spec.id, a.anim, a.frames, a.playing, a.frame);
  const shown = useFrame(cache, spec.id, a.anim, a.frame);
  a.busy.current = shown.loading;
  return (
    <Flex direction="column" gap="3">
      <Heading size="4">{spec.id}</Heading>
      {spec.beschreibung && <Text size="2" color="gray">{spec.beschreibung}</Text>}
      {a.names.length > 0 && (
        <ChipSwitch label="Animation" options={a.names.map((n) => ({ value: n, label: n }))} value={a.anim} onChange={a.setAnimation} />
      )}
      {/* Das alte Bild bleibt beim Laden stehen, der Spinner liegt darüber (WB2/AC-03) */}
      <div className="as-stage">
        {shown.frame && <ZoomedFrame src={shown.frame.bild} alt={`${spec.id} ${shown.anim} Frame ${shown.nr + 1}`} />}
        {shown.frame && overlay?.(shown.frame, spec)}
        {shown.loading && <LoadingNote id="frame" text={`Lädt ${a.anim} Frame ${a.frame + 1}`} overlay />}
        {shown.error && <Text color="red" size="1" className="as-stage-error">{shown.error}</Text>}
      </div>
      {a.frames > 1 && <FramePlayer frame={a.frame} frames={a.frames} playing={a.playing} onFrame={a.setFrame} onPlaying={a.setPlaying} />}
      {footer?.(spec, shown.frame)}
    </Flex>
  );
}

/** Vorschau doppelt so groß wie der gerenderte Frame (B-732), höchstens so breit wie die Spalte (`.as-frame`). Bis die
 *  Bildgröße bekannt ist, bleibt das Bild verborgen, damit es nicht erst klein aufblitzt; ohne Größe (-1) in Normalgröße. */
const ZOOM = 2;

function ZoomedFrame({ src, alt }: { src: string; alt: string }) {
  const [width, setWidth] = useState(0);
  return (
    <img src={src} alt={alt} className="as-frame" data-zoom={ZOOM}
      style={width > 0 ? { width: width * ZOOM } : width === 0 ? { visibility: 'hidden' } : undefined}
      onLoad={(e) => setWidth(e.currentTarget.naturalWidth || -1)} />
  );
}
