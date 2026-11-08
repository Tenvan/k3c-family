/** Ein Gerät, das einem Platz beitreten kann (Tastatur, Controller, Touch). */
export type JoinDevice = 'keyboard' | 'pad' | 'touch';
export interface JoinCandidate {
  device: JoinDevice;
  /** Das Gerät steuert schon einen Spieler */
  bound: boolean;
}

/**
 * Welche Geräte der Beitritts-Hinweis nennt (B-391). Solange niemand beigetreten ist, das zuletzt benutzte Gerät;
 * danach jedes Gerät, das noch keinen Spieler steuert.
 * Das zuletzt benutzte Gerät steht vorn, ein Gerät höchstens einmal.
 */
export function joinHintDevices(candidates: readonly JoinCandidate[], last: JoinDevice): JoinDevice[] {
  if (!candidates.some((c) => c.bound)) return [last];
  const free = candidates.filter((c) => !c.bound).map((c) => c.device);
  const unique = [...new Set(free)];
  return unique.sort((a, b) => Number(b === last) - Number(a === last));
}
