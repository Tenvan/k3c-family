// Zeitreise (Idee: Agent Time Machine): jeder Tool-Aufruf des Haupt-Agents ist ein nummerierter Schritt; nach Aufrufen, die
// Dateien ändern können, hält ein Snapshot den Stand des Arbeitsbaums fest. Daraus: Zeitleiste, „Was hat sich seitdem
// geändert?“ und „Zurück auf diesen Stand“ (mit Sicherung davor). Ein Snapshot ist ein git-Tree-Objekt aus einem eigenen
// Index (`<git-dir>/cockpit-tm.index`); Index, Branch und Arbeitsbaum des Nutzers bleiben unberührt. Rein, ohne `$`.

import type { UiLang } from '../types'
import { oneLine } from './core'

export const MAX_STEPS = 60
export const SHOWN_STEPS = 30
export const MAX_RESTORE = 200

export type StepKind = 'read' | 'edit' | 'shell' | 'other'
// `snap`: Tree-Hash nach dem Schritt (nur nach Aufrufen, die Dateien ändern können); `base`: Stand vor dem ersten Schritt.
export type Step = { n: number; at: number; tool: string; target: string; kind: StepKind; ok: boolean; snap?: string; id?: string }
export type TlState = { steps: Step[]; seq: number; base: string; safety: { n: number; snap: string } | null }

export const newTimeline = (): TlState => ({ steps: [], seq: 0, base: '', safety: null })

const SHELL = new Set(['Bash', 'PowerShell'])
const READ = new Set(['Read', 'Glob', 'Grep', 'WebFetch', 'WebSearch'])
const EDIT = new Set(['Edit', 'MultiEdit', 'Write', 'NotebookEdit'])

export const kindOf = (tool: string): StepKind => (EDIT.has(tool) ? 'edit' : SHELL.has(tool) ? 'shell' : READ.has(tool) ? 'read' : 'other')

// Neuer Schritt; liefert ihn zurück, damit der Snapshot später dazugeschrieben werden kann.
export function addStep(st: TlState, s: { at: number; tool: string; target: string; ok: boolean; id?: string }): Step {
  st.seq++
  const step: Step = { n: st.seq, at: s.at, tool: s.tool, target: oneLine(s.target, 60), kind: kindOf(s.tool), ok: s.ok, ...(s.id !== undefined ? { id: s.id } : {}) }
  st.steps.push(step)
  if (st.steps.length > MAX_STEPS) st.steps.splice(0, st.steps.length - MAX_STEPS)
  return step
}

export const stepByNumber = (st: TlState, n: number): Step | undefined => st.steps.find(x => x.n === n)

// Der leere Baum (SHA-1): git liest Attribute daraus statt aus `.gitattributes` im Arbeitsbaum (`--attr-source`, git ≥ 2.40;
// ältere git oder SHA-256-Repos lehnen ab, dann entsteht kein Snapshot).
export const EMPTY_TREE = '4b825dc642cb6eb9a060e54bf8d69288fbee4904'

