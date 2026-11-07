// Frames einer figur: Umhang hinter allen Teilen (README › D-08), Teile mit Versatz der Pose, Papier-Filter.
import type { Spec } from '../schema/types';
import { num } from '../schema/hash';
import { weltMatrizen } from '../render/baum';
import { type Bild, geometrie, type Kontext } from '../render/bild';
import { verschiebe } from '../render/matrix';
import { frameGroesse } from '../render/raster';
import { type Ausgabe, ausgabeSpec } from '../render/stufen';
import { teileSvg } from '../render/svg';
import { simuliere } from './animation';
import { animationVon, keyframeAusgeblendet } from '../render/pose';
import { reitBild } from './reiten';
import { umhangSvg } from './umhang';

export function figurBild(spec: Spec, a: Ausgabe, k: Kontext): Bild {
  if (animationVon(spec, a.animation).reittier) return reitBild(spec, a, k);
  const s = ausgabeSpec(spec, a);
  const anim = animationVon(s, a.animation);
  const frames = simuliere(s, a.animation).map((f, n) => {
    const aus = keyframeAusgeblendet(anim.keyframes, n / anim.frames);
    const [vx, vy] = f.versatz;
    const tf = num(vx) === '0' && num(vy) === '0' ? '' : ` transform="translate(${num(vx)} ${num(vy)})"`;
    const umhang = f.umhang && s.umhang ? umhangSvg(f.umhang, s.umhang, k.stil) : '';
    return {
      svg: `<g filter="url(#paper)">${umhang}<g${tf}>${teileSvg(s, k.stil, f.winkel, { ausblenden: aus })}</g></g>`,
      geometrie: geometrie(s.teile, weltMatrizen(s, f.winkel, verschiebe(vx, vy)), aus),
    };
  });
  return { groesse: frameGroesse(spec.ausgabe), frames };
}
