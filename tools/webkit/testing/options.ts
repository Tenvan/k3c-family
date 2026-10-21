import { act } from 'react';
import { pickOption, selectTrigger } from './select';

// Test-Helfer zu select.ts: Einträge einer Auswahlliste (atoms/SelectField, Radix Select) lesen. Radix rendert sie erst
// nach dem Öffnen im Portal; die Helfer öffnen, lesen und schließen wieder (Esc).

/** Eintrag einer Auswahl, wie ihn die Liste zeigt. */
export interface ShownOption {
  text: string;
  /** Wert des Eintrags (`data-value`, atoms/SelectField). */
  value: string;
  disabled: boolean;
}

/** Alle Einträge der Auswahl mit diesem `aria-label`, in Anzeige-Reihenfolge. */
export async function shownOptions(root: ParentNode, label: string): Promise<ShownOption[]> {
  const trigger = selectTrigger(root, label);
  await act(async () => trigger.click());
  const shown = [...document.querySelectorAll<HTMLElement>('[role="option"]')].map((o) => ({
    text: o.textContent ?? '',
    value: o.dataset.value ?? '',
    disabled: o.hasAttribute('data-disabled'),
  }));
  const content = document.querySelector<HTMLElement>('[role="listbox"]');
  await act(async () => (content ?? trigger).dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true })));
  return shown;
}

/** Texte aller Einträge der Auswahl mit diesem `aria-label`, in Anzeige-Reihenfolge. */
export const optionTexts = async (root: ParentNode, label: string): Promise<string[]> => (await shownOptions(root, label)).map((o) => o.text);

/** Wählt den ersten Eintrag, dessen Text `match` erfüllt (z. B. bei langen oder errechneten Beschriftungen). */
export async function pickWhere(root: ParentNode, label: string, match: (text: string) => boolean): Promise<void> {
  const texts = await optionTexts(root, label);
  const text = texts.find(match);
  if (text === undefined) throw new Error(`Kein passender Eintrag in „${label}“ (da: ${texts.join(', ')})`);
  await pickOption(root, label, text);
}
