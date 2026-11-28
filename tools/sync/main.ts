// Einstieg des Paketbaus (task rust:game:dist): `bun tools/sync/main.ts quelle=ziel …` (Logik: sync.ts).

import { main } from './sync';

process.exitCode = main(process.argv.slice(2));
