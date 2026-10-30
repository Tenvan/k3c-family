import { BIOMES } from '@game/model/biome';
import { Button, SelectField, Text, TextField } from '@webkit/atoms';
import { ChipSwitch } from '@webkit/molecules';
import { useState } from 'react';
import { changesText } from './changes';
import { GameLauncher } from './GameLauncher';
import { SizePicker } from './SizePicker';
import type { Guard } from './WorldGuard';
import type { PlanState } from './usePlan';
import type { WorldDraftState } from './useWorldDraft';

export type Mode = 'welt' | 'stufe';
export const MODES: readonly Mode[] = ['welt', 'stufe'];
const MODE_OPTIONS = [{ value: 'welt' as const, label: 'Welt-Struktur' }, { value: 'stufe' as const, label: 'Stufe bearbeiten' }];

const biomeName = (id: string) => BIOMES.find((b) => b.id === id)?.name ?? id;

/** Neue Welt mit Namen anlegen: leer (eine Insel) oder als Kopie der gewählten. */
function AddWorld({ plan, guard }: { plan: PlanState; guard: Guard }) {
  const [newName, setNewName] = useState('');
  const off = newName.trim() === '';
  const add = (copy: boolean) => guard.run(() => { void (copy ? plan.copyWorld(newName) : plan.addWorld(newName)); setNewName(''); });
  return (
    <>
      <TextField.Root className="wd-seed" aria-label="Name der neuen Welt" placeholder="Name der neuen Welt" value={newName} onChange={(e) => setNewName(e.target.value)} />
      <Button variant="soft" disabled={off} title="Neue Welt mit einer Insel anlegen (eigene JSON-Datei unter data/worlds/)" onClick={() => add(false)}>Welt +</Button>
      <Button variant="soft" disabled={off} title="Kopie der gewählten Welt mit allen Inseln, Planung und Inselgrößen" onClick={() => add(true)}>Kopie</Button>
    </>
  );
}

/** Welt wählen (Wechsel mit Rückfrage), im Modus Welt-Struktur auch anlegen, kopieren und löschen (die Standard-Welt bleibt). */
function WorldPick({ plan, guard, manage }: { plan: PlanState; guard: Guard; manage: boolean }) {
  const { worlds } = plan;
  const active = worlds?.worlds.find((w) => w.id === worlds.active);
  const isStandard = !worlds || worlds.active === 'standard';
  const remove = () => guard.run(() => {
    if (active && window.confirm(`Welt „${active.name}“ mit allen Inseln löschen?`)) void plan.removeWorld(active.id);
  });
  return (
    <>
      <SelectField aria-label="Welt" value={worlds?.active ?? ''} disabled={!worlds} onChange={(v) => guard.run(() => void plan.selectWorld(v))}
        options={(worlds?.worlds ?? []).map((w) => ({ value: w.id, label: w.name }))} />
      {manage && <AddWorld plan={plan} guard={guard} />}
      {manage && <Button variant="outline" color="red" disabled={isStandard} title={isStandard ? 'Die Standard-Welt bleibt' : 'Gewählte Welt löschen (ihre Datei)'} onClick={remove}>Welt −</Button>}
    </>
  );
}

/** Insel, Stufe und Inselgröße der Stufe (Modus Stufe bearbeiten); Wechsel mit Rückfrage. */
function StagePick({ plan, guard }: { plan: PlanState; guard: Guard }) {
  const { island, stage, view } = plan;
  if (!island) return null;
  return (
    <>
      <span aria-hidden className="wd-sep">/</span>
      <SelectField aria-label="Insel" value={island.id} onChange={(v) => guard.run(() => plan.selectIsland(v))}
        options={(view?.islands ?? []).map((i) => ({ value: i.id, label: i.name }))} />
      <span aria-hidden className="wd-sep">/</span>
      <SelectField aria-label="Stufe" value={String(stage)} onChange={(v) => guard.run(() => plan.selectStage(Number(v)))}
        options={island.depths.map((_, i) => ({ value: String(i), label: `Stufe ${i + 1} · ${biomeName(island.biomes[i] ?? '')}` }))} />
      <SizePicker plan={plan} />
    </>
  );
}

/** Speichern und Verwerfen des Entwurfs im aktiven Modus, mit der Zahl der Änderungen (B-664/AC-03). */
function SaveArea({ mode, plan, world }: { mode: Mode; plan: PlanState; world: WorldDraftState }) {
  const welt = mode === 'welt';
  const state = welt ? world : plan;
  const lock = welt ? world.lock : undefined;
  return (
    <div className="wd-bar wd-save">
      {state.dirty && <Text size="2" color="orange" role="status" className="wd-changes">{changesText(state.changes)}</Text>}
      {!state.check.ok && state.dirty && <Text size="2" color="red">ungültig</Text>}
      <Button variant="outline" disabled={!state.dirty} onClick={state.discard}>Verwerfen</Button>
      <Button disabled={!state.dirty || !state.check.ok || lock !== undefined} title={lock} onClick={() => void state.save()}>{welt ? 'Welt speichern' : 'Speichern'}</Button>
      {!welt && <Button variant="soft" color="gray" disabled={plan.dirty} title={plan.dirty ? 'Erst speichern oder verwerfen' : 'Planung neu von der Platte laden'} onClick={() => void plan.reload()}>Neu laden</Button>}
    </div>
  );
}

/** Kopf des Welten Editors (B-664): Umschalter der zwei Modi, Auswahlzeile (Welt, im Modus Stufe bearbeiten auch Insel und
 *  Stufe), Speichern an einer Stelle und „Im Spiel öffnen“. Meldungen von Planung und Welt-Entwurf stehen darunter. */
export function WorldHeader({ mode, onMode, plan, world, guard, seed, biome }: {
  mode: Mode; onMode: (mode: Mode) => void; plan: PlanState; world: WorldDraftState; guard: Guard; seed: string; biome: string;
}) {
  const notice = mode === 'welt' ? world.notice : (plan.notice ?? (plan.dirty ? world.notice : null)); // Chat-Meldung auch bei offener Planung
  return (
    <header className="wd-head">
      <div className="wd-bar">
        <ChipSwitch label="Modus" options={MODE_OPTIONS} value={mode} onChange={(m) => { if (m !== mode) guard.run(() => onMode(m)); }} />
        <WorldPick plan={plan} guard={guard} manage={mode === 'welt'} />
        {mode === 'stufe' && <StagePick plan={plan} guard={guard} />}
        <span className="wd-grow" />
        <SaveArea mode={mode} plan={plan} world={world} />
        <GameLauncher seed={seed} biome={biome} />
      </div>
      {guard.prompt}
      {notice && <Text as="p" size="2" role="status" color={notice.kind === 'hint' ? 'green' : 'red'} className="wd-notice">{notice.text}{notice.kind === 'stale' ? ' (Entwurf bleibt)' : ''}</Text>}
    </header>
  );
}
