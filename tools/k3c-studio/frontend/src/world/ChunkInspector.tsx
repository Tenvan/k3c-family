import { BIOMES } from '@game/model/biome';
import { Button, Text, TextField } from '@webkit/atoms';
import type { PlannedChunk } from '../api';
import { ConvertButton } from './ConvertButton';
import { KindPicker } from './KindPicker';
import { RuinField } from './RuinField';
import { iconOf, nameOf } from './kindIcons';
import { CHUNK_KINDS, OBJECT_KINDS, addChunk, addObject, moveObject, removeChunk, removeObject, setRuin, slotLabel, updateChunk } from './plan';
import type { PlanState } from './usePlan';

const defaultKind = (biome = ''): string => Object.keys(BIOMES.find((b) => b.id === biome)?.chunkWeights ?? {})[0] ?? 'forest';
const number = (value: string, fallback: number) => (value === '' || Number.isNaN(Number(value)) ? fallback : Number(value));

/** Felder eines geplanten Chunks: Art, Chunk-Seed, feste Objekte, Ruine (WZ14.7), Umwandeln (B-485/AC-04) und Entfernen.
 *  Jede Änderung (nicht das Entfernen) zentriert die Vorschauen auf den Chunk (`change`, WZ17.5). */
function PlannedFields({ plan, seed, c, i, change, onSlot }: { plan: PlanState; seed: string; c: PlannedChunk; i: number; change: (next: PlannedChunk[]) => void; onSlot: (slot: number) => void }) {
  const { draft, check } = plan;
  return (
    <div className="wd-planned" data-slot={c.slot}>
      <div className="wd-bar">
        <KindPicker label={`Art Platz ${slotLabel(c.slot)}`} value={c.kind} options={CHUNK_KINDS} onChange={(k) => change(updateChunk(draft, i, { kind: k }))} />
        <TextField.Root className="wd-seed" aria-label={`Chunk-Seed Platz ${slotLabel(c.slot)}`} placeholder="Chunk-Seed (leer = Seed des Levels)" value={c.seed ?? ''} onChange={(e) => change(updateChunk(draft, i, { seed: e.target.value }))} />
      </div>
      <Text size="2" weight="bold">Feste Objekte</Text>
      {(c.objects ?? []).map((o, j) => (
        <div key={j} className="wd-object">
          <KindPicker label={`Objekt ${j + 1} Platz ${slotLabel(c.slot)}`} value={o.kind} options={OBJECT_KINDS}
            onChange={(k) => change(draft.map((x, m) => (m === i ? { ...x, objects: (x.objects ?? []).map((y, n) => (n === j ? { ...y, kind: k } : y)) } : x)))} />
          <TextField.Root className="wd-slot" type="number" aria-label={`Lage Objekt ${j + 1} Platz ${slotLabel(c.slot)}`} value={o.at} onChange={(e) => change(moveObject(draft, i, j, number(e.target.value, o.at)))} />
          <Button variant="outline" color="red" aria-label={`Objekt ${j + 1} Platz ${slotLabel(c.slot)} löschen`} onClick={() => change(removeObject(draft, i, j))}>×</Button>
        </div>
      ))}
      <RuinField chunk={c} kinds={plan.view?.ruinKinds ?? []} onChange={(ruin) => change(setRuin(draft, i, ruin))} />
      <div className="wd-bar">
        <Button variant="soft" onClick={() => change(addObject(draft, i, { kind: OBJECT_KINDS[0] ?? 'tree', at: 10 }))}>+ Objekt</Button>
        <ConvertButton plan={plan} seed={seed} index={i} onDone={onSlot} />
      </div>
      {!check.ok && check.slot === c.slot && <Text as="p" size="1" color="red" role="alert" className="wd-error" data-slot={c.slot}>{check.error}</Text>}
      <Button variant="outline" color="red" size="1" aria-label={`Chunk Platz ${slotLabel(c.slot)} löschen`} onClick={() => plan.edit(removeChunk(draft, i))}>Planung dieses Abschnitts entfernen</Button>
    </div>
  );
}

/** Kopf des gewählten Abschnitts: Platz, Art im Level und ob er geplant ist */
function ChunkHead({ slot, kind, planned }: { slot: number; kind?: string; planned: boolean }) {
  return (
    <div className="wd-bar">
      <Text size="3" weight="bold">Abschnitt {slot === 0 ? 'Hub' : slotLabel(slot)}</Text>
      {kind && <Text size="2" className="wd-kindname">{iconOf(kind)} {nameOf(kind)}</Text>}
      {planned && <Text size="1" color="blue">geplant</Text>}
    </div>
  );
}

