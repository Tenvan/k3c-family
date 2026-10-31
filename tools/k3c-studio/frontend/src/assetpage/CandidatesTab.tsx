import { ActionButton, Flex, Text, TextField } from '@webkit/atoms';
import { useDebounced } from '@webkit/lib/useDebounced';
import { NoticeCard } from '@webkit/molecules';
import { useState } from 'react';
import type { AssetSlot } from '../api';
import '../styles/assetInspector.css';
import { CandidateRow } from './CandidateRow';
import { useCandidates } from './useCandidates';

interface Props {
  slot: AssetSlot;
  /** Slot-Register neu laden (nach Zuordnen oder Lizenz-Bestätigung) */
  onReload: () => void;
}

/** Reiter Kandidaten (B-677): Zuordnungen, Spiel (Katalog), Vorrat und Netz in einer Liste. Katalog-Suche oben, „Im Netz
 *  suchen“ startet die Recherche. Setze `key={slot.id}`, damit Liste und Hinweis nicht in den nächsten Slot wandern. */
export function CandidatesTab({ slot, onReload }: Props) {
  const [query, setQuery] = useState('');
  const debounced = useDebounced(query);
  const { view, loading, error, note, searching, reload, research } = useCandidates(slot.id, debounced.trim());
  const changed = () => { onReload(); void reload(); };
  return (
    <div className="ap-cands">
      <Flex gap="2" align="center" wrap="wrap">
        <TextField.Root className="ap-cands-query" value={query} onChange={(e) => setQuery(e.target.value)} aria-label="Katalog durchsuchen"
          placeholder={`Katalog durchsuchen (leer: ${slot.beschreibung.split(':').pop()?.trim() || slot.id})`} />
        <ActionButton size="1" variant="soft" disabled={searching || view?.offline} onClick={() => research(query.trim())}>Im Netz suchen</ActionButton>
      </Flex>
      {view?.offline && <NoticeCard title="Offline" tone="warn">Netz-Kandidaten sind sichtbar, Zuordnen ist gesperrt.</NoticeCard>}
      {note && <Text size="1" color="gray" data-testid="research-note">{note}</Text>}
      {error && <p className="svc-error">{error}</p>}
      {loading && !view && <Text size="2" color="gray">Lädt …</Text>}
      {view && view.kandidaten.length === 0 && <Text size="2" color="gray">Keine Kandidaten: anderen Suchbegriff versuchen oder im Netz suchen.</Text>}
      {view && view.kandidaten.length > 0 && (
        <ul className="ap-cand-list">
          {view.kandidaten.map((c) => (
            <CandidateRow key={c.id} c={c} slotId={slot.id} art={slot.art} stand={view.stand} offline={view.offline} onChanged={changed} onStale={reload} />
          ))}
        </ul>
      )}
    </div>
  );
}
