import { useEffect, useState } from 'react';
import { errorText } from '@webkit/lib/errors';
import { cached } from './sizeCache';
import { assetsFrame, assetsGet, assetsGroesse, assetsGroesseBasis, assetsHintergruende, assetsHintergrund, type Groesse, type Hintergrund, type Spec } from '../api';

/** Daten des Größenvergleichs (B-733): Spec, ein Frame, Spiel-Maßstab und Biom-Hintergründe, gecacht auf Modul-Ebene wie
 *  der Katalog (catalogStore.ts, Cache in sizeCache.ts); „Neu laden“ in Katalog und Review leert ihn. */

/** Eine Figur der Szene: Bild, Rahmen [x, y, Breite, Höhe] in Einheiten (Fuß bei 0/0) und Spiel-Pixel je Einheit. */
export interface Figur {
  id: string;
  bild: string;
  rahmen: [number, number, number, number];
  pxProEinheit: number;
  quelle: string;
  animation: string;
}

/** Im Vergleich steht dieses Asset rechts, dem Asset zugewandt. */
export const MONARCH = 'monarch';

/** Animation des Vergleichs: `idle`, sonst die erste ohne Reittier (eigener Rahmen), sonst die erste. */
export function vergleichsAnimation(spec: Spec): string {
  const namen = Object.keys(spec.animationen ?? {});
  if (namen.includes('idle')) return 'idle';
  return namen.find((n) => !spec.animationen?.[n]?.reittier) ?? namen[0] ?? '';
}

/** Spec und Maßstab: Arbeitsstand (leere Basis) oder Stand der Basis (B-853, ein Aufruf liefert beides). */
async function specUndGroesse(id: string, basis: string): Promise<[Spec, Groesse]> {
  if (!basis) return Promise.all([assetsGet(id), assetsGroesse(id)]);
  const g = await assetsGroesseBasis(id, basis);
  return [g.spec, g];
}

async function ladeFigur(id: string, basis: string): Promise<Figur> {
  const [spec, groesse] = await specUndGroesse(id, basis);
  const animation = vergleichsAnimation(spec);
  if (!spec.ausgabe || !animation) throw new Error(`${id}: ohne Ausgabe-Rahmen oder Animation kein Größenvergleich`);
  const frame = await assetsFrame(id, animation, 0, '', basis);
  return { id, bild: frame.bild, rahmen: spec.ausgabe.rahmen, pxProEinheit: groesse.pxProEinheit, quelle: groesse.quelle, animation };
}

/** Figur im Arbeitsstand oder, mit Basis, im Stand der Basis (Vorher-Bild, B-853). */
export const figur = (id: string, basis = '') => cached(basis ? `figur|${id}|${basis}` : `figur|${id}`, () => ladeFigur(id, basis));
export const hintergruende = () => cached('hintergruende', assetsHintergruende);
export const hintergrund = (teil: string) => cached(`hg|${teil}`, () => assetsHintergrund(teil));

/** Ergebnis eines Ladevorgangs; `wert` bleibt beim Nachladen stehen. */
export interface Geladen<T> {
  wert?: T;
  fehler?: string;
}

/** Lädt per `load` neu, sobald sich `key` ändert; leerer Schlüssel = nichts laden. */
export function useGeladen<T>(key: string, load: () => Promise<T>): Geladen<T> {
  const [state, setState] = useState<Geladen<T>>({});
  useEffect(() => {
    if (!key) {
      setState({});
      return;
    }
    let live = true;
    load().then((wert) => live && setState({ wert }), (e) => live && setState({ fehler: errorText(e) }));
    return () => {
      live = false;
    };
    // load hängt nur an key
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [key]);
  return state;
}

export type { Groesse, Hintergrund };
