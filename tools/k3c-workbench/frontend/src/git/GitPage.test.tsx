// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { GitPage } from './GitPage';

// B-580/AC-02 und AC-04: Die Seite zeigt mit dem Mock-Backend (Testlauf = Mock) alle vier Karten; jede Aktion, die sich
// nicht einfach zurücknehmen lässt, läuft erst nach der Bestätigung im Dialog.

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;

async function flush() {
  for (let i = 0; i < 5; i++) await act(async () => new Promise((r) => setTimeout(r, 0)));
}

/** Wartet die künstliche Dauer der Mock-Prüfung (300 ms) ab. */
async function settle() {
  await act(async () => new Promise((r) => setTimeout(r, 400)));
  await flush();
}

async function mount(active = true): Promise<HTMLElement> {
  const host = document.createElement('div');
  document.body.appendChild(host);
  root = createRoot(host);
  act(() => root!.render(<Theme><GitPage active={active} /></Theme>));
  await flush();
  return host;
}

describe('Ablauf und Laden', () => {
  it('Karten in der Reihenfolge Dateien, Commit-Status, PRs, Branches; gestaged getrennt von nicht gestaged', async () => {
    const host = await mount();
    const cards = [...host.querySelectorAll('section.git-card')].map((s) => s.getAttribute('aria-label'));
    expect(cards).toEqual(['Geänderte Dateien', 'Commit-Status', 'Pull Requests', 'Branches']);
    const files = host.querySelector('[aria-label="Geänderte Dateien"]')!.textContent!;
    expect(files.indexOf('Gestaged')).toBeLessThan(files.indexOf('uiapi.go'));
    expect(files.indexOf('Nicht gestaged')).toBeLessThan(files.indexOf('app_git.go'));
    expect(files.indexOf('uiapi.go')).toBeLessThan(files.indexOf('Nicht gestaged'));
  });

  it('liest erst beim ersten Öffnen und danach nur über „Neu laden“', async () => {
    const spy = vi.spyOn(backend, 'gitOverview');
    const prs = vi.spyOn(backend, 'gitPRs');
    await mount(false);
    expect(spy).not.toHaveBeenCalled();
    act(() => root!.render(<Theme><GitPage active /></Theme>));
    await flush();
    act(() => root!.render(<Theme><GitPage active={false} /></Theme>));
    act(() => root!.render(<Theme><GitPage active /></Theme>));
    await flush();
    expect(spy).toHaveBeenCalledTimes(1);
    act(() => button('Neu laden').click());
    await flush();
    expect(spy).toHaveBeenCalledTimes(2);
    expect(prs).toHaveBeenCalledTimes(2);
  });
});

const button = (label: string, within: ParentNode = document.body) => {
  const b = [...within.querySelectorAll('button')].find((x) => x.textContent?.trim() === label);
  if (!b) throw new Error(`Knopf „${label}“ fehlt`);
  return b;
};

const dialog = () => document.querySelector('[role=alertdialog]');

afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
  vi.restoreAllMocks();
});

describe('Seite Git mit Mock (B-580/AC-02)', () => {
  it('zeigt Status, Branches, geänderte Dateien und PRs', async () => {
    const host = await mount();
    const text = host.textContent ?? '';
    for (const want of ['feat/git-seite', '1 nicht gepusht', 'sprint/bd2', 'per PR #431 gemergt', 'per Squash gemergt',
      'origin/sprint/k5', 'tools/k3c-workbench/app_git.go', '#452', 'K5: Debug-Overlay nur mit ?dev=1', 'CI rot']) {
      expect(text).toContain(want);
    }
  });

  it('Dateien der Commits erst nach „Dateien zeigen“', async () => {
    const host = await mount();
    expect(host.textContent).toContain('Commits seit origin/develop: 3 Dateien');
    expect(host.textContent).not.toContain('internal/gitops/status.go');
    act(() => button('Dateien zeigen').click());
    expect(host.textContent).toContain('internal/gitops/status.go');
    act(() => button('Dateien ausblenden').click());
    expect(host.textContent).not.toContain('internal/gitops/status.go');
  });

  it('markierbar ist jeder lokale Branch außer Basis, aktuellem und Worktree', async () => {
    const host = await mount();
    const boxes = [...host.querySelectorAll('[role=checkbox][aria-label$=markieren]')].map((b) => b.getAttribute('aria-label'));
    expect(boxes).toEqual(['w11-lauf markieren', 'backup/wz19-alt markieren', 'sprint/bd2 markieren', 'sprint/gr8 markieren',
      'k4-weiter-arbeit markieren']);
  });
});

