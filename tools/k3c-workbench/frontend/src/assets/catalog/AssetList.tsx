import { useState } from 'react';
import { Box, Flex, Heading, Text, TextField } from '@webkit/atoms';
import { StatusBadge } from '@webkit/atoms';
import { LoadingNote } from '@webkit/molecules';
import type { AssetEintrag } from '../api';

interface Props {
  list: AssetEintrag[];
  /** Liste lädt noch: Lade-Anzeige statt „Keine Assets“ (WB2/AC-03). */
  loading?: boolean;
  inDiff: Set<string>;
  selected: string;
  onSelect: (id: string) => void;
}

/** Liste der Specs nach Gruppe; ungültige rot mit `feld · regel` und Text (005 › 7), Abzeichen „im Diff“. */
export function AssetList({ list, loading = false, inDiff, selected, onSelect }: Props) {
  const [query, setQuery] = useState('');
  const q = query.trim().toLowerCase();
  const shown = list.filter((a) => !q || a.id.includes(q) || a.rig.includes(q) || a.gruppe.includes(q));
  const groups = [...new Set(shown.map((a) => a.gruppe))].sort();
  return (
    <Flex direction="column" gap="3">
      <Box className="as-list-filter">
        <TextField.Root size="1" placeholder="Filter: ID, Rig, Gruppe" aria-label="Filter" value={query} onChange={(e) => setQuery(e.target.value)} />
      </Box>
      {groups.map((g) => (
        <Flex key={g} direction="column" gap="1">
          <Heading size="2" color="gray">{g}</Heading>
          {shown.filter((a) => a.gruppe === g).map((a) => (
            <Entry key={a.id} asset={a} diff={inDiff.has(a.id)} active={a.id === selected} onSelect={onSelect} />
          ))}
        </Flex>
      ))}
      {loading && <LoadingNote id="list" text="Lädt Assets …" />}
      {!loading && !shown.length && <Text size="1" color="gray">Keine Assets</Text>}
    </Flex>
  );
}

function Entry({ asset, diff, active, onSelect }: { asset: AssetEintrag; diff: boolean; active: boolean; onSelect: (id: string) => void }) {
  const invalid = !asset.gueltig;
  return (
    <Box role="button" tabIndex={0} data-asset={asset.id} data-invalid={String(invalid)} aria-pressed={active}
      className={`as-entry${active ? ' as-entry-active' : ''}${invalid ? ' as-entry-invalid' : ''}`}
      onClick={() => onSelect(asset.id)} onKeyDown={(e) => (e.key === 'Enter' || e.key === ' ') && onSelect(asset.id)}>
      <Flex gap="2" align="center" wrap="wrap">
        <Text size="2" weight="bold" color={invalid ? 'red' : undefined}>{asset.id}</Text>
        <Text size="1" color="gray">{asset.rig}</Text>
        {diff && <StatusBadge tone="info">im Diff</StatusBadge>}
        {invalid && <StatusBadge tone="error">ungültig</StatusBadge>}
      </Flex>
      {asset.fehler?.map((f) => (
        <Text key={`${f.feld}|${f.regel}`} as="div" size="1" color="red">
          {f.feld} · {f.regel}: {f.text}
        </Text>
      ))}
    </Box>
  );
}
