// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act, useState } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { pickOption, selectedText } from '../testing/select';
import { SelectField } from './SelectField';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
afterEach(() => {
  act(() => root?.unmount());
  document.body.innerHTML = '';
});

const seen: string[] = [];

function Probe() {
  const [v, setV] = useState('');
  return (
    <SelectField aria-label="Aktion" value={v} onChange={(x) => { seen.push(x); setV(x); }}
      options={[{ value: '', label: 'Keine' }, { value: 'hit', label: 'Schlag' }, { value: 'bow', label: 'Bogen' }]} />
  );
}

async function mount(): Promise<HTMLElement> {
  const host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><Probe /></Theme>));
  return host;
}

function Placeholder() {
  return <SelectField aria-label="Ziel" placeholder="Bitte wählen" value="" onChange={() => {}} options={[{ value: 'a', label: 'A' }]} />;
}

describe('SelectField', () => {
  it('ohne leeren Eintrag zeigt "" den Platzhalter', async () => {
    const host = document.createElement('div');
    document.body.append(host);
    root = createRoot(host);
    await act(async () => root!.render(<Theme><Placeholder /></Theme>));
    expect(selectedText(host, 'Ziel')).toBe('Bitte wählen');
  });

  it('zeigt den leeren Wert als Eintrag und meldet die Wahl als Wert', async () => {
    const h = await mount();
    expect(selectedText(h, 'Aktion')).toBe('Keine');
    await pickOption(h, 'Aktion', 'Bogen');
    expect(selectedText(h, 'Aktion')).toBe('Bogen');
    await pickOption(h, 'Aktion', 'Keine');
    expect(seen).toEqual(['bow', '']);
  });
});
