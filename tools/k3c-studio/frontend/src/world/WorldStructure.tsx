import { BIOMES } from '@game/model/biome';
import { Button, Text } from '@radix-ui/themes';
import { useState } from 'react';
import { GameLauncher } from './GameLauncher';
import { SizePicker } from './SizePicker';
import type { PlanState } from './usePlan';

const biomeName = (id: string) => BIOMES.find((b) => b.id === id)?.name ?? id;
const lockOf = (plan: PlanState) => (plan.dirty ? 'Erst speichern oder verwerfen' : undefined);

/** Welt wählen, anlegen (Name, eigene JSON-Datei unter data/worlds/) und löschen (die Standard-Welt bleibt). */
function WorldBar({ plan }: { plan: PlanState }) {
  const [newName, setNewName] = useState('');
  const { worlds, dirty } = plan;
  const lock = lockOf(plan);
  const active = worlds?.worlds.find((w) => w.id === worlds.active);
  const isStandard = !worlds || worlds.active === 'standard';
  const remove = () => {
    if (active && window.confirm(`Welt „${active.name}“ mit allen Inseln löschen?`)) void plan.removeWorld(active.id);
  };
  return (
    <div className="wd-bar">
      <Text size="2" weight="bold">Welt</Text>
      <select className="wd-select" aria-label="Welt" value={worlds?.active ?? ''} disabled={dirty || !worlds} title={lock} onChange={(e) => void plan.selectWorld(e.target.value)}>
        {worlds?.worlds.map((w) => <option key={w.id} value={w.id}>{w.name}</option>)}
      </select>
      <input className="wd-seed" aria-label="Name der neuen Welt" placeholder="Name der neuen Welt" value={newName} onChange={(e) => setNewName(e.target.value)} />
      <Button variant="soft" disabled={dirty || newName.trim() === ''} title={lock ?? 'Neue Welt mit einer Insel anlegen (eigene JSON-Datei unter data/worlds/)'}
        onClick={() => { void plan.addWorld(newName); setNewName(''); }}>Welt +</Button>
      <Button variant="outline" color="red" disabled={dirty || isStandard} title={lock ?? (isStandard ? 'Die Standard-Welt bleibt' : 'Gewählte Welt löschen (ihre Datei)')} onClick={remove}>Welt −</Button>
    </div>
  );
}

/** Insel und Stufe wählen, Insel anlegen (Kopie der letzten) und entfernen (die erste bleibt), Inselgröße der Stufe. */
function IslandBar({ plan }: { plan: PlanState }) {
  const { island, stage, dirty, view } = plan;
  const lock = lockOf(plan);
  const islands = view?.islands ?? [];
  const isFirst = island !== undefined && island.id === islands[0]?.id;
  return (
    <>
      <div className="wd-bar">
        <Text size="2" weight="bold">Inseln</Text>
        <span className="wd-grow" />
        <Button variant="soft" disabled={dirty || !view} title={lock ?? 'Neue Insel als Kopie der letzten'} onClick={() => void plan.addIsland()}>Insel +</Button>
        <Button variant="outline" color="red" disabled={dirty || !island || isFirst} title={lock ?? (isFirst ? 'Die erste Insel bleibt' : 'Gewählte Insel entfernen')} onClick={() => void plan.removeIsland()}>Insel −</Button>
      </div>
      {island && (
        <div className="wd-bar">
          <select className="wd-select" aria-label="Insel" value={island.id} disabled={dirty} title={lock} onChange={(e) => plan.selectIsland(e.target.value)}>
            {islands.map((i) => <option key={i.id} value={i.id}>{i.name}</option>)}
          </select>
          <select className="wd-select" aria-label="Stufe" value={stage} disabled={dirty} title={lock} onChange={(e) => plan.selectStage(Number(e.target.value))}>
            {island.depths.map((d, i) => <option key={d} value={i}>{`Stufe ${i + 1} · ${biomeName(island.biomes[i] ?? '')}`}</option>)}
          </select>
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
          <button type="button" className="wd-node" disabled={dirty && i.id !== island?.id} onClick={() => plan.selectIsland(i.id)}>
            🏝️ {i.name} <Text size="1" color="gray">{i.depths.length} Stufen</Text>
          </button>
          <ul>
            {i.depths.map((d, s) => {
              const count = i.chunkCounts[String(d)];
              const planned = i.planned[String(d)]?.length ?? 0;
              return (
                <li key={d} data-selected={i.id === island?.id && s === stage}>
                  <button type="button" className="wd-node" disabled={dirty && !(i.id === island?.id && s === stage)} onClick={() => pick(i.id, s)}>
                    Stufe {s + 1} · {biomeName(i.biomes[s] ?? '')}{' '}
                    <Text size="1" color="gray">{count ? `${count} Abschnitte` : 'Biom-Länge'}{planned > 0 ? ` · ${planned} geplant` : ''}</Text>
                  </button>
                </li>
              );
            })}
          </ul>
        </li>
      ))}
    </ul>
  );
}

/** Welten- und Inselstruktur (Welten Editor, linke Spalte): Welt wählen, anlegen und löschen, darunter ihre Inseln und
 *  Stufen als Baum, Insel und Stufe wählen (die Vorschauen und der Editor für geplante Chunks folgen der Wahl), Insel anlegen
 *  und entfernen, Inselgröße der Stufe und „Im Spiel öffnen“. Alles Wechseln, Anlegen und Löschen geht nur ohne offenen Entwurf. */
export function WorldStructure({ plan, seed, biome }: { plan: PlanState; seed: string; biome: string }) {
  return (
    <section className="wd-struct" aria-label="Weltstruktur">
      <WorldBar plan={plan} />
      <IslandBar plan={plan} />
      <StructureTree plan={plan} />
      <GameLauncher seed={seed} biome={biome} />
    </section>
  );
}
