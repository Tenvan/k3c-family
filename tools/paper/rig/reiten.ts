// Reiten (README › D-09, D-10): Reittier bis vor `hals`, Umhang, Reiter (ohne oberschenkel-h-Kette), `hals` und Rest,
// Zügel. Alles in der Ride-Gruppe (Welt-Einheiten); Rahmen = Hülle aus Reittier- und Reiter-Rahmen.
import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { basename, dirname, join } from 'node:path';
import { checkSpecFile } from '../schema/check';
import { num, specHash } from '../schema/hash';
import type { Punkt, Spec, Stil } from '../schema/types';
import { weltMatrizen } from '../render/baum';
import { type Bild, geometrie, type Kontext } from '../render/bild';
import { Abgelehnt, Aufruf } from '../render/fehler';
import { anwenden, drehung, type M, mul, skaliere, verschiebe } from '../render/matrix';
import { animationVon } from '../render/pose';
import { tausche } from '../render/stil';
import type { FrameGroesse } from '../render/raster';
import { type Ausgabe, ausgabeSpec } from '../render/stufen';
import { lageSvg, mitNachkommen, teileSvg } from '../render/svg';
import { frameSchritte, umhangLauf } from './animation';
import { GANGARTEN, type GangPose, gangPose } from './gangarten';
import { umhangSvg } from './umhang';
import { rideMatrix, rideText, vierbeinerMatrizen } from './vierbeiner';
import { gangartVon, pferdTeile, pruefeVierbeiner } from './vierbeinerBild';

/** Ordner? Nicht lesbare Einträge (fremde Rechte) zählen nicht. */
const istOrdner = (p: string) => {
  try {
    return statSync(p).isDirectory();
  } catch {
    return false;
  }
};
const abgelehnt = (text: string) => new Abgelehnt([{ feld: 'animationen.reittier', regel: 'reittier-unbekannt', text }]);

/** Spec des Reittiers: erst im Ordner der Spec, dann in Geschwister-Gruppen (../<gruppe>/<id>.paper.json). */
export function findeReittier(specPfad: string, id: string): string {
  if (!/^[a-z0-9][a-z0-9-]*$/.test(id)) throw abgelehnt(`Reittier-id „${id}“ ist kein Asset-Name`);
  const ordner = dirname(specPfad), eltern = dirname(ordner), name = `${id}.paper.json`;
  if (existsSync(join(ordner, name))) return join(ordner, name);
  const gruppen = existsSync(eltern) ? readdirSync(eltern).sort().map((g) => join(eltern, g)) : [];
  const treffer = gruppen.find((g) => g !== ordner && existsSync(join(g, name)) && istOrdner(g));
  if (!treffer) throw abgelehnt(`Reittier „${id}“ fehlt neben ${basename(specPfad)} und in den Nachbar-Gruppen`);
  return join(treffer, name);
}

/** Hashes der Specs, auf die Reit-Animationen verweisen (`reittier`, bei Reittier-Ebenen `reiter`; sortiert nach id), damit der renderKey des Reiters sie enthält (B-713). */
export function reittierHash(spec: Spec, specPfad: string | undefined): string | undefined {
  const ids = [...new Set(Object.values(spec.animationen ?? {}).flatMap((a) => [...(a.reittier ? [a.reittier] : []), ...(a.reiter ? [a.reiter] : [])]))].sort();
  if (!specPfad || !ids.length) return undefined;
  return ids.map((id) => `${id}=${specHash(readFileSync(findeReittier(specPfad, id), 'utf8'))}`).join(',');
}

export function ladeReittier(specPfad: string | undefined, id: string, stil: Stil): Spec {
  if (!specPfad) throw new Aufruf('Reiten braucht den Pfad der Spec');
  const pfad = findeReittier(specPfad, id), fehler = checkSpecFile(pfad, stil);
  if (fehler.length) throw new Abgelehnt(fehler.map((f) => ({ ...f, feld: `${basename(pfad)}:${f.feld}` })));
  const spec = JSON.parse(readFileSync(pfad, 'utf8')) as Spec;
  if (spec.rig !== 'vierbeiner') throw abgelehnt(`„${id}“ ist kein vierbeiner (rig ${spec.rig})`);
  pruefeVierbeiner(spec);
  return spec;
}

