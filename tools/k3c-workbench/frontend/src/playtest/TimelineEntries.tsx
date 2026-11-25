import { useState } from 'react';
import { createPortal } from 'react-dom';
import { ActionButton, CardButton, Checkbox, Flex, StatusBadge, Text } from '@webkit/atoms';
import { frameUrl } from './api';
import { clock } from './RecordBar';
import type { PlaytestEntry, PlaytestTimeline } from './types';

/** Vorschau eines Marker-Bilds: klein in der Zeile, beim Daraufzeigen groß. Die große Ansicht hängt am Seitenende (Portal,
 *  fixed), damit sie vor allen Zeilen und Karten liegt; sie sitzt unter dem Bild, am unteren Rand darüber. */
function Thumb({ src, alt, dim }: { src: string; alt: string; dim: boolean }) {
  const [at, setAt] = useState<{ left: number; top: number } | null>(null);
  const show = (el: HTMLElement) => {
    const r = el.getBoundingClientRect();
    const h = Math.min(560, window.innerWidth * 0.8) * 0.5625 + 8; // etwa 16:9
    const below = r.bottom + h < window.innerHeight;
    setAt({ left: Math.max(8, Math.min(r.left, window.innerWidth - 568)), top: below ? r.bottom + 4 : Math.max(8, r.top - h) });
  };
  return (
    <span style={{ display: 'inline-block', lineHeight: 0 }} onMouseEnter={(ev) => show(ev.currentTarget)} onMouseLeave={() => setAt(null)}>
      <img src={src} alt={alt} loading="lazy" style={{ height: 44, borderRadius: 3, opacity: dim ? 0.3 : 1 }} />
      {at && createPortal(
        <img src={src} alt="" data-preview style={{ position: 'fixed', left: at.left, top: at.top, width: 560, maxWidth: '80vw', zIndex: 10000,
          borderRadius: 6, boxShadow: '0 8px 24px rgba(0,0,0,0.5)', pointerEvents: 'none' }} />,
        document.body,
      )}
    </span>
  );
}

interface EntryProps {
  e: PlaytestEntry;
  /** Bild-URL des Markers, sonst null. */
  img: string | null;
  onSeek: (ms: number) => void;
  onSkipFrame: (nr: number, skip: boolean) => void;
  onDescribe: (nr: number) => void;
  describeBusy: boolean;
  describeReady: boolean;
}

/** Zeit der Zeile; bei einer Zeile mit Bild ein eigener Klick, der im Player springt (ohne den Haken umzuschalten). */
function EntryTime({ e, pick, onSeek }: { e: PlaytestEntry; pick: boolean; onSeek: (ms: number) => void }) {
  if (!pick) return <Text size="2" color="gray" style={{ minWidth: 56, fontVariantNumeric: 'tabular-nums' }}>{clock(e.videoMs)}</Text>;
  return (
    <Text size="2" color="blue" data-seek style={{ minWidth: 56, fontVariantNumeric: 'tabular-nums', textDecoration: 'underline' }}
      title="Im Player hierhin springen" onClick={(ev) => { ev.stopPropagation(); onSeek(e.videoMs); }}>{clock(e.videoMs)}</Text>
  );
}

/** Text oder Marker-Marke der Zeile mit Bildvorschau und Beschreibung. */
function EntryText({ e, img }: { e: PlaytestEntry; img: string | null }) {
  return (
    <Flex direction="column" gap="1" align="start">
      <Flex gap="2" align="center" wrap="wrap">
        {e.kind === 'marker'
          ? <StatusBadge tone="warn">{e.text}</StatusBadge>
          : <Text size="2">{e.text}</Text>}
        {img && <Thumb src={img} alt={`Bild zu Marker ${e.nr}`} dim={!!e.skip} />}
      </Flex>
      {e.desc && <Text size="1" color="gray" data-desc style={{ whiteSpace: 'normal', textAlign: 'left' }}>{e.desc}</Text>}
    </Flex>
  );
}

/** Haken „Bild verwenden“ und Einzel-Knopf „Beschreiben“ einer Zeile mit Bild. */
function EntryActions({ e, onSkipFrame, onDescribe, describeBusy, describeReady }: Omit<EntryProps, 'img' | 'onSeek'>) {
  return (
    <>
      <Checkbox size="1" checked={!e.skip} aria-label={`Bild von Marker ${e.nr} verwenden`}
        title="Markierte Bilder kommen in den Prompt und werden beschrieben; Haken weg: Bild ausblenden (spart Tokens)"
        onCheckedChange={(v) => onSkipFrame(e.nr!, v !== true)} />
      {!e.skip && (
        <ActionButton size="1" variant="ghost" disabled={describeBusy || !describeReady} data-describe-one
          title="Dieses Bild mit dem Bildmodell beschreiben" onClick={() => onDescribe(e.nr!)}>Beschreiben</ActionButton>
      )}
    </>
  );
}

function EntryRow(p: EntryProps) {
  const { e, img, onSeek, onSkipFrame } = p;
  const pick = !!img && !!e.nr; // Zeile mit Bild: Klick schaltet den Haken, die Zeit springt im Player
  return (
    <Flex role="listitem" align="center" gap="1">
      <CardButton data-entry={e.kind} variant="ghost" style={{ flex: 1, margin: 0 }}
        onClick={() => (pick ? onSkipFrame(e.nr!, !e.skip) : onSeek(e.videoMs))}
        title={pick ? 'Klick wählt das Bild ab oder an' : 'Im Player hierhin springen'}>
        <Flex gap="3" align="center">
          <EntryTime e={e} pick={pick} onSeek={onSeek} />
          <EntryText e={e} img={img} />
        </Flex>
      </CardButton>
      {pick && <EntryActions e={e} onSkipFrame={onSkipFrame} onDescribe={p.onDescribe} describeBusy={p.describeBusy} describeReady={p.describeReady} />}
    </Flex>
  );
}

interface EntriesProps {
  id: string;
  timeline: PlaytestTimeline | null;
  onSeek: (ms: number) => void;
  onSkipFrame: (nr: number, skip: boolean) => void;
  onDescribe: (nr: number) => void;
  describeBusy: boolean;
  describeReady: boolean;
}

export function Entries({ id, timeline, ...rest }: EntriesProps) {
  if (!timeline) return null;
  if (timeline.entries.length === 0) return <Text size="2" color="gray">Noch kein Transkript und keine Marker.</Text>;
  return (
    <Flex direction="column" role="list" aria-label="Zeitleiste">
      {!timeline.hasTranscript && <Text size="2" color="gray">Noch kein Transkript, nur Marker.</Text>}
      {timeline.entries.map((e) => (
        <EntryRow key={`${e.kind}-${e.ms}-${e.nr ?? 0}`} e={e} img={e.frame && e.nr ? frameUrl(id, e.nr) : null} {...rest} />
      ))}
    </Flex>
  );
}
