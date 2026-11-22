// Teil-Auswahl per Klick in die Vorschau (WB1.3, Sprint-README WB1 › D-05).
import type { Box } from '../api';

interface Size {
  width: number;
  height: number;
}

/** Teil unter dem Punkt (Bildpixel): kleinste Box gewinnt, bei Gleichstand das spätere Teil in `reihenfolge`
 *  (oben gezeichnet); daneben `''`. */
export function teilAn(teile: Record<string, Box>, reihenfolge: string[], x: number, y: number): string {
  let best = '';
  let bestArea = Infinity;
  let bestRank = -1;
  for (const [id, [bx, by, b, h]] of Object.entries(teile)) {
    if (x < bx || y < by || x >= bx + b || y >= by + h) continue;
    const area = b * h;
    const rank = reihenfolge.indexOf(id);
    if (area < bestArea || (area === bestArea && rank > bestRank)) {
      best = id;
      bestArea = area;
      bestRank = rank;
    }
  }
  return best;
}

/** Punkt in der Anzeige → Bildpixel; ohne bekannte Größe 1:1. */
export function bildPunkt(x: number, y: number, shown: Size, natural: Size): [number, number] {
  const sx = shown.width > 0 && natural.width > 0 ? natural.width / shown.width : 1;
  const sy = shown.height > 0 && natural.height > 0 ? natural.height / shown.height : 1;
  return [x * sx, y * sy];
}