/** Ungeplanter Abschnitt: „Chunk planen“ (Art aus `chunkWeights` des Bioms); der Hub-Abschnitt lässt sich nicht planen. */
function FreeChunk({ plan, slot, onSlot }: { plan: PlanState; slot: number; onSlot: (slot: number) => void }) {
  const free = slot !== 0;
  return (
    <div className="wd-card">
      <Text size="2" color="gray">{free ? 'Dieser Abschnitt ist zufällig.' : 'Der Hub-Abschnitt lässt sich nicht planen.'}</Text>
      <Button disabled={!free} title={free ? undefined : 'Nur Abschnitte links und rechts des Hubs sind planbar'}
        onClick={() => { plan.edit(addChunk(plan.draft, slot, defaultKind(plan.island?.biomes[plan.stage]))); onSlot(slot); }}>Chunk planen</Button>
    </div>
  );
}

/** Die geplanten Chunks der Stufe als Chips; ein Klick wählt den Platz, ein Prüffehler färbt den Chip rot. */
function PlannedChips({ plan, slot, onSlot }: { plan: PlanState; slot: number | null; onSlot: (slot: number) => void }) {
  const { draft, check } = plan;
  return (
    <div className="wd-card">
      <Text size="2" weight="bold">Geplant auf dieser Stufe</Text>
      <ul className="wd-chunklist wd-chips-row" aria-label="Geplante Chunks">
        {draft.length === 0 && <li><Text size="1" color="gray">Keine Planung: Die Stufe bleibt rein zufällig.</Text></li>}
        {draft.map((p) => {
          const bad = !check.ok && check.slot === p.slot;
          return (
            <li key={p.slot} data-slot={p.slot} data-error={bad}>
              <Button size="1" variant={p.slot === slot ? 'solid' : 'soft'} color={bad ? 'red' : p.slot === slot ? undefined : 'gray'} onClick={() => onSlot(p.slot)}>
                {slotLabel(p.slot)} · {nameOf(p.kind)}
              </Button>
            </li>
          );
        })}
      </ul>
    </div>
  );
}

/** Reiter Chunk des Inspektors (B-665, Glossar: Inspektor): der in Chunk-Vorschau oder Band gewählte Chunk-Platz. Ungeplant
 *  gibt es „Chunk planen“, geplant bearbeitet man seine Felder (`PlannedFields`); darunter stehen die geplanten Chunks der
 *  Stufe als Chips. Prüffehler stehen am Chunk, ein Fehler ohne Chunk in der Liste oben. `kind` ist die Art des Abschnitts im Level. */
export function ChunkInspector({ plan, seed, slot, kind, onSlot }: { plan: PlanState; seed: string; slot: number | null; kind?: string; onSlot: (slot: number) => void }) {
  const { island, draft, check } = plan;
  if (!island) return <section className="wd-plan" aria-label="Planung"><Text size="2">{plan.notice?.text ?? 'Lade Planung …'}</Text></section>;
  const i = slot === null ? -1 : draft.findIndex((c) => c.slot === slot);
  const c = i >= 0 ? draft[i] : undefined;
  const orphan = !check.ok && !draft.some((x) => x.slot === check.slot);
  /** Entwurf ändern und den Chunk zentrieren (die Vorschauen springen auf ihn, WZ17.5) */
  const change = (next: PlannedChunk[]) => { plan.edit(next); if (slot !== null) onSlot(slot); };
  return (
    <section className="wd-plan" aria-label="Planung">
      {orphan && <Text as="p" size="2" color="red" role="alert" className="wd-error">{check.error}</Text>}
      {slot === null && <Text size="2" color="gray">Wähle einen Abschnitt in der Chunk-Vorschau oder im Band über der Bühne.</Text>}
      {slot !== null && <ChunkHead slot={slot} kind={kind} planned={c !== undefined} />}
      {slot !== null && !c && <FreeChunk plan={plan} slot={slot} onSlot={onSlot} />}
      {c && <PlannedFields plan={plan} seed={seed} c={c} i={i} change={change} onSlot={onSlot} />}
      <PlannedChips plan={plan} slot={slot} onSlot={onSlot} />
    </section>
  );
}
