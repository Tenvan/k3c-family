import { Button, Flex, Heading, IconButton, Splitter, Text } from '@webkit/atoms';
import { useSplit } from '@webkit/lib/split';
import { useState, type CSSProperties } from 'react';
import type { AssetChoice, AssetSlot } from '../api';
import { move, removeChoice, withCut } from '../assets/assets';
import { CutEditor } from '../assets/CutEditor';
import { MiniScene } from '../assets/MiniScene';
import * as prefs from '../lib/prefs';
import '../styles/assetInspector.css';
import { playOn } from './transport';

interface Props {
  slot: AssetSlot;
  onSave: (id: string, wahl: AssetChoice[]) => Promise<boolean>;
}

/** Reiter Belegung (B-677): Präferenzen mit Ziehen, ↑ und ✕, bei Sound je Zeile ▶; bei Grafik Ausschnitt und darunter
 *  (Splitter) die Mini-Szene. Setze `key={slot.id}`, damit die gewählte Präferenz nicht in den nächsten Slot wandert. */
export function Belegung({ slot, onSave }: Props) {
  const [chosen, setActive] = useState(0);
  const active = Math.min(chosen, Math.max(0, slot.wahl.length - 1));
  const choice = slot.wahl[active];
  const [split, setSplit, resetSplit] = useSplit(prefs, `${slot.art}.scene`, 40);
  const save = (wahl: AssetChoice[]) => onSave(slot.id, wahl);
  const scene = slot.art === 'grafik';
  return (
    <div className="ap-belegung" style={{ '--ap-scene': `${split}%` } as CSSProperties}>
      <div className="ap-belegung-body">
        <Heading size="4">{slot.id} <Text size="2" color="gray">{slot.beschreibung} · {slot.art}/{slot.kategorie}{slot.bus ? ` · Bus ${slot.bus}` : ''}</Text></Heading>
        <Text size="2" color="gray">Präferenz 1 ist im Spiel aktiv, die weiteren sind Reserve und Vergleich. Reihenfolge per Ziehen.</Text>
        <ChoiceList slot={slot} active={active} onActive={setActive}
          onMove={(from, to) => void save(move(slot.wahl, from, to)).then((ok) => ok && setActive(to))}
          onRemove={(i) => void save(removeChoice(slot.wahl, i)).then(() => setActive(0))} />
        {choice && scene && <CutEditor key={`${active}:${choice.datei}`} choice={choice} onCut={(cut) => save(withCut(slot.wahl, active, cut))} />}
      </div>
      {scene && <Splitter horizontal value={split} onChange={setSplit} onReset={resetSplit} />}
      {scene && <div className="ap-scene-pane"><MiniScene wahl={slot.wahl} active={active} /></div>}
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
  if (slot.wahl.length === 0) return <p className="ap-gap">Lücke: noch keine Zuordnung. Im Reiter Kandidaten eine Datei zuordnen.</p>;
  const sound = slot.art === 'sound';
  return (
    <ol className="ap-choices">
      {slot.wahl.map((c, i) => (
        <li key={`${i}:${c.datei}`} draggable className={`ap-choice${i === active ? ' ap-choice-on' : ''}${drag === i ? ' ap-drag' : ''}`}
          onDragStart={(e) => { setDrag(i); e.dataTransfer.effectAllowed = 'move'; }}
          onDragOver={(e) => e.preventDefault()}
          onDrop={(e) => { e.preventDefault(); if (drag !== null) onMove(drag, i); setDrag(null); }}
          onDragEnd={() => setDrag(null)}>
          <span className="ap-grip" aria-hidden>⠿</span>
          {sound && <PlayButtons slot={slot} choice={c} />}
          <Button variant="ghost" color="gray" highContrast className="ap-choice-main" onClick={() => onActive(i)} aria-label={`Präferenz ${i + 1} wählen`}>
            <b>{i + 1}.</b> {c.datei}{c.sprite ? `#${c.sprite}` : ''}{c.sheet ? ` · Sheet ${c.sheet}` : ''}
            {c.ausschnitt && <span className="ap-dim"> · Ausschnitt {c.ausschnitt.x},{c.ausschnitt.y},{c.ausschnitt.w},{c.ausschnitt.h}</span>}
          </Button>
          <IconButton size="1" variant="ghost" color="gray" onClick={() => i > 0 && onMove(i, i - 1)} disabled={i === 0} aria-label="nach oben">↑</IconButton>
          <IconButton size="1" variant="ghost" color="gray" onClick={() => onRemove(i)} aria-label="entfernen">✕</IconButton>
        </li>
      ))}
    </ol>
  );
}

/** ▶ spielt die Präferenz wie im Spiel; ein Atlas-Ton hat zusätzlich ▶ „fern“ (gedämpft wie in der anderen Bildhälfte). */
function PlayButtons({ slot, choice }: { slot: AssetSlot; choice: AssetChoice }) {
  return (
    <Flex gap="1" align="center">
      <IconButton size="1" variant="soft" color="gray" aria-label="Abspielen" onClick={() => void playOn(slot, choice)}>▶</IconButton>
      {choice.sprite && <IconButton size="1" variant="soft" color="gray" aria-label="Abspielen fern" onClick={() => void playOn(slot, choice, true)}>▶ fern</IconButton>}
    </Flex>
  );
}
