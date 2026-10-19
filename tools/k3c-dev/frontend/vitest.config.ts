import { defineConfig } from 'vitest/config';
import { ALIAS, DEDUPE } from './paths.ts';

// Tests des k3c-dev-Frontends (task check:dev). Das Vitest der Wurzel nimmt tools/k3c-dev/frontend aus, weil es den
// Alias @webkit nicht kennt.
export default defineConfig({
  root: import.meta.dirname,
  resolve: { alias: ALIAS, dedupe: DEDUPE },
  test: { include: ['src/**/*.test.{ts,tsx}'] },
});
