// @vitest-environment happy-dom
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { backend, type PlanView } from '../api';
import { mockAnswer } from '../api/mockWorldDraft';
import type { PlanState } from './usePlan';
import { useWorldDraft, type WorldDraftState } from './useWorldDraft';

// B-514/AC-04 und AC-05: Der Mock setzt from wie Go; ein Entwurf wandert beim Wechsel der Welt nicht mit.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
afterEach(() => act(() => root?.unmount()));

describe('Entwurf und Weltwechsel (B-514)', () => {
  it('verwirft den eigenen Entwurf, wenn die Welt wechselt', async () => {
    const view: PlanView = await backend.planLoad();
    let state!: WorldDraftState;
    const Probe = ({ active }: { active: string }) => {
      const plan = { view, reload: async () => {}, refreshWorlds: async () => {}, dirty: false, worlds: { worlds: [], active, error: '' } } as unknown as PlanState;
      state = useWorldDraft(plan);
      return null;
    };
    const host = document.createElement('div');
    root = createRoot(host);
    await act(async () => root!.render(<Probe active="a" />));
    act(() => state.edit({ ...state.draft!, name: 'Geändert' }));
    expect(state.draft!.name).toBe('Geändert');
    await act(async () => root!.render(<Probe active="a" />));
    expect(state.draft!.name).toBe('Geändert'); // gleiche Welt: bleibt
    await act(async () => root!.render(<Probe active="b" />));
    expect(state.draft!.name).toBe(view.name);
    expect(state.dirty).toBe(false);
  });

  it('der Mock setzt from für Inseln der Welt-Datei und lässt es für neue leer', () => {
    const answer = mockAnswer('probe', JSON.stringify({ name: 'W', islands: [
      { id: 'alt', name: 'Alt', depths: [0, 4], depthScaling: { hp: 1, damage: 1, speed: 1 } },
      { id: 'neu', name: 'Neu', depths: [0, 4], depthScaling: { hp: 1, damage: 1, speed: 1 } }] }), ['alt']);
    expect(answer.draft!.islands.map((i) => i.from)).toEqual(['alt', '']);
  });
});
