// Git-Panel: Status lesen, Phase im Ablauf Commit → MR → Merge → Aufräumen und die Hauptaktion daraus ableiten,
// Branch-Baum (nur Abhängigkeiten, keine Commits), Aufräum-Plan nach dem Merge, Aufträge an Claude.
// Rein, ohne `$`: die git-Befehle laufen in register.tsx, hier wird nur gelesen und entschieden.

import type { GitBranch, GitCommit, GitFile, GitOp, GitState, Mr, UiLang } from '../types'
import { t } from './i18n'

export type GitPhase = 'commit' | 'mr' | 'merge' | 'cleanup' | 'done'
// Die vier Stationen der Ablauf-Leiste; „done“ heißt: alle erledigt.
export const PHASES = ['commit', 'mr', 'merge', 'cleanup'] as const

// Was der Hauptknopf der Status-Karte tut; null, wenn nichts ansteht.
// `force` nach eigenem Rebase/Amend (alles vom Upstream steckt patch-gleich lokal drin); `commitClaude`, wenn Repo-Hooks
// den Commit prüfen sollen (das Panel startet git ohne Hooks).
export type GitAction = 'resolve' | 'continue' | 'branchOff' | 'cleanup' | 'pullRebase' | 'force' | 'pull' | 'commit' | 'commitClaude' | 'syncParent' | 'mrDraft' | 'push' | 'merge'

const NUL = String.fromCharCode(0)
const NL = String.fromCharCode(10)

export type StatusInfo = Pick<GitState, 'branch' | 'detached' | 'upstream' | 'ahead' | 'behind' | 'staged' | 'changed' | 'untracked' | 'conflicts'>

// `git status --porcelain=v2 --branch -z`: Kopfzeilen (# branch.*) und je Datei ein Eintrag; bei Umbenennungen (Typ 2)
// folgt der alte Pfad als eigener Eintrag. X = Index (gestaged), Y = Arbeitsbaum; „.“ = unverändert.
export function parseStatus(text: string): StatusInfo {
  const out: StatusInfo = { branch: '', detached: false, upstream: '', ahead: 0, behind: 0, staged: [], changed: [], untracked: [], conflicts: [] }
  const recs = text.split(NUL)
  for (let i = 0; i < recs.length; i++) {
    const r = recs[i] ?? ''
    if (r.startsWith('# branch.head ')) {
      const h = r.slice(14)
      out.detached = h === '(detached)'
      out.branch = out.detached ? '' : h
    } else if (r.startsWith('# branch.upstream ')) out.upstream = r.slice(18)
    else if (r.startsWith('# branch.ab ')) {
      const m = /^\+(\d+) -(\d+)$/.exec(r.slice(12))
      if (m) {
        out.ahead = Number(m[1])
        out.behind = Number(m[2])
      }
    } else if (r.startsWith('1 ') || r.startsWith('2 ')) {
      const f = r.split(' ')
      const xy = f[1] ?? '..'
      const path = f.slice(r.startsWith('1 ') ? 8 : 9).join(' ')
      if (r.startsWith('2 ')) i++
      if (xy[0] !== '.') out.staged.push({ path, code: xy[0] ?? 'M' })
      if (xy[1] !== '.') out.changed.push({ path, code: xy[1] ?? 'M' })
    } else if (r.startsWith('u ')) out.conflicts.push(r.split(' ').slice(10).join(' '))
    else if (r.startsWith('? ')) out.untracked.push(r.slice(2))
  }
  return out
}

// Laufende Operation aus den Markern, die git währenddessen setzt (MERGE_HEAD, REBASE_HEAD …).
export function opOf(m: { merge: boolean; rebase: boolean; pick: boolean; revert: boolean }): GitOp {
  return m.rebase ? 'rebase' : m.merge ? 'merge' : m.pick ? 'cherry-pick' : m.revert ? 'revert' : ''
}

// `git for-each-ref refs/heads --format=%(refname:short)%00%(upstream:short)%00%(upstream:track,nobracket)`.
export function parseRefs(text: string): { name: string; upstream: string; gone: boolean }[] {
  return text
    .split(/\r?\n/)
    .filter(l => l.trim() !== '')
    .map(l => {
      const [name = '', upstream = '', track = ''] = l.split(NUL)
      return { name, upstream, gone: track.trim() === 'gone' }
    })
}

// Eltern-Branches aus der Git-Town-Konfiguration: `git config --get-regexp ^git-town-branch\..*\.parent$`.
// git schreibt Abschnitt und Schlüssel klein, den Branch-Namen dazwischen aber unverändert.
export function parseTownParents(text: string): Record<string, string> {
  const out: Record<string, string> = {}
  for (const l of text.split(/\r?\n/)) {
    const m = /^git-town-branch\.(.+)\.parent\s+(\S+)\s*$/.exec(l.trim())
    if (m?.[1] && m[2]) out[m[1]] = m[2]
  }
  return out
}

