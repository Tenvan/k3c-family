import { expect, test } from 'claude-code/testing'

import {
  BLAST_TEXT, MAX_LIST,
  analyze, analyzeGrep, bandText, blastScan, clean, extractSpecs, normalizeTarget, paneView, parseCommand, parseStatus, reportText, stripComments, tx,
} from './blast'
import type { BlastIo, BlastResult } from './blast'

const res = (over: Partial<BlastResult> = {}): BlastResult => ({ targets: ['src/a.ts'], affected: ['src/b.ts', 'src/c.ts', 'src/d.ts', 'src/e.ts', 'src/f.ts'], tests: ['tests/a.test.ts', 'src/b.spec.ts'], unresolved: [], incomplete: false, ...over })

test('Graph: direkte und transitive Importeure, Ziel selbst nicht betroffen', () => {
  const r = analyze({
    'src/a.ts': 'export const a = 1',
    'src/b.ts': "import { a } from './a'",
    'src/c.ts': "export * from './b'",
    'src/d.ts': "import './c'",
    'src/x.ts': "import { y } from './y'",
    'src/y.ts': '',
  }, ['src/a.ts'])
  expect(r.affected).toEqual(['src/b.ts', 'src/c.ts', 'src/d.ts'])
  expect(r.tests).toEqual([])
  expect(r.unresolved).toEqual([])
})

test('Graph: Zyklus terminiert', () => {
  const r = analyze({ 'a.ts': "import './b'", 'b.ts': "import './c'", 'c.ts': "import './a'" }, ['a.ts'])
  expect(r.affected).toEqual(['b.ts', 'c.ts'])
})

test('Graph: Index-Dateien und Verzeichnisimporte', () => {
  const r = analyze({
    'lib/util/index.ts': '',
    'lib/app.ts': "import u from './util'",
    'lib/page.tsx': "import u from './util/index'",
    'lib/dyn.ts': "const m = await import('./util')",
    'lib/cjs.js': "const u = require('./util')",
  }, ['lib/util/index.ts'])
  expect(r.affected).toEqual(['lib/app.ts', 'lib/cjs.js', 'lib/dyn.ts', 'lib/page.tsx'])
})

test('Graph: .js-Endung löst auf .ts/.tsx auf, ../ wird aufgelöst', () => {
  const r = analyze({
    'src/core.ts': '',
    'src/view.tsx': '',
    'src/deep/user.ts': "import { c } from '../core.js'\nimport v from '../view.js'",
  }, ['src/core.ts'])
  expect(r.affected).toEqual(['src/deep/user.ts'])
  expect(analyze({ 'src/view.tsx': '', 'src/deep/user.ts': "import v from '../view.js'" }, ['src/view.tsx']).affected).toEqual(['src/deep/user.ts'])
})

test('Graph: Aliase @/ und ~/ erscheinen als unaufgelöst, unbeteiligte nicht', () => {
  const r = analyze({
    'src/util/fmt.ts': '',
    'src/a.ts': "import { f } from '@/util/fmt'",
    'src/b.ts': "import { f } from '~/util/fmt.js'",
    'src/c.ts': "import x from '@/other/thing'",
  }, ['src/util/fmt.ts'])
  expect(r.affected).toEqual([])
  expect(r.unresolved).toEqual(['src/a.ts → util/fmt', 'src/b.ts → util/fmt.js'])
})

test('Graph: Importe in Kommentaren zählen nicht, Strings mit // bleiben heil', () => {
  const r = analyze({
    'a.ts': '',
    'b.ts': "// import './a'\nconst x = 1 // require('./a')",
    'c.ts': "/* import './a'\n   import x from './a' */\nconst u = 'http://x'; import './a'",
  }, ['a.ts'])
  expect(r.affected).toEqual(['c.ts'])
  expect(stripComments("a // c\nb /* x\ny */ c")).toBe('a \nb     \n     c')
  expect(extractSpecs(`import a from "./a"; export { b } from './b'; require('./c'); import('./d')`)).toEqual(['./a', './b', './c', './d'])
})

test('Graph: Nicht auflösbare und nicht relative Importe werden ignoriert', () => {
  const r = analyze({ 'a.ts': '', 'b.ts': "import fs from 'node:fs'\nimport z from './fehlt'\nimport r from 'react'" }, ['a.ts'])
  expect(r).toEqual({ affected: [], tests: [], unresolved: [] })
})

