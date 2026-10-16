import { Text, TextField } from '@radix-ui/themes';
import { useEffect, useState } from 'react';
import { backend, type AssetKind, type CatalogAsset } from '../api';
import { errorText } from '../lib/errors';
import { useDebounced } from '../lib/useDebounced';
import { ActionButton, StatusBadge } from '../ui/parts';

const LIMIT = 60;

/** Katalog-Suche (B-298/AC-02): Treffer der Art des Slots mit Lizenz-Kennzeichen; „+“ hängt als letzte Präferenz an. */
export function CatalogPicker({ art, onAdd }: { art: AssetKind; onAdd: (datei: string) => Promise<boolean> }) {
  const [query, setQuery] = useState('');
  const q = useDebounced(query, 250);
  const [hits, setHits] = useState<CatalogAsset[]>([]);
  const [error, setError] = useState('');
  useEffect(() => {
    let live = true;
    backend.assetsCatalog(q, art, LIMIT).then(
      (list) => live && (setHits(list), setError('')),
      (e) => live && setError(errorText(e)),
    );
    return () => {
      live = false;
    };
  }, [q, art]);
  return (
    <div className="as-catalog">
      <TextField.Root placeholder="Katalog durchsuchen: Name, Pack, Gruppe, Lizenz" value={query} onChange={(e) => setQuery(e.target.value)} />
      {error && <p className="svc-error">{error}</p>}
      <Text size="1" color="gray">{hits.length >= LIMIT ? `erste ${LIMIT} Treffer` : `${hits.length} Treffer`}</Text>
      <ul className="as-hits">
        {hits.map((a) => (
          <li key={a.datei}>
            <ActionButton size="1" variant="soft" disabled={a.kennzeichen === 'ND'} onClick={() => onAdd(a.datei)} aria-label={`${a.datei} hinzufügen`}>+</ActionButton>
            <span className="as-hit-file">{a.datei}</span>
            <span className="as-dim">{a.pack}{a.gruppe ? ` · ${a.gruppe}` : ''}{a.w ? ` · ${a.w}×${a.h}` : ''} · {a.lizenz}</span>
            {a.kennzeichen && <StatusBadge tone={a.kennzeichen === 'SA' ? 'warn' : 'error'}>{a.kennzeichen}</StatusBadge>}
          </li>
        ))}
      </ul>
    </div>
  );
}
