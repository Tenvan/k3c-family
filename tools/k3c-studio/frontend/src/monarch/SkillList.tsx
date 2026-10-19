import { Text } from '@radix-ui/themes';
import { useMemo } from 'react';
import { GapBadge } from '../playback/PlaybackControls';
import { skillRows } from './actions';

/** Alle Skills des Monarchen (B-463/AC-06, AC-07): Linie, Stufe, Art. Jeder Skill ohne Darstellung im Spiel trägt die Marke
 *  „Lücke“; Skills der Presets, die in `data/monarch.json` fehlen, stehen mit eigener Lücke in der Liste. */
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
              <td><GapBadge gap={r.gap} /></td>
            </tr>
          ))}
        </tbody>
      </table>
    </section>
  );
}
