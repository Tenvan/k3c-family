// Golden-Regel (docs/standards/paper-spec.md › 6): Stand PV1.5 = exakt (SHA-256 gleich). Rückfall `toleranz`:
// gleich, wenn höchstens 0,1 % der Pixel eine Kanal-Differenz > 2 haben (oder die Größe abweicht → ungleich).
import { createHash } from 'node:crypto';
import { abweichend, liesPng } from '../render/png';

export type GoldenRegel = 'exakt' | 'toleranz';
export const GOLDEN_REGEL: GoldenRegel = 'exakt';
const TOLERANZ_ANTEIL = 0.001;

const sha256 = (b: Buffer) => createHash('sha256').update(b).digest('hex');

export function gleich(a: Buffer, b: Buffer, regel: GoldenRegel = GOLDEN_REGEL): boolean {
  if (sha256(a) === sha256(b)) return true;
  if (regel === 'exakt') return false;
  const pa = liesPng(a), pb = liesPng(b);
  if (pa.breite !== pb.breite || pa.hoehe !== pb.hoehe) return false;
  const anders = abweichend(pa, pb);
  let n = 0;
  for (let y = 0; y < pb.hoehe; y++) for (let x = 0; x < pb.breite; x++) if (anders(x, y)) n++;
  return n <= TOLERANZ_ANTEIL * pb.breite * pb.hoehe;
}
