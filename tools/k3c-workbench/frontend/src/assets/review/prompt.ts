// Platzhalter der Prompt-Vorlagen am Review-Eintrag (docs/standards/paper-mcp.md › 3). Gleiche Regeln wie
// assets/prompts/fill.ts aus WB1 (nie ein halb gefüllter Prompt); nach dem Zusammenführen dorthin umziehen (B-701).
import type { ReviewAsset, Vorlage } from './types';

export const PLATZHALTER = ['asset', 'rig', 'teil', 'pfad', 'zustand', 'basis', 'eingabe'] as const;
export type Platzhalter = (typeof PLATZHALTER)[number];
export type Werte = Partial<Record<Platzhalter, string | undefined>>;

const MUSTER = /\{([a-z]+)\}/g;

/** Setzt die Werte ein; wirft bei fehlender Eingabe (Ort `review`), fehlendem Wert oder unbekanntem Platzhalter. */
export function fillPrompt(vorlage: Vorlage, werte: Werte): string {
  const leer = (k: Platzhalter) => !werte[k]?.trim();
  if (vorlage.ort === 'teil' && leer('teil')) throw new Error('Teil wählen: Klick in die Vorschau');
  if (vorlage.ort === 'review' && leer('eingabe')) throw new Error('Eingabe fehlt');
  for (const [, name] of vorlage.text.matchAll(MUSTER)) {
    if (!(PLATZHALTER as readonly string[]).includes(name!)) throw new Error(`Vorlage ${vorlage.id}: unbekannter Platzhalter {${name}}`);
    if (leer(name as Platzhalter)) throw new Error(`Vorlage ${vorlage.id}: Wert für {${name}} fehlt`);
  }
  return vorlage.text.replace(MUSTER, (_, name: Platzhalter) => werte[name]!.trim());
}

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
