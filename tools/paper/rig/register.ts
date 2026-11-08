// Trägt die bewegten Rigs in den Renderer ein (render/bild.ts › RIG_BILDER); Import mit Seiteneffekt.
import { RIG_BILDER } from '../render/bild';
import { animationVon } from '../render/pose';
import { figurBild } from './figurBild';
import { reittierEbeneBild } from './reiten';
import { vierbeinerBild } from './vierbeinerBild';

RIG_BILDER.figur = figurBild;
// Reittier-Ebenen (B-752) tragen `ebene` an der Animation; sonst das ganze Tier
RIG_BILDER.vierbeiner = (spec, a, k) => (animationVon(spec, a.animation).ebene ? reittierEbeneBild(spec, a, k) : vierbeinerBild(spec, a, k));