/** Hülle aus reittier.rahmen · massstab und reiter.rahmen + [−2, −54], nach außen gerundet; Pixel je Einheit des Reiters. */
export function reitRahmen(reiter: Spec, reittier: Spec): FrameGroesse {
  const m = reittier.ausgabe.massstab ?? 1, [tx, ty, tb, th] = reittier.ausgabe.rahmen, [rx, ry, rb, rh] = reiter.ausgabe.rahmen;
  const x0 = Math.floor(Math.min(tx * m, rx - 2)), y0 = Math.floor(Math.min(ty * m, ry - 54));
  const x1 = Math.ceil(Math.max((tx + tb) * m, rx + rb - 2)), y1 = Math.ceil(Math.max((ty + th) * m, ry + rh - 54));
  const px = reiter.ausgabe.hoehe_px / rh;
  return { rahmen: [x0, y0, x1 - x0, y1 - y0], frameW: Math.round((x1 - x0) * px), frameH: Math.round((y1 - y0) * px), eigen: true };
}

const reiterBasis = (p: GangPose): M => mul(verschiebe(...p.reiter.versatz), drehung(p.reiter.drehung, 0, -50));

export type ReitFrame = { pose: GangPose; umhang?: Punkt[] };

/** Posen und Umhang je Frame; Physik wie die Figur, Boden −103, Wind der Gangart. */
export function reitZustand(reiter: Spec, reittier: Spec, a: Ausgabe, _k: Kontext): ReitFrame[] {
  const anim = animationVon(reiter, a.animation), g = gangartVon(anim, a.animation, 'schritt'), freq = GANGARTEN[g].freq;
  const posen = Array.from({ length: anim.frames }, (_, n) => gangPose(reittier, g, n / anim.frames));
  const u = reiter.umhang;
  if (!u) return posen.map((pose) => ({ pose }));
  const anker = (p: GangPose) => anwenden(weltMatrizen(reiter, p.reiter.winkel, reiterBasis(p)).get(u.anker)!, u.punkt);
  const zustand = (k: number) => {
    const p = gangPose(reittier, g, ((((k / 120) * freq) % 1) + 1) % 1);
    return { anker: anker(p), wind: p.wind };
  };
  const ketten = umhangLauf(u, frameSchritte(freq, anim.frames), zustand, -103);
  return posen.map((pose, n) => ({ pose, umhang: ketten[n] }));
}

function zuegel(reiter: Spec, reittier: Spec, p: GangPose, stil: Stil): string {
  const hand = weltMatrizen(reiter, p.reiter.winkel, reiterBasis(p)).get('unterarm-v');
  const maul = weltMatrizen(reittier, p.winkel, skaliere(p.massstab)).get('hals');
  if (!hand || !maul) return '';
  const h = anwenden(hand, [0, -50]), m = anwenden(maul, [60, -91]);
  const form = `M${num(h[0])} ${num(h[1])}Q${num((h[0] + m[0]) / 2)} ${num((h[1] + m[1]) / 2 + 8)} ${num(m[0])} ${num(m[1])}`;
  return lageSvg({ art: 'falz', form, breite: 1.3, material: 'leder', ton: 'schatten' }, stil);
}

/** Ebenen von hinten nach vorn (D-09); die Reittier-Teile ab `hals` liegen vor dem Reiter */
export const EBENEN = ['reittier-hinten', 'umhang', 'reiter', 'reittier-vorn', 'zuegel'] as const;
export type Segment = (typeof EBENEN)[number];

/** Teile des Reittiers, die vor dem Reiter liegen (ab `hals`) */
const vornVon = (reittier: Spec): Set<string> => {
  const ab = reittier.teile.findIndex((t) => t.id === 'hals');
  return new Set(reittier.teile.slice(ab < 0 ? reittier.teile.length : ab).map((t) => t.id));
};

