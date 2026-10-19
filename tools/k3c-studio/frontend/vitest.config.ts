import { defineConfig } from 'vitest/config';
import { ALIAS, DEDUPE } from './paths.ts';

// Tests des Studio-Frontends (task check:studio). Das Vitest der Wurzel nimmt tools/k3c-studio aus, weil es die Aliase
// @game/@data nicht kennt.
export default defineConfig({
  root: import.meta.dirname,
  resolve: { alias: ALIAS, dedupe: DEDUPE },
  test: { include: ['src/**/*.test.{ts,tsx}'] },
});
