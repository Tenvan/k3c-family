// Werte der Prompt-Vorlagen am Review-Eintrag (docs/standards/paper-mcp.md › 3); das Einsetzen übernimmt fillPrompt aus
// assets/prompts/fill.ts, gemeinsam mit dem Katalog (B-703).
import type { ReviewAsset } from '../api';
import type { Werte } from '../prompts/fill';

const wert = (v: unknown) => (typeof v === 'string' ? v : JSON.stringify(v));

/** Feldänderungen kurz: `pfad: alt → neu`, mit `; ` verbunden. */
export const felderText = (a: ReviewAsset) => a.felder.map((f) => `${f.pfad}: ${wert(f.alt)} → ${wert(f.neu)}`).join('; ');

const STATUS = { neu: 'neu', geaendert: 'geändert', geloescht: 'gelöscht', ungueltig: 'ungültig' } as const;

/** Zustand des Eintrags für `{zustand}`: Status, Feldänderungen, Stiländerung, Regelverstöße. */
export function zustandVon(a: ReviewAsset): string {
  const teile: string[] = [STATUS[a.status]];
  if (a.felder.length) teile.push(felderText(a));
  if (a.stil) teile.push('Stil geändert');
  if (a.fehler?.length) teile.push(a.fehler.map((f) => f.text).join('; '));
  return teile.join(', ');
}

/** Werte eines Review-Eintrags für die Vorlagen mit Ort `review`. */
export function reviewWerte(a: ReviewAsset, basis: string, eingabe: string): Werte {
  return { asset: a.id, rig: a.rig, pfad: a.pfad, basis, eingabe, zustand: zustandVon(a) };
}
