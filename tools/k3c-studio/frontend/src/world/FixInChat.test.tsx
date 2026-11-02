// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import * as bridge from '../chat/chatBridge';
import { FixInChat, fixText, worldErrors } from './FixInChat';
import type { PlanState } from './usePlan';
import type { WorldDraftState } from './useWorldDraft';

// B-686: Fehlerbanner des Welten Editors mit „Im Chat korrigieren“.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;

afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
  vi.restoreAllMocks();
});

const HARBOR = 'data/worlds/nordsee.json: Insel insel1: kein Hafen in den geplanten Chunks der Tiefe 0, die Insel ist nicht die letzte der Welt (geplante-chunks.md § 10)';
const plan = (problems: string[]) => ({ dirty: false, check: { ok: true, slot: 0, error: '' }, view: { problems } }) as unknown as PlanState;
const world = (error: string) => ({ dirty: error !== '', check: { ok: error === '', error } }) as unknown as WorldDraftState;

describe('Im Chat korrigieren (B-686)', () => {
  it('sammelt Entwurfsfehler, Meldung und Fehlerliste der Datei ohne Doppelte', () => {
    const errs = worldErrors(true, plan([HARBOR]), world('Entwurf kaputt'), { kind: 'error', text: HARBOR });
    expect(errs).toEqual(['Entwurf kaputt', HARBOR]);
    expect(worldErrors(true, plan([]), world(''), null)).toEqual([]);
  });

  it('zeigt die Fehlerliste der gespeicherten Datei und legt alle Fehler in den Chat', async () => {
    const ask = vi.spyOn(bridge, 'askChat').mockImplementation(() => {});
    host = document.createElement('div');
    document.body.append(host);
    root = createRoot(host);
    await act(async () => root!.render(<Theme><FixInChat errors={[HARBOR]} problems={[HARBOR]} /></Theme>));
    expect(host.textContent).toContain('Die gespeicherte Welt ist ungültig');
    expect(host.textContent).toContain('kein Hafen');
    const btn = [...host.querySelectorAll('button')].find((b) => b.textContent === 'Im Chat korrigieren')!;
    await act(async () => btn.click());
    expect(ask).toHaveBeenCalledWith(fixText([HARBOR]));
    expect(fixText([HARBOR])).toContain(`- ${HARBOR}`);
  });

  it('ohne Fehler kein Banner', async () => {
    host = document.createElement('div');
    document.body.append(host);
    root = createRoot(host);
    await act(async () => root!.render(<Theme><FixInChat errors={[]} problems={[]} /></Theme>));
    expect(host.querySelector('button')).toBeNull();
  });
});
