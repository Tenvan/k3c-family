import { useEffect, useState } from 'react';
import { ActionButton, Flex, Heading, StatusBadge, Text, TextField } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';
import { backend } from '../../api';
import type { KatalogDatei, Quellen, Treffer } from '../api';

/** Seite Quellen im Bereich Assets (WB1.4, D-03): Katalog unter public/ mit Lizenz-Kennzeichen und die Recherche-Treffer
 *  je Slot. Recherche, Download, Import und Lizenz-Bestätigung bleiben MCP (assets_research, assets_hits, assets_import). */
export function SourcesPage() {
  const [q, setQ] = useState<Quellen | null>(null);
  const [error, setError] = useState('');
  const [filter, setFilter] = useState('');
  useEffect(() => {
    let live = true;
    backend.studioSources().then((v) => live && setQ(v), (e) => live && setError(errorText(e)));
    return () => {
      live = false;
    };
  }, []);
  if (error) return <Text color="red" m="4">{error}</Text>;
  if (!q) return <Text color="gray" m="4">Lädt …</Text>;
  const f = filter.trim().toLowerCase();
  const katalog = q.katalog.filter((d) => !f || `${d.datei} ${d.pack} ${d.lizenz}`.toLowerCase().includes(f));
  const treffer = Object.entries(q.treffer).flatMap(([slot, hits]) => hits.map((h) => ({ slot, h })));
  return (
    <Flex direction="column" gap="4" p="4">
      <Heading size="4">Recherche-Treffer</Heading>
      {treffer.length ? treffer.map(({ slot, h }) => <Hit key={`${slot}|${h.id}`} slot={slot} hit={h} />) : <Text color="gray">Keine Treffer</Text>}
      <Heading size="4">Katalog ({q.katalog.length})</Heading>
      <TextField.Root size="1" placeholder="Filter: Datei, Pack, Lizenz" aria-label="Filter" value={filter} onChange={(e) => setFilter(e.target.value)} />
      {katalog.map((d) => <Datei key={d.datei} datei={d} />)}
    </Flex>
  );
}

function Hit({ slot, hit }: { slot: string; hit: Treffer }) {
  return (
    <Flex gap="2" align="center" wrap="wrap" data-treffer={hit.id} className="as-material">
      <Text size="1" color="gray">{slot}</Text>
      <Text size="2" weight="bold">{hit.titel}</Text>
      <Text size="1">{hit.quelle}</Text>
      {hit.lizenz && <Text size="1">{hit.lizenz}</Text>}
      <StatusBadge tone={hit.lizenzStatus === 'ausgeschlossen' ? 'error' : hit.lizenzStatus === 'pruefen' ? 'warn' : 'ok'}>{hit.lizenzStatus}</StatusBadge>
      <ActionButton size="1" variant="ghost" onClick={() => backend.openUrl(hit.link)}>Öffnen</ActionButton>
    </Flex>
  );
}

function Datei({ datei }: { datei: KatalogDatei }) {
  return (
    <Flex gap="2" align="center" wrap="wrap" data-datei={datei.datei}>
      <Text size="1">{datei.datei}</Text>
      <Text size="1" color="gray">{datei.pack}</Text>
      <Text size="1">{datei.lizenz}</Text>
      {datei.kennzeichen && <StatusBadge tone={datei.kennzeichen === 'ND' ? 'error' : 'warn'}>{datei.kennzeichen}</StatusBadge>}
    </Flex>
  );
}
