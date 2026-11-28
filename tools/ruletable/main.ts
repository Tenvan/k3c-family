// Einstieg von task gen:rules: `bun tools/ruletable/main.ts [-root <ordner>]` (Logik: app.ts, table.ts).

import { main } from './app';

process.exitCode = main(process.argv.slice(2));