test('Testkandidaten: tests/-Ordner, __tests__, .test und .spec; Ziel-Test zählt mit', () => {
  const r = analyze({
    'src/a.ts': '',
    'tests/a.ts': "import '../src/a'",
    'src/__tests__/b.ts': "import '../a'",
    'src/a.test.ts': "import './a'",
    'src/a.spec.js': "import './a.js'",
    'src/test/c.ts': "import '../a'",
    'src/latest/d.ts': "import '../a'",
    'src/contest.ts': "import './a'",
  }, ['src/a.ts', 'src/a.test.ts'])
  expect(r.tests).toEqual(['src/__tests__/b.ts', 'src/a.spec.js', 'src/a.test.ts', 'src/test/c.ts', 'tests/a.ts'])
  expect(r.affected).toEqual(['src/contest.ts', 'src/latest/d.ts'])
})

test('analyzeGrep: gleiche Antwort aus Zeilen pfad:zeile:text', () => {
  const grep = [
    "src/b.ts:1:import { a } from './a'",
    "src/c.ts:3:} from './b'",
    "src/c.ts:7:// import './a'",
    "src/d.ts:2: * import './a'",
    "src/e.ts:4:import x from '@/a' // alias",
    'fremd/f.ts:1:import "../src/a"',
  ].join('\n')
  const r = analyzeGrep(grep, ['src/a.ts', 'src/b.ts', 'src/c.ts', 'src/d.ts', 'src/e.ts'], ['src/a.ts'])
  expect(r.affected).toEqual(['src/b.ts', 'src/c.ts'])
  expect(r.unresolved).toEqual(['src/e.ts → a'])
})

test('normalizeTarget: relativ, absolut, Ausbruch und ungültig', () => {
  expect(normalizeTarget('C:\\proj', 'src\\a.ts')).toBe('src/a.ts')
  expect(normalizeTarget('C:\\proj', './src/../src/a.ts')).toBe('src/a.ts')
  expect(normalizeTarget('C:\\proj', 'c:/Proj/src/a.ts')).toBe('src/a.ts')
  expect(normalizeTarget('/home/x', '/home/x/src/a.ts')).toBe('src/a.ts')
  expect(normalizeTarget('/home/x', '../y/a.ts')).toBe(null)
  expect(normalizeTarget('/home/x', 'src/../../a.ts')).toBe(null)
  expect(normalizeTarget('/home/x', '/home/xy/a.ts')).toBe(null)
  expect(normalizeTarget('/home/x', '/etc/passwd')).toBe(null)
  expect(normalizeTarget('C:\\proj', 'D:\\other\\a.ts')).toBe(null)
  expect(normalizeTarget('C:\\proj', 'D:a.ts')).toBe(null)
  expect(normalizeTarget('/home/x', '')).toBe(null)
  expect(normalizeTarget('/home/x', '.')).toBe(null)
  expect(normalizeTarget('/home/x', 'a\u0000b.ts')).toBe(null)
  expect(normalizeTarget('/home/x', 'a\u001b[31m.ts')).toBe(null)
})

test('clean: ANSI, OSC, Steuerzeichen und Geheimnisse', () => {
  expect(clean('src/\u001b[31mred\u001b[0m.ts')).toBe('src/red.ts')
  expect(clean('a\u001b]0;Titel\u0007b.ts')).toBe('ab.ts')
  expect(clean('a\u001b]8;;http://x\u001b\\b\u0000\r\n\u007f.ts')).toBe('ab.ts')
  expect(clean('keys/ghp_abcdefghijklmnop1234/a.ts')).toBe('keys/***/a.ts')
  expect(clean('cfg?token=abc123&x=1')).toBe('cfg?***&x=1')
})

test('Views säubern Pfade mit Steuerzeichen', () => {
  const r = res({ targets: ['src/\u001b[31mevil\u001b[0m.ts'], affected: ['a\u0007b.ts'], tests: [] })
  const text = reportText(r, [], 'de')
  expect(text).toContain('src/evil.ts')
  expect(text).toContain('ab.ts')
  expect(/[\u0000-\u0008\u000b-\u001f\u007f]/.test(text)).toBe(false)
  const values = paneView(r, [], 'de').flatMap(s => s.rows.map(x => x.value))
  expect(values.some(v => v.includes('\u001b'))).toBe(false)
})

