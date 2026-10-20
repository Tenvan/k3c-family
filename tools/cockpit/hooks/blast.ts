// Blast-Radius: Wer importiert die geänderte Datei, transitiv? Liefert betroffene Dateien und Testkandidaten.
// Rein beobachtend, nur Text (Desktop und Terminal). Ziele sind die geänderten Dateien laut `git status` plus vom Nutzer
// beobachtete Pfade (`/cockpit-blast watch <pfad>`).
//
// Verdrahtung (register.tsx, `$` verlässt das Modul nicht): der Aufrufer baut das `BlastIo` aus der Engine-API
//   const io: BlastIo = {
//     cwd: e.cwd,
//     read: path => $.fs.read(path),                       // Text; lehnt ab bei fehlender Datei oder > 4 MiB
//     run: argv => $.process.run(argv, { cwd: e.cwd, timeoutMs: 10_000 }),   // ohne Shell, { exitCode, stdout, stderr }
//   }
// und ruft `blastScan(io, watched)`; den Zeitstempel setzt der Aufrufer. `$.fs` hat keine Batch-Variante (nur read, list,
// stat), 2.000 Einzel-Reads wären teuer: deshalb holt `blastScan` die Importzeilen mit EINEM `git grep` (plus je ein Aufruf
// für Status, Dateiliste und Verzeichnis-Präfix) und nutzt `read` nur als Rückfall, wenn `git grep` scheitert.
//
// Grenzen: nur statische relative JS/TS-Importe (import/export … from, import 'x', require('x'), import('x') mit
// String-Literal); `.js`→`.ts`-Auflösung und Index-Dateien; `@/`- und `~/`-Aliase, tsconfig `paths`, Laufzeit-Importe und
// Framework-Auflösung bleiben unbeachtet (Aliase werden als „unaufgelöst“ gemeldet). Der git-grep-Weg arbeitet zeilenweise:
// Block-Kommentare über mehrere Zeilen werden nur erkannt, wenn die Folgezeilen mit `*` beginnen.

import type { UiLang } from '../types'

export type BlastIo = {
  cwd: string
  read(path: string): Promise<string>
  run(argv: readonly string[]): Promise<{ exitCode: number; stdout: string; stderr: string }>
}
export type BlastResult = { targets: string[]; affected: string[]; tests: string[]; unresolved: string[]; incomplete: boolean; at?: never }
export type BlastSection = { title: string; rows: { label: string; value: string; tone?: 'ok' | 'warn' | 'bad' | 'dim' }[]; note?: string }
export type BlastCommand =
  | { kind: 'help' } | { kind: 'reset' } | { kind: 'auto' }
  | { kind: 'watch'; path: string } | { kind: 'unwatch'; path: string } | { kind: 'target'; path: string }

export const MAX_FILES = 2000
export const MAX_BYTES = 2 * 1024 * 1024
export const MAX_LIST = 200
export const MAX_VISIBLE = 20
export const MAX_REPORT = 32 * 1024

const CODE = /\.[cm]?[jt]sx?$/
const TEST = /(^|\/)(tests?|__tests__)\/|\.(test|spec)\./
const isTest = (p: string): boolean => TEST.test(p)

// ── Pfade ──

// Projekt-relativer Pfad mit `/`, oder null bei leer, Steuerzeichen, Laufwerks-Relativpfad und Ausbruch aus `cwd`.
export function normalizeTarget(cwd: string, input: string): string | null {
  if (/[\u0000-\u001f\u007f]/.test(input)) return null
  let p = input.trim().replace(/\\/g, '/')
  if (p === '' || /^[a-zA-Z]:(?!\/)/.test(p)) return null
  if (/^([a-zA-Z]:)?\//.test(p)) {
    const root = cwd.replace(/\\/g, '/').replace(/\/+$/, '')
    const fold = (s: string): string => (/^[a-zA-Z]:/.test(s) ? s.toLowerCase() : s)
    if (!fold(p).startsWith(`${fold(root)}/`)) return null
    p = p.slice(root.length + 1)
  }
  const out: string[] = []
  for (const seg of p.split('/')) {
    if (seg === '' || seg === '.') continue
    if (seg === '..') {
      if (out.pop() === undefined) return null
    } else out.push(seg)
  }
  return out.length === 0 ? null : out.join('/')
}

