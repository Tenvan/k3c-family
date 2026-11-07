// Frames eines Vierbeiners: Ride-Gruppe (dy, Neigung) in Welt-Einheiten, darin scale(massstab) mit den Teilen in
// Spec-Reihenfolge; der Beinpfad steht an der Stelle von oberbein-*. viewBox = rahmen · massstab (gleiche Pixel).
import { num } from '../schema/hash';
import type { Animation, Spec, Stil } from '../schema/types';
import { type Bild, geometrie, type Kontext } from '../render/bild';
import { Abgelehnt } from '../render/fehler';
import { animationVon } from '../render/pose';
import { type FrameGroesse, frameGroesse } from '../render/raster';
import { type Ausgabe, ausgabeSpec } from '../render/stufen';
import { kette, teileSvg } from '../render/svg';
import { beinpfad } from './beinpfad';
import { type Bein, type Gangart, GANGARTEN, type GangPose, gangPose } from './gangarten';
import { pruefeGangarten } from './proportionen';
import { rideText, vierbeinerMatrizen } from './vierbeiner';

/** Gangart der Animation oder Ablehnung animation-unbekannt. */
export function gangartVon(anim: Animation, name: string, standard?: Gangart): Gangart {
  const g = anim.gangart ?? standard;
  if (!g) throw new Abgelehnt([{ feld: `animationen.${name}`, regel: 'animation-unbekannt', text: `vierbeiner braucht eine gangart (${Object.keys(GANGARTEN).join(', ')})` }]);
  return g;
}

/** Der Renderer braucht alle Beinteile und `bein` (das Schema verlangt sie nicht); sonst pflicht-teil-fehlt. */
export function pruefeVierbeiner(spec: Spec): void {
  const ids = new Set(spec.teile.map((t) => t.id));
  const fehlt = ['oberbein', 'unterbein', 'huf'].flatMap((t) => ['hf', 'hn', 'vf', 'vn'].map((b) => `${t}-${b}`)).filter((id) => !ids.has(id));
  const fehler = fehlt.map((id) => ({ feld: 'teile', regel: 'pflicht-teil-fehlt', text: `vierbeiner braucht das Teil „${id}“ für den Beinpfad` }));
  if (!spec.bein) fehler.push({ feld: 'bein', regel: 'pflicht-teil-fehlt', text: 'vierbeiner braucht bein (Breiten des Beinpfads)' });
  if (fehler.length) throw new Abgelehnt(fehler);
  pruefeGangarten(spec);
}

export function vierbeinerGroesse(spec: Spec): FrameGroesse {
  const m = spec.ausgabe.massstab ?? 1, g = frameGroesse(spec.ausgabe);
  return { ...g, rahmen: spec.ausgabe.rahmen.map((v) => v * m) as FrameGroesse['rahmen'] };
}

/** Teile des Vierbeiners (ohne Ride-Transformation): scale(massstab), Beinpfade an oberbein-*; nur = welche Teile. */
export function pferdTeile(spec: Spec, p: GangPose, stil: Stil, nur: (id: string) => boolean): string {
  const k = kette(spec.teile, p.winkel);
  const einschub = (id: string) => {
    const b = id.startsWith('oberbein-') ? (id.slice(9) as Bein) : undefined;
    if (!b || !nur(id)) return '';
    const eltern = spec.teile.find((t) => t.id === id)!.eltern!, tf = k.get(eltern);
    return `<g${tf ? ` transform="${tf}"` : ''}>${beinpfad(spec, b, p.winkel, stil)}</g>`;
  };
  const aus = new Set(spec.teile.filter((t) => !nur(t.id)).map((t) => t.id));
  return `<g transform="scale(${num(p.massstab)})">${teileSvg(spec, stil, p.winkel, { einschub, ausblenden: aus })}</g>`;
}

export function vierbeinerBild(spec: Spec, a: Ausgabe, k: Kontext): Bild {
  pruefeVierbeiner(spec);
  const s = ausgabeSpec(spec, a), anim = animationVon(s, a.animation), g = gangartVon(anim, a.animation);
  const frames = Array.from({ length: anim.frames }, (_, n) => {
    const p = gangPose(s, g, n / anim.frames);
    return {
      svg: `<g filter="url(#paper)"><g transform="${rideText(p.ride.dy, p.ride.pitch)}">${pferdTeile(s, p, k.stil, () => true)}</g></g>`,
      geometrie: geometrie(s.teile, vierbeinerMatrizen(s, p)),
    };
  });
  return { groesse: vierbeinerGroesse(spec), frames };
}
