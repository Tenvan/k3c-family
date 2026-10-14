import { Heading, Text } from '@radix-ui/themes';
import { useState } from 'react';
import type { AssetChoice, AssetSlot } from '../api';
import { addChoice, fileUrl, move, removeChoice, sceneUrl, withCut } from './assets';
import { CatalogPicker } from './CatalogPicker';
import { CutEditor } from './CutEditor';
import { ResearchPanel } from './ResearchPanel';

interface Props {
  slot: AssetSlot;
  /** Adresse des Vite-Dienstes; leer = keine Bilder, keine Mini-Szene */
  preview: string;
  onSave: (id: string, wahl: AssetChoice[]) => Promise<boolean>;
}

/** Detail eines Asset-Slots: Zuordnungen nach Präferenz (Ziehen), Ausschnitt, Katalog-Suche, Mini-Szene und Recherche. */
export function SlotDetail({ slot, preview, onSave }: Props) {
  const [active, setActive] = useState(0);
  const choice = slot.wahl[active];
  const save = (wahl: AssetChoice[]) => onSave(slot.id, wahl);
  return (
    <div className="as-detail-body">
      <Heading size="4">{slot.id} <Text size="2" color="gray">{slot.beschreibung} · {slot.art}/{slot.kategorie}{slot.bus ? ` · Bus ${slot.bus}` : ''}</Text></Heading>
      <Text size="2" color="gray">Präferenz 1 ist im Spiel aktiv, die weiteren sind Reserve und Vergleich. Reihenfolge per Ziehen.</Text>
      <ChoiceList slot={slot} active={active} onActive={setActive}
        onMove={(from, to) => void save(move(slot.wahl, from, to)).then((ok) => ok && setActive(to))}
        onRemove={(i) => void save(removeChoice(slot.wahl, i)).then(() => setActive(0))} />
      {choice && slot.art === 'grafik' && (
        <CutEditor key={`${active}:${choice.datei}`} choice={choice} preview={preview}
          onCut={(cut) => save(withCut(slot.wahl, active, cut))} />
      )}
      {choice && slot.art === 'sound' && <SoundProbe choice={choice} preview={preview} />}
      <CatalogPicker art={slot.art} onAdd={(datei) => save(addChoice(slot.wahl, datei))} />
      <ScenePreview slot={slot} pref={active + 1} preview={preview} />
      <ResearchPanel slot={slot} />
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
          <button type="button" className="as-choice-main" onClick={() => onActive(i)} aria-label={`Präferenz ${i + 1} wählen`}>
            <b>{i + 1}.</b> {c.datei}{c.sprite ? `#${c.sprite}` : ''}{c.sheet ? ` · Sheet ${c.sheet}` : ''}
            {c.ausschnitt && <span className="as-dim"> · Ausschnitt {c.ausschnitt.x},{c.ausschnitt.y},{c.ausschnitt.w},{c.ausschnitt.h}</span>}
          </button>
          <button type="button" className="as-icon" onClick={() => i > 0 && onMove(i, i - 1)} disabled={i === 0} aria-label="nach oben">↑</button>
          <button type="button" className="as-icon" onClick={() => onRemove(i)} aria-label="entfernen">✕</button>
        </li>
      ))}
    </ol>
  );
}

function SoundProbe({ choice, preview }: { choice: AssetChoice; preview: string }) {
  if (!preview) return <Text size="2" color="gray">Hörprobe braucht den Vite-Dienst.</Text>;
  if (choice.sprite) return <Text size="2" color="gray">Ton „{choice.sprite}“ aus dem Audio-Atlas: in der Mini-Szene abspielen.</Text>;
  return <audio className="as-audio" controls src={fileUrl(preview, choice.datei)} />;
}

function ScenePreview({ slot, pref, preview }: { slot: AssetSlot; pref: number; preview: string }) {
  const url = sceneUrl(preview, slot, pref);
  if (!url) return <Text size="2" color="gray">Mini-Szene braucht den Vite-Dienst (oben „Vite starten“).</Text>;
  return (
    <div className="as-scene">
      <Text size="2" color="gray">Mini-Szene „{slot.kategorie}“ mit Präferenz {pref} für {slot.id}</Text>
      <iframe title={`Mini-Szene ${slot.kategorie}`} src={url} />
    </div>
  );
}
