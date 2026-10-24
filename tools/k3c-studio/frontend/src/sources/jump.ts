// Sprung „Vorschläge für diesen Slot“ (B-601): Sound- und Grafik-Seite melden den Slot, die Rahmen-App öffnet die Seite
// Quellen, die den Slot als Filter übernimmt. Nur die offene Seite ist eingehängt, deshalb liegt der Slot hier zwischen.

export const GOTO_SOURCES = 'studio:goto-sources';

let pending = '';

/** Öffnet die Seite Quellen, gefiltert auf den Slot. */
export function gotoSources(slot: string): void {
  pending = slot;
  window.dispatchEvent(new Event(GOTO_SOURCES));
}

/** Holt den gemeldeten Slot (einmalig). */
export function takePendingSlot(): string {
  const slot = pending;
  pending = '';
  return slot;
}
