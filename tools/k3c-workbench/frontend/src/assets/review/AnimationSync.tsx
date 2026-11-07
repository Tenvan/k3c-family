import { Flex, SelectField } from '@webkit/atoms';
import { FramePlayer } from '@webkit/molecules';
import { useEffect } from 'react';

/** Name, Frame-Anzahl und fps einer Animation (aus der Spec des Arbeitsstands). */
export interface Animation {
  name: string;
  frames: number;
  fps: number;
}

interface Props {
  animationen: Animation[];
  anim: string;
  nr: number;
  spielt: boolean;
  onAnim: (name: string) => void;
  onNr: (nr: number | ((n: number) => number)) => void;
  onSpielt: (spielt: boolean) => void;
}

/** Gemeinsame Steuerung für Vorher und Nachher: Animation, Frame-Schieber, Abspielen im Takt der fps; feste Breiten über
 *  den `FramePlayer` (WB2), damit beim Abspielen nichts springt. */
export function AnimationSync({ animationen, anim, nr, spielt, onAnim, onNr, onSpielt }: Props) {
  const aktuell = animationen.find((a) => a.name === anim);
  const frames = aktuell?.frames ?? 1;
  const fps = aktuell?.fps ?? 8;
  useEffect(() => {
    if (!spielt) return;
    const t = setInterval(() => onNr((n) => (n + 1) % frames), 1000 / fps);
    return () => clearInterval(t);
  }, [spielt, frames, fps, onNr]);
  if (!aktuell) return null;
  return (
    <Flex gap="3" align="center" wrap="wrap" data-part="animation">
      <SelectField size="1" aria-label="Animation" value={anim} options={animationen.map((a) => ({ value: a.name, label: a.name }))}
        onChange={(name) => {
          onAnim(name);
          onNr(0);
        }} />
      <FramePlayer frame={nr} frames={frames} playing={spielt} onFrame={onNr} onPlaying={onSpielt} />
    </Flex>
  );
}
