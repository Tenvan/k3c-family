import { Button, Text } from '@webkit/atoms';
import { ChipSwitch } from '@webkit/molecules';
import { useMemo, useState } from 'react';
import { GapBadge } from '../playback/PlaybackControls';
import { SKILL_LINES, type SkillRow, skillRows } from './actions';

const ALL = 'alle';
const FILTERS = [{ value: ALL, label: 'Alle' }, ...SKILL_LINES.map((l) => ({ value: l.name, label: l.name }))];

/** Skills des Monarchen als Seitenpanel (B-463/AC-06, AC-07; B-671): Linie, Stufe, Art und der Effekt des Spiels (B-562,
 *  B-578: Farbe und Bewegung; eigener Effekt oder Ring der Linie), Filter nach Linie, ▶ spielt den Skill auf der Bühne.
 *  Nur ein Skill ohne Darstellung trägt die Marke „Lücke“. */
export function SkillPanel({ current, onPlay, onClose }: { current: string | null; onPlay: (id: string) => void; onClose: () => void }) {
  const rows = useMemo(skillRows, []);
  const [line, setLine] = useState(ALL);
  const shown = line === ALL ? rows : rows.filter((r) => r.line === line);
  return (
    <aside className="wd-skill-panel" aria-label="Skills">
      <div className="wd-skill-head">
        <Text size="2" weight="bold">Skills ({rows.length})</Text>
        <Button size="1" variant="ghost" aria-label="Skills schließen" onClick={onClose}>✕</Button>
      </div>
      <ChipSwitch label="Linie" options={FILTERS} value={line} onChange={setLine} />
      <table className="wd-skills">
        <thead><tr><th aria-label="Abspielen" /><th>Skill</th><th>Linie</th><th>Stufe</th><th>Art</th><th>Darstellung</th></tr></thead>
        <tbody>
          {shown.map((r) => (
            <tr key={r.id} data-skill={r.id} aria-selected={r.id === current}>
              <td><Button size="1" variant="ghost" aria-label={`Skill ${r.id} abspielen`} onClick={() => onPlay(r.id)}>▶</Button></td>
              <td>{r.id}</td><td>{r.line}</td><td>{r.tier ?? ''}</td><td>{r.kind}</td>
              <td>{r.gap ? <GapBadge gap={r.gap} /> : <SkillLookCell row={r} />}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </aside>
  );
}

const MOTION: Record<string, string> = { burst: 'Kreis', rise: 'steigt', fall: 'fällt', wave: 'Welle' };

/** Farbfeld und Bewegung des Skill-Effekts aus dem Spiel */
function SkillLookCell({ row }: { row: SkillRow }) {
  const look = row.look!;
  const hex = `#${look.color.toString(16).padStart(6, '0')}`;
  return (
    <span className="wd-skill-look" title={hex}>
      <span className="wd-skill-swatch" style={{ background: hex }} aria-hidden /> {row.own ? MOTION[look.motion] : 'Ring der Linie'}
    </span>
  );
}