test('parseCommand: alle Formen', () => {
  expect(parseCommand('')).toEqual({ kind: 'auto' })
  expect(parseCommand('   ')).toEqual({ kind: 'auto' })
  expect(parseCommand('help')).toEqual({ kind: 'help' })
  expect(parseCommand('HILFE')).toEqual({ kind: 'help' })
  expect(parseCommand('reset')).toEqual({ kind: 'reset' })
  expect(parseCommand('watch src/a.ts')).toEqual({ kind: 'watch', path: 'src/a.ts' })
  expect(parseCommand('Watch  "dir mit leer/a.ts" ')).toEqual({ kind: 'watch', path: 'dir mit leer/a.ts' })
  expect(parseCommand('unwatch src/a.ts')).toEqual({ kind: 'unwatch', path: 'src/a.ts' })
  expect(parseCommand('watch')).toEqual({ kind: 'help' })
  expect(parseCommand('src/a.ts')).toEqual({ kind: 'target', path: 'src/a.ts' })
  expect(parseCommand("'src/a b.ts'")).toEqual({ kind: 'target', path: 'src/a b.ts' })
  expect(parseCommand('reset foo')).toEqual({ kind: 'target', path: 'reset foo' })
})

test('Texte: de und en haben dieselben Schlüssel mit blast.-Präfix und Platzhaltern', () => {
  const de = Object.keys(BLAST_TEXT.de).sort()
  expect(de).toEqual(Object.keys(BLAST_TEXT.en).sort())
  expect(de.every(k => k.startsWith('blast.'))).toBe(true)
  for (const k of de) {
    const ph = (s: string) => [...s.matchAll(/\{(\w+)\}/g)].map(m => m[1]).sort()
    expect(ph(BLAST_TEXT.de[k as keyof typeof BLAST_TEXT.de])).toEqual(ph(BLAST_TEXT.en[k as keyof typeof BLAST_TEXT.en]))
  }
  expect(BLAST_TEXT.de['blast.title.unresolved']).toBe('Unaufgelöst')
  expect(tx('de', 'blast.more', { n: 3 })).toBe('… +3')
  expect(tx('en', 'blast.more')).toBe('… +{n}')
})

test('bandText: beide Sprachen, leer ohne Ziele, unvollständig', () => {
  expect(bandText(res(), 'de')).toBe('Blast: 5 betroffen · 2 Tests')
  expect(bandText(res(), 'en')).toBe('Blast: 5 affected · 2 tests')
  expect(bandText(res({ incomplete: true }), 'de')).toBe('Blast: 5 betroffen · 2 Tests (unvollständig)')
  expect(bandText(res({ incomplete: true }), 'en')).toBe('Blast: 5 affected · 2 tests (incomplete)')
  expect(bandText(null, 'de')).toBe('')
  expect(bandText(res({ targets: [], affected: [], tests: [] }), 'en')).toBe('')
})

test('paneView: Abschnitte, Töne, Kappung auf 20 sichtbar', () => {
  const many = Array.from({ length: 25 }, (_, i) => `src/f${i}.ts`)
  const v = paneView(res({ targets: ['src/a.ts', 'src/w.ts'], affected: many, unresolved: ['src/x.ts → a'] }), ['src/w.ts'], 'de')
  expect(v.map(s => s.title)).toEqual(['Ziele', 'Betroffene Dateien', 'Testkandidaten', 'Unaufgelöst', 'Hinweis'])
  expect(v[0]?.rows).toEqual([{ label: 'geändert', value: 'src/a.ts', tone: 'warn' }, { label: 'beobachtet', value: 'src/w.ts' }])
  expect(v[1]?.rows.length).toBe(21)
  expect(v[1]?.rows[20]).toEqual({ label: '', value: '… +5', tone: 'dim' })
  expect(v[3]?.rows[0]?.tone).toBe('warn')
  expect(v[4]?.note).toBe('Nur statische relative Importe; Aliase, Laufzeit-Importe, Framework-Auflösung manuell prüfen.')
  const en = paneView(res(), [], 'en')
  expect(en.map(s => s.title)).toEqual(['Targets', 'Affected files', 'Test candidates', 'Note'])
  expect(en[0]?.rows[0]?.label).toBe('changed')
})

