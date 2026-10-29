// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { ActionMenu } from './ActionMenu';
import { ChipSwitch } from './ChipSwitch';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
});

function mount(node: React.ReactNode): HTMLElement {
  const host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  act(() => root!.render(<Theme>{node}</Theme>));
  return host;
}

describe('ChipSwitch', () => {
  it('markiert den gewählten Chip und meldet Klicks', () => {
    const pick = vi.fn();
    const host = mount(<ChipSwitch label="Filter" value="b" onChange={pick} options={[{ value: 'a', label: 'A', count: 2 }, { value: 'b', label: 'B' }]} />);
    const [a, b] = [...host.querySelectorAll('button')];
    expect(b.getAttribute('aria-pressed')).toBe('true');
    expect(a.getAttribute('aria-pressed')).toBe('false');
    expect(a.textContent).toContain('2');
    act(() => a.click());
    expect(pick).toHaveBeenCalledWith('a');
  });
});

describe('ActionMenu', () => {
  it('öffnet per Klick und schließt nach der Auswahl', async () => {
    const run = vi.fn();
    const host = mount(<ActionMenu label="Mehr" actions={[{ label: 'Tun', onSelect: run }]} />);
    await act(async () => host.querySelector('button')!.click());
    const item = [...document.querySelectorAll('button')].find((b) => b.textContent === 'Tun')!;
    await act(async () => item.click());
    expect(run).toHaveBeenCalledOnce();
  });
});
