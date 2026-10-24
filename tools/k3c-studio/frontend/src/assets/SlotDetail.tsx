import { Button, Heading, IconButton, Splitter, Text } from '@webkit/atoms';
import { useSplit } from '@webkit/lib/split';
import { useState, type CSSProperties } from 'react';
import type { AssetChoice, AssetSlot } from '../api';
import * as prefs from '../lib/prefs';
import { SoundProbe } from '../sound/SoundProbe';
import { addChoice, move, removeChoice, withCut } from './assets';
import { CatalogPicker } from './CatalogPicker';
import { CutEditor } from './CutEditor';
import { MiniScene } from './MiniScene';
import { ResearchPanel } from './ResearchPanel';
import { SlotInfo } from './SlotInfo';

interface Props {
  slot: AssetSlot;
  onSave: (id: string, wahl: AssetChoice[]) => Promise<boolean>;
  /** Slot-Liste neu laden (nach einem Import) */
  onReload: () => void;
  /** alle im Register zugeordneten Dateien (Katalog blendet sie aus) */
  used: ReadonlySet<string>;
}

/** Detail eines Asset-Slots: Zuordnungen nach Präferenz (Ziehen), Ausschnitt, Katalog-Suche und Recherche; darunter (Splitter) die Mini-Szene. */
export function SlotDetail({ slot, onSave, onReload, used }: Props) {
  const [active, setActive] = useState(0);
  const choice = slot.wahl[active];
  const [split, setSplit, resetSplit] = useSplit(prefs, `${slot.art}.scene`, 40);
  const save = (wahl: AssetChoice[]) => onSave(slot.id, wahl);
  const scene = slot.art === 'grafik';
  return (
    <div className="as-detail-split" style={{ '--as-scene': `${split}%` } as CSSProperties}>
    <div className="as-detail-body">
      <Heading size="4">{slot.id} <Text size="2" color="gray">{slot.beschreibung} · {slot.art}/{slot.kategorie}{slot.bus ? ` · Bus ${slot.bus}` : ''}</Text></Heading>
      <SlotInfo slot={slot} />
      <Text size="2" color="gray">Präferenz 1 ist im Spiel aktiv, die weiteren sind Reserve und Vergleich. Reihenfolge per Ziehen.</Text>
      <ChoiceList slot={slot} active={active} onActive={setActive}
        onMove={(from, to) => void save(move(slot.wahl, from, to)).then((ok) => ok && setActive(to))}
        onRemove={(i) => void save(removeChoice(slot.wahl, i)).then(() => setActive(0))} />
      {choice && slot.art === 'grafik' && (
        <CutEditor key={`${active}:${choice.datei}`} choice={choice}
          onCut={(cut) => save(withCut(slot.wahl, active, cut))} />
      )}
      {choice && slot.art === 'sound' && <SoundProbe key={`${active}:${choice.datei}`} slot={slot} choice={choice} />}
      <CatalogPicker art={slot.art} used={used} onAdd={(datei) => save(addChoice(slot.wahl, datei))} />
      <ResearchPanel slot={slot} onImported={onReload} />
    </div>
    {scene && <Splitter horizontal value={split} onChange={setSplit} onReset={resetSplit} />}
    {scene && <div className="as-scene-pane"><MiniScene wahl={slot.wahl} active={active} /></div>}
    </div>
  );
}

interface ListProps {
  slot: AssetSlot;
  active: number;
  onActive: (i: number) => void;
  onMove: (from: number, to: number) => void;
  onRemove: (i: number) => void;
}

function ChoiceList({ slot, active, onActive, onMove, onRemove }: ListProps) {
  const [drag, setDrag] = useState<number | null>(null);
  if (slot.wahl.length === 0) return <p className="as-gap">Lücke: noch keine Zuordnung. Unten im Katalog suchen und hinzufügen.</p>;
  return (
    <ol className="as-choices">
      {slot.wahl.map((c, i) => (
        <li key={`${i}:${c.datei}`} draggable className={`as-choice${i === active ? ' as-choice-on' : ''}${drag === i ? ' as-drag' : ''}`}
          onDragStart={(e) => { setDrag(i); e.dataTransfer.effectAllowed = 'move'; }}
          onDragOver={(e) => e.preventDefault()}
          onDrop={(e) => { e.preventDefault(); if (drag !== null) onMove(drag, i); setDrag(null); }}
          onDragEnd={() => setDrag(null)}>
          <span className="as-grip" aria-hidden>⠿</span>
          <Button variant="ghost" color="gray" highContrast className="as-choice-main" onClick={() => onActive(i)} aria-label={`Präferenz ${i + 1} wählen`}>
            <b>{i + 1}.</b> {c.datei}{c.sprite ? `#${c.sprite}` : ''}{c.sheet ? ` · Sheet ${c.sheet}` : ''}
            {c.ausschnitt && <span className="as-dim"> · Ausschnitt {c.ausschnitt.x},{c.ausschnitt.y},{c.ausschnitt.w},{c.ausschnitt.h}</span>}
          </Button>
          <IconButton size="1" variant="ghost" color="gray" onClick={() => i > 0 && onMove(i, i - 1)} disabled={i === 0} aria-label="nach oben">↑</IconButton>
          <IconButton size="1" variant="ghost" color="gray" onClick={() => onRemove(i)} aria-label="entfernen">✕</IconButton>
        </li>
      ))}
    </ol>
  );
}
