import { useEffect, useState, type ReactNode } from 'react';
import { ActionButton, Flex, Heading, Text, ValueSlider } from '@webkit/atoms';
import { ChipSwitch } from '@webkit/molecules';
import type { Frame, Spec } from '../api';
import { useFrame, usePlayback, useSpec } from './useCatalog';

interface Props {
  id: string;
  /** Unter dem Bild, z. B. Prompt-Buttons (WB1.3); bekommt Spec und Frame. */
  footer?: (spec: Spec, frame: Frame | null) => ReactNode;
  /** Über dem Bild liegende Ebene (Teil-Markierung, WB1.3). */
  overlay?: (frame: Frame) => ReactNode;
}

/** Vorschau eines Assets: Animation wählen, Frame-Schieber, Abspielen (nur Ansehen). */
export function Preview({ id, footer, overlay }: Props) {
  const { spec, error } = useSpec(id);
  if (error) return <Text color="red">{error}</Text>;
  if (!spec) return <Text color="gray">Lädt …</Text>;
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

function SpecPreview({ spec, footer, overlay }: { spec: Spec } & Omit<Props, 'id'>) {
  const a = useAnimation(spec);
  const shown = useFrame(spec.id, a.anim, a.frame);
  return (
    <Flex direction="column" gap="3">
      <Heading size="4">{spec.id}</Heading>
      {spec.beschreibung && <Text size="2" color="gray">{spec.beschreibung}</Text>}
      {a.names.length > 0 && (
        <ChipSwitch label="Animation" options={a.names.map((n) => ({ value: n, label: n }))} value={a.anim} onChange={a.setAnimation} />
      )}
      <div className="as-stage">
        {shown.frame && <img src={shown.frame.bild} alt={`${spec.id} ${a.anim} Frame ${a.frame + 1}`} className="as-frame" />}
        {shown.frame && overlay?.(shown.frame)}
        {shown.error && <Text color="red">{shown.error}</Text>}
      </div>
      {a.frames > 1 && (
        <Flex gap="3" align="center">
          <ActionButton size="1" variant="soft" onClick={() => a.setPlaying(!a.playing)}>{a.playing ? 'Anhalten' : 'Abspielen'}</ActionButton>
          <ValueSlider aria-label="Frame" value={a.frame} min={0} max={a.frames - 1} onChange={a.setFrame} style={{ flex: 1 }} />
          <Text size="1" color="gray">{a.frame + 1}/{a.frames}</Text>
        </Flex>
      )}
      {footer?.(spec, shown.frame)}
    </Flex>
  );
}
