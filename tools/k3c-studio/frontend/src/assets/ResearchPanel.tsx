import { Button, Flex, Text, TextField } from '@radix-ui/themes';
import { useEffect, useState } from 'react';
import { backend, type AssetHit, type AssetSlot, type LicenseStatus } from '../api';
import { errorText } from '../lib/errors';
import { ActionButton, StatusBadge, type Tone } from '../ui/parts';

const STATUS_TONE: Record<LicenseStatus, Tone> = { erkannt: 'ok', pruefen: 'warn', ausgeschlossen: 'error', bestaetigt: 'ok' };
const STATUS_LABEL: Record<LicenseStatus, string> = {
  erkannt: 'Lizenz erkannt', pruefen: 'Lizenz prüfen', ausgeschlossen: 'nicht übernehmbar', bestaetigt: 'Lizenz bestätigt',
};

/** Web-Recherche je Slot (B-298/AC-04): Treffer mit Link, Quelle, Lizenz und Vorschau; bleiben, bis 🧑 sie verwirft. */
export function ResearchPanel({ slot, onImported }: { slot: AssetSlot; onImported: () => void }) {
  const [hits, setHits] = useState<AssetHit[]>([]);
  const [query, setQuery] = useState(() => slot.beschreibung.split(':').pop()?.trim() ?? slot.id);
  const [notes, setNotes] = useState<string[]>([]);
  const [error, setError] = useState('');
  useEffect(() => {
    backend.assetsHits(slot.id).then(setHits, (e) => setError(errorText(e)));
  }, [slot.id]);
  const search = async () => {
    try {
      const res = await backend.assetsResearch(slot.id, query);
      setHits(res.treffer);
      setNotes([`${res.neu} neue Treffer`, ...res.hinweise]);
      setError('');
    } catch (e) {
      setError(errorText(e));
    }
  };
  /** Import, Download oder Bestätigen; danach Treffer und (bei Import) die Slot-Liste neu laden */
  const act = async (run: () => Promise<unknown>, imported: boolean) => {
    try {
      await run();
      setHits(await backend.assetsHits(slot.id));
      if (imported) onImported();
      setError('');
    } catch (e) {
      setError(errorText(e));
    }
  };
  const drop = async (hit: string) => {
    try {
      await backend.assetsHitDrop(slot.id, hit);
      setHits((list) => list.filter((h) => h.id !== hit));
    } catch (e) {
      setError(errorText(e));
    }
  };
  return (
    <div className="as-research">
      <Flex gap="2" align="center" wrap="wrap">
        <Text size="2" weight="bold">Recherche im Netz</Text>
        <TextField.Root className="as-filter" value={query} onChange={(e) => setQuery(e.target.value)} placeholder="Suchbegriff (englisch findet mehr)" />
        <ActionButton size="1" onClick={search}>Suchen</ActionButton>
      </Flex>
      {notes.length > 0 && <Text size="1" color="gray">{notes.join(' · ')}</Text>}
      {error && <p className="svc-error">{error}</p>}
      {hits.length === 0 ? <Text size="2" color="gray">Noch keine Treffer gespeichert.</Text> : (
        <ul className="as-hits as-research-hits">
          {hits.map((h) => <HitRow key={h.id} hit={h} slot={slot} onDrop={() => drop(h.id)} onAct={act} />)}
        </ul>
      )}
    </div>
  );
}

type Act = (run: () => Promise<unknown>, imported: boolean) => Promise<void>;

function HitRow({ hit, slot, onDrop, onAct }: { hit: AssetHit; slot: AssetSlot; onDrop: () => Promise<void>; onAct: Act }) {
  return (
    <li>
      <HitPreview hit={hit} sound={slot.art === 'sound'} />
      <span className="as-hit-main">
        <Button size="1" variant="ghost" onClick={() => backend.openUrl(hit.link)}>{hit.titel}</Button>
        <span className="as-dim">{hit.quelle}{hit.urheber ? ` · ${hit.urheber}` : ''} · {hit.lizenz || 'Lizenz unbekannt'}{hit.download ? ' · direkter Download' : ''}</span>
      </span>
      <StatusBadge tone={STATUS_TONE[hit.lizenzStatus]}>{STATUS_LABEL[hit.lizenzStatus]}</StatusBadge>
      {hit.kennzeichen && <StatusBadge tone={hit.kennzeichen === 'SA' ? 'warn' : 'error'}>{hit.kennzeichen}</StatusBadge>}
      <HitActions hit={hit} slot={slot} onAct={onAct} />
      <ActionButton size="1" variant="soft" color="gray" onClick={onDrop}>Verwerfen</ActionButton>
    </li>
  );
}

/** Übernehmen nur mit direktem Download und erkannter Lizenz, sonst manuell laden und importieren (B-298/AC-05). */
function HitActions({ hit, slot, onAct }: { hit: AssetHit; slot: AssetSlot; onAct: Act }) {
  if (hit.lizenzStatus === 'ausgeschlossen') return null;
  const direct = !!hit.download && hit.lizenzStatus !== 'pruefen';
  return (
    <>
      {hit.datei && <span className="as-dim">importiert: {hit.datei}</span>}
      {direct && <ActionButton size="1" onClick={() => onAct(() => backend.assetsDownload(slot.id, hit.id), true)}>Übernehmen</ActionButton>}
      <ActionButton size="1" variant="soft" onClick={() => onAct(() => backend.assetsImport(slot.id, hit.id), true)}>Importieren …</ActionButton>
      {hit.lizenzStatus !== 'bestaetigt' && (
        <ActionButton size="1" variant="soft" color="green" onClick={() => onAct(() => backend.assetsLicenseOk(slot.id, hit.id), false)}>Lizenz bestätigen</ActionButton>
      )}
    </>
  );
}

function HitPreview({ hit, sound }: { hit: AssetHit; sound: boolean }) {
  if (!hit.vorschau) return <span className="as-thumb as-dim">Link</span>;
  if (sound || /\.(mp3|ogg)$/i.test(hit.vorschau)) return <audio className="as-hit-audio" controls preload="none" src={hit.vorschau} />;
  return <img className="as-thumb" src={hit.vorschau} alt="" loading="lazy" />;
}
