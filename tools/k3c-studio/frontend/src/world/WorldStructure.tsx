import { BIOMES } from '@game/model/biome';
import { Button, Text } from '@radix-ui/themes';
import { GameLauncher } from './GameLauncher';
import { SizePicker } from './SizePicker';
import type { PlanState } from './usePlan';

const biomeName = (id: string) => BIOMES.find((b) => b.id === id)?.name ?? id;

/** Welten- und Inselstruktur (Welten Editor, linke Spalte): die Welt mit ihren Inseln und deren Stufen als Baum, Insel und
 *  Stufe wählen (die Vorschauen und der Editor für geplante Chunks folgen der Wahl), Insel anlegen (Kopie der letzten) und
 *  entfernen (die erste bleibt), Inselgröße der Stufe und „Im Spiel öffnen“. Anlegen, Entfernen und Wechseln geht nur ohne
 *  offenen Entwurf. */
export function WorldStructure({ plan, seed, biome }: { plan: PlanState; seed: string; biome: string }) {
  const { island, stage, dirty, view } = plan;
  const lock = dirty ? 'Erst speichern oder verwerfen' : undefined;
  const islands = view?.islands ?? [];
  const isFirst = island !== undefined && island.id === islands[0]?.id;
  return (
    <section className="wd-struct" aria-label="Weltstruktur">
      <div className="wd-bar">
        <Text size="2" weight="bold">Welt</Text>
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
      <ul className="wd-structure" aria-label="Inseln und Stufen">
        {islands.map((i) => (
          <li key={i.id} data-selected={i.id === island?.id}>
            <button type="button" className="wd-node" disabled={dirty && i.id !== island?.id} onClick={() => plan.selectIsland(i.id)}>
              🏝️ {i.name} <Text size="1" color="gray">{i.depths.length} Stufen</Text>
            </button>
            <ul>
              {i.depths.map((d, s) => (
                <li key={d} data-selected={i.id === island?.id && s === stage}>
                  <button type="button" className="wd-node" disabled={dirty && !(i.id === island?.id && s === stage)}
                    onClick={() => { if (i.id !== island?.id) plan.selectIsland(i.id); plan.selectStage(s); }}>
                    Stufe {s + 1} · {biomeName(i.biomes[s] ?? '')}{' '}
                    <Text size="1" color="gray">{i.chunkCounts[String(d)] ? `${i.chunkCounts[String(d)]} Abschnitte` : 'Biom-Länge'}{(i.planned[String(d)]?.length ?? 0) > 0 ? ` · ${i.planned[String(d)]!.length} geplant` : ''}</Text>
                  </button>
                </li>
              ))}
            </ul>
          </li>
        ))}
      </ul>
      <GameLauncher seed={seed} biome={biome} />
    </section>
  );
}
