import { ActionButton, StatusBadge, Text } from '@webkit/atoms';
import { useCallback, useEffect, useState } from 'react';
import { backend, type AssetKind, type AssetSuggestion } from '../api';
import { useAssign } from './chatBridge';
import { SuggestionCard } from './SuggestionCard';

/** Seite des Studios → Art der Vorschläge; andere Seiten haben keine Vorschlagsliste. */
export function suggestionKind(page: string | undefined): AssetKind | undefined {
  return page === 'sound' ? 'sound' : page === 'grafik' ? 'grafik' : undefined;
}

/** Vorschlagskarten des Chats (B-600, WZ30.5) für die Seite Sound oder Grafik: mehrere in einer Liste, einzeln übernehmbar
 *  oder verwerfbar. Mit gewähltem Slot (Seitenkontext) die Vorschläge des Slots, sonst die des Chats; `round` lädt nach
 *  jeder Antwort neu. Offline zeigt der Bereich „offline“, die Seite bleibt nutzbar. */
export function SuggestionList({ kind, round = 0 }: { kind: AssetKind; round?: number }) {
  const slot = useAssign()?.slot ?? '';
  const [items, setItems] = useState<AssetSuggestion[]>([]);
  const [offline, setOffline] = useState(false);
  const [stand, setStand] = useState('');
  const [note, setNote] = useState('');
  const load = useCallback(async () => {
    try {
      const view = await backend.assetSuggestions(slot);
      setOffline(view.offline);
      setStand(view.stand);
      setItems(view.vorschlaege.filter((s) => s.art === kind));
      setNote('');
    } catch (e) {
      setOffline(false);
      setItems([]);
      setNote(e instanceof Error ? e.message : String(e));
    }
  }, [kind, slot]);
  useEffect(() => { void load(); }, [load, round]);

  const done = (s: AssetSuggestion) => setItems((list) => list.filter((x) => x.id !== s.id));
  const adopt = async (s: AssetSuggestion) => {
    try {
      setStand(await backend.assetSuggestionAdopt(s.slot, s.id, stand));
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
