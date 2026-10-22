import { defineConfig } from 'vitest/config';
import { ALIAS, DEDUPE, WEBKIT } from './paths.ts';

// Tests des k3c-workbench-Frontends (task check:dev). Das Vitest der Wurzel nimmt tools/k3c-workbench/frontend aus, weil es den
// Alias @webkit nicht kennt.
export default defineConfig({
  root: import.meta.dirname,
  resolve: { alias: ALIAS, dedupe: DEDUPE },
  server: { fs: { allow: ['.', WEBKIT] } },
  // Die Tests der gemeinsamen Bausteine (tools/webkit) laufen hier mit.
  test: { include: ['src/**/*.test.{ts,tsx}', '../../webkit/**/*.test.{ts,tsx}'] },
});
