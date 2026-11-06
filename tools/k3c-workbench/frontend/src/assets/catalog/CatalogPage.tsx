import { useState } from 'react';
import { Flex, Text } from '@webkit/atoms';
import { AssetList } from './AssetList';
import { Preview } from './Preview';
import { useCatalog } from './useCatalog';

/** Seite Katalog im Bereich Assets (WB1.2): Liste aller Specs, Vorschau des gewählten Assets. Nur Ansehen; Änderungen
 *  laufen über die MCP-Tools assets_* (Entscheidung 005 › 4). */
export function CatalogPage() {
  const { list, inDiff, error } = useCatalog();
  const [selected, setSelected] = useState('');
  return (
    <Flex gap="4" p="4" className="as-page">
      <AssetList list={list} inDiff={inDiff} selected={selected} onSelect={setSelected} />
      <Flex direction="column" gap="2" className="as-detail">
        {error && <Text color="red">{error}</Text>}
        {selected ? <Preview id={selected} /> : <Text color="gray">Asset links wählen</Text>}
      </Flex>
    </Flex>
  );
}