test('paneView: ohne Ergebnis, ohne Ziele und unvollständig', () => {
  const none = paneView(null, ['src/w.ts'], 'en')
  expect(none.map(s => s.title)).toEqual(['Targets', 'Note'])
  expect(none[0]?.rows).toEqual([{ label: 'watched', value: 'src/w.ts' }, { label: 'Status', value: 'not scanned yet', tone: 'dim' }])
  const empty = paneView(res({ targets: [], affected: [], tests: [] }), [], 'de')
  expect(empty.map(s => s.title)).toEqual(['Ziele', 'Hinweis'])
  expect(empty[0]?.rows[0]?.tone).toBe('dim')
  const inc = paneView(res({ incomplete: true }), [], 'de')
  expect(inc[0]?.rows.some(x => x.tone === 'warn' && x.label === 'Status')).toBe(true)
  const zero = paneView(res({ affected: [], tests: [] }), [], 'de')
  expect(zero[1]?.rows).toEqual([{ label: 'keine', value: '', tone: 'dim' }])
})

test('reportText: Klartext in beiden Sprachen, höchstens 32 KiB', () => {
  const de = reportText(res({ unresolved: ['src/x.ts → a'] }), [], 'de')
  expect(de).toContain('Blast-Radius: 5 betroffen, 2 Testkandidaten')
  expect(de).toContain('Betroffene Dateien')
  expect(de).toContain('→ src/b.ts')
  expect(reportText(res({ incomplete: true }), [], 'en')).toContain('Blast radius: 5 affected, 2 test candidates (incomplete)')
  expect(reportText(null, [], 'de')).toContain('noch nicht gescannt')
  const long = 'x'.repeat(5000)
  const big = reportText(res({ targets: Array.from({ length: 30 }, (_, i) => `${long}${i}.ts`) }), [], 'de')
  expect(big.length).toBeLessThan(32 * 1024 + 1)
})

// ── blastScan mit gefaktem Io ──

type Reply = { exitCode: number; stdout: string; stderr: string }
const ok = (stdout: string): Reply => ({ exitCode: 0, stdout, stderr: '' })
function fake(opts: { status?: string; files?: string[]; grep?: Reply | 'throw'; prefix?: string; texts?: Record<string, string> }) {
  const calls: string[][] = []
  const reads: string[] = []
  const io: BlastIo = {
    cwd: '/proj',
    read: async p => {
      reads.push(p)
      const t = opts.texts?.[p]
      if (t === undefined) throw new Error('ENOENT')
      return t
    },
    run: async argv => {
      calls.push([...argv])
      const sub = argv.find(a => ['status', 'rev-parse', 'ls-files', 'grep'].includes(a))
      if (sub === 'status') return ok(opts.status ?? '')
      if (sub === 'rev-parse') return ok(`${opts.prefix ?? ''}\n`)
      if (sub === 'ls-files') return ok((opts.files ?? []).join('\0'))
      if (opts.grep === 'throw') throw new Error('kein git')
      return opts.grep ?? { exitCode: 1, stdout: '', stderr: '' }
    },
  }
  return { io, calls, reads }
}

test('parseStatus: Umbenennung, Präfix, untracked', () => {
  const out = ' M src/a.ts\0R  src/new.ts\0src/old.ts\0?? src/n.ts\0A  other/x.ts\0'
  expect(parseStatus(out)).toEqual(['src/a.ts', 'src/new.ts', 'src/n.ts', 'other/x.ts'])
  expect(parseStatus(out, 'src/')).toEqual(['a.ts', 'new.ts', 'n.ts'])
})

