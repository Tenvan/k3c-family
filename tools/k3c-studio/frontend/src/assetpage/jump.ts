// Sprung zur Assets-Seite mit einem Slot (B-677): Planer, Monarch, Gegner, Chat und Ticket melden den Slot, die Rahmen-App
// öffnet die Seite Assets, die ihn beim Einhängen wählt. Nur die offene Seite ist eingehängt, deshalb liegt der Slot hier zwischen.

export const GOTO_ASSETS = 'studio:goto-assets';

let pending = '';

/** Öffnet die Seite Assets mit diesem Slot im Inspektor. */
export function gotoAssets(slot: string): void {
  pending = slot;
  window.dispatchEvent(new Event(GOTO_ASSETS));
}

/** Holt den gemeldeten Slot (einmalig). */
export function takePendingSlot(): string {
  const slot = pending;
  pending = '';
  return slot;
}
