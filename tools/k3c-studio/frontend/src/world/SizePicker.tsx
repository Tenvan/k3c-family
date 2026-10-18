import type { PlanState } from './usePlan';

/** Eintrag ohne Chunk-Anzahl: die Länge kommt aus lengthUnits des Bioms (kein Eintrag in chunkCounts). */
export const BIOME_LENGTH = 'Biom-Länge';

/** Auswahl der Inselgröße der Stufe (B-485/AC-04, geplante-chunks.md § 7): „Biom-Länge“ oder eine Größe der Tabelle
 *  (Name und Chunk-Anzahl aus data/island-sizes.json), kein Freitext. Eine gespeicherte Anzahl außerhalb der Tabelle
 *  erscheint als eigener Eintrag, damit die Prüfung sie am Entwurf meldet. */
export function SizePicker({ plan }: { plan: PlanState }) {
  const unknown = plan.count !== 0 && !plan.sizes.some((s) => s.chunkCount === plan.count);
  return (
    <select className="wd-select" aria-label="Inselgröße" title="Chunk-Anzahl der Stufe" value={plan.count} onChange={(e) => plan.setSize(Number(e.target.value))}>
      <option value={0}>{BIOME_LENGTH}</option>
      {plan.sizes.map((s) => <option key={s.chunkCount} value={s.chunkCount}>{`${s.name} · ${s.chunkCount}`}</option>)}
      {unknown && <option value={plan.count}>{`${plan.count} (keine Inselgröße)`}</option>}
    </select>
  );
}
