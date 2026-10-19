import { fileURLToPath } from 'node:url';

// Pfade des Studio-Frontends zur Repo-Wurzel, gemeinsam für vite.config.ts und vitest.config.ts (B-459).
// Spiel-Code und Daten liest das Studio nur über die Aliase @game (src/) und @data (data/), nie relativ.

/** Repo-Wurzel, von tools/k3c-studio/frontend/ aus */
export const REPO = fileURLToPath(new URL('../../../', import.meta.url));

export const ALIAS = {
  '@game': fileURLToPath(new URL('../../../src', import.meta.url)),
  '@data': fileURLToPath(new URL('../../../data', import.meta.url)),
  '@webkit': fileURLToPath(new URL('../../webkit', import.meta.url)),
};

/** Pakete, die auch Dateien außerhalb des Frontends (tools/webkit) aus diesem node_modules beziehen */
export const DEDUPE = ['react', 'react-dom', '@radix-ui/themes'];

/** public/ des Spiels: liefert im Dev-Modus der Studio-Vite, sonst der Spielserver aus dist/ */
export const PUBLIC = fileURLToPath(new URL('../../../public', import.meta.url));

/** Ausgabe des Builds: der Studio-Teil von dist/, den der Go-Server unter /studio/ ausliefert */
export const DIST = fileURLToPath(new URL('../../../dist/studio', import.meta.url));
