import { Box, Button, Flex, Text, ValueSlider } from '../atoms';

export interface FramePlayerProps {
  /** Gezeigter Frame, 0-basiert. */
  frame: number;
  frames: number;
  playing: boolean;
  onFrame: (frame: number) => void;
  onPlaying: (playing: boolean) => void;
  /** Breite des Schiebers in Pixeln (Standard 240). */
  sliderWidth?: number;
  /** Text vor dem Zähler (Standard „Frame“, leer = nur `n/m`). */
  prefix?: string;
}

/** Breite des Zählers in `ch` für die längste Anzeige (`<prefix> m/m`); mit `tabular-nums` sind alle Ziffern gleich breit. */
export function counterWidth(frames: number, prefix: string): string {
  const n = String(Math.max(1, frames));
  return `${(prefix ? `${prefix} ` : '').length + n.length * 2 + 1}ch`;
}

/** Breite des Knopfs: passt für „Abspielen“ und „Anhalten“, damit der Wechsel nichts verschiebt. */
const PLAY_WIDTH = '6.5rem';

/** Abspielen, Frame-Schieber und Zähler mit festen Breiten (WB2): wechselnde Zahlen und Knopftexte verschieben nichts.
 *  Bei nur einem Frame sind Knopf und Schieber gesperrt. Das Weiterzählen im Takt übernimmt die Seite. */
export function FramePlayer({ frame, frames, playing, onFrame, onPlaying, sliderWidth = 240, prefix = 'Frame' }: FramePlayerProps) {
  const single = frames < 2;
  const label = `${prefix ? `${prefix} ` : ''}${frame + 1}/${frames}`;
  return (
    <Flex gap="3" align="center" wrap="nowrap" className="wk-frameplayer">
      <Button size="1" variant="soft" data-part="play" disabled={single} style={{ width: PLAY_WIDTH, flex: 'none' }} onClick={() => onPlaying(!playing)}>
        {playing ? 'Anhalten' : 'Abspielen'}
      </Button>
      <Box data-part="slider" style={{ width: `${sliderWidth}px`, flex: 'none' }}>
        <ValueSlider aria-label="Frame" value={frame} min={0} max={Math.max(frames - 1, 0)} disabled={single} onChange={onFrame} />
      </Box>
      <Text size="1" color="gray" data-part="count" style={{ width: counterWidth(frames, prefix), flex: 'none', textAlign: 'right', whiteSpace: 'nowrap', fontVariantNumeric: 'tabular-nums' }}>
        {label}
      </Text>
    </Flex>
  );
}
