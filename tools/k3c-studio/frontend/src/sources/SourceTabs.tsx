import { Button, Flex, Link, Text } from '@webkit/atoms';
import { ChipSwitch } from '@webkit/molecules';
import type { SourceInfo } from '../api';

const ALL = 'alle';

/** Startseite je Quelle für den Link unter dem Reiter. */
const HOME: Record<string, string> = { OpenGameArt: 'https://opengameart.org', Kenney: 'https://kenney.nl', 'itch.io': 'https://itch.io', freesound: 'https://freesound.org' };

interface Props {
  quellen: readonly SourceInfo[];
  total: number;
  /** gewählte Quelle, "" = alle */
  tab: string;
  onTab: (tab: string) => void;
  offline: boolean;
  /** Wege zum Füllen anzeigen (bei leerer Quelle stehen sie im Hinweis darunter) */
  showActions: boolean;
  onSearch: () => void;
  onResearch: () => void;
}

/** Quellen als Reiter mit Anzahl (B-673); darunter in einer Zeile die Lizenzregel, der Link zur Quelle und die Wege,
 *  Vorschläge zu füllen (Claude suchen lassen, aus Recherche übernehmen; Übernahme offline gesperrt). */
export function SourceTabs({ quellen, total, tab, onTab, offline, showActions, onSearch, onResearch }: Props) {
  const info = quellen.find((q) => q.name === tab);
  return (
    <Flex direction="column" gap="1" data-testid="source-tabs">
      <ChipSwitch
        label="Quelle"
        value={tab || ALL}
        onChange={(v) => onTab(v === ALL ? '' : v)}
        options={[{ value: ALL, label: 'Alle', count: total }, ...quellen.map((q) => ({ value: q.name, label: q.name, count: q.vorgeschlagen }))]}
      />
      <Flex gap="3" align="center" wrap="wrap">
        <Text size="1" color="gray" data-testid="source-rules">{info ? info.lizenzregeln : 'Lizenz je Eintrag: CC0, CC-BY, OGA-BY, CC-BY-SA; NC und ND nie (Q70).'}</Text>
        {HOME[tab] && <Link size="1" href={HOME[tab]} target="_blank" rel="noopener noreferrer">{tab} öffnen</Link>}
        {showActions && <Button size="1" variant="ghost" onClick={onSearch}>Claude suchen lassen</Button>}
        {showActions && <Button size="1" variant="ghost" disabled={offline} title={offline ? 'offline: Größe per HEAD nicht möglich' : undefined} onClick={onResearch}>Aus Recherche übernehmen</Button>}
      </Flex>
    </Flex>
  );
}
