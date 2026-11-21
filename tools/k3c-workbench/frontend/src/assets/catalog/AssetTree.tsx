import { useMemo, type KeyboardEvent, type ReactNode } from 'react';
import { StatusBadge, Text } from '@webkit/atoms';
import { TreeBar } from '../../lib/TreeBar';
import { useTreeOpen } from '../../lib/useTreeOpen';
import type { AssetEintrag } from '../api';
import { groupIcon, groupName, rigIcon } from './assetIcons';

// Baum der Assets (B-1216): Gruppen (Symbol, Name, Zahl) mit ihren Assets (Symbol der Art, ID, Art, Abzeichen). Alle Gruppen sind zunächst
// zugeklappt (Leiste „Alle auf/zu“); die ganze Zeile einer Gruppe klappt sie auf und zu; ein Klick auf ein Asset wählt es. Seite Katalog und Reiter Katalog des Welten-Editors
// zeigen denselben Baum; ungültige Assets stehen rot mit ihren Fehlern.

interface Props {
  assets: readonly AssetEintrag[];
  selected: string;
  onSelect: (id: string) => void;
  /** Abzeichen hinter dem Namen eines Assets (im Diff, Herkunft …) */
  badges?: (a: AssetEintrag) => ReactNode;
}

const onKey = (act: () => void) => (e: KeyboardEvent) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); act(); } };

function Leaf({ asset, active, onSelect, badges }: { asset: AssetEintrag; active: boolean; onSelect: (id: string) => void; badges?: ReactNode }) {
  const invalid = !asset.gueltig;
  const pick = () => onSelect(asset.id);
  return (
    <div role="treeitem" tabIndex={0} aria-selected={active} aria-level={2} data-asset={asset.id} data-invalid={String(invalid)}
      className={`at-node${active ? ' is-on' : ''}${invalid ? ' at-invalid' : ''}`} onClick={pick} onKeyDown={onKey(pick)}>
      <div className="at-row">
        <span aria-hidden="true" className="vt-icon">{rigIcon(asset.rig)}</span>
        <Text size="2" weight="bold" color={invalid ? 'red' : undefined}>{asset.id}</Text>
        <Text size="1" color="gray">{asset.rig}</Text>
        {badges}
        {asset.offen && <StatusBadge tone="neutral">prüft …</StatusBadge>}
        {invalid && <StatusBadge tone="error">ungültig</StatusBadge>}
      </div>
      {asset.fehler?.map((f) => (
        <Text key={`${f.feld}|${f.regel}`} as="div" size="1" color="red" className="at-error">{f.feld} · {f.regel}: {f.text}</Text>
      ))}
    </div>
  );
}

export function AssetTree({ assets, selected, onSelect, badges }: Props) {
  const groups = useMemo(() => [...new Set(assets.map((a) => a.gruppe))].sort(), [assets]);
  const tree = useTreeOpen(groups, [assets.find((a) => a.id === selected)?.gruppe ?? '']);
  return (
    <div className="at-wrap">
      <TreeBar tree={tree} />
      <div role="tree" aria-label="Assets nach Gruppe" className="at-tree">
        {groups.map((g) => {
          const own = assets.filter((a) => a.gruppe === g);
          const open = tree.isOpen(g);
          return (
            <div key={g} role="group" aria-label={g} className="at-branch" data-group={g}>
              <div role="button" tabIndex={0} className="vt-row at-group" aria-expanded={open} aria-label={`${groupName(g)} ${open ? 'einklappen' : 'aufklappen'}`}
                onClick={() => tree.flip(g)} onKeyDown={onKey(() => tree.flip(g))}>
                <span className="vt-toggle" aria-hidden="true"><span className="vt-chev" /></span>
                <span aria-hidden="true" className="vt-icon">{groupIcon(g)}</span>
                <Text size="2" weight="bold">{groupName(g)}</Text>
                <Text size="1" color="gray">{own.length}</Text>
              </div>
              {open && (
                <div className="vt-children">
                  {own.map((a) => <Leaf key={a.id} asset={a} active={a.id === selected} onSelect={onSelect} badges={badges?.(a)} />)}
                </div>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}
