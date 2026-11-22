import { defineConfig } from 'vitest/config';
import { ALIAS, DEDUPE, MOCK_DATA, WEBKIT } from './paths.ts';

// Tests des k3c-studio-Frontends (task k3c-studio:vitest). Das Vitest der Wurzel nimmt tools/*/frontend aus, weil es den
// Alias @webkit nicht kennt; die Tests von tools/webkit laufen im Vitest von k3c-workbench.
export default defineConfig({
  root: import.meta.dirname,
  resolve: { alias: ALIAS, dedupe: DEDUPE },
  server: { fs: { allow: ['.', WEBKIT, ...MOCK_DATA] } },
  // Zeitlimit über dem Standard (5 s): unter paralleler Last warten die Katalog-Tests auf Bedingungen (B-837), ein Test mit
  // Abspielen braucht dann mehr als 5 s, ohne zu hängen.
  test: { include: ['src/**/*.test.{ts,tsx}'], testTimeout: 20_000 },
});
