import { useState } from 'react';
import { Box, Button, Flex, Heading, Text, TextField } from '@webkit/atoms';
import { StatusBadge } from '@webkit/atoms';
import { CheckField, LoadingNote } from '@webkit/molecules';
import type { AssetEintrag } from '../api';

interface Props {
  list: AssetEintrag[];
  /** Liste lädt noch: Lade-Anzeige statt „Keine Assets“ (WB2/AC-03). */
  loading?: boolean;
  inDiff: Set<string>;
  /** Noch ungeprüfte Einträge (B-732): Fortschritt über der Liste. */
  open?: number;
  /** „Neu laden“: Cache leeren und den Katalog neu holen (B-732). */
  onReload?: () => void;
  selected: string;
  onSelect: (id: string) => void;
}

/** Liste der Specs nach Gruppe; ungültige rot mit `feld · regel` und Text (005 › 7), Abzeichen „im Diff“. */
export function AssetList({ list, loading = false, inDiff, open = 0, onReload, selected, onSelect }: Props) {
  const [query, setQuery] = useState('');
  const [onlyDiff, setOnlyDiff] = useState(false);
  const q = query.trim().toLowerCase();
  const shown = list.filter((a) => (!onlyDiff || inDiff.has(a.id)) && (!q || a.id.includes(q) || a.rig.includes(q) || a.gruppe.includes(q)));
  const groups = [...new Set(shown.map((a) => a.gruppe))].sort();
  return (
    <Flex direction="column" gap="3">
      <Box className="as-list-filter">
        <Flex gap="2" align="center" mb="1">
          <Box flexGrow="1">
            <TextField.Root size="1" placeholder="Filter: ID, Rig, Gruppe" aria-label="Filter" value={query} onChange={(e) => setQuery(e.target.value)} />
          </Box>
          {onReload && <Button size="1" variant="soft" disabled={loading} onClick={onReload}>Neu laden</Button>}
        </Flex>
        {/* nur Assets mit Änderung gegen HEAD, wie das Abzeichen „im Diff“ (B-732) */}
        <CheckField size="1" label={`nur geänderte (${list.filter((a) => inDiff.has(a.id)).length})`} checked={onlyDiff} onCheckedChange={(c) => setOnlyDiff(c === true)} />
        {open > 0 && <Text as="div" size="1" color="gray" data-checking>Prüft {list.length - open}/{list.length} …</Text>}
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
        {asset.offen && <StatusBadge tone="neutral">prüft …</StatusBadge>}
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
