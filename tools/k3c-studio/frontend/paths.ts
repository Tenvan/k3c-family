import { fileURLToPath } from 'node:url';

// Pfade des Studio-Frontends zur Repo-Wurzel, gemeinsam für vite.config.ts und vitest.config.ts (B-459).
// Spiel-Code und Daten liest das Studio nur über die Aliase @game (src/) und @data (data/), nie relativ.

/** Repo-Wurzel, von tools/k3c-studio/frontend/ aus */
export const REPO = fileURLToPath(new URL('../../../', import.meta.url));

export const ALIAS = {
  '@game': fileURLToPath(new URL('../../../src', import.meta.url)),
  '@data': fileURLToPath(new URL('../../../data', import.meta.url)),
};

/** public/ des Spiels: liefert im Dev-Modus der Studio-Vite, in der EXE der Asset-Handler (public.go) */
export const PUBLIC = fileURLToPath(new URL('../../../public', import.meta.url));