// Globale git-Optionen für `add` und `restore` der Zeitreise, damit dabei kein Filter läuft: jeder konfigurierte Filter (Namen
// aus `git config --name-only --get-regexp ^filter\.`) wird geleert, Attribute kommen aus dem leeren Baum. `--attr-source`
// allein reicht nicht, `.git/info/attributes` gilt weiter. Ein Name, der sich nicht sicher als `-c` übergeben lässt, sperrt (null).
export function noFilterArgs(configNames: readonly string[]): string[] | null {
  const names = new Set<string>()
  for (const line of configNames) {
    const m = /^filter\.(.+)\.(clean|smudge|process|required)$/i.exec(line.trim())
    if (m === null) continue
    const name = m[1] ?? ''
    if (name === '' || /[=\s"']/.test(name)) return null
    names.add(name)
  }
  const off = [...names].flatMap(n => ['-c', `filter.${n}.clean=`, '-c', `filter.${n}.smudge=`, '-c', `filter.${n}.process=`, '-c', `filter.${n}.required=false`])
  return [...off, `--attr-source=${EMPTY_TREE}`]
}

// Rückkehrbar sind Schritte mit Snapshot; der Stand vor dem ersten Schritt zählt als 0.
export function snapOf(st: TlState, n: number): string {
  return n === 0 ? st.base : (stepByNumber(st, n)?.snap ?? '')
}

// `git diff --name-status -z <alt> <neu>`: Status und Pfad als getrennte NUL-Einträge (R/C nicht, weil ohne -M/-C).
export type Change = { status: 'A' | 'M' | 'D' | 'T'; path: string }
export function parseNameStatus(out: string): Change[] {
  const parts = out.split(String.fromCharCode(0)).filter(x => x !== '')
  const list: Change[] = []
  for (let i = 0; i + 1 < parts.length; i += 2) {
    const s = parts[i]?.[0]
    const path = parts[i + 1] ?? ''
    if ((s === 'A' || s === 'M' || s === 'D' || s === 'T') && path !== '') list.push({ status: s, path })
  }
  return list
}

// Was „Zurück“ tut: geänderte und gelöschte Dateien werden auf den alten Stand gesetzt; seit damals neue Dateien bleiben
// liegen (ein Löschen wäre nicht umkehrbar genug) und stehen nur in der Meldung. Über MAX_RESTORE Dateien: nicht ausführen.
export function restorePlan(changes: readonly Change[]): { restore: string[]; keep: string[]; tooMany: boolean } {
  const restore = changes.filter(c => c.status !== 'A').map(c => c.path)
  return { restore: restore.slice(0, MAX_RESTORE), keep: changes.filter(c => c.status === 'A').map(c => c.path), tooMany: restore.length > MAX_RESTORE }
}

// ── Texte ──

const de = {
  'tl.cardTitle': 'Zeitreise',
  'tl.title.steps': 'Letzte Schritte',
  'tl.title.diff': 'Seit Schritt {n}',
  'tl.title.result': 'Ergebnis',
  'tl.mark.snap': '◆',
  'tl.mark.error': '✗',
  'tl.start': 'Start (vor dem ersten Schritt)',
  'tl.none': 'Noch kein Schritt aufgezeichnet.',
  'tl.noRepo': 'Kein Snapshot: kein Git-Repo, oder das Repo ist gesperrt (Git-Panel → Entsperren).',
  'tl.diff': 'Diff',
  'tl.back': 'Zurück',
  'tl.backAsk': 'Wirklich zurück?',
  'tl.diffNone': 'Seit diesem Schritt hat sich nichts geändert.',
  'tl.diffFail': 'Der Vergleich ist fehlgeschlagen: {why}',
  'tl.restored': 'Zurückgesetzt auf Schritt {n}: {k} Dateien. Sicherung vor dem Zurücksetzen: Schritt {s}.',
  'tl.restoredKeep': '{k} seit damals neue Dateien bleiben liegen.',
  'tl.restoreFail': 'Zurücksetzen fehlgeschlagen: {why}',
  'tl.tooMany': 'Mehr als {max} Dateien betroffen: bitte im Git-Panel oder per git zurücksetzen.',
  'tl.fill': 'Ich habe die Dateien auf den Stand von Schritt {n} zurückgesetzt ({files}). Deine früheren Lesezugriffe darauf sind veraltet; lies sie neu, bevor du weiterarbeitest.',
  'tl.hint': 'Snapshots sind git-Objekte (kein Branch, kein Commit, Index und Arbeitsbaum bleiben unberührt) und nur nach Datei-ändernden Aufrufen. Neue (untracked) Dateien sind im Snapshot, werden beim Zurücksetzen aber nicht gelöscht. Die Liste gilt nur für diese Session.',
  'tl.report.head': 'Zeitreise: {n} Schritte, {s} mit Snapshot',
  'tl.safety': 'Sicherung vor dem Zurücksetzen',
} as const
const en = {
  'tl.cardTitle': 'Time travel',
  'tl.title.steps': 'Latest steps',
  'tl.title.diff': 'Since step {n}',
  'tl.title.result': 'Result',
  'tl.mark.snap': '◆',
  'tl.mark.error': '✗',
  'tl.start': 'Start (before the first step)',
  'tl.none': 'No step recorded yet.',
  'tl.noRepo': 'No snapshot: not a Git repo, or the repo is locked (Git panel → Unlock).',
  'tl.diff': 'Diff',
  'tl.back': 'Back',
  'tl.backAsk': 'Really go back?',
  'tl.diffNone': 'Nothing has changed since this step.',
  'tl.diffFail': 'The comparison failed: {why}',
  'tl.restored': 'Reset to step {n}: {k} files. Safety copy before the reset: step {s}.',
  'tl.restoredKeep': '{k} files that are new since then stay in place.',
  'tl.restoreFail': 'Reset failed: {why}',
  'tl.tooMany': 'More than {max} files affected: please reset in the Git panel or with git.',
  'tl.fill': 'I reset the files to the state of step {n} ({files}). Your earlier reads of them are stale; read them again before you continue.',
  'tl.hint': 'Snapshots are git objects (no branch, no commit, index and working tree stay untouched) and only follow calls that change files. New (untracked) files are in the snapshot but are not deleted on reset. The list covers this session only.',
  'tl.report.head': 'Time travel: {n} steps, {s} with snapshot',
  'tl.safety': 'Safety copy before the reset',
} as const satisfies Record<keyof typeof de, string>
export const TL_TEXT = { de, en } as const
type Key = keyof typeof de

export function tx(L: UiLang, key: Key, vars?: Record<string, string | number>): string {
  const text: string = TL_TEXT[L][key]
  return vars === undefined ? text : text.replace(/\{(\w+)\}/g, (m, name: string) => (name in vars ? String(vars[name]) : m))
}

// ── Anzeige ──

const clock = (at: number): string => {
  const d = new Date(at)
  return `${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}:${String(d.getSeconds()).padStart(2, '0')}`
}

export function stepLine(s: Step): string {
  return `${s.snap === undefined ? '  ' : `${TL_TEXT.de['tl.mark.snap']} `}${s.ok ? '' : `${TL_TEXT.de['tl.mark.error']} `}${s.tool}${s.target === '' ? '' : ` ${s.target}`}`
}

export const shownSteps = (st: TlState): Step[] => st.steps.slice(-SHOWN_STEPS).reverse()

export function reportText(st: TlState, L: UiLang): string {
  const lines = [tx(L, 'tl.report.head', { n: st.steps.length, s: st.steps.filter(x => x.snap !== undefined).length })]
  if (st.steps.length === 0) lines.push(`  ${tx(L, 'tl.none')}`)
  for (const s of shownSteps(st)) lines.push(`  #${s.n} ${clock(s.at)} ${stepLine(s)}`)
  return lines.join('\n')
}

export { clock as stepClock }
