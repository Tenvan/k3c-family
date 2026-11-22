// Symbole und Namen der Asset-Gruppen und Rig-Arten für den Baum des Katalogs (B-1216). Reine Zuordnung, kein DOM;
// unbekannte Gruppen und Arten zeigen ein neutrales Symbol und ihren technischen Namen. Emojis aus einem Codepunkt.

const GROUPS: Readonly<Record<string, readonly [string, string]>> = {
  bosse: ['👑', 'Bosse'], figuren: ['🧑', 'Figuren'], 'gebaeude-paper': ['🏠', 'Gebäude'], gebaeude: ['🏠', 'Gebäude'],
  gegner: ['👹', 'Gegner'], hintergruende: ['🖼', 'Hintergründe'], icons: ['🔹', 'Icons'], objekte: ['🌳', 'Objekte'],
  reittiere: ['🐴', 'Reittiere'], reittiere2: ['🐴', 'Reittiere'], truppen: ['⚔', 'Truppen'], requisiten: ['🚩', 'Requisiten'],
};

const RIGS: Readonly<Record<string, string>> = { figur: '🧑', vierbeiner: '🐴', gebaeude: '🏠', requisit: '🚩', objekt: '🌳', hintergrund: '🖼' };

const has = (t: Readonly<Record<string, unknown>>, k: string) => Object.hasOwn(t, k);

export const groupIcon = (gruppe: string): string => (has(GROUPS, gruppe) ? GROUPS[gruppe]![0] : '📦');
export const groupName = (gruppe: string): string => (has(GROUPS, gruppe) ? GROUPS[gruppe]![1] : gruppe);
export const rigIcon = (rig: string): string => (has(RIGS, rig) ? RIGS[rig]! : '🔸');