// Anzeige: ANSI/OSC-Sequenzen, Steuerzeichen und offensichtliche Geheimnisse raus (Best Effort).
const SECRET = /\b(?:ghp_|gho_|ghs_|github_pat_|glpat-|xox[abprs]-|sk-|AKIA)[A-Za-z0-9_-]{8,}|(?:password|passwd|token|secret|api[_-]?key)=[^\s/&]+/gi
export function clean(s: string): string {
  return s
    .replace(/\u001b\][^\u0007\u001b]*(?:\u0007|\u001b\\)?/g, '')
    .replace(/\u001b\[[0-9;?]*[ -/]*[@-~]/g, '')
    .replace(/\u001b[@-Z\\-_]/g, '')
    .replace(/[\u0000-\u001f\u007f-\u009f]/g, '')
    .replace(SECRET, '***')
}

// ── Importe lesen ──

// Ersetzt Kommentare durch Leerraum (Zeilenumbrüche bleiben); Strings und Template-Literale bleiben unberührt.
// ponytail: Regex-Literale kennt der Scanner nicht, ein `//` oder Anführungszeichen darin kann ihn verwirren.
export function stripComments(src: string): string {
  let out = ''
  let i = 0
  while (i < src.length) {
    const c = src[i]!
    const n = src[i + 1]
    if (c === '/' && n === '/') {
      while (i < src.length && src[i] !== '\n') i++
    } else if (c === '/' && n === '*') {
      const end = src.indexOf('*/', i + 2)
      const stop = end < 0 ? src.length : end + 2
      out += src.slice(i, stop).replace(/[^\n]/g, ' ')
      i = stop
    } else if (c === '"' || c === "'" || c === '`') {
      let j = i + 1
      while (j < src.length && src[j] !== c && !(c !== '`' && src[j] === '\n')) j += src[j] === '\\' ? 2 : 1
      out += src.slice(i, j + 1)
      i = j + 1
    } else {
      out += c
      i++
    }
  }
  return out
}

