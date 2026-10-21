import { BIOMES } from '@game/model/biome';
import { Button, SelectField, Text, TextField } from '@webkit/atoms';
import { useState } from 'react';
import { GameLauncher } from './GameLauncher';
import { SizePicker } from './SizePicker';
import type { PlanState } from './usePlan';
import { useChatDraft } from './useChatDraft';
import { useWorldDraft } from './useWorldDraft';
import { WorldDraftEditor } from './WorldDraftEditor';

const biomeName = (id: string) => BIOMES.find((b) => b.id === id)?.name ?? id;
const lockOf = (plan: PlanState) => (plan.dirty ? 'Erst speichern oder verwerfen' : undefined);

/** Neue Welt mit Namen anlegen: leer (eine Insel) oder als Kopie der gewählten; gesperrt mit Grund lock. */
function AddWorld({ plan, lock }: { plan: PlanState; lock?: string }) {
  const [newName, setNewName] = useState('');
  const off = lock !== undefined || newName.trim() === '';
  const add = (copy: boolean) => { void (copy ? plan.copyWorld(newName) : plan.addWorld(newName)); setNewName(''); };
  return (
    <>
      <TextField.Root className="wd-seed" aria-label="Name der neuen Welt" placeholder="Name der neuen Welt" value={newName} onChange={(e) => setNewName(e.target.value)} />
      <Button variant="soft" disabled={off} title={lock ?? 'Neue Welt mit einer Insel anlegen (eigene JSON-Datei unter data/worlds/)'} onClick={() => add(false)}>Welt +</Button>
      <Button variant="soft" disabled={off} title={lock ?? 'Kopie der gewählten Welt mit allen Inseln, Planung und Inselgrößen'} onClick={() => add(true)}>Kopie</Button>
    </>
  );
}

/** Welt wählen, anlegen (leer oder als Kopie der gewählten, eigene JSON-Datei unter data/worlds/) und löschen (die
 *  Standard-Welt bleibt); gesperrt, solange ein Entwurf (Planung oder Welt) offen ist. */
function WorldBar({ plan, draftOpen }: { plan: PlanState; draftOpen: boolean }) {
  const { worlds } = plan;
  const dirty = plan.dirty || draftOpen;
  const lock = dirty ? 'Erst speichern oder verwerfen' : undefined;
  const active = worlds?.worlds.find((w) => w.id === worlds.active);
  const isStandard = !worlds || worlds.active === 'standard';
  const remove = () => {
    if (active && window.confirm(`Welt „${active.name}“ mit allen Inseln löschen?`)) void plan.removeWorld(active.id);
  };
  return (
    <div className="wd-bar">
      <Text size="2" weight="bold">Welt</Text>
      <SelectField aria-label="Welt" value={worlds?.active ?? ''} disabled={dirty || !worlds} title={lock} onChange={(v) => void plan.selectWorld(v)}
        options={(worlds?.worlds ?? []).map((w) => ({ value: w.id, label: w.name }))} />
      <AddWorld plan={plan} lock={lock} />
      <Button variant="outline" color="red" disabled={dirty || isStandard} title={lock ?? (isStandard ? 'Die Standard-Welt bleibt' : 'Gewählte Welt löschen (ihre Datei)')} onClick={remove}>Welt −</Button>
    </div>
  );
}

/** Insel und Stufe der gespeicherten Welt wählen und die Inselgröße der Stufe (Planung); Inseln anlegen und ordnen geht im
 *  Welt-Entwurf. */
function IslandBar({ plan }: { plan: PlanState }) {
  const { island, stage, dirty, view } = plan;
  const lock = lockOf(plan);
  const islands = view?.islands ?? [];
  return (
    <>
      {island && (
        <div className="wd-bar">
          <SelectField aria-label="Insel" value={island.id} disabled={dirty} title={lock} onChange={plan.selectIsland}
            options={islands.map((i) => ({ value: i.id, label: i.name }))} />
          <SelectField aria-label="Stufe" value={String(stage)} disabled={dirty} title={lock} onChange={(v) => plan.selectStage(Number(v))}
            options={island.depths.map((_, i) => ({ value: String(i), label: `Stufe ${i + 1} · ${biomeName(island.biomes[i] ?? '')}` }))} />
          <SizePicker plan={plan} />
        </div>
      )}
    </>
  );
}

/** Baum der gewählten Welt: Inseln und ihre Stufen mit Abschnittszahl und Zahl geplanter Chunks; Klick wählt. */
function StructureTree({ plan }: { plan: PlanState }) {
  const { island, stage, dirty, view } = plan;
  const pick = (id: string, s: number) => { if (id !== island?.id) plan.selectIsland(id); plan.selectStage(s); };
  return (
    <ul className="wd-structure" aria-label="Inseln und Stufen">
      {(view?.islands ?? []).map((i) => (
        <li key={i.id} data-selected={i.id === island?.id}>
          <Button variant={i.id === island?.id ? 'soft' : 'ghost'} color="gray" highContrast className="wd-node" disabled={dirty && i.id !== island?.id} onClick={() => plan.selectIsland(i.id)}>
            🏝️ {i.name} <Text size="1" color="gray">{i.depths.length} Stufen</Text>
          </Button>
          <ul>
            {i.depths.map((d, s) => {
              const count = i.chunkCounts[String(d)];
              const planned = i.planned[String(d)]?.length ?? 0;
              return (
                <li key={d} data-selected={i.id === island?.id && s === stage}>
                  <Button variant={i.id === island?.id && s === stage ? 'soft' : 'ghost'} color="gray" highContrast className="wd-node" disabled={dirty && !(i.id === island?.id && s === stage)} onClick={() => pick(i.id, s)}>
                    Stufe {s + 1} · {biomeName(i.biomes[s] ?? '')}{' '}
                    <Text size="1" color="gray">{count ? `${count} Abschnitte` : 'Biom-Länge'}{planned > 0 ? ` · ${planned} geplant` : ''}</Text>
                  </Button>
                </li>
              );
            })}
          </ul>
        </li>
      ))}
    </ul>
  );
}

/** Welten- und Inselstruktur (Welten Editor, linke Spalte): Welt wählen, anlegen (leer, Kopie) und löschen, der Welt-Entwurf
 *  (Name, Notizen, Inseln mit Stufen und Skalierung, WZ18.6), darunter die gespeicherten Inseln und Stufen als Baum, Insel und
 *  Stufe wählen (die Vorschauen und der Editor für geplante Chunks folgen der Wahl), Inselgröße der Stufe und „Im Spiel öffnen“
 *  (startet die gewählte Welt). Welt wechseln, anlegen und löschen geht nur ohne offenen Entwurf. */
export function WorldStructure({ plan, seed, biome }: { plan: PlanState; seed: string; biome: string }) {
  const world = useWorldDraft(plan);
  useChatDraft(world);
  return (
    <section className="wd-struct" aria-label="Weltstruktur">
      <WorldBar plan={plan} draftOpen={world.dirty} />
      <WorldDraftEditor world={world} />
      <IslandBar plan={plan} />
      <StructureTree plan={plan} />
      <GameLauncher seed={seed} biome={biome} />
    </section>
  );
}
