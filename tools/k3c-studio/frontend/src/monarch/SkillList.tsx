import { Text } from '@webkit/atoms';
import { useMemo } from 'react';
import { GapBadge } from '../playback/PlaybackControls';
import { type SkillRow, skillRows } from './actions';

/** Alle Skills des Monarchen (B-463/AC-06, AC-07): Linie, Stufe, Art und der Effekt des Spiels (B-562, B-578: Farbe und
 *  Bewegung; eigener Effekt oder Ring der Linie). Nur ein Skill ohne Darstellung trägt die Marke „Lücke“. */
export function SkillList() {
  const rows = useMemo(skillRows, []);
  return (
    <section aria-label="Skills">
      <Text size="2" weight="bold">Skills ({rows.length})</Text>
      <table className="wd-skills">
        <thead><tr><th>Skill</th><th>Linie</th><th>Stufe</th><th>Art</th><th>Darstellung</th></tr></thead>
        <tbody>
          {rows.map((r) => (
            <tr key={r.id} data-skill={r.id}>
              <td>{r.id}</td><td>{r.line}</td><td>{r.tier ?? ''}</td><td>{r.kind}</td>
              <td>{r.gap ? <GapBadge gap={r.gap} /> : <SkillLookCell row={r} />}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </section>
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