// Eltern je Branch: zuerst Git Town, sonst der nächstgelegene lokale Branch, dessen Spitze Vorfahr ist (`--merged <b>`)
// und der mehr eigene Commits über main hat als jeder andere Kandidat, aber weniger als b selbst; sonst main.
// `depth`: Commits über main (`rev-list --count main..b`). Zyklen aus der Konfiguration enden bei main.
export function inferParents(
  names: readonly string[],
  main: string,
  town: Record<string, string>,
  mergedInto: Record<string, readonly string[]>,
  depth: Record<string, number>,
): Record<string, string | null> {
  const has = new Set(names)
  const out: Record<string, string | null> = {}
  for (const b of names) {
    if (b === main) {
      out[b] = null
      continue
    }
    const own = town[b]
    if (own !== undefined && own !== b && has.has(own)) {
      out[b] = own
      continue
    }
    const d = depth[b] ?? 0
    let best: string | null = null
    for (const c of mergedInto[b] ?? []) {
      const dc = depth[c] ?? 0
      if (c === b || c === main || !has.has(c) || dc <= 0 || dc >= d) continue
      if (best === null || dc > (depth[best] ?? 0)) best = c
    }
    out[b] = best ?? (has.has(main) ? main : null)
  }
  // Zyklen (a → b → a) brechen: wer auf dem Weg nach oben sich selbst wiedertrifft, hängt an main.
  for (const b of names) {
    const seen = new Set<string>([b])
    let p = out[b] ?? null
    while (p !== null && !seen.has(p)) {
      seen.add(p)
      p = out[p] ?? null
    }
    if (p !== null) out[b] = b === main ? null : main
  }
  return out
}

// Branches, die das Panel nie löscht (Entscheidung Ralf), dazu der erkannte Haupt-Branch des Repos.
export const PROTECTED: readonly string[] = ['main', 'master', 'develop']
export const isProtected = (name: string, main: string): boolean => name === main || PROTECTED.includes(name)

// Gemergt: Upstream gelöscht (nach dem Merge auf der Plattform) oder Spitze in main enthalten, obwohl schon gepusht
// (ein frischer Branch ohne Commits und ohne Upstream zählt nicht).
export const isMerged = (b: { name: string; upstream: string; gone: boolean }, main: string, depth: number): boolean =>
  b.name !== main && (b.gone || (depth === 0 && b.upstream !== ''))

// Zeilen des Branch-Baums in Tiefensuche: Präfix aus Baumzeichen, gemergte Zweige zuletzt. Höchstens `max` Zeilen.
// ponytail: feste Obergrenze statt Scrollen; viele Branches → „… n weitere“ in register.tsx.
export function treeRows(branches: readonly GitBranch[], main: string, max = 20): { b: GitBranch; prefix: string }[] {
  const rows: { b: GitBranch; prefix: string }[] = []
  const kids = (p: string | null) =>
    branches
      .filter(b => b.parent === p && b.name !== p)
      .sort((x, y) => Number(x.merged) - Number(y.merged) || x.name.localeCompare(y.name))
  const walk = (p: string, lead: string) => {
    const list = kids(p)
    list.forEach((b, i) => {
      const last = i === list.length - 1
      rows.push({ b, prefix: `${lead}${last ? '└─' : '├─'}` })
      walk(b.name, `${lead}${last ? '  ' : '│ '}`)
    })
  }
  const roots = branches.filter(b => b.parent === null || !branches.some(x => x.name === b.parent))
  roots.sort((x, y) => Number(y.name === main) - Number(x.name === main) || x.name.localeCompare(y.name))
  for (const r of roots) {
    rows.push({ b: r, prefix: '' })
    walk(r.name, '')
  }
  return rows.slice(0, max)
}

const dirtyOf = (s: GitState): number => s.staged.length + s.changed.length + s.untracked.length
export const isDirty = (s: GitState): boolean => dirtyOf(s) > 0
export const onMain = (s: GitState): boolean => s.detached || s.branch === s.main

// Der MR gehört zum Branch (mrCur kann nach einem Branch-Wechsel kurz noch der alte sein); ein geschlossener zählt nicht.
const ownMr = (s: GitState, mr: Mr | null): Mr | null => (mr !== null && mr.source === s.branch && mr.state !== 'closed' ? mr : null)
const self = (s: GitState): GitBranch | undefined => s.branches.find(b => b.name === s.branch)

