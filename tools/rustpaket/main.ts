// Einstieg des Paketbaus (task rust:game:build, rust:game:dist:check): `bun tools/rustpaket/main.ts lizenzen|pruefen …`
// (Logik: app.ts, lizenzen.ts, pruefen.ts). Die Crate-Liste kommt aus `cargo metadata`.

import { main } from './app';

process.exitCode = main(process.argv.slice(2));
