import { Heading, Text } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';
import { DataList, NoticeCard } from '@webkit/molecules';
import { useEffect, useState } from 'react';
import { backend, type AssetSlot, type AssetSlotDetails } from '../api';
import { infoGroups } from './slotRows';

/** Details eines Slots (WZ28.1): Hinweise oben, darunter Definition, Sheet, Atlas, Zuordnung und Verwendung. Lädt beim Slot-Wechsel und bei `assets:changed`. */
export function SlotInfo({ slot }: { slot: AssetSlot }) {
  const [details, setDetails] = useState<AssetSlotDetails | null>(null);
  const [error, setError] = useState('');
  useEffect(() => {
    let live = true;
    const load = () => backend.assetsSlotDetails(slot.id).then(
      (d) => { if (live) { setDetails(d); setError(''); } },
      (e) => { if (live) setError(errorText(e)); },
    );
    void load();
    const off = backend.on('assets:changed', () => void load());
    return () => { live = false; off(); };
  }, [slot.id]);
  if (error) return <NoticeCard title="Details nicht lesbar" tone="error">{error}</NoticeCard>;
  if (!details) return null;
  return (
    <div className="as-info">
      {details.hinweise.map((h) => <NoticeCard key={h} title="Hinweis" tone="warn">{h}</NoticeCard>)}
      {infoGroups(details).map((g) => (
        <section key={g.title}>
          <Heading size="2">{g.title}</Heading>
          <DataList.Root size="1">
            {g.rows.map((r, i) => (
              <DataList.Item key={`${i}:${r.label}`}>
                <DataList.Label minWidth="140px"><Text size="1">{r.label}</Text></DataList.Label>
                <DataList.Value><Text size="1">{r.value}</Text></DataList.Value>
              </DataList.Item>
            ))}
          </DataList.Root>
        </section>
      ))}
    </div>
  );
}
