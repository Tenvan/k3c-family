import { BIOMES } from '@game/model/biome';
import { Button, Text } from '@radix-ui/themes';
import { useState } from 'react';
import { ConvertButton } from './ConvertButton';
import { KindPicker } from './KindPicker';
import { CHUNK_KINDS, OBJECT_KINDS, addChunk, addObject, moveObject, removeChunk, removeObject, slotLabel, updateChunk } from './plan';
import { SizePicker } from './SizePicker';
import type { PlanState } from './usePlan';

const biomeName = (id: string) => BIOMES.find((b) => b.id === id)?.name ?? id;
const defaultKind = (biome = ''): string => Object.keys(BIOMES.find((b) => b.id === biome)?.chunkWeights ?? {})[0] ?? 'forest';
/** Klick auf ein Bedienelement im Eintrag (Feld, Knopf, Liste): kein Sprung */
const isControl = (target: EventTarget) => target instanceof Element && target.closest('input, button, select, textarea, [role="listbox"]') !== null;
const number = (value: string, fallback: number) => (value === '' || Number.isNaN(Number(value)) ? fallback : Number(value));

/** Editor für geplante Chunks im Welt-Betrachter (B-461/AC-09): Insel und Stufe wählen, Chunks am Platz anlegen, Art,
 *  Chunk-Seed und feste Objekte bearbeiten, die Inselgröße der Stufe wählen und einen Seed-Chunk mit dem Seed `seed` des
 *  Levels in feste Objekte umwandeln (B-485/AC-04, AC-06); ein Klick auf einen Eintrag (nicht auf ein Feld) und jede Änderung eines Chunks (nicht sein Löschen) melden seinen
 *  Platz an `onSlot`, bei neuem Platz den neuen (die Vorschauen zentrieren ihn, WZ17.5); Art und Objekt in der durchsuchbaren Liste mit Emoji (B-485/AC-01). Prüffehler stehen am Platz (AC-06); Speichern nur, wenn der Entwurf gültig ist. */
export function PlanEditor({ plan, seed, onSlot }: { plan: PlanState; seed: string; onSlot?: (slot: number) => void }) {
  const { island, stage, draft, check, notice, dirty } = plan;
  const [slot, setSlot] = useState(1);
  if (!island) return <section className="wd-plan" aria-label="Planung"><Text size="2">{notice?.text ?? 'Lade Planung …'}</Text></section>;
  const lock = dirty ? 'Erst speichern oder verwerfen' : undefined;
  const orphan = !check.ok && !draft.some((c) => c.slot === check.slot);
  /** Entwurf ändern und den geänderten Chunk melden */
  const change = (next: typeof draft, at: number) => { plan.edit(next); onSlot?.(at); };
  return (
    <section className="wd-plan" aria-label="Planung">
      <div className="wd-bar">
        <select className="wd-select" aria-label="Insel" value={island.id} disabled={dirty} title={lock} onChange={(e) => plan.selectIsland(e.target.value)}>
          {plan.view?.islands.map((i) => <option key={i.id} value={i.id}>{i.name}</option>)}
        </select>
        <select className="wd-select" aria-label="Stufe" value={stage} disabled={dirty} title={lock} onChange={(e) => plan.selectStage(Number(e.target.value))}>
          {island.depths.map((d, i) => <option key={d} value={i}>{`Stufe ${i + 1} · ${biomeName(island.biomes[i] ?? '')}`}</option>)}
        </select>
        <SizePicker plan={plan} />
        <input className="wd-slot" type="number" aria-label="Neuer Platz" value={slot} onChange={(e) => setSlot(number(e.target.value, slot))} />
        <Button variant="soft" onClick={() => change(addChunk(draft, slot, defaultKind(island.biomes[stage])), slot)}>Chunk hinzufügen</Button>
        <span className="wd-grow" />
        <Button disabled={!dirty || !check.ok} onClick={() => void plan.save()}>Speichern</Button>
        <Button variant="outline" disabled={!dirty} onClick={plan.discard}>Verwerfen</Button>
        <Button variant="soft" disabled={dirty} title={lock} onClick={() => void plan.reload()}>Neu laden</Button>
      </div>
      {notice && <Text as="p" size="2" role="status" color={notice.kind === 'hint' ? 'green' : 'red'} className="wd-notice">{notice.text}{notice.kind === 'stale' ? ' (Entwurf bleibt)' : ''}</Text>}
      {orphan && <Text as="p" size="2" color="red" role="alert" className="wd-error">{check.error}</Text>}
      <ul className="wd-chunklist" aria-label="Geplante Chunks">
        {draft.length === 0 && <li><Text size="1" color="gray">Keine Planung: Die Stufe bleibt rein zufällig.</Text></li>}
        {draft.map((c, i) => (
          <li key={i} className="wd-planned" data-slot={c.slot} onClick={(e) => { if (!isControl(e.target)) onSlot?.(c.slot); }}>
            <div className="wd-bar">
              <input className="wd-slot" type="number" aria-label={`Platz ${slotLabel(c.slot)}`} value={c.slot} onChange={(e) => { const to = number(e.target.value, c.slot); change(updateChunk(draft, i, { slot: to }), to); }} />
              <KindPicker label={`Art Platz ${slotLabel(c.slot)}`} value={c.kind} options={CHUNK_KINDS} onChange={(kind) => change(updateChunk(draft, i, { kind }), c.slot)} />
              <input className="wd-seed" aria-label={`Chunk-Seed Platz ${slotLabel(c.slot)}`} placeholder="Chunk-Seed" value={c.seed ?? ''} onChange={(e) => change(updateChunk(draft, i, { seed: e.target.value }), c.slot)} />
              <Button variant="soft" onClick={() => change(addObject(draft, i, { kind: OBJECT_KINDS[0] ?? 'tree', at: 10 }), c.slot)}>Objekt</Button>
              <ConvertButton plan={plan} seed={seed} index={i} onDone={onSlot} />
              <Button variant="outline" color="red" aria-label={`Chunk Platz ${slotLabel(c.slot)} löschen`} onClick={() => plan.edit(removeChunk(draft, i))}>Löschen</Button>
            </div>
            {(c.objects ?? []).map((o, j) => (
              <div key={j} className="wd-object">
                <KindPicker label={`Objekt ${j + 1} Platz ${slotLabel(c.slot)}`} value={o.kind} options={OBJECT_KINDS}
                  onChange={(kind) => change(draft.map((x, k) => (k === i ? { ...x, objects: (x.objects ?? []).map((y, m) => (m === j ? { ...y, kind } : y)) } : x)), c.slot)} />
                <input className="wd-slot" type="number" aria-label={`Lage Objekt ${j + 1} Platz ${slotLabel(c.slot)}`} value={o.at} onChange={(e) => change(moveObject(draft, i, j, number(e.target.value, o.at)), c.slot)} />
                <Button variant="outline" color="red" aria-label={`Objekt ${j + 1} Platz ${slotLabel(c.slot)} löschen`} onClick={() => change(removeObject(draft, i, j), c.slot)}>×</Button>
              </div>
            ))}
            {!check.ok && check.slot === c.slot && <Text as="p" size="1" color="red" role="alert" className="wd-error" data-slot={c.slot}>{check.error}</Text>}
          </li>
        ))}
      </ul>
    </section>
  );
}
