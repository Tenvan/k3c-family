import { defineConfig } from 'vitest/config';
import { ALIAS, DEDUPE } from './paths.ts';

// Tests des Studio-Frontends (task check:studio). Das Vitest der Wurzel nimmt tools/k3c-studio aus, weil es die Aliase
// @game/@data nicht kennt.
// Die Seiten-Tests rendern ganze Seiten mit Radix Themes in happy-dom und brauchen einzeln schon 1,5–3 s; mit einem
// Worker je Kern werden sie bis zu dreimal langsamer und rissen die Vorgabe von 5 s. Halb so viele Worker sind gleich
// schnell (die Tests sind CPU-gebunden), die Zeitgrenze lässt Luft für einen ausgelasteten Rechner.
export default defineConfig({
  root: import.meta.dirname,
  resolve: { alias: ALIAS, dedupe: DEDUPE },
  test: { include: ['src/**/*.test.{ts,tsx}'], maxWorkers: '50%', testTimeout: 20_000, hookTimeout: 20_000 },
});