/** SVG der gewünschten Segmente (`nur`, sonst alle) in Zeichenreihenfolge; nicht gewünschte werden nicht gerechnet. */
export function reitSegmente(reiter: Spec, reittier: Spec, f: ReitFrame, stil: Stil, nur?: ReadonlySet<Segment>): { name: Segment; svg: string }[] {
  const p = f.pose, vorn = vornVon(reittier);
  const [vx, vy] = p.reiter.versatz;
  const svg: Record<Segment, () => string> = {
    'reittier-hinten': () => pferdTeile(reittier, p, stil, (id) => !vorn.has(id)),
    umhang: () => (f.umhang && reiter.umhang ? umhangSvg(f.umhang, reiter.umhang, stil) : ''),
    reiter: () => `<g transform="translate(${num(vx)} ${num(vy)}) rotate(${num(p.reiter.drehung)} 0 -50)">` +
      `${teileSvg(reiter, stil, p.reiter.winkel, { ausblenden: mitNachkommen(reiter.teile, 'oberschenkel-h') })}</g>`,
    'reittier-vorn': () => pferdTeile(reittier, p, stil, (id) => vorn.has(id)),
    zuegel: () => zuegel(reiter, reittier, p, stil),
  };
  return EBENEN.filter((n) => !nur || nur.has(n)).map((name) => ({ name, svg: svg[name]() }));
}

export function reitBild(spec: Spec, a: Ausgabe, k: Kontext): Bild {
  if (animationVon(spec, a.animation).ebene === 'reiter') return reiterEbeneBild(spec, a, k);
  const reiter = ausgabeSpec(spec, a), id = animationVon(spec, a.animation).reittier!;
  // Varianten färben auch die Satteldecke des Reittiers (B-717/AC-03, B-711): der Tausch der Variante gilt für beide.
  const geladen = ladeReittier(k.specPfad, id, k.stil);
  const reittier = a.variante ? tausche(geladen, spec.varianten![a.variante].tausch) : geladen;
  const ohne = mitNachkommen(reiter.teile, 'oberschenkel-h');
  const frames = reitZustand(reiter, reittier, a, k).map((f) => {
    const p = f.pose, R0 = rideMatrix(p.ride.dy, p.ride.pitch, 1);
    const inhalt = reitSegmente(reiter, reittier, f, k.stil).map((s) => s.svg).join('');
    return {
      svg: `<g filter="url(#paper)"><g transform="${rideText(p.ride.dy, p.ride.pitch)}">${inhalt}</g></g>`,
      geometrie: [...geometrie(reittier.teile, vierbeinerMatrizen(reittier, p)),
        ...geometrie(reiter.teile, weltMatrizen(reiter, p.reiter.winkel, mul(R0, reiterBasis(p))), ohne)],
    };
  });
  return { groesse: reitRahmen(spec, reittier), frames };
}

// ── Ebenen (B-752): Reiter und Reittier getrennt rendern, der Client setzt sie zusammen ──────────────────────────────
// Alle Ebenen teilen Koordinatensystem (Welt-Einheiten, Fußpunkt des Reittiers = 0/0) und Pixel je Einheit (die des
// Reiters); jede hat ihren eigenen engen Rahmen mit Rand für die Bewegung. Der Client legt sie nach `rahmen` übereinander.

/** Rand um die Rahmen der Ebenen in Einheiten: deckt Ride-Hub, Neigung und Reiter-Versatz ab (Test prüft leere Ränder) */
const EBENEN_RAND = 14;

/** `randX`: seitlicher Rand in Einheiten (der Umhang flattert über den Rahmen des Reiters hinaus) */
function ebenenGroesse(rahmen: [number, number, number, number], reiter: Spec, randX = EBENEN_RAND): FrameGroesse {
  const px = reiter.ausgabe.hoehe_px / reiter.ausgabe.rahmen[3], [x, y, b, h] = rahmen;
  const x0 = Math.floor(x - randX), y0 = Math.floor(y - EBENEN_RAND);
  const x1 = Math.ceil(x + b + randX), y1 = Math.ceil(y + h + EBENEN_RAND);
  return { rahmen: [x0, y0, x1 - x0, y1 - y0], frameW: Math.round((x1 - x0) * px), frameH: Math.round((y1 - y0) * px), eigen: true };
}

