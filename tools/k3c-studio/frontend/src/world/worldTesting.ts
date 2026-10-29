import { act } from 'react';

// Gemeinsame Handgriffe der DOM-Tests des Welten Editors (B-664, B-665). Das Mock-Level hat den Hub bei Abschnitt 5; der
// Abschnitt am Chunk-Platz n liegt bei Abschnitt 5 + n (rules/geplante-chunks.md § 1).
export const HUB_INDEX = 5;

const flush = () => act(async () => { for (let i = 0; i < 5; i++) await new Promise((r) => setTimeout(r, 0)); });
const named = (h: HTMLElement, label: string) => [...h.querySelectorAll('button')].find((b) => b.textContent === label || b.getAttribute('aria-label') === label) as HTMLButtonElement | undefined;

/** Abschnitt des Levels in der Chunk-Vorschau (nach Index) */
export const chunkAt = (h: HTMLElement, index: number) => [...h.querySelectorAll<HTMLElement>('.wd-chunk')][index]!;

/** Modus des Welten Editors wählen: „Welt-Struktur“ oder „Stufe bearbeiten“ (Umschalter im Kopf) */
export async function toMode(h: HTMLElement, label: 'Welt-Struktur' | 'Stufe bearbeiten'): Promise<void> {
  await act(async () => named(h, label)!.click());
  await flush();
}

/** Chunk-Platz `slot` wählen: Klick auf seinen Abschnitt in der Chunk-Vorschau (der Inspektor zeigt dann den Reiter Chunk) */
export async function selectSlot(h: HTMLElement, slot: number): Promise<void> {
  await act(async () => chunkAt(h, HUB_INDEX + slot).click());
  await flush();
}

/** Chunk-Platz `slot` wählen und „Chunk planen“ klicken (wie das frühere „Chunk hinzufügen“ am Platz) */
export async function planSlot(h: HTMLElement, slot: number): Promise<void> {
  await selectSlot(h, slot);
  await act(async () => named(h, 'Chunk planen')!.click());
  await flush();
}

/** Gespeicherte Modus-Wahl der Seite vergessen (jeder Test beginnt im Modus Stufe bearbeiten) */
export const forgetMode = () => localStorage.removeItem('k3c-studio.world.mode');
