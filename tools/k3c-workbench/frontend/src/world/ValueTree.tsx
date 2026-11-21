import { Badge, Button, Text } from '@webkit/atoms';
import { fileIcon, fileName, groupIcon, groupName } from './valueIcons';
import { groupRows, type ValueRow } from './worldOverrides';

// Baum der Ansicht „Werte“ (B-1216): „Alle“, darunter die Basis-Dateien mit Symbol, Name, Zahl der Felder und der
// überschriebenen; die gewählte Datei klappt ihre Themen auf. Die Wahl steuert den Filter der Tabelle (WorldValues).

/** Wahl im Baum: Datei ('' = alle) und Thema (null = die ganze Datei). */
export interface TreePick {
  file: string;
  group: string | null;
}

function Leaf({ on, icon, name, count, own, depth, id, onPick }: { on: boolean; icon: string; name: string; count: number; own: number; depth: number; id: string; onPick: () => void }) {
  return (
    <Button type="button" variant={on ? 'soft' : 'ghost'} color="gray" role="treeitem" aria-selected={on} aria-level={depth} className="vt-node" data-node={id} onClick={onPick}>
      <span aria-hidden="true" className="vt-icon">{icon}</span>
      <Text size="2" weight={depth === 1 ? 'bold' : 'regular'}>{name}</Text>
      <Text size="1" color="gray">{count}</Text>
      {own > 0 && <Badge color="orange" size="1">{own}</Badge>}
    </Button>
  );
}

/** `rows`: die Zeilen nach Suche und Modus, aber ohne Wahl im Baum. */
export function ValueTree({ rows, pick, onPick }: { rows: readonly ValueRow[]; pick: TreePick; onPick: (p: TreePick) => void }) {
  const groups = groupRows(rows);
  const files = new Map<string, { count: number; own: number; groups: typeof groups }>();
  for (const g of groups) {
    const f = files.get(g.file) ?? files.set(g.file, { count: 0, own: 0, groups: [] }).get(g.file)!;
    f.count += g.rows.length;
    f.own += g.own;
    f.groups.push(g);
  }
  const own = rows.filter((r) => r.status === 'überschrieben').length;
  return (
    <div role="tree" aria-label="Basis-Dateien und Themen" className="vt-tree">
      <Leaf on={pick.file === ''} icon="🗂" name="Alle Dateien" count={rows.length} own={own} depth={1} id="*" onPick={() => onPick({ file: '', group: null })} />
      {[...files].map(([file, f]) => (
        <div key={file} role="group">
          <Leaf on={pick.file === file && pick.group === null} icon={fileIcon(file)} name={fileName(file)} count={f.count} own={f.own} depth={1} id={file}
            onPick={() => onPick({ file, group: null })} />
          {pick.file === file && f.groups.map((g) => (
            <Leaf key={g.key} on={pick.group === g.group} icon={groupIcon(g.group)} name={groupName(g.group)} count={g.rows.length} own={g.own} depth={2}
              id={`${file}:${g.group}`} onPick={() => onPick({ file, group: g.group })} />
          ))}
        </div>
      ))}
    </div>
  );
}
