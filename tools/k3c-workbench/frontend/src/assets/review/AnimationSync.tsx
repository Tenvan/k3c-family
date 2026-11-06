import { Box, Button, Flex, SelectField, Text, ValueSlider } from '@webkit/atoms';
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

/** Gemeinsame Steuerung für Vorher und Nachher: Animation, Frame-Schieber, Abspielen im Takt der fps. */
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
    <Flex gap="3" align="center" wrap="wrap">
      <SelectField size="1" aria-label="Animation" value={anim} options={animationen.map((a) => ({ value: a.name, label: a.name }))}
        onChange={(name) => {
          onAnim(name);
          onNr(0);
        }} />
      <Box style={{ width: 200 }}>
        <ValueSlider aria-label="Frame" value={nr} min={0} max={Math.max(frames - 1, 0)} onChange={onNr} />
      </Box>
      <Text size="1" color="gray">
        Frame {nr + 1}/{frames}
      </Text>
      <Button size="1" variant="soft" onClick={() => onSpielt(!spielt)}>
        {spielt ? 'Anhalten' : 'Abspielen'}
      </Button>
    </Flex>
  );
}
