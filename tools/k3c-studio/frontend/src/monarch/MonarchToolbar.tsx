import { Button, SelectField, Text, plainOptions, type SelectOption } from '@webkit/atoms';
import { ChipSwitch } from '@webkit/molecules';
import { useMemo } from 'react';
import { GapChip, PlaybackControls } from '../playback/PlaybackControls';
import type { FigureInfo } from '../stage/PreviewStage';
import { ACTIONS, SKILL_LINES, skillRows, type ActionId } from './actions';
import { MONARCH_ANIMS, MOUNT_KEYS } from './monarchLogic';
import { stepZoom } from './monarchZoom';
import { MODES, type Monarch } from './useMonarch';

const ACTION_OPTIONS: SelectOption<ActionId>[] = ACTIONS.map((a) => ({ value: a.id, label: a.label }));

/** Skills nach Linie gruppiert (Reihenfolge der Linien aus den Daten), Anzeige „Linie · Skill“ */
function skillOptions(): SelectOption<string>[] {
  const order = new Map(SKILL_LINES.map((l, i) => [l.name, i]));
  const rows = [...skillRows()].sort((a, b) => (order.get(a.line) ?? 99) - (order.get(b.line) ?? 99));
  return rows.map((r) => ({ value: r.id, label: `${r.line || '–'} · ${r.id}` }));
}

/** Zoom als Segment: − Wert +; ein Klick auf den Wert stellt den Spielmaßstab her (Alt + Mausrad zoomt ebenfalls). */
function ZoomSegment({ zoom, onZoom }: { zoom: number; onZoom: (z: number) => void }) {
  return (
    <div className="wd-segment" role="group" aria-label="Zoom" title="Alt + Mausrad zoomt">
      <Button size="1" variant="soft" aria-label="Verkleinern" onClick={() => onZoom(stepZoom(zoom, -1))}>−</Button>
      <Button size="1" variant="soft" aria-label="Zoomstufe" title="Spielmaßstab" disabled={zoom === 1} onClick={() => onZoom(1)}>{Math.round(zoom * 100)} %</Button>
      <Button size="1" variant="soft" aria-label="Vergrößern" onClick={() => onZoom(stepZoom(zoom, 1))}>+</Button>
    </div>
  );
}

/** Eine Werkzeugleiste (B-671): links Reittier, Modus und je nach Modus Animation, Aktion oder Skill; rechts Wiedergabe, Tempo,
 *  Zoom, der Knopf des Skills-Panels und die Lücken als ein Chip. Nicht geladene Grafiken meldet die Leiste. */
export function MonarchToolbar({ m, info, zoom, onZoom, panelOpen, onPanel, missing }: {
  m: Monarch; info: FigureInfo | undefined; zoom: number; onZoom: (z: number) => void; panelOpen: boolean; onPanel: (open: boolean) => void; missing: string[];
}) {
  const skills = useMemo(skillOptions, []);
  const moving = m.mode !== 'action' && m.mode !== 'skill';
  const gaps = [info?.gap, m.mountChoice?.gap, ...(m.action?.gaps ?? []), ...(m.skill?.gaps ?? [])];
  return (
    <div className="wd-figbar" role="toolbar" aria-label="Monarch">
      <SelectField size="1" aria-label="Reittier" value={m.mount} options={plainOptions(MOUNT_KEYS)} onChange={m.setMount} />
      <ChipSwitch label="Modus" options={MODES} value={m.mode} onChange={m.setMode} />
      {moving && <SelectField size="1" aria-label="Animation" value={m.anim} options={plainOptions(MONARCH_ANIMS)} onChange={m.setAnim} />}
      {m.mode === 'action' && <SelectField size="1" aria-label="Aktion" value={m.actionId} options={ACTION_OPTIONS} onChange={m.setActionId} />}
      {m.mode === 'skill' && <SelectField size="1" aria-label="Skill" value={m.skillId} options={skills} onChange={m.setSkillId} />}
      {missing.length > 0 && <Text size="1" color="red" role="status">Grafiken nicht geladen: {missing.join(', ')}</Text>}
      <div className="wd-figbar-end">
        <PlaybackControls control={m.control} onChange={m.setControl} frames={info?.frames ?? 1} />
        <ZoomSegment zoom={zoom} onZoom={onZoom} />
        <Button size="1" variant={panelOpen ? 'solid' : 'soft'} aria-pressed={panelOpen} onClick={() => onPanel(!panelOpen)}>Skills {skills.length}</Button>
        <GapChip gaps={gaps} />
      </div>
    </div>
  );
}
