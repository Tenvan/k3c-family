// Einstieg von task check:diff: `bun tools/checkdiff/main.ts [-base <ref>] [-plan] [-only js|rust]` (Logik: app.ts).

import { main } from './app';

// exitCode statt exit: so geht keine gepufferte Ausgabe verloren.
void main(process.argv.slice(2)).then((code) => {
  process.exitCode = code;
});
