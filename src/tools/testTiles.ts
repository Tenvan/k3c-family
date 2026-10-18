/**
 * Fokus-Weiterschaltung der Testseite (`testing.html`) per Controller. Szenarien stehen in `testScenarios.ts`.
 */
export type FocusKey = 'prev' | 'next';

/** Index der Kachel, die nach `key` den Fokus bekommt; an den Rändern bleibt der Fokus stehen. */
export function nextFocus(count: number, at: number, key: FocusKey): number {
  if (count <= 0) return 0;
  const from = Math.min(count - 1, Math.max(0, at));
  return key === 'next' ? Math.min(count - 1, from + 1) : Math.max(0, from - 1);
}
