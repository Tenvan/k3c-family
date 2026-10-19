import { Button, Text } from '@radix-ui/themes';
import { TEMPO_STEPS, resume, setTempo, stepFrame, toggleLoop, type Control, type Gap } from './figure';

/** Steuerung der Wiedergabe (B-463/AC-01): Schleife an/aus, Einzelbild vor und zurück, Tempo 0,5×, 1×, 2×. `frames` ist
 *  die Bildzahl der gespielten Animation, `current` das Bild, bei dem sie steht (für den ersten Einzelschritt). */
export function PlaybackControls({ control, onChange, frames, current = 0 }: { control: Control; onChange: (c: Control) => void; frames: number; current?: number }) {
  const stopped = control.frame !== null;
  return (
    <div className="wd-bar wd-playback" role="group" aria-label="Wiedergabe">
      <Button variant={control.loop ? 'solid' : 'soft'} aria-pressed={control.loop} onClick={() => onChange(toggleLoop(control))}>Schleife</Button>
      <Button variant="soft" aria-label="Bild zurück" onClick={() => onChange(stepFrame(control, frames, -1, current))}>◀︎</Button>
      <Button variant="soft" aria-label="Bild vor" onClick={() => onChange(stepFrame(control, frames, 1, current))}>▶︎</Button>
      <Button variant="soft" disabled={!stopped} onClick={() => onChange(resume(control))}>Weiter</Button>
      <select className="wd-select" aria-label="Tempo" value={control.tempo} onChange={(e) => onChange(setTempo(control, Number(e.target.value)))}>
        {TEMPO_STEPS.map((t) => <option key={t} value={t}>{`${t}×`}</option>)}
      </select>
      {stopped && <Text size="1" color="gray">Bild {(control.frame ?? 0) + 1} von {frames}</Text>}
    </div>
  );
}

/** Marke „Lücke“ mit dem Grund (B-463): zeigt an, dass das Spiel diese Darstellung nicht hat. Ohne Lücke nichts. */
export function GapBadge({ gap }: { gap: Gap | null | undefined }) {
  if (!gap) return null;
  return <span className="wd-gap" data-kind={gap.kind} title={gap.reason}>Lücke: {gap.reason.replace(/^Lücke:\s*/, '')}</span>;
}