// Phase im Ablauf aus Git-Stand und MR.
export function phaseOf(s: GitState, mr: Mr | null): GitPhase {
  if (onMain(s)) return isDirty(s) ? 'commit' : 'done'
  const m = ownMr(s, mr)
  if (m?.state === 'merged') return 'cleanup'
  if (isDirty(s) || s.op !== '') return 'commit'
  if (m?.state === 'opened') return 'merge'
  if ((self(s)?.ahead ?? 0) > 0 || s.ahead > 0) return 'mr'
  return 'done'
}

// Hauptaktion der Status-Karte, in der Reihenfolge der Dringlichkeit.
export function primaryAction(s: GitState, mr: Mr | null): GitAction | null {
  if (s.op !== '') return s.conflicts.length > 0 ? 'resolve' : 'continue'
  if (onMain(s) && isDirty(s)) return s.detached ? null : 'branchOff'
  const phase = phaseOf(s, mr)
  if (phase === 'cleanup') return 'cleanup'
  if (s.upstream !== '' && s.ahead > 0 && s.behind > 0) return s.upstreamOnly === 0 ? 'force' : 'pullRebase'
  if (s.upstream !== '' && s.behind > 0) return 'pull'
  if (phase === 'commit') return s.hooks.some(h => h !== 'pre-push') ? 'commitClaude' : 'commit'
  const m = ownMr(s, mr)
  if (phase === 'mr') return (self(s)?.behind ?? 0) > 0 ? 'syncParent' : m === null ? 'mrDraft' : 'push'
  if (phase === 'merge') return s.ahead > 0 || s.upstream === '' ? 'push' : 'merge'
  return null
}

export type CleanupStep = { argv: string[]; optional: boolean }

// Aufräumen nach dem Merge: holen, auf den Eltern-Branch wechseln (ist er in einem anderen Worktree ausgecheckt, nur dessen
// Remote-Stand ohne Branch), Kinder in Git Town umhängen, Branch löschen. `-D` nur, wenn die Plattform den Merge bestätigt
// (Squash-Merges sind für git nicht „gemergt“) und jeder Commit des Branches auf einem Remote liegt (`localOnly` 0):
// ein lokaler Commit nach dem Merge ginge sonst verloren, `-d` scheitert dann hörbar. Optionale Schritte dürfen scheitern.
// ponytail: Der Worktree der Session bleibt stehen (die Session läuft darin); die Desktop-App räumt ihn beim Archivieren ab.
export function cleanupPlan(s: GitState, o: { parent: string; mrMerged: boolean; parentElsewhere: boolean }): CleanupStep[] {
  const r = s.remote
  const steps: CleanupStep[] = [{ argv: ['fetch', '--prune', r], optional: false }]
  if (o.parentElsewhere) steps.push({ argv: ['switch', '--detach', `${r}/${o.parent}`], optional: false })
  else steps.push({ argv: ['switch', o.parent], optional: false }, { argv: ['merge', '--ff-only', `${r}/${o.parent}`], optional: true })
  for (const c of s.branches.filter(b => b.parent === s.branch && b.name !== s.branch)) {
    if (s.town) steps.push({ argv: ['config', `git-town-branch.${c.name}.parent`, o.parent], optional: true })
  }
  if (!isProtected(s.branch, s.main)) steps.push({ argv: ['branch', o.mrMerged && s.localOnly === 0 ? '-D' : '-d', s.branch], optional: false })
  if (s.town) steps.push({ argv: ['config', '--unset', `git-town-branch.${s.branch}.parent`], optional: true })
  return steps
}

// `git worktree list --porcelain`: Branches, die in einem anderen Worktree als `top` ausgecheckt sind
// (dorthin lässt sich nicht wechseln). Pfade ohne Rücksicht auf Groß-/Kleinschreibung und Schrägstrich-Art verglichen.
export function parseWorktrees(text: string, top: string): string[] {
  const norm = (p: string) => p.replace(/\\/g, '/').replace(/\/+$/, '').toLowerCase()
  const out: string[] = []
  for (const block of text.split(/\r?\n\r?\n/)) {
    const path = /^worktree (.+)$/m.exec(block)?.[1] ?? ''
    const branch = /^branch refs\/heads\/(.+)$/m.exec(block)?.[1] ?? ''
    if (path !== '' && branch !== '' && norm(path) !== norm(top)) out.push(branch)
  }
  return out
}

