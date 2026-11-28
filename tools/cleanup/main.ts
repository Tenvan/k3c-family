// Einstieg von task ops:aufraeumen: `bun tools/cleanup/main.ts [-root <ordner>] [-dry]` (Logik: app.ts).

import { main } from './app';

process.exitCode = main(process.argv.slice(2));
