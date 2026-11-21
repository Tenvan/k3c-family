import { useState } from 'react';
import { Flex, Text } from '@webkit/atoms';
import { ListDetail } from '@webkit/templates';
import { GameOpen } from '../game/GameOpen';
import { AssetDetail } from './AssetDetail';
import { AssetList } from './AssetList';
import { useCatalog } from './useCatalog';

/** Seite Katalog im Bereich Assets (WB1.2, WB1.3): Liste aller Specs, Vorschau des gewählten Assets mit Teil-Auswahl per
 *  Klick und Prompt-Buttons (`AssetDetail`). Nur Ansehen; Änderungen laufen über die MCP-Tools assets_* (Entscheidung 005 › 4, 5). */
export function CatalogPage() {
  const { list, inDiff, prompts, error, loading, open, reload, stand } = useCatalog();
  const [selected, setSelected] = useState('');
  // Liste und Vorschau scrollen je für sich (WB2/AC-04)
  return (
    <ListDetail listWidth={300} list={<AssetList list={list} loading={loading} inDiff={inDiff} open={open} onReload={reload} selected={selected} onSelect={setSelected} />}>
      <Flex direction="column" gap="2">
        <GameOpen />
        {error && <Text color="red">{error}</Text>}
        <AssetDetail key={selected} selected={selected} list={list} prompts={prompts} stand={stand} />
      </Flex>
    </ListDetail>
  );
}
