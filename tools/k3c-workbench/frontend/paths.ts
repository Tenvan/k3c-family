import { fileURLToPath } from 'node:url';

// Gemeinsame Auflösung für vite.config.ts und vitest.config.ts: Bausteine aus tools/webkit über @webkit.

export const ALIAS = { '@webkit': fileURLToPath(new URL('../../webkit', import.meta.url)) };

/** Pakete, die auch Dateien außerhalb des Frontends (tools/webkit) aus diesem node_modules beziehen */
export const DEDUPE = ['react', 'react-dom', '@radix-ui/themes'];

/** tools/webkit liegt außerhalb des Frontends; der Dev-Server darf es ausliefern. */
export const WEBKIT = ALIAS['@webkit'];

/** Dateien außerhalb des Frontends, die die Mocks lesen: keine mehr, seit Assets (WZ63.3) und Balancing (WZ63.6) im Studio liegen. */
export const MOCK_DATA: string[] = [];
