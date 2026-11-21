import { iconOf, nameOf } from './kindIcons';

// Symbol und deutscher Name je Basis-Datei und Thema der Werte (B-1216, Baum der Ansicht „Werte“). Reine Zuordnung, kein
// DOM; unbekannte Dateien und Themen zeigen ihren technischen Namen mit einem neutralen Symbol. Emojis aus einem
// Codepunkt (kein U+FE0F).

const FILES: Readonly<Record<string, readonly [string, string]>> = {
  buildings: ['🏗', 'Gebäude'], economy: ['🪙', 'Wirtschaft'], troops: ['⚔', 'Truppen'], enemies: ['👹', 'Gegner'],
  waves: ['🌙', 'Wellen'], bosses: ['👑', 'Bosse'], hub: ['🏰', 'Hub'], monarch: ['🤴', 'Monarch'], difficulty: ['🎚', 'Schwierigkeit'],
  coop: ['🤝', 'Koop'], goals: ['🎯', 'Ziele'], guards: ['🛡', 'Wachen'], hermits: ['🧙', 'Einsiedler'], events: ['🎲', 'Ereignisse'],
  festival: ['🎉', 'Fest'], cargo: ['📦', 'Fracht'], ship: ['⛵', 'Schiff'], mounts: ['🐴', 'Reittiere'], ruins: ['🏚', 'Ruinen'],
  light: ['💡', 'Licht'], weather: ['⛈', 'Wetter'], look: ['🎨', 'Aussehen'], render: ['🖼', 'Darstellung'], sizes: ['📏', 'Größen'],
  paper: ['✂', 'Papierschnitt'], sounds: ['🔊', 'Töne'], story: ['📖', 'Geschichte'], native: ['🧱', 'Native Regeln'], stresstests: ['🧪', 'Belastungstests'],
};

/** Themen (zweiter Pfadteil) von Gebäuden, Truppen und Gegnern, die kindIcons nicht kennt. */
const GROUPS: Readonly<Record<string, readonly [string, string]>> = {
  wall: ['🧱', 'Mauer'], tower: ['🗼', 'Turm'], workshop: ['🔧', 'Werkstatt'], bowyer: ['🏹', 'Bogner'], storage: ['📦', 'Lager'],
  nursery: ['🌱', 'Baumschule'], barracks: ['🏕', 'Kaserne'], stairsUp: ['🔼', 'Treppe auf'], stairsDown: ['🔽', 'Treppe ab'],
  tavern: ['🍺', 'Taverne'], healer: ['⛑', 'Heiler'], smithy: ['⚒', 'Schmiede'], armory: ['🛡', 'Rüstkammer'],
  archer: ['🏹', 'Bogenschütze'], warrior: ['🗡', 'Krieger'], eliteArcher: ['🎯', 'Elite-Bogenschütze'], eliteWarrior: ['⚔', 'Elite-Krieger'],
  peasant: ['🧑', 'Bauer'], vagrant: ['🚶', 'Landstreicher'], citizen: ['🧑', 'Bürger'], craftsman: ['🔨', 'Handwerker'],
  allgemein: ['📋', 'allgemein'], purse: ['👛', 'Geldbeutel'], veins: ['⛏', 'Adern'], merchant: ['🧳', 'Händler'],
  gatherables: ['🌾', 'Sammelbares'], chestFountain: ['⛲', 'Truhen-Fontäne'], recruitCamp: ['⛺', 'Camp'], enemyResourceDrop: ['💧', 'Gegner-Beute'],
};

const entry = (table: Readonly<Record<string, readonly [string, string]>>, key: string) => (Object.hasOwn(table, key) ? table[key] : undefined);

export const fileIcon = (file: string): string => entry(FILES, file)?.[0] ?? '📄';
export const fileName = (file: string): string => entry(FILES, file)?.[1] ?? file;
export const groupIcon = (group: string): string => entry(GROUPS, group)?.[0] ?? iconOf(group);
export const groupName = (group: string): string => entry(GROUPS, group)?.[1] ?? nameOf(group);
