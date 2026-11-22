// Platzhalter-Ersetzung der Prompt-Vorlagen (docs/standards/paper-mcp.md › 3), gemeinsam für Katalog (WB1.3) und
// Review (RV1, Sprint-README WB1 › D-02). Reine Funktionen; nie ein halb gefüllter Prompt.
import type { AssetEintrag, Spec, Vorlage } from '../api';

export const PLATZHALTER = ['asset', 'rig', 'teil', 'pfad', 'zustand', 'basis', 'eingabe', 'material', 'ton', 'slot', 'datei', 'pack', 'lizenz', 'treffer'] as const;
export type Platzhalter = (typeof PLATZHALTER)[number];
export type Werte = Partial<Record<Platzhalter, string | undefined>>;

const MUSTER = /\{([a-z]+)\}/g;

/** Setzt die Werte ein; wirft bei fehlendem Pflichtwert (`teil` bei Ort `teil`, `eingabe` bei Ort `review`), fehlendem
 *  Wert eines benutzten Platzhalters oder unbekanntem Platzhalter. */
export function fillPrompt(vorlage: Vorlage, werte: Werte): string {
  const leer = (k: Platzhalter) => !werte[k]?.trim();
  if (vorlage.ort === 'teil' && leer('teil')) throw new Error('Teil wählen: Klick in die Vorschau');
  if (vorlage.ort === 'review' && leer('eingabe')) throw new Error('Eingabe fehlt');
  for (const [, name] of vorlage.text.matchAll(MUSTER)) {
    if (!(PLATZHALTER as readonly string[]).includes(name)) throw new Error(`Vorlage ${vorlage.id}: unbekannter Platzhalter {${name}}`);
    if (leer(name as Platzhalter)) throw new Error(`Vorlage ${vorlage.id}: Wert für {${name}} fehlt`);
  }
  return vorlage.text.replace(MUSTER, (_, name: Platzhalter) => werte[name]!.trim());
}

/** Kurzer Zustand für `{zustand}`: „gültig, Version n“ oder die Regelverstöße `feld · regel`. */
export function zustandVon(eintrag: AssetEintrag | undefined, spec: Spec): string {
  if (eintrag && !eintrag.gueltig) {
    return `ungültig: ${(eintrag.fehler ?? []).map((f) => `${f.feld} · ${f.regel}`).join('; ')}`;
  }
  return `gültig, Version ${spec.version}`;
}
