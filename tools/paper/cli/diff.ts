// diff (paper-spec.md › 8): Feld-Unterschiede zweier Specs; Teile über ihre id (Reihenfolge egal), `$schema` zählt nicht.
import type { Spec } from '../schema/types';

export type Feld = { pfad: string; alt: unknown; neu: unknown };

const istObjekt = (v: unknown): v is Record<string, unknown> => v !== null && typeof v === 'object' && !Array.isArray(v);

/** Objekte je Schlüssel, Listen je Index, Werte per JSON-Vergleich; `teile` auf oberster Ebene behandelt teileDiff. */
function felder(alt: unknown, neu: unknown, pfad: string, out: Feld[]) {
  if (Array.isArray(alt) && Array.isArray(neu)) {
    for (let i = 0; i < Math.max(alt.length, neu.length); i++) felder(alt[i], neu[i], `${pfad}[${i}]`, out);
  } else if (istObjekt(alt) && istObjekt(neu)) {
    const schluessel = [...new Set([...Object.keys(alt), ...Object.keys(neu)])].filter((k) => k !== '$schema' && !(pfad === '' && k === 'teile'));
    for (const k of schluessel) felder(alt[k], neu[k], pfad ? `${pfad}.${k}` : k, out);
  } else if (JSON.stringify(alt) !== JSON.stringify(neu)) out.push({ pfad, alt: alt ?? null, neu: neu ?? null });
}

/** Teile über ihre id; Pfad mit dem Index im neuen Stand (gelöschte: im alten). Liefert neue, gelöschte, geänderte. */
function teileDiff(alt: Spec, neu: Spec, out: Feld[]): string[] {
  const geaendert: string[] = [];
  for (const id of new Set([...neu.teile.map((t) => t.id), ...alt.teile.map((t) => t.id)])) {
    const ia = alt.teile.findIndex((t) => t.id === id), ineu = neu.teile.findIndex((t) => t.id === id);
    const vorher = out.length;
    felder(alt.teile[ia], neu.teile[ineu], `teile[${ineu >= 0 ? ineu : ia}]`, out);
    if (out.length > vorher) geaendert.push(id);
  }
  return geaendert;
}

export function diffSpecs(alt: Spec, neu: Spec): { felder: Feld[]; teile: string[] } {
  const out: Feld[] = [];
  felder(alt, neu, '', out);
  const teile = teileDiff(alt, neu, out);
  return { felder: out, teile };
}
