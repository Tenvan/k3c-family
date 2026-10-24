import { ActionButton, StatusBadge, Text } from '@webkit/atoms';
import { useCallback, useEffect, useState } from 'react';
import { backend, type AssetKind, type AssetSuggestion } from '../api';
import { SuggestionCard } from './SuggestionCard';

/** Seite des Studios → Art der Vorschläge; andere Seiten haben keine Vorschlagsliste. */
export function suggestionKind(page: string | undefined): AssetKind | undefined {
  return page === 'sound' ? 'sound' : page === 'grafik' ? 'grafik' : undefined;
}

/** Vorschlagskarten des Chats (B-600, WZ30.5) für die Seite Sound oder Grafik: mehrere in einer Liste, einzeln übernehmbar
 *  oder verwerfbar. Offline zeigt der Bereich „offline“, die Seite bleibt nutzbar. */
export function SuggestionList({ kind }: { kind: AssetKind }) {
  const [items, setItems] = useState<AssetSuggestion[]>([]);
  const [offline, setOffline] = useState(false);
  const [note, setNote] = useState('');
  const load = useCallback(async () => {
    try {
      const view = await backend.assetSuggestions('');
      setOffline(view.offline);
      setItems(view.vorschlaege.filter((s) => s.art === kind));
      setNote('');
    } catch (e) {
      setOffline(false);
      setItems([]);
      setNote(e instanceof Error ? e.message : String(e));
    }
  }, [kind]);
  useEffect(() => { void load(); }, [load]);

  const done = (s: AssetSuggestion) => setItems((list) => list.filter((x) => x.id !== s.id));
  const adopt = async (s: AssetSuggestion) => {
    try {
      await backend.assetSuggestionAdopt(s.slot, s.id);
      done(s);
      setNote(`Übernommen: ${s.slot} ← ${s.datei}`);
    } catch (e) {
      setNote(e instanceof Error ? e.message : String(e));
    }
  };
  const drop = async (s: AssetSuggestion) => {
    try {
      await backend.assetSuggestionDrop(s.slot, s.id);
      done(s);
      setNote(`Verworfen: ${s.titel}`);
    } catch (e) {
      setNote(e instanceof Error ? e.message : String(e));
    }
  };

  return (
    <section className="chat-sugs" aria-label="Vorschläge" data-testid="suggestions">
      <div className="chat-head-row">
        <span className="chat-title">Vorschläge</span>
        {offline && <StatusBadge tone="warn">offline</StatusBadge>}
        <ActionButton size="1" variant="soft" onClick={load}>Aktualisieren</ActionButton>
      </div>
      {offline && <Text size="1" color="gray">Chat und Internet sind nicht erreichbar; die Seite bleibt nutzbar.</Text>}
      {!offline && items.length === 0 && !note && <Text size="1" color="gray">Keine Vorschläge.</Text>}
      {note && <div className="chat-note" role="status">{note}</div>}
      <ul className="chat-sug-list">
        {items.map((s) => <SuggestionCard key={s.id} suggestion={s} onAdopt={adopt} onDrop={drop} />)}
      </ul>
    </section>
  );
}
