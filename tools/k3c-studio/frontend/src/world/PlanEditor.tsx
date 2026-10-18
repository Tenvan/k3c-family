import { BIOMES } from '@game/model/biome';
import { Button, Text } from '@radix-ui/themes';
import { useState } from 'react';
import { CHUNK_KINDS, OBJECT_KINDS, addChunk, addObject, moveObject, removeChunk, removeObject, slotLabel, updateChunk } from './plan';
import type { PlanState } from './usePlan';

const biomeName = (id: string) => BIOMES.find((b) => b.id === id)?.name ?? id;
const defaultKind = (biome = ''): string => Object.keys(BIOMES.find((b) => b.id === biome)?.chunkWeights ?? {})[0] ?? 'forest';
const number = (value: string, fallback: number) => (value === '' || Number.isNaN(Number(value)) ? fallback : Number(value));

/** Editor für geplante Chunks im Welt-Betrachter (B-461/AC-09): Insel und Stufe wählen, Chunks am Platz anlegen, Art,
 *  Chunk-Seed und feste Objekte bearbeiten. Prüffehler stehen am Platz (AC-06); Speichern nur, wenn der Entwurf gültig ist. */
export function PlanEditor({ plan }: { plan: PlanState }) {
  const { island, stage, draft, check, notice, dirty } = plan;
  const [slot, setSlot] = useState(1);
  if (!island) return <section className="wd-plan" aria-label="Planung"><Text size="2">{notice?.text ?? 'Lade Planung …'}</Text></section>;
  const lock = dirty ? 'Erst speichern oder verwerfen' : undefined;
  const orphan = !check.ok && !draft.some((c) => c.slot === check.slot);
  return (
    <section className="wd-plan" aria-label="Planung">
      <div className="wd-bar">
        <select className="wd-select" aria-label="Insel" value={island.id} disabled={dirty} title={lock} onChange={(e) => plan.selectIsland(e.target.value)}>
          {plan.view?.islands.map((i) => <option key={i.id} value={i.id}>{i.name}</option>)}
        </select>
        <select className="wd-select" aria-label="Stufe" value={stage} disabled={dirty} title={lock} onChange={(e) => plan.selectStage(Number(e.target.value))}>
          {island.depths.map((d, i) => <option key={d} value={i}>{`Stufe ${i + 1} · ${biomeName(island.biomes[i] ?? '')}`}</option>)}
        </select>
        <input className="wd-slot" type="number" aria-label="Neuer Platz" value={slot} onChange={(e) => setSlot(number(e.target.value, slot))} />
        <Button variant="soft" onClick={() => plan.edit(addChunk(draft, slot, defaultKind(island.biomes[stage])))}>Chunk hinzufügen</Button>
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
          <li key={i} className="wd-planned" data-slot={c.slot}>
            <div className="wd-bar">
              <input className="wd-slot" type="number" aria-label={`Platz ${slotLabel(c.slot)}`} value={c.slot} onChange={(e) => plan.edit(updateChunk(draft, i, { slot: number(e.target.value, c.slot) }))} />
              <input className="wd-kind" list="wd-chunk-kinds" aria-label={`Art Platz ${slotLabel(c.slot)}`} value={c.kind} onChange={(e) => plan.edit(updateChunk(draft, i, { kind: e.target.value }))} />
              <input className="wd-seed" aria-label={`Chunk-Seed Platz ${slotLabel(c.slot)}`} placeholder="Chunk-Seed" value={c.seed ?? ''} onChange={(e) => plan.edit(updateChunk(draft, i, { seed: e.target.value }))} />
              <Button variant="soft" onClick={() => plan.edit(addObject(draft, i, { kind: OBJECT_KINDS[0] ?? 'tree', at: 10 }))}>Objekt</Button>
              <Button variant="outline" color="red" aria-label={`Chunk Platz ${slotLabel(c.slot)} löschen`} onClick={() => plan.edit(removeChunk(draft, i))}>Löschen</Button>
            </div>
            {(c.objects ?? []).map((o, j) => (
              <div key={j} className="wd-object">
                <select className="wd-select" aria-label={`Objekt ${j + 1} Platz ${slotLabel(c.slot)}`} value={o.kind}
                  onChange={(e) => plan.edit(draft.map((x, k) => (k === i ? { ...x, objects: (x.objects ?? []).map((y, m) => (m === j ? { ...y, kind: e.target.value } : y)) } : x)))}>
                  {OBJECT_KINDS.map((k) => <option key={k} value={k}>{k}</option>)}
                </select>
                <input className="wd-slot" type="number" aria-label={`Lage Objekt ${j + 1} Platz ${slotLabel(c.slot)}`} value={o.at} onChange={(e) => plan.edit(moveObject(draft, i, j, number(e.target.value, o.at)))} />
                <Button variant="outline" color="red" aria-label={`Objekt ${j + 1} Platz ${slotLabel(c.slot)} löschen`} onClick={() => plan.edit(removeObject(draft, i, j))}>×</Button>
              </div>
            ))}
            {!check.ok && check.slot === c.slot && <Text as="p" size="1" color="red" role="alert" className="wd-error" data-slot={c.slot}>{check.error}</Text>}
          </li>
        ))}
      </ul>
      <datalist id="wd-chunk-kinds">{CHUNK_KINDS.map((k) => <option key={k} value={k} />)}</datalist>
    </section>
  );
}
