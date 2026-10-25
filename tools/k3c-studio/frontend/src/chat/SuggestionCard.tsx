import { ActionButton, Button, Flex, StatusBadge, Text } from '@webkit/atoms';
import { useState } from 'react';
import type { AssetSuggestion } from '../api';
import { playChoice, stopAll } from '../sound/audio';

/** Größe in Bytes für die Karte („1,4 MB“, „86 kB“). */
export function formatSize(bytes: number): string {
  if (bytes >= 1_000_000) return `${(bytes / 1_000_000).toFixed(1).replace('.', ',')} MB`;
  return `${Math.max(1, Math.round(bytes / 1000))} kB`;
}

/** Vorschau: Bild bei Grafik, Hörprobe (wie die Hörprobe am Slot) bei Sound. */
function Preview({ s }: { s: AssetSuggestion }) {
  const [status, setStatus] = useState('');
  if (!s.vorschau) return <Text size="1" color="gray">Vorschau nach dem Laden auf der Seite Quellen</Text>;
  if (s.art === 'grafik') return <img className="chat-sug-img" src={s.vorschau} alt={`Vorschau ${s.titel}`} />;
  const play = async () => setStatus((await playChoice({ bus: s.bus }, { datei: s.vorschau, sprite: s.sprite })) ? 'Spielt' : 'Nicht abspielbar');
  return (
    <Flex gap="2" align="center" wrap="wrap">
      <ActionButton size="1" variant="soft" onClick={play}>Hörprobe</ActionButton>
      <Button size="1" variant="soft" color="gray" onClick={() => { stopAll(); setStatus(''); }}>Stopp</Button>
      {status && <Text size="1" color="gray">{status}</Text>}
    </Flex>
  );
}

export interface SuggestionCardProps {
  suggestion: AssetSuggestion;
  onAdopt: (s: AssetSuggestion) => Promise<void>;
  onDrop: (s: AssetSuggestion) => Promise<void>;
}

/** Grund, warum „Übernehmen“ gesperrt ist (leer = frei): Lizenzverstoß (Q70) oder Datei nicht im Vorrat (Download nur
 *  auf der Seite Quellen nach Bestätigung, B-601). */
export function adoptBlock(s: AssetSuggestion): string {
  if (!s.lizenzErlaubt) return s.lizenzGrund ?? 'Lizenz nicht erlaubt';
  if (s.status !== 'geladen') return 'Noch nicht im Vorrat: erst auf der Seite Quellen laden (mit Bestätigung)';
  return '';
}

/** Karte eines Vorschlags: Slot, Vorschau/Hörprobe, Quelle, Urheber, Lizenz, Größe; „Übernehmen“ ist bei Lizenzverstoß
 *  oder fehlender Datei gesperrt und nennt den Grund. */
export function SuggestionCard({ suggestion: s, onAdopt, onDrop }: SuggestionCardProps) {
  const block = adoptBlock(s);
  return (
    <li className="chat-sug" data-testid="suggestion" data-slot={s.slot} data-allowed={s.lizenzErlaubt}>
      <Flex gap="2" align="center" justify="between" wrap="wrap">
        <Text size="2" weight="bold">{s.titel}</Text>
        <StatusBadge tone={s.lizenzErlaubt ? 'ok' : 'error'}>{s.lizenz}</StatusBadge>
      </Flex>
      <Text size="1" color="gray">Slot {s.slot} · {s.datei}</Text>
      <Preview s={s} />
      <Text size="1" color="gray">{s.quelle}{s.urheber ? ` · ${s.urheber}` : ''} · {formatSize(s.groesse)}</Text>
      {block && <div className="chat-note chat-sug-block" role="status">Gesperrt: {block}</div>}
      <Flex gap="2">
        <ActionButton size="1" disabled={block !== ''} title={block || undefined} onClick={() => onAdopt(s)}>Übernehmen</ActionButton>
        <ActionButton size="1" variant="soft" color="gray" onClick={() => onDrop(s)}>Verwerfen</ActionButton>
      </Flex>
    </li>
  );
}
