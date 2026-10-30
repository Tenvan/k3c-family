import { Badge, Button, Text, Tip } from '@webkit/atoms';
import { ChipSwitch } from '@webkit/molecules';
import { TEMPO_STEPS, pause, resume, setTempo, stepFrame, toggleLoop, type Control, type Gap } from './figure';

const TEMPO_LABEL: Record<number, string> = { 0.5: '½×', 1: '1×', 2: '2×' };
const TEMPO_OPTIONS = TEMPO_STEPS.map((t) => ({ value: String(t), label: TEMPO_LABEL[t] ?? `${t}×` }));

/** Kompakte Wiedergabe (B-463/AC-01, B-671, B-672): Symbol-Knöpfe Schleife, Bild zurück, Pause/Weiter, Bild vor und das Tempo
 *  als Segment (½× 1× 2×). `frames` ist die Bildzahl der gespielten Animation, `current` das Bild, bei dem sie steht. */
export function PlaybackControls({ control, onChange, frames, current = 0 }: { control: Control; onChange: (c: Control) => void; frames: number; current?: number }) {
  const stopped = control.frame !== null;
  return (
    <div className="wd-bar wd-playback" role="group" aria-label="Wiedergabe">
      <Button size="1" variant={control.loop ? 'solid' : 'soft'} aria-label="Schleife" title="Schleife" aria-pressed={control.loop} onClick={() => onChange(toggleLoop(control))}>⟲</Button>
      <Button size="1" variant="soft" aria-label="Bild zurück" title="Bild zurück" onClick={() => onChange(stepFrame(control, frames, -1, current))}>⏮</Button>
      {stopped
        ? <Button size="1" variant="soft" aria-label="Weiter" title="Weiter" onClick={() => onChange(resume(control))}>▶</Button>
        : <Button size="1" variant="soft" aria-label="Pause" title="Pause" onClick={() => onChange(pause(control, current))}>⏸</Button>}
      <Button size="1" variant="soft" aria-label="Bild vor" title="Bild vor" onClick={() => onChange(stepFrame(control, frames, 1, current))}>⏭</Button>
      <ChipSwitch label="Tempo" options={TEMPO_OPTIONS} value={String(control.tempo)} onChange={(v) => onChange(setTempo(control, Number(v)))} />
      {stopped && <Text size="1" color="gray">Bild {(control.frame ?? 0) + 1} von {frames}</Text>}
    </div>
  );
}

const reasonOf = (g: Gap): string => g.reason.replace(/^Lücke:\s*/, '');

/** Marke „Lücke“ mit dem Grund (B-463): zeigt an, dass das Spiel diese Darstellung nicht hat. Ohne Lücke nichts. */
export function GapBadge({ gap }: { gap: Gap | null | undefined }) {
  if (!gap) return null;
  return <Badge color="amber" variant="outline" className="wd-gap" data-kind={gap.kind} title={gap.reason}>Lücke: {reasonOf(gap)}</Badge>;
}

/** Lücken als ein Chip mit Anzahl, die Gründe im Tooltip (B-671, B-672); doppelte Gründe zählen einmal. Ohne Lücke nichts. */
export function GapChip({ gaps, label = 'Lücken' }: { gaps: readonly (Gap | null | undefined)[]; label?: string }) {
  const reasons = [...new Set(gaps.flatMap((g) => (g ? [reasonOf(g)] : [])))];
  if (reasons.length === 0) return null;
  const list = <ul className="wd-gap-list">{reasons.map((r) => <li key={r}>{r}</li>)}</ul>;
  return (
    <Tip content={list}>
      <Badge color="amber" variant="soft" className="wd-gap-chip" data-count={reasons.length} title={reasons.join('\n')}>{label} {reasons.length}</Badge>
    </Tip>
  );
}
