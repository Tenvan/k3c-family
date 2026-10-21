import { act } from 'react';

// Test-Helfer für Auswahllisten (atoms/SelectField, Radix Select) in happy-dom: Radix rendert kein `<select>`, die
// Einträge stehen erst nach dem Öffnen im Portal am <body>.

/** Auslöser der Auswahl mit diesem `aria-label` (role="combobox"). */
export function selectTrigger(root: ParentNode, label: string): HTMLElement {
  const el = root.querySelector<HTMLElement>(`[role="combobox"][aria-label="${label}"]`);
  if (!el) throw new Error(`Auswahl „${label}“ fehlt`);
  return el;
}

/** Öffnet die Auswahl und klickt den Eintrag mit genau diesem Text. */
export async function pickOption(root: ParentNode, label: string, text: string): Promise<void> {
  const trigger = selectTrigger(root, label);
  await act(async () => trigger.click());
  const items = [...document.querySelectorAll<HTMLElement>('[role="option"]')];
  const item = items.find((o) => o.textContent === text);
  if (!item) throw new Error(`Eintrag „${text}“ fehlt in „${label}“ (da: ${items.map((o) => o.textContent).join(', ')})`);
  await act(async () => item.click());
}

/** Angezeigter Wert der Auswahl (Text des gewählten Eintrags). */
export const selectedText = (root: ParentNode, label: string): string => selectTrigger(root, label).textContent ?? '';

/** Wert der Auswahl (`data-value` am Auslöser von atoms/SelectField). */
export const selectedValue = (root: ParentNode, label: string): string => selectTrigger(root, label).dataset.value ?? '';

/** Öffnet die Auswahl und klickt den Eintrag mit diesem Wert (`data-value` am Eintrag, atoms/SelectField). */
export async function pickValue(root: ParentNode, label: string, value: string): Promise<void> {
  const trigger = selectTrigger(root, label);
  await act(async () => trigger.click());
  const items = [...document.querySelectorAll<HTMLElement>('[role="option"]')];
  const item = items.find((o) => o.dataset.value === value);
  if (!item) throw new Error(`Wert „${value}“ fehlt in „${label}“ (da: ${items.map((o) => o.dataset.value).join(', ')})`);
  await act(async () => item.click());
}
