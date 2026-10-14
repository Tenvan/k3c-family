/** Längste Meldung in Zeichen; die Leistenzeile ist eine Zeile hoch, längerer Text wird mit „…“ gekürzt (B-414, 🧑 2026-10-14). */
export const BANNER_MAX_CHARS = 56;

/** Meldung für die einzeilige Leiste: auf die Länge gekürzt, nie über den Rand. */
export function fitBanner(text: string, max = BANNER_MAX_CHARS): string {
  const one = text.replace(/\s+/g, ' ').trim();
  return one.length <= max ? one : `${one.slice(0, max - 1).trimEnd()}…`;
}

/** Hängt eine Meldung an die Warteschlange; dieselbe Meldung steht nur einmal (laufend oder wartend), eine neue ersetzt die laufende nicht. */
export function enqueueBanner(queue: readonly string[], showing: string | null, text: string): string[] {
  const fitted = fitBanner(text);
  if (fitted === '' || fitted === showing || queue.includes(fitted)) return [...queue];
  return [...queue, fitted];
}
