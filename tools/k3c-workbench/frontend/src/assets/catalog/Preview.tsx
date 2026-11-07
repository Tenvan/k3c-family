import { useEffect, useRef, useState, type ReactNode } from 'react';
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
  const playback = usePlayback(frames, info?.fps ?? 8, setFrame);
  const { setPlaying } = playback;
  useEffect(() => {
    setFrame(0);
    setPlaying(false);
  }, [anim, setPlaying]);
  return { names, anim, setAnimation, frame: Math.min(frame, frames - 1), setFrame, frames, ...playback };
}

/** Beim Abspielen alle Frames der Animation vorab laden, damit der Lauf nicht auf das Backend wartet (WB2). */
function usePrefetch(cache: FrameCache, id: string, anim: string, frames: number, playing: boolean) {
  useEffect(() => {
    if (!playing || !anim) return;
    for (let n = 0; n < frames; n++) void cachedFrame(cache, id, anim, n).catch(() => undefined);
  }, [cache, id, anim, frames, playing]);
}

function SpecPreview({ spec, footer, overlay }: { spec: Spec } & Omit<Props, 'id'>) {
  const a = useAnimation(spec);
  const cache = useRef<FrameCache>(new Map()).current;
  usePrefetch(cache, spec.id, a.anim, a.frames, a.playing);
  const shown = useFrame(cache, spec.id, a.anim, a.frame);
  return (
    <Flex direction="column" gap="3">
      <Heading size="4">{spec.id}</Heading>
      {spec.beschreibung && <Text size="2" color="gray">{spec.beschreibung}</Text>}
      {a.names.length > 0 && (
        <ChipSwitch label="Animation" options={a.names.map((n) => ({ value: n, label: n }))} value={a.anim} onChange={a.setAnimation} />
      )}
      {/* Das alte Bild bleibt beim Laden stehen, der Spinner liegt darüber (WB2/AC-03) */}
      <div className="as-stage">
        {shown.frame && <img src={shown.frame.bild} alt={`${spec.id} ${shown.anim} Frame ${shown.nr + 1}`} className="as-frame" />}
        {shown.frame && overlay?.(shown.frame, spec)}
        {shown.loading && <LoadingNote id="frame" text={`Lädt ${a.anim} Frame ${a.frame + 1}`} overlay />}
        {shown.error && <Text color="red">{shown.error}</Text>}
      </div>
      {a.frames > 1 && <FramePlayer frame={a.frame} frames={a.frames} playing={a.playing} onFrame={a.setFrame} onPlaying={a.setPlaying} />}
      {footer?.(spec, shown.frame)}
    </Flex>
  );
}