/** Reiter-Ebene (Umhang + Reiter): hängt von Gangart und Phase ab, nicht vom Reittier (alle teilen den Maßstab) */
function reiterEbeneBild(spec: Spec, a: Ausgabe, k: Kontext): Bild {
  const reiter = ausgabeSpec(spec, a), anim = animationVon(spec, a.animation);
  const reittier = ladeReittier(k.specPfad, anim.reittier!, k.stil);
  const [rx, ry, rb, rh] = reiter.ausgabe.rahmen;
  const frames = reitZustand(reiter, reittier, a, k).map((f) => {
    const p = f.pose, R0 = rideMatrix(p.ride.dy, p.ride.pitch, 1);
    const inhalt = reitSegmente(reiter, reittier, f, k.stil, new Set<Segment>(['umhang', 'reiter'])).map((s) => s.svg).join('');
    return {
      svg: `<g filter="url(#paper)"><g transform="${rideText(p.ride.dy, p.ride.pitch)}">${inhalt}</g></g>`,
      geometrie: geometrie(reiter.teile, weltMatrizen(reiter, p.reiter.winkel, mul(R0, reiterBasis(p))), mitNachkommen(reiter.teile, 'oberschenkel-h')),
    };
  });
  // Galopp: der Umhang reicht gemessen bis 0,35 · Länge hinter und 0,3 · Länge vor den Reiter-Rahmen
  const randX = EBENEN_RAND + (reiter.umhang ? Math.ceil(reiter.umhang.laenge * 0.4) : 0);
  return { groesse: ebenenGroesse([rx - 2, ry - 54, rb, rh], reiter, randX), frames };
}

/** Reiter, dessen Hand die Zügel hält (Pflichtangabe `reiter` der Reittier-Ebenen); Figur im selben Ordner oder in Nachbar-Gruppen */
export function ladeReiter(specPfad: string | undefined, id: string, stil: Stil): Spec {
  if (!specPfad) throw new Aufruf('Reittier-Ebenen brauchen den Pfad der Spec');
  const pfad = findeReittier(specPfad, id), fehler = checkSpecFile(pfad, stil);
  if (fehler.length) throw new Abgelehnt(fehler.map((f) => ({ ...f, feld: `${basename(pfad)}:${f.feld}` })));
  const spec = JSON.parse(readFileSync(pfad, 'utf8')) as Spec;
  if (spec.rig !== 'figur') throw abgelehnt(`„${id}“ ist keine figur (rig ${spec.rig})`);
  return spec;
}

/** Reittier-Ebene `hinten` oder `vorn` (vorn samt Zügeln des Standard-Reiters); der Tausch der Spielerfarbe gilt hier nicht */
export function reittierEbeneBild(spec: Spec, a: Ausgabe, k: Kontext): Bild {
  pruefeVierbeiner(spec);
  const s = ausgabeSpec(spec, a), anim = animationVon(s, a.animation), g = gangartVon(anim, a.animation);
  const reiter = ladeReiter(k.specPfad, anim.reiter!, k.stil), vorn = anim.ebene === 'vorn';
  const nur = new Set<Segment>(vorn ? ['reittier-vorn', 'zuegel'] : ['reittier-hinten']);
  const frames = Array.from({ length: anim.frames }, (_, n) => {
    const p = gangPose(s, g, n / anim.frames);
    const inhalt = reitSegmente(reiter, s, { pose: p }, k.stil, nur).map((x) => x.svg).join('');
    return { svg: `<g filter="url(#paper)"><g transform="${rideText(p.ride.dy, p.ride.pitch)}">${inhalt}</g></g>`, geometrie: geometrie(s.teile, vierbeinerMatrizen(s, p)) };
  });
  const m = s.ausgabe.massstab ?? 1;
  return { groesse: ebenenGroesse(s.ausgabe.rahmen.map((v) => v * m) as [number, number, number, number], reiter), frames };
}
