import { useEffect, useMemo, useState } from 'react';
import { ActionButton, Button, Flex, StatusBadge, Text } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';
import { ListFilter } from '@webkit/molecules';
import { backend } from '../../api';
import type { KatalogDatei, Quellen, Treffer } from '../api';
import { PromptButtons } from '../prompts/PromptButtons';
import { useVorlagen, type VorlagenJeOrt } from '../prompts/useVorlagen';
import { eintraege, QUELLEN_FILTER, type QuellenEintrag } from './sourcesFilter';

/** Seite Quellen im Bereich Assets (WB1.4, D-03): Katalog unter public/ mit Lizenz-Kennzeichen und die Recherche-Treffer
 *  je Slot, gemeinsam gefiltert nach Herkunft, Lizenz, Pack und Art; Klick auf eine Datei oder einen Treffer zeigt
 *  Prompt-Buttons (B-729). Recherche, Download, Import und Lizenz-Bestätigung bleiben MCP (assets_research, assets_hits,
 *  assets_import). */
export function SourcesPage() {
  const [q, setQ] = useState<Quellen | null>(null);
  const [error, setError] = useState('');
  const [wahl, setWahl] = useState('');
  const vorlagen = useVorlagen();
  useEffect(() => {
    let live = true;
    backend.studioSources().then((v) => live && setQ(v), (e) => live && setError(errorText(e)));
    return () => {
      live = false;
    };
  }, []);
  const items = useMemo(() => (q ? eintraege(q) : []), [q]);
  if (error) return <Text color="red" m="4">{error}</Text>;
  if (!q) return <Text color="gray" m="4">Lädt …</Text>;
  const zeile = (e: QuellenEintrag) => {
    const an = wahl === e.key;
    const props = { an, waehle: () => setWahl(an ? '' : e.key), jeOrt: vorlagen.jeOrt };
    return e.treffer ? <Hit slot={e.slot!} hit={e.treffer} eintrag={e} {...props} /> : <Datei datei={e.datei!} eintrag={e} {...props} />;
  };
  return (
    <Flex direction="column" gap="4" p="4">
      {vorlagen.error && <Text color="red">{vorlagen.error}</Text>}
      <ListFilter {...QUELLEN_FILTER} items={items} defaultGroup="herkunft" emptyHint="Keine Quellen" itemKey={(e) => e.key} renderItem={zeile} />
    </Flex>
  );
}

interface Wahl {
  eintrag: QuellenEintrag;
  an: boolean;
  waehle: () => void;
  jeOrt: VorlagenJeOrt;
}

/** Prompt-Buttons des gewählten Eintrags mit Datei, Pack und Lizenz (B-729/AC-02). */
function Prompts({ eintrag, jeOrt }: Pick<Wahl, 'eintrag' | 'jeOrt'>) {
  const ort = eintrag.treffer ? 'treffer' : 'datei';
  const werte = { datei: eintrag.ort, pack: eintrag.pack || 'ohne Pack', lizenz: eintrag.lizenzText, slot: eintrag.slot, treffer: eintrag.treffer?.id };
  return <PromptButtons caption={eintrag.treffer ? `Treffer: ${eintrag.treffer.titel}` : `Datei: ${eintrag.ort}`} vorlagen={jeOrt(ort)} werte={werte} leer="Keine Prompt-Vorlagen" />;
}

function Hit({ slot, hit, eintrag, an, waehle, jeOrt }: Wahl & { slot: string; hit: Treffer }) {
  return (
    <Flex direction="column" gap="1" data-treffer={hit.id} className="as-material">
      <Flex gap="2" align="center" wrap="wrap">
        <Text size="1" color="gray">{slot}</Text>
        <Button size="1" variant={an ? 'solid' : 'ghost'} aria-pressed={an} data-wahl="treffer" onClick={waehle}>{hit.titel}</Button>
        <Text size="1">{hit.quelle}</Text>
        {hit.lizenz && <Text size="1">{hit.lizenz}</Text>}
        <StatusBadge tone={hit.lizenzStatus === 'ausgeschlossen' ? 'error' : hit.lizenzStatus === 'pruefen' ? 'warn' : 'ok'}>{hit.lizenzStatus}</StatusBadge>
        <ActionButton size="1" variant="ghost" onClick={() => backend.openUrl(hit.link)}>Öffnen</ActionButton>
      </Flex>
      {an && <Prompts eintrag={eintrag} jeOrt={jeOrt} />}
    </Flex>
  );
}

function Datei({ datei, eintrag, an, waehle, jeOrt }: Wahl & { datei: KatalogDatei }) {
  return (
    <Flex direction="column" gap="1" data-datei={datei.datei}>
      <Flex gap="2" align="center" wrap="wrap">
        <Button size="1" variant={an ? 'solid' : 'ghost'} aria-pressed={an} data-wahl="datei" onClick={waehle}>{datei.datei}</Button>
        <Text size="1" color="gray">{datei.pack}</Text>
        <Text size="1">{datei.lizenz}</Text>
        {datei.kennzeichen && <StatusBadge tone={datei.kennzeichen === 'ND' ? 'error' : 'warn'}>{datei.kennzeichen}</StatusBadge>}
      </Flex>
      {an && <Prompts eintrag={eintrag} jeOrt={jeOrt} />}
    </Flex>
  );
}
