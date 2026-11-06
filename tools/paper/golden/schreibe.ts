// Schreibt tools/paper/golden/fixtures.golden.json neu (nur bei gewollter Änderung, Begründung in den Commit).
// Aufruf: bun tools/paper/golden/schreibe.ts
import { writeFileSync } from 'node:fs';
import { FIXTURES, GOLDEN_DATEI, goldenEintrag, rendereFixture } from './fixtures';

const golden = Object.fromEntries(FIXTURES.map((id) => [id, goldenEintrag(rendereFixture(id))]));
writeFileSync(GOLDEN_DATEI, JSON.stringify(golden, null, 2) + '\n');
console.log(JSON.stringify({ ok: true, golden }, null, 2));