// Schlüssel der lokalen Repo-Konfiguration, über die schreibende oder Netzwerk-Befehle (merge, rebase, fetch, push, stash)
// Programme starten oder Ziele umleiten können, zusätzlich zu `RISKY_GIT_CONFIG` aus core.ts (das schon das Lesen sperrt).
// Ist einer gesetzt, führt das Panel keine Aktion aus; der Weg über Claude oder das Terminal bleibt.
export const RISKY_ACTION_CONFIG =
  '^(merge\\..*\\.driver|credential\\..*|core\\.(askpass|gitproxy|alternaterefscommand)|remote\\..*\\.(uploadpack|receivepack|proxy|vcs)|url\\..*\\.(insteadof|pushinsteadof)|sequence\\.editor|protocol\\..*|http\\..*\\.(proxy|sslcainfo)|http\\.proxy)$'

// Immer mitgegeben: keine Transporte, die Befehle starten oder Dateideskriptoren lesen (ext::, fd::).
export const GIT_SAFE_ARGS = ['-c', 'protocol.ext.allow=never', '-c', 'protocol.fd.allow=never'] as const

// Gültiger Branch-Name (Kurzform der Regeln aus `git check-ref-format`); git prüft beim Anlegen ohnehin nach.
export const isBranchName = (s: string): boolean =>
  /^[A-Za-z0-9._/-]+$/.test(s) && !/^[-/.]|[/.]$|\.\.|\/\/|\/\.|\.lock$|@\{/.test(s) && s !== 'HEAD'

// Alter als kurze Einheit, sprachneutral: „<1m“, „12m“, „5h“, „3d“.
export function ageText(ms: number): string {
  const m = Math.floor(Math.max(0, ms) / 60_000)
  if (m < 1) return '<1m'
  if (m < 60) return `${m}m`
  const h = Math.floor(m / 60)
  return h < 48 ? `${h}h` : `${Math.floor(h / 24)}d`
}

// `git log --format=%h%x00%an%x00%ct%x00%s`.
export function parseLog(text: string): GitCommit[] {
  return text
    .split(/\r?\n/)
    .filter(l => l.trim() !== '')
    .map(l => {
      const [sha = '', author = '', ct = '0', subject = ''] = l.split(NUL)
      return { sha, author, at: Number(ct) * 1000, subject }
    })
}

// ---- Commit-Vorschlag (Haiku) ----

// Auftrag an Haiku: eine Commit-Zeile im Stil der letzten Commits aus Diffstat und gekürztem Diff.
export function commitAsk(stat: string, diff: string, recent: string, lang: UiLang): string {
  return [
    t(lang, 'git.suggestAsk'),
    '',
    `${t(lang, 'git.suggestRecent')}:`,
    recent.trim() || '-',
    '',
    'git diff --cached --stat:',
    stat.trim(),
    '',
    'git diff --cached:',
    diff.length > 8000 ? `${diff.slice(0, 8000)}${NL}[…]` : diff,
  ].join(NL)
}

// Erste sinnvolle Zeile der Antwort, ohne Codeblock, Anführungszeichen und Präfixe wie „Commit:“; höchstens 120 Zeichen.
export function parseSuggestion(text: string): string {
  const line = text
    .split(/\r?\n/)
    .map(l => l.trim())
    .find(l => l !== '' && !l.startsWith('```')) ?? ''
  return line
    .replace(/^(commit( message)?|nachricht)\s*:\s*/i, '')
    .replace(/^["'`„“]+|["'`„“]+$/g, '')
    .trim()
    .slice(0, 120)
}

// ---- Aufträge an Claude (✦): landen im Eingabefeld, erst Enter sendet ----

export const commitPrompt = (L: UiLang): string => t(L, 'git.p.commit')
export const splitPrompt = (L: UiLang): string => t(L, 'git.p.split')
export const reviewPrompt = (L: UiLang): string => t(L, 'git.p.review')
export const incomingPrompt = (n: number, upstream: string, L: UiLang): string => t(L, 'git.p.incoming', { n, upstream })
export const resolvePrompt = (op: GitOp, files: readonly string[], L: UiLang): string =>
  t(L, 'git.p.resolve', { op: op === '' ? 'merge' : op, files: files.join(', ') })
export const mrPrompt = (branch: string, parent: string, L: UiLang): string => t(L, 'git.p.mr', { branch, parent })
export const cleanupPrompt = (branch: string, parent: string, mr: string, L: UiLang): string => t(L, 'git.p.cleanup', { branch, parent, mr })
export const tidyPrompt = (L: UiLang): string => t(L, 'git.p.tidy')
// Fehlertext gekürzt auf die letzten 1500 Zeichen: das Ende der Ausgabe nennt meist die Ursache.
export const errorPrompt = (cmd: string, out: string, L: UiLang): string =>
  t(L, 'git.p.error', { cmd, out: out.length > 1500 ? `…${out.slice(-1500)}` : out })

// Kurze Zeile je Datei für die Änderungen-Karte: Code und Pfad.
export const fileLine = (f: GitFile): string => `${f.code} ${f.path}`
