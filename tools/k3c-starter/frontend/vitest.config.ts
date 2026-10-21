import { defineConfig } from 'vitest/config';
import { ALIAS, DEDUPE, WEBKIT } from './paths.ts';

// Tests des Starter-Frontends (task check:starter). Das Vitest der Wurzel nimmt tools/k3c-starter/frontend aus, weil es den
// Alias @webkit nicht kennt.
export default defineConfig({
  root: import.meta.dirname,
  resolve: { alias: ALIAS, dedupe: DEDUPE },
  server: { fs: { allow: ['.', WEBKIT] } },
  test: { include: ['src/**/*.test.{ts,tsx}'] },
});