describe('Squash-Prüfung auf Knopfdruck', () => {
  it('„Ungeprüfte prüfen“ prüft alle, eine Zeile prüft nur sich', async () => {
    const spy = vi.spyOn(backend, 'gitCheckMerged');
    const host = await mount();
    const row = (name: string) => [...host.querySelectorAll('tr')].find((r) => r.textContent?.includes(name))!;
    expect(row('w11-lauf').textContent).toContain('ungeprüft');
    act(() => button('Prüfen', row('w11-lauf')).click());
    await settle();
    expect(spy).toHaveBeenCalledWith(['w11-lauf']);
    expect(row('w11-lauf').textContent).toContain('per PR #427 gemergt');
    expect(row('backup/wz19-alt').textContent).toContain('ungeprüft');
    act(() => button('Ungeprüfte prüfen (1)').click());
    await settle();
    expect(spy).toHaveBeenLastCalledWith([]);
    expect(row('backup/wz19-alt').textContent).toContain('ungemergt');
    expect(row('backup/wz19-alt').textContent).not.toContain('ungeprüft');
  });
});

describe('Bestätigung vor Zerstörendem (B-580/AC-04)', () => {
  it('Squash-Merge läuft erst nach „Squash-Merge“ im Dialog', async () => {
    const spy = vi.spyOn(backend, 'gitPRAction');
    const host = await mount();
    const row = [...host.querySelectorAll('tr')].find((r) => r.textContent?.includes('#452'))!;
    act(() => button('Squash', row).click());
    await flush();
    expect(dialog()?.textContent).toContain('PR #452 per Squash mergen?');
    expect(spy).not.toHaveBeenCalled();
    act(() => button('Squash-Merge').click());
    await flush();
    expect(spy).toHaveBeenCalledWith(452, 'squash');
  });

  it('Abbrechen löscht keinen Branch; ungemergte markierte bleiben stehen', async () => {
    const spy = vi.spyOn(backend, 'gitDeleteBranches');
    const host = await mount();
    for (const name of ['sprint/bd2', 'backup/wz19-alt']) act(() => (host.querySelector(`[aria-label="${name} markieren"]`) as HTMLElement).click());
    expect(host.querySelector('[aria-label="Prompt kopieren: 2 markierte aufräumen"]')).not.toBeNull();
    act(() => button('Gemergte löschen (1)').click());
    await flush();
    expect(dialog()?.textContent).toContain('sprint/bd2 lokal');
    expect(dialog()?.textContent).toContain('Bleiben stehen (ungemergt oder ungeprüft): backup/wz19-alt');
    act(() => button('Abbrechen').click());
    await flush();
    expect(spy).not.toHaveBeenCalled();
    act(() => button('Gemergte löschen (1)').click());
    await flush();
    act(() => button('Löschen').click());
    await flush();
    expect(spy).toHaveBeenCalledWith(['sprint/bd2'], false);
    expect(host.textContent).toContain('1 Branches gelöscht');
  });

  it('Schließen und Stash zurück fragen ebenfalls nach', async () => {
    const pr = vi.spyOn(backend, 'gitPRAction');
    const act2 = vi.spyOn(backend, 'gitAction');
    const host = await mount();
    const row = [...host.querySelectorAll('tr')].find((r) => r.textContent?.includes('#447'))!;
    act(() => button('Schließen', row).click());
    await flush();
    expect(dialog()?.textContent).toContain('PR #447 schließen?');
    act(() => button('Abbrechen').click());
    await flush();
    act(() => button('Stash').click());
    await flush();
    act(() => button('Stash zurück').click());
    await flush();
    expect(dialog()?.textContent).toContain('Letzten Stash zurückholen?');
    expect(pr).not.toHaveBeenCalled();
    expect(act2).toHaveBeenCalledTimes(1);
    expect(act2).toHaveBeenCalledWith('stash');
  });
});
