import { Badge, Button, Flex, IconButton, Text } from '@webkit/atoms';
import { TreeBar } from '../lib/TreeBar';
import { useTreeOpen } from '../lib/useTreeOpen';
import { fileIcon, fileName, groupIcon, groupName } from './valueIcons';
import { groupRows, type ValueRow } from './worldOverrides';

// Baum der Ansicht „Werte“ (B-1216): die Basis-Dateien mit Symbol, Name, Zahl der Felder und der
// überschriebenen; jede Datei klappt ihre Themen auf und zu (Pfeil), Linien führen zu den Kindern. Die Wahl steuert den
// Filter der Tabelle (WorldValues).

/** Wahl im Baum: Datei ('' = alle) und Thema (null = die ganze Datei). */
export interface TreePick {
  file: string;
  group: string | null;
}

interface Counts {
  count: number;
  own: number;
}

function Row({ on, icon, name, count, own, level, id, open, onToggle, onPick }:
  Counts & { on: boolean; icon: string; name: string; level: number; id: string; open?: boolean; onToggle?: () => void; onPick: () => void }) {
  return (
    <Flex align="center" gap="1" className="vt-row" data-level={level}>
      {onToggle
        ? (
          <IconButton size="1" variant="ghost" color="gray" aria-label={`${name} ${open ? 'einklappen' : 'aufklappen'}`} aria-expanded={open} className="vt-toggle" onClick={onToggle}>
            <span className="vt-chev" aria-hidden="true" />
          </IconButton>
        )
        : <span className="vt-toggle" aria-hidden="true" />}
      <Button type="button" variant="ghost" color="gray" role="treeitem" aria-selected={on} aria-level={level} aria-expanded={onToggle ? open : undefined}
        className={`vt-node${on ? ' is-on' : ''}`} data-node={id} onClick={onPick}>
        <span aria-hidden="true" className="vt-icon">{icon}</span>
        <Text size="2" weight={level === 1 ? 'bold' : 'regular'}>{name}</Text>
        <Text size="1" color="gray">{count}</Text>
        {own > 0 && <Badge color="orange" size="1">{own}</Badge>}
      </Button>
    </Flex>
  );
}

/** `rows`: die Zeilen nach Suche und Modus, aber ohne Wahl im Baum. */
export function ValueTree({ rows, pick, onPick }: { rows: readonly ValueRow[]; pick: TreePick; onPick: (p: TreePick) => void }) {
  const groups = groupRows(rows);
  const files = new Map<string, Counts & { groups: typeof groups }>();
  for (const g of groups) {
    const f = files.get(g.file) ?? files.set(g.file, { count: 0, own: 0, groups: [] }).get(g.file)!;
    f.count += g.rows.length;
    f.own += g.own;
    f.groups.push(g);
  }
  // alles zugeklappt; nur die schon gewählte Datei ist am Anfang offen
  const tree = useTreeOpen([...files.keys()], pick.file ? [pick.file] : []);
  // Ein zweiter Klick auf den gewählten Knoten hebt die Wahl auf (alle Dateien); ein Klick öffnet die Datei
  const select = (p: TreePick) => {
    if (pick.file === p.file && pick.group === p.group) return onPick({ file: '', group: null });
    tree.open(p.file);
    onPick(p);
  };
  return (
    <div className="vt-tree">
      <TreeBar tree={tree} />
      <div role="tree" aria-label="Basis-Dateien und Themen" className="at-tree">
      {[...files].map(([file, f]) => (
        <div key={file} role="group" className="vt-branch">
          <Row on={pick.file === file && pick.group === null} icon={fileIcon(file)} name={fileName(file)} count={f.count} own={f.own} level={1} id={file}
            open={tree.isOpen(file)} onToggle={() => tree.flip(file)} onPick={() => select({ file, group: null })} />
          {tree.isOpen(file) && (
            <div role="group" className="vt-children">
              {f.groups.map((g) => (
                <Row key={g.key} on={pick.file === file && pick.group === g.group} icon={groupIcon(g.group)} name={groupName(g.group)} count={g.rows.length} own={g.own}
                  level={2} id={`${file}:${g.group}`} onPick={() => select({ file, group: g.group })} />
              ))}
            </div>
          )}
        </div>
      ))}
      </div>
    </div>
  );
}