const SPEC = /(?:\bfrom\s*|\bimport\s*\(\s*|\brequire\s*\(\s*|\bimport\s+)(['"])([^'"\n]+)\1/g

export function extractSpecs(src: string): string[] {
  const out: string[] = []
  for (const m of stripComments(src).matchAll(SPEC)) out.push(m[2]!)
  return out
}

const isRelative = (s: string): boolean => s === '.' || s === '..' || s.startsWith('./') || s.startsWith('../')
const aliasRest = (s: string): string | null => (s.startsWith('@/') || s.startsWith('~/') ? s.slice(2) : null)

const EXT = ['.ts', '.tsx', '.js', '.jsx', '.mts', '.cts', '.mjs', '.cjs']
function resolveSpec(importer: string, spec: string, known: ReadonlySet<string>): string | null {
  const out = importer.split('/').slice(0, -1)
  for (const seg of spec.split('/')) {
    if (seg === '' || seg === '.') continue
    if (seg === '..') {
      if (out.pop() === undefined) return null
    } else out.push(seg)
  }
  const base = out.join('/')
  if (base === '') return null
  const stem = base.replace(/\.(?:js|jsx|mjs|cjs)$/, '')
  const swapped = stem === base ? [] : ['.ts', '.tsx', '.mts', '.cts'].map(e => stem + e)
  const found = [base, ...swapped, ...EXT.map(e => base + e), ...EXT.map(e => `${base}/index${e}`)]
  return found.find(c => known.has(c)) ?? null
}

const stemOf = (p: string): string => p.replace(/\.[cm]?[jt]sx?$/, '').replace(/\/index$/, '')

// ── Graph ──

type Imports = ReadonlyMap<string, readonly string[]>

function graph(imports: Imports, known: ReadonlySet<string>, changed: readonly string[]): { affected: string[]; tests: string[]; unresolved: string[] } {
  const rev = new Map<string, Set<string>>()
  const aliases: [string, string][] = []
  for (const [file, specs] of imports) {
    for (const spec of specs) {
      const rest = aliasRest(spec)
      if (rest !== null) aliases.push([file, rest])
      else if (isRelative(spec)) {
        const to = resolveSpec(file, spec, known)
        if (to !== null && to !== file) (rev.get(to) ?? rev.set(to, new Set()).get(to)!).add(file)
      }
    }
  }
  const targets = new Set(changed)
  const seen = new Set<string>(changed)
  const queue = [...changed]
  for (let q = 0; q < queue.length; q++) {
    for (const importer of rev.get(queue[q]!) ?? []) {
      if (seen.has(importer)) continue
      seen.add(importer)
      queue.push(importer)
    }
  }
  const hit = [...seen].sort()
  // Ein Alias kann auf ein Ziel zeigen, wenn Pfadende und Importtext übereinstimmen; nur solche melden.
  const stems = changed.map(stemOf)
  const unresolved = [...new Set(aliases.filter(([, rest]) => {
    const r = stemOf(rest.replace(/\.(?:js|jsx|mjs|cjs)$/, ''))
    return stems.some(s => s === r || s.endsWith(`/${r}`))
  }).map(([file, rest]) => `${file} → ${rest}`))].sort()
  return {
    affected: hit.filter(p => !targets.has(p) && !isTest(p)),
    tests: hit.filter(p => isTest(p)),
    unresolved,
  }
}

// Rein: Dateitexte (Pfad → Inhalt) und geänderte Pfade → betroffene Dateien, Testkandidaten, unaufgelöste Aliase.
export function analyze(files: Record<string, string>, changed: string[]): { affected: string[]; tests: string[]; unresolved: string[] } {
  const imports = new Map<string, string[]>()
  for (const [p, text] of Object.entries(files)) imports.set(p, extractSpecs(text))
  return graph(imports, new Set(Object.keys(files)), changed)
}

// Rein: Ausgabe von `git grep -n` (Zeilen `pfad:zeile:text`) und Dateiliste → dasselbe Ergebnis wie `analyze`.
export function analyzeGrep(grepOut: string, fileList: readonly string[], changed: string[]): { affected: string[]; tests: string[]; unresolved: string[] } {
  const known = new Set(fileList)
  const imports = new Map<string, string[]>()
  for (const line of grepOut.split(/\r?\n/)) {
    const m = /^(.+?):\d+:(.*)$/.exec(line)
    if (m === null || !known.has(m[1]!)) continue
    const text = m[2]!.trim()
    if (text.startsWith('*') || text.startsWith('/*')) continue
    const specs = extractSpecs(text)
    if (specs.length > 0) imports.set(m[1]!, [...(imports.get(m[1]!) ?? []), ...specs])
  }
  return graph(imports, known, changed)
}

// ── Git ──

// `git status --porcelain=v1 -z`: `XY pfad\0`, bei Umbenennung/Kopie folgt der alte Pfad als eigener Eintrag.
export function parseStatus(out: string, prefix = ''): string[] {
  const parts = out.split('\0')
  const paths: string[] = []
  for (let i = 0; i < parts.length; i++) {
    const e = parts[i]!
    if (e.length < 4) continue
    if (e[0] === 'R' || e[0] === 'C' || e[1] === 'R' || e[1] === 'C') i++
    const p = e.slice(3)
    if (p.startsWith(prefix)) paths.push(p.slice(prefix.length))
  }
  return paths
}

const GREP = '(from|require[[:space:]]*\\(|import[[:space:]]*\\()[[:space:]]*[\'"]|^[[:space:]]*import[[:space:]]+[\'"]'
const GIT = ['git', '-c', 'core.quotePath=false'] as const

export async function blastScan(io: BlastIo, targets: readonly string[]): Promise<BlastResult> {
  const run = (argv: readonly string[]) => io.run(argv).catch(() => null)
  const [status, prefixRun, ls] = await Promise.all([
    run([...GIT, 'status', '--porcelain=v1', '-z', '--untracked-files=all']),
    run(['git', 'rev-parse', '--show-prefix']),
    run(['git', 'ls-files', '-z', '--cached', '--others', '--exclude-standard']),
  ])
  let incomplete = status === null || status.exitCode !== 0 || ls === null || ls.exitCode !== 0
  const prefix = prefixRun?.exitCode === 0 ? prefixRun.stdout.trim() : ''
  const watched = targets.map(t => normalizeTarget(io.cwd, t)).filter((p): p is string => p !== null)
  const changed = status?.exitCode === 0 ? parseStatus(status.stdout, prefix).map(p => normalizeTarget(io.cwd, p)).filter((p): p is string => p !== null && CODE.test(p)) : []
  const all = [...new Set([...changed, ...watched])].sort()
  if (all.length === 0) return { targets: [], affected: [], tests: [], unresolved: [], incomplete }

  const listed = ls?.exitCode === 0 ? ls.stdout.split('\0').filter(p => p !== '' && CODE.test(p) && !p.includes('node_modules/')) : []
  if (listed.length > MAX_FILES) incomplete = true
  const files = listed.slice(0, MAX_FILES)

  let found: ReturnType<typeof analyze> | null = null
  const grep = await run([...GIT, 'grep', '-n', '-I', '-E', '--untracked', '--no-color', '-e', GREP, '--', '*.ts', '*.tsx', '*.js', '*.jsx', '*.mts', '*.cts', '*.mjs', '*.cjs'])
  if (grep !== null && grep.exitCode <= 1) {
    let out = grep.stdout
    if (out.length > MAX_BYTES) {
      out = out.slice(0, Math.max(0, out.lastIndexOf('\n', MAX_BYTES)))
      incomplete = true
    }
    found = analyzeGrep(out, files, all)
  } else {
    // Rückfall: Dateien einzeln lesen, bis 2 MiB erreicht sind.
    const texts: Record<string, string> = {}
    let size = 0
    for (const p of files) {
      if (size >= MAX_BYTES) {
        incomplete = true
        break
      }
      const text = await io.read(p).catch(() => null)
      if (text === null) incomplete = true
      else {
        texts[p] = text
        size += text.length
      }
    }
    found = analyze(texts, all)
  }

  const cap = (l: string[]): string[] => {
    if (l.length > MAX_LIST) incomplete = true
    return l.slice(0, MAX_LIST)
  }
  return { targets: cap(all), affected: cap(found.affected), tests: cap(found.tests), unresolved: cap(found.unresolved), incomplete }
}

// ── Kommando ──

export function parseCommand(args: string): BlastCommand {
  const s = args.trim()
  if (s === '') return { kind: 'auto' }
  const m = /^(\S+)(?:\s+([\s\S]*))?$/.exec(s)
  const word = (m?.[1] ?? '').toLowerCase()
  const rest = (m?.[2] ?? '').trim().replace(/^(["'])(.*)\1$/, '$2')
  if (word === 'help' || word === 'hilfe' || word === '?' || word === '--help') return { kind: 'help' }
  if (word === 'reset' && rest === '') return { kind: 'reset' }
  if (word === 'watch' || word === 'unwatch') return rest === '' ? { kind: 'help' } : { kind: word, path: rest }
  return { kind: 'target', path: s.replace(/^(["'])(.*)\1$/, '$2') }
}

// ── Texte ──

const de = {
  'blast.band': 'Blast: {n} betroffen · {t} Tests',
  'blast.incomplete': ' (unvollständig)',
  'blast.title.targets': 'Ziele',
  'blast.title.affected': 'Betroffene Dateien',
  'blast.title.tests': 'Testkandidaten',
  'blast.title.unresolved': 'Unaufgelöst',
  'blast.title.hint': 'Hinweis',
  'blast.label.changed': 'geändert',
  'blast.label.watched': 'beobachtet',
  'blast.label.status': 'Status',
  'blast.label.none': 'keine',
  'blast.value.notScanned': 'noch nicht gescannt',
  'blast.value.incomplete': 'unvollständig (Grenzen erreicht oder Git-Fehler)',
  'blast.value.noTargets': 'keine Ziele: nichts geändert, nichts beobachtet',
  'blast.more': '… +{n}',
  'blast.hint': 'Nur statische relative Importe; Aliase, Laufzeit-Importe, Framework-Auflösung manuell prüfen.',
  'blast.report.head': 'Blast-Radius: {n} betroffen, {t} Testkandidaten',
  'blast.help': 'Verwendung: /cockpit-blast [watch <pfad> | unwatch <pfad> | <pfad> | reset]\n  (ohne Argument)  geänderte und beobachtete Dateien auswerten\n  <pfad>           eine einzelne Datei auswerten\n  watch <pfad>     Datei dauerhaft beobachten\n  unwatch <pfad>   Beobachtung beenden\n  reset            Beobachtungen und Ergebnis löschen',
} as const
const en = {
  'blast.band': 'Blast: {n} affected · {t} tests',
  'blast.incomplete': ' (incomplete)',
  'blast.title.targets': 'Targets',
  'blast.title.affected': 'Affected files',
  'blast.title.tests': 'Test candidates',
  'blast.title.unresolved': 'Unresolved',
  'blast.title.hint': 'Note',
  'blast.label.changed': 'changed',
  'blast.label.watched': 'watched',
  'blast.label.status': 'Status',
  'blast.label.none': 'none',
  'blast.value.notScanned': 'not scanned yet',
  'blast.value.incomplete': 'incomplete (limits reached or Git error)',
  'blast.value.noTargets': 'no targets: nothing changed, nothing watched',
  'blast.more': '… +{n}',
  'blast.hint': 'Static relative imports only; check aliases, runtime imports and framework resolution manually.',
  'blast.report.head': 'Blast radius: {n} affected, {t} test candidates',
  'blast.help': 'Usage: /cockpit-blast [watch <path> | unwatch <path> | <path> | reset]\n  (no argument)    analyse changed and watched files\n  <path>           analyse a single file\n  watch <path>     watch a file permanently\n  unwatch <path>   stop watching\n  reset            clear watches and result',
} as const satisfies Record<keyof typeof de, string>
export const BLAST_TEXT = { de, en } as const
type Key = keyof typeof de

export function tx(L: UiLang, key: Key, vars?: Record<string, string | number>): string {
  const text: string = BLAST_TEXT[L][key]
  return vars === undefined ? text : text.replace(/\{(\w+)\}/g, (m, name: string) => (name in vars ? String(vars[name]) : m))
}

// ── Anzeige ──

export function bandText(r: BlastResult | null, L: UiLang): string {
  if (r === null || r.targets.length === 0) return ''
  return tx(L, 'blast.band', { n: r.affected.length, t: r.tests.length }) + (r.incomplete ? tx(L, 'blast.incomplete') : '')
}

type Row = BlastSection['rows'][number]
function listRows(items: readonly string[], L: UiLang): Row[] {
  const rows: Row[] = items.slice(0, MAX_VISIBLE).map(p => ({ label: '→', value: clean(p) }))
  if (items.length > MAX_VISIBLE) rows.push({ label: '', value: tx(L, 'blast.more', { n: items.length - MAX_VISIBLE }), tone: 'dim' })
  return rows
}

export function paneView(r: BlastResult | null, watched: readonly string[], L: UiLang): BlastSection[] {
  const isWatched = new Set(watched)
  const shown = r === null ? [...watched] : r.targets
  const targetRows: Row[] = shown.slice(0, MAX_VISIBLE).map(p => (
    isWatched.has(p) ? { label: tx(L, 'blast.label.watched'), value: clean(p) } : { label: tx(L, 'blast.label.changed'), value: clean(p), tone: 'warn' }
  ))
  if (shown.length > MAX_VISIBLE) targetRows.push({ label: '', value: tx(L, 'blast.more', { n: shown.length - MAX_VISIBLE }), tone: 'dim' })
  if (r === null) targetRows.push({ label: tx(L, 'blast.label.status'), value: tx(L, 'blast.value.notScanned'), tone: 'dim' })
  else if (r.targets.length === 0) targetRows.push({ label: tx(L, 'blast.label.status'), value: tx(L, 'blast.value.noTargets'), tone: 'dim' })
  if (r?.incomplete) targetRows.push({ label: tx(L, 'blast.label.status'), value: tx(L, 'blast.value.incomplete'), tone: 'warn' })
  const sections: BlastSection[] = [{ title: tx(L, 'blast.title.targets'), rows: targetRows }]
  if (r !== null && r.targets.length > 0) {
    sections.push({ title: tx(L, 'blast.title.affected'), rows: r.affected.length === 0 ? [{ label: tx(L, 'blast.label.none'), value: '', tone: 'dim' }] : listRows(r.affected, L) })
    sections.push({ title: tx(L, 'blast.title.tests'), rows: r.tests.length === 0 ? [{ label: tx(L, 'blast.label.none'), value: '', tone: 'dim' }] : listRows(r.tests, L) })
    if (r.unresolved.length > 0) sections.push({ title: tx(L, 'blast.title.unresolved'), rows: listRows(r.unresolved, L).map(x => ({ ...x, tone: x.tone ?? 'warn' })) })
  }
  sections.push({ title: tx(L, 'blast.title.hint'), rows: [], note: tx(L, 'blast.hint') })
  return sections
}

export function reportText(r: BlastResult | null, watched: readonly string[], L: UiLang): string {
  const lines: string[] = []
  if (r === null) lines.push(tx(L, 'blast.value.notScanned'))
  else lines.push(tx(L, 'blast.report.head', { n: r.affected.length, t: r.tests.length }) + (r.incomplete ? tx(L, 'blast.incomplete') : ''))
  for (const s of paneView(r, watched, L)) {
    lines.push('', s.title)
    for (const x of s.rows) lines.push(`  ${x.label}${x.label !== '' && x.value !== '' ? ' ' : ''}${x.value}`.trimEnd())
    if (s.note !== undefined) lines.push(`  ${s.note}`)
  }
  const text = lines.join('\n')
  return text.length > MAX_REPORT ? `${text.slice(0, MAX_REPORT - 4)}\n…` : text
}
