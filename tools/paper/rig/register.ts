// Trägt die bewegten Rigs in den Renderer ein (render/bild.ts › RIG_BILDER); Import mit Seiteneffekt.
import { RIG_BILDER } from '../render/bild';
import { figurBild } from './figurBild';
import { vierbeinerBild } from './vierbeinerBild';

RIG_BILDER.figur = figurBild;
RIG_BILDER.vierbeiner = vierbeinerBild;