test('blastScan: Status und beobachtete Pfade, ein einziger git grep', async () => {
  const f = fake({
    status: ' M src/a.ts\0?? README.md\0',
    files: ['src/a.ts', 'src/b.ts', 'src/b.test.ts', 'src/w.ts', 'src/v.ts', 'README.md'],
    grep: ok("src/b.ts:1:import { a } from './a'\nsrc/b.test.ts:1:import './b'\nsrc/v.ts:1:import './w'\nsrc/v.ts:2:import x from '@/a'\n"),
  })
  const r = await blastScan(f.io, ['src/w.ts', '../draussen.ts', ''])
  expect(r).toEqual({ targets: ['src/a.ts', 'src/w.ts'], affected: ['src/b.ts', 'src/v.ts'], tests: ['src/b.test.ts'], unresolved: ['src/v.ts → a'], incomplete: false })
  expect('at' in r).toBe(false)
  expect(f.calls.filter(c => c.includes('grep')).length).toBe(1)
  expect(f.reads).toEqual([])
  expect(f.calls.find(c => c.includes('status'))).toEqual(['git', '-c', 'core.quotePath=false', 'status', '--porcelain=v1', '-z', '--untracked-files=all'])
})

test('blastScan: Unterverzeichnis (Präfix) macht Status-Pfade cwd-relativ', async () => {
  const f = fake({ status: ' M pkg/src/a.ts\0 M sonst/b.ts\0', prefix: 'pkg/', files: ['src/a.ts', 'src/b.ts'], grep: ok("src/b.ts:1:import './a'\n") })
  const r = await blastScan(f.io, [])
  expect(r.targets).toEqual(['src/a.ts'])
  expect(r.affected).toEqual(['src/b.ts'])
})

test('blastScan: keine Ziele, kein git grep; nur Nicht-Code-Änderungen', async () => {
  const f = fake({ status: ' M README.md\0', files: ['a.ts'] })
  expect(await blastScan(f.io, [])).toEqual({ targets: [], affected: [], tests: [], unresolved: [], incomplete: false })
  expect(f.calls.some(c => c.includes('grep'))).toBe(false)
})

test('blastScan: git scheitert, Ergebnis ist unvollständig statt Fehler', async () => {
  const io: BlastIo = { cwd: '/proj', read: async () => '', run: async () => { throw new Error('kein git') } }
  const r = await blastScan(io, ['src/a.ts'])
  expect(r.incomplete).toBe(true)
  expect(r.targets).toEqual(['src/a.ts'])
  expect(r.affected).toEqual([])
})

test('blastScan: git grep scheitert, Rückfall liest Dateien einzeln', async () => {
  const f = fake({
    status: ' M a.ts\0',
    files: ['a.ts', 'b.ts', 'weg.ts'],
    grep: { exitCode: 128, stdout: '', stderr: 'fatal' },
    texts: { 'a.ts': '', 'b.ts': "import './a'" },
  })
  const r = await blastScan(f.io, [])
  expect(r.affected).toEqual(['b.ts'])
  expect(f.reads).toEqual(['a.ts', 'b.ts', 'weg.ts'])
  // weg.ts war nicht lesbar: unvollständig
  expect(r.incomplete).toBe(true)
})

test('blastScan: mehr als 2.000 Dateien und mehr als 200 Treffer, incomplete', async () => {
  const files = ['t.ts', ...Array.from({ length: 2100 }, (_, i) => `d/f${String(i).padStart(4, '0')}.ts`)]
  const grep = files.slice(1).map(p => `${p}:1:import '../t'`).join('\n')
  const r = await blastScan(fake({ status: ' M t.ts\0', files, grep: ok(grep) }).io, [])
  expect(r.incomplete).toBe(true)
  expect(r.affected.length).toBe(MAX_LIST)
  expect(r.affected[0]).toBe('d/f0000.ts')
})

test('blastScan: genau 200 Treffer sind noch vollständig', async () => {
  const files = ['t.ts', ...Array.from({ length: 200 }, (_, i) => `d/f${i}.ts`)]
  const grep = files.slice(1).map(p => `${p}:1:import '../t'`).join('\n')
  const r = await blastScan(fake({ status: ' M t.ts\0', files, grep: ok(grep) }).io, [])
  expect(r.affected.length).toBe(200)
  expect(r.incomplete).toBe(false)
})

test('blastScan: Ausgabe über 2 MiB wird abgeschnitten und als unvollständig gemeldet', async () => {
  const pad = `d/p.ts:1:${'x'.repeat(2 * 1024 * 1024)}\n`
  const r = await blastScan(fake({ status: ' M t.ts\0', files: ['t.ts', 'd/p.ts', 'd/q.ts'], grep: ok(`${pad}d/q.ts:1:import '../t'\n`) }).io, [])
  expect(r.incomplete).toBe(true)
  expect(r.affected).toEqual([])
})
