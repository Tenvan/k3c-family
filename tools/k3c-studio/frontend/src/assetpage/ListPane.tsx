import { Button, Flex, Text } from '@webkit/atoms';
import type { ReactNode } from 'react';
import type { AssetKind } from '../api';
import { startSourcesChat } from '../chat/chatBridge';
import type { Group } from './filter';
import type { FileRow, SlotRow } from './model';
import { FilesView } from './FilesView';
import { searchPrompt } from './pageLogic';
import { SceneView } from './SceneView';
import { SlotsView } from './SlotsView';

/** Hinweis statt leerer Netz-Liste: Claude suchen lassen oder Treffer aus der Recherche übernehmen (Werkzeuge). */
function EmptyNet({ art, onTools }: { art: AssetKind | undefined; onTools: () => void }) {
  return (
    <Flex direction="column" gap="2" p="3" align="start" className="ap-empty" data-testid="net-empty">
      <Text>Noch nichts aus dem Netz vorgeschlagen.</Text>
      <Flex gap="2" wrap="wrap">
        <Button size="1" onClick={() => startSourcesChat(searchPrompt(art))}>Claude suchen lassen</Button>
        <Button size="1" variant="soft" onClick={onTools}>Aus Recherche übernehmen …</Button>
      </Flex>
    </Flex>
  );
}

interface Props {
  sicht: 'slots' | 'dateien' | 'szene';
  art: AssetKind | undefined;
  slotGroups: readonly Group<SlotRow>[];
  fileGroups: readonly Group<FileRow>[];
  selected: string;
  picked: ReadonlySet<string>;
  offline: boolean;
  /** der Fokus-Chip „Aus dem Netz“ ist aktiv */
  netFocus: boolean;
  onSelect: (id: string) => void;
  onPlaySlot: (r: SlotRow) => void;
  onPlayFile: (f: FileRow) => void;
  onPick: (keys: string[], on: boolean) => void;
  onTools: () => void;
}

/** Inhalt der Liste je Sicht; ein leerer Fokus „Aus dem Netz“ zeigt die beiden Wege, Vorschläge zu bekommen. */
export function ListPane(p: Props): ReactNode {
  if (p.sicht === 'szene') return <SceneView art={p.art ?? ''} />;
  const files = p.sicht === 'dateien';
  const empty = (files ? p.fileGroups : p.slotGroups).length === 0;
  return (
    <>
      {files
        ? <FilesView groups={p.fileGroups} selected={p.selected} picked={p.picked} offline={p.offline} onPick={p.onPick} onSelect={p.onSelect} onPlay={p.onPlayFile} />
        : <SlotsView groups={p.slotGroups} selected={p.selected} onSelect={p.onSelect} onPlay={p.onPlaySlot} />}
      {empty && p.netFocus && <EmptyNet art={p.art} onTools={p.onTools} />}
    </>
  );
}