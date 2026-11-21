import type { WorldDetail, WorldIsland } from '../api';
import type { AssetEintrag } from '../api/typesAssets';

// Reine Logik des Reiters „Katalog“ im Welten-Editor (B-1216): welche Katalog-Einträge (Assets) zu einer Insel gehören.
// Das Asset-Format kennt keine Insel; die Zuordnung ist darum eine Namensregel (Vorschlag, im Ticket dokumentiert): Ein
// Asset gehört zu einer Insel, wenn es kein Biom nennt (neutral: Figuren, Gebäude, Icons …) oder ein Biom der Insel.

/** Wörter im Asset-Namen oder Pfad, die auf ein Biom deuten (Biom-ID → Stichwörter, klein). */
export const BIOME_WORDS: Readonly<Record<string, readonly string[]>> = {
  forest: ['wald', 'forest'],
  cave: ['hoehle', 'höhle', 'cave'],
  mine: ['mine', 'bergwerk'],
  ironhold: ['eisen', 'stollen', 'iron'],
  crystal: ['kristall', 'crystal'],
};

/** Biome, die der Name des Assets nennt (leer = neutral). */
export function biomesOf(a: AssetEintrag): string[] {
  const text = `${a.id} ${a.pfad}`.toLowerCase();
  return Object.keys(BIOME_WORDS).filter((b) => BIOME_WORDS[b]!.some((w) => text.includes(w)));
}

/** Gehört das Asset zur Insel? Neutral oder mit einem Biom der Insel; `island` leer = alle Inseln der Welt. */
export function belongsTo(a: AssetEintrag, detail: WorldDetail, island: string): boolean {
  const biomes = biomesOf(a);
  if (biomes.length === 0) return true;
  const isl: WorldIsland[] = island ? detail.islands.filter((i) => i.id === island) : detail.islands;
  return isl.some((i) => i.biomes.some((b) => biomes.includes(b)));
}
