import { expect, mock, test } from 'claude-code/testing'
import type { TestBody } from 'claude-code/testing'

import type { GitBranch, GitState, Mr } from '../types'
import { prioOf } from './ui'
import { ageText, cleanupPlan, isProtected, inferParents, isBranchName, isMerged, mrPrompt, opOf, parseLog, parseRefs, parseStatus, parseSuggestion, parseTownParents, parseWorktrees, phaseOf, primaryAction, resolvePrompt, treeRows } from './git'

const Z = String.fromCharCode(0)
const GIT_PROPS = { title: 'Cockpit · Git', isFocused: false, bodyColumns: 120, placement: 'dock', scroll: { offset: 0, bodyRows: 60 }, view: {} } as const

const br = (name: string, parent: string | null, o: Partial<GitBranch> = {}): GitBranch => ({ name, parent, upstream: `origin/${name}`, gone: false, ahead: 0, behind: 0, merged: false, ...o })

const STATE: GitState = {
  top: 'C:/repo/x',
  branch: 'feature/x',
  detached: false,
  upstream: 'origin/feature/x',
  ahead: 0,
  behind: 0,
  localOnly: 0,
  upstreamOnly: 0,
  hooks: [],
  staged: [],
  changed: [],
  untracked: [],
  conflicts: [],
  op: '',
  main: 'main',
  remote: 'origin',
  branches: [br('main', null), br('feature/x', 'main', { ahead: 2 }), br('feature/x-tests', 'feature/x', { upstream: '', ahead: 1 })],
  incoming: [],
  history: [],
  stashes: 0,
  fetchedAt: 0,
  town: true,
  elsewhere: [],
  readAt: 0,
}

const MR = (state: string): Mr => ({
  forge: 'gitlab', iid: 42, title: 'X', state, draft: false, url: '', source: 'feature/x', target: 'main', mergeStatus: 'mergeable', conflicts: false,
  notes: 0, threads: 0, pipeline: null, approvals: null, description: '', author: '', updatedAt: '', changes: '', threadList: [],
})

test('parseStatus: Kopf, gestagt/geändert, Umbenennung mit altem Pfad, Konflikt, neue Datei, detached', () => {
  const text = [
    '# branch.oid abc',
    '# branch.head feature/x',
    '# branch.upstream origin/feature/x',
    '# branch.ab +2 -1',
    '1 M. N... 100644 100644 100644 h1 h2 hooks/git.ts',
    '1 .M N... 100644 100644 100644 h1 h2 mit leerzeichen.md',
    '2 R. N... 100644 100644 100644 h1 h2 R100 neu.ts',
    'alt.ts',
    'u UU N... 100644 100644 100644 100644 h1 h2 h3 SPEC.md',
    '? hooks/git.test.ts',
    '',
  ].join(Z)
  const s = parseStatus(text)
  expect(s.branch).toBe('feature/x')
  expect([s.upstream, s.ahead, s.behind]).toEqual(['origin/feature/x', 2, 1])
  expect(s.staged).toEqual([{ path: 'hooks/git.ts', code: 'M' }, { path: 'neu.ts', code: 'R' }])
  expect(s.changed).toEqual([{ path: 'mit leerzeichen.md', code: 'M' }])
  expect(s.conflicts).toEqual(['SPEC.md'])
  expect(s.untracked).toEqual(['hooks/git.test.ts'])
  expect(parseStatus(`# branch.head (detached)${Z}`)).toEqual({ ...parseStatus(''), detached: true })
})

test('Refs, Git-Town-Eltern, Worktrees, Log, laufende Operation', () => {
  expect(parseRefs(`main${Z}origin/main${Z}\nfix/a${Z}origin/fix/a${Z}gone\nneu${Z}${Z}\n`)).toEqual([
    { name: 'main', upstream: 'origin/main', gone: false },
    { name: 'fix/a', upstream: 'origin/fix/a', gone: true },
    { name: 'neu', upstream: '', gone: false },
  ])
  expect(parseTownParents('git-town-branch.feature/A.parent main\ngit-town-branch.feature/a-tests.parent feature/A\n')).toEqual({ 'feature/A': 'main', 'feature/a-tests': 'feature/A' })
  const wt = 'worktree C:/w/repo\nHEAD a\nbranch refs/heads/main\n\nworktree C:/w/repo-wt\nHEAD b\nbranch refs/heads/feature/x\n\nworktree C:/w/det\nHEAD c\ndetached\n'
  expect(parseWorktrees(wt, 'C:\\w\\repo-wt\\')).toEqual(['main'])
  expect(parseLog(`abc${Z}Ralf${Z}1700000000${Z}feat: a\n`)).toEqual([{ sha: 'abc', author: 'Ralf', at: 1_700_000_000_000, subject: 'feat: a' }])
  expect(opOf({ merge: true, rebase: true, pick: false, revert: false })).toBe('rebase')
  expect(opOf({ merge: false, rebase: false, pick: false, revert: false })).toBe('')
})

test('inferParents: Git Town zuerst, sonst nächster Vorfahr mit eigenen Commits; gemergte hängen an main, Zyklen werden gebrochen', () => {
  const names = ['main', 'feature/a', 'feature/a-tests', 'fix/old', 'x', 'y']
  const p = inferParents(
    names,
    'main',
    { x: 'y', y: 'x' },
    { 'feature/a': ['main', 'fix/old', 'feature/a'], 'feature/a-tests': ['main', 'fix/old', 'feature/a', 'feature/a-tests'], 'fix/old': ['main', 'fix/old'] },
    { 'feature/a': 2, 'feature/a-tests': 3, 'fix/old': 0 },
  )
  expect(p).toEqual({ main: null, 'feature/a': 'main', 'feature/a-tests': 'feature/a', 'fix/old': 'main', x: 'main', y: 'x' })
  expect(isMerged({ name: 'fix/old', upstream: 'origin/fix/old', gone: false }, 'main', 0)).toBe(true)
  expect(isMerged({ name: 'neu', upstream: '', gone: false }, 'main', 0)).toBe(false)
  expect(isMerged({ name: 'sq', upstream: 'origin/sq', gone: true }, 'main', 3)).toBe(true)
})

test('treeRows: Baumzeichen, Kinder unter Eltern, gemergte zuletzt', () => {
  const rows = treeRows([br('main', null), br('b', 'main', { merged: true }), br('a', 'main'), br('a2', 'a'), br('c', 'main')], 'main')
  expect(rows.map(r => `${r.prefix}${r.b.name}`)).toEqual(['main', '├─a', '│ └─a2', '├─c', '└─b'])
})

test('Phase und Hauptaktion: Commit → MR → Merge → Aufräumen, Schutz auf main, Divergenz, Konflikt', () => {
  const dirty = { ...STATE, changed: [{ path: 'a', code: 'M' }] }
  expect([phaseOf(dirty, null), primaryAction(dirty, null)]).toEqual(['commit', 'commit'])
  const onMain = { ...dirty, branch: 'main' }
  expect([phaseOf(onMain, null), primaryAction(onMain, null)]).toEqual(['commit', 'branchOff'])
  expect([phaseOf(STATE, null), primaryAction(STATE, null)]).toEqual(['mr', 'mrDraft'])
  const parentAhead = { ...STATE, branches: [br('main', null), br('feature/x', 'main', { ahead: 2, behind: 3 })] }
  expect(primaryAction(parentAhead, null)).toBe('syncParent')
  expect([phaseOf(STATE, MR('opened')), primaryAction(STATE, MR('opened'))]).toEqual(['merge', 'merge'])
  expect(primaryAction({ ...STATE, ahead: 1 }, MR('opened'))).toBe('push')
  expect([phaseOf(STATE, MR('merged')), primaryAction(STATE, MR('merged'))]).toEqual(['cleanup', 'cleanup'])
  // Ein MR eines anderen Branches zählt nicht.
  expect(phaseOf(STATE, { ...MR('merged'), source: 'anders' })).toBe('mr')
  expect(primaryAction({ ...STATE, ahead: 2, behind: 1, upstreamOnly: 1 }, null)).toBe('pullRebase')
  // Nach eigenem Rebase/Amend steckt alles vom Upstream patch-gleich lokal drin: Force statt Pull.
  expect(primaryAction({ ...STATE, ahead: 2, behind: 1, upstreamOnly: 0 }, null)).toBe('force')
  // Repo-Hooks prüfen den Commit nur im Terminal: dann über Claude committen.
  expect(primaryAction({ ...dirty, hooks: ['pre-commit'] }, null)).toBe('commitClaude')
  expect(primaryAction({ ...dirty, hooks: ['pre-push'] }, null)).toBe('commit')
  // Ein geschlossener MR zählt nicht: neuer Entwurf.
  expect(primaryAction(STATE, MR('closed'))).toBe('mrDraft')
  expect(primaryAction({ ...STATE, behind: 1 }, null)).toBe('pull')
  expect(primaryAction({ ...STATE, op: 'rebase', conflicts: ['a'] }, null)).toBe('resolve')
  expect(primaryAction({ ...STATE, op: 'merge' }, null)).toBe('continue')
  const clean = { ...STATE, branch: 'main' }
  expect([phaseOf(clean, null), primaryAction(clean, null)]).toEqual(['done', null])
})

test('cleanupPlan: holen, wechseln, Kinder umhängen, löschen; -D nur nach bestätigtem Merge, detach bei Worktree', () => {
  const plan = cleanupPlan(STATE, { parent: 'main', mrMerged: true, parentElsewhere: false }).map(s => s.argv.join(' '))
  expect(plan).toEqual([
    'fetch --prune origin',
    'switch main',
    'merge --ff-only origin/main',
    'config git-town-branch.feature/x-tests.parent main',
    'branch -D feature/x',
    'config --unset git-town-branch.feature/x.parent',
  ])
  // Lokaler Commit nach dem Merge: nur `-d`, das scheitert sichtbar statt den Commit zu löschen.
  expect(cleanupPlan({ ...STATE, localOnly: 1 }, { parent: 'main', mrMerged: true, parentElsewhere: false }).some(s => s.argv.join(' ') === 'branch -d feature/x')).toBe(true)
  // main, master und develop löscht das Panel nie, auch nicht beim Aufräumen.
  expect(['main', 'master', 'develop', 'trunk'].map(n => isProtected(n, 'trunk'))).toEqual([true, true, true, true])
  expect(isProtected('feature/x', 'main')).toBe(false)
  expect(cleanupPlan({ ...STATE, branch: 'develop' }, { parent: 'main', mrMerged: true, parentElsewhere: false }).some(s => s.argv[0] === 'branch')).toBe(false)
  const away = cleanupPlan({ ...STATE, town: false }, { parent: 'main', mrMerged: false, parentElsewhere: true }).map(s => s.argv.join(' '))
  expect(away).toEqual(['fetch --prune origin', 'switch --detach origin/main', 'branch -d feature/x'])
})

test('Kleinkram: Branch-Namen, Vorschlag aus Haiku-Antwort, Alter, Prompts', () => {
  expect(['feature/git-pane', 'fix/a.b', 'x_1'].every(isBranchName)).toBe(true)
  expect(['', '-x', 'a..b', 'a/', 'a//b', 'x.lock', 'a b', 'HEAD', 'a@{1}', '/a'].some(isBranchName)).toBe(false)
  expect(parseSuggestion('```\nCommit: "✨ feat(mods): Git-Panel"\n```')).toBe('✨ feat(mods): Git-Panel')
  expect(parseSuggestion('')).toBe('')
  expect([ageText(10_000), ageText(12 * 60_000), ageText(5 * 3_600_000), ageText(72 * 3_600_000)]).toEqual(['<1m', '12m', '5h', '3d'])
  expect(mrPrompt('feature/x', 'main', 'de')).toContain('feature/x')
  expect(['git-m-commit', 'git-fetch', 'git-reload', 'git-st-b-0', 'git-discard', 'git-o-switch', 'git-o-delete'].map(prioOf)).toEqual(['high', 'normal', 'low', 'low', 'low', 'normal', 'low'])
  expect(['mr-a-merge', 'mr-a-rebase', 'mr-a-refresh', 'mine-squash-17'].map(prioOf)).toEqual(['high', 'normal', 'low', 'low'])
  // Übrige Panels: Hauptaktion vorn, Klappen, Blättern, Schalter hinten, der Rest gewöhnlich.
  expect(['cu-refresh', 'hist-day-report', 'hist-week-report', 'card-fold-cardLog', 'hist-prev', 'grp-on-2', 'diag-refresh', 'brake-a1'].map(prioOf)).toEqual(['high', 'high', 'normal', 'low', 'low', 'low', 'normal', 'normal'])
  expect(resolvePrompt('rebase', ['a.ts', 'b.ts'], 'en')).toContain('rebase (a.ts, b.ts)')
})

// ---- Panel ----

type Fake = { code: number; out: string; err?: string }
const STATUS = ['# branch.head feature/x', '# branch.upstream origin/feature/x', '# branch.ab +1 -0', '1 A. N... 100644 100644 100644 h h hooks/git.ts', '1 .M N... 100644 100644 100644 h h hooks/register.tsx', ''].join(Z)

function fakeGit(argv: readonly string[], risky = false, actionRisky = false): Fake {
  const a = argv.join(' ')
  if (a === 'rev-parse --show-toplevel') return { code: 0, out: 'C:/w/demo\n' }
  if (a.startsWith('config --local --includes --name-only --get-regexp ^(merge')) return actionRisky ? { code: 0, out: 'credential.helper\n' } : { code: 1, out: '' }
  if (a.startsWith('config --local --includes --name-only --get-regexp')) return risky ? { code: 0, out: 'core.fsmonitor\n' } : { code: 1, out: '' }
  if (a.startsWith('config --global')) return { code: 1, out: '' }
  if (a.startsWith('status --porcelain=v2')) return { code: 0, out: STATUS }
  if (a.startsWith('for-each-ref refs/heads --format=')) return { code: 0, out: `main${Z}origin/main${Z}\nfeature/x${Z}origin/feature/x${Z}\n` }
  if (a.startsWith('for-each-ref refs/heads --merged')) return { code: 0, out: 'main\nfeature/x\n' }
  if (a.startsWith('rev-list --count')) return { code: 0, out: '2\n' }
  if (a.startsWith('rev-list --left-right')) return { code: 0, out: '0\t2\n' }
  if (a.startsWith('log -n 5')) return { code: 0, out: `abc1234${Z}Ralf${Z}1700000000${Z}feat: vorher\n` }
  if (a === 'branch --show-current') return { code: 0, out: 'feature/x\n' }
  if (a.startsWith('config --get branch.')) return { code: 0, out: 'origin\n' }
  if (a === 'remote') return { code: 0, out: 'origin\n' }
  if (a.startsWith('symbolic-ref')) return { code: 0, out: 'origin/main\n' }
  if (a.startsWith('rev-parse --path-format=absolute --git-path')) return { code: 0, out: '' }
  if (a.startsWith('rev-parse -q --verify')) return { code: 1, out: '' }
  if (a.startsWith('config --get-regexp') || a.startsWith('config --get git-town')) return { code: 1, out: '' }
  if (a.startsWith('config --worktree')) return { code: 1, out: '' }
  return { code: 0, out: '' }
}

function wireGit(on: Parameters<TestBody>[1], calls: string[][], risky = false, actionRisky = false) {
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  on('ui.toast', () => ({ value: undefined }) as never)
  on('ui.open', () => ({ value: { isPlaced: true as const } }))
  on('process.run', (_$, e) => {
    const argv = (e as { argv: readonly string[] }).argv.slice(4).filter((x, i, all) => !(x === '-c' || all[i - 1] === '-c'))
    calls.push([...argv])
    const f = fakeGit(argv, risky, actionRisky)
    return { value: { exitCode: f.code, stdout: f.out, stderr: f.err ?? '', isStdoutTruncated: false, isStderrTruncated: false } } as never
  })
  on('model.complete', () => ({ value: { isAnswered: true, text: '✨ feat(mods): Git-Panel' } }) as never)
}

async function waitFind(view: { find(q: { type: 'Text'; text: string | RegExp }): Promise<unknown> }, text: string | RegExp) {
  for (let i = 0; i < 3000; i++) {
    const hit = await view.find({ type: 'Text', text })
    if (hit !== undefined) return hit
    await Promise.resolve()
  }
  return undefined
}

test('Git-Panel: Stand, Ablauf-Leiste und Hauptaktion; Commit mit Haikus Vorschlag', async ($, on) => {
  mock.clock(on, { now: 1_700_000_600_000 })
  const calls: string[][] = []
  wireGit(on, calls)
  // Git ist ein Reiter des einen Cockpit-Panels; /cockpit-git öffnet ihn.
  await $.command.run({ command: 'cockpit-git', args: '' } as never)
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: GIT_PROPS })
  expect(await waitFind(pane, '● 2 Dateien geändert')).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: '● Commit' })).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: 'Branches (2)' })).not.toBe(undefined)
  // Haiku schlägt vor, sobald etwas gestaged ist; Committen nimmt den Vorschlag.
  for (let i = 0; i < 3000 && !JSON.stringify(await pane.find({ type: 'Input', key: 'git-msg' })).includes('Git-Panel'); i++) await Promise.resolve()
  await pane.press({ key: 'git-m-commit' })
  for (let i = 0; i < 3000 && !calls.some(c => c[0] === 'commit'); i++) await Promise.resolve()
  expect(calls.find(c => c[0] === 'commit')).toEqual(['commit', '-m', '✨ feat(mods): Git-Panel'])
  // Nur lesende Befehle beim Laden, kein Netzwerk.
  expect(calls.some(c => ['fetch', 'pull', 'push'].includes(c[0] ?? ''))).toBe(false)
})

test('Git-Panel: Verwerfen fragt erst nach, zweiter Klick führt aus', async ($, on) => {
  const clock = mock.clock(on, { now: 1_700_000_600_000 })
  const calls: string[][] = []
  wireGit(on, calls)
  // Git ist ein Reiter des einen Cockpit-Panels; /cockpit-git öffnet ihn.
  await $.command.run({ command: 'cockpit-git', args: '' } as never)
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: GIT_PROPS })
  expect(await waitFind(pane, '● 2 Dateien geändert')).not.toBe(undefined)
  await pane.press({ key: 'git-discard' })
  for (let i = 0; i < 3000 && !JSON.stringify(await pane.find({ type: 'Button', key: 'git-discard' })).includes('Wirklich'); i++) await Promise.resolve()
  expect(calls.some(c => c[0] === 'restore')).toBe(false)
  // Die Klick-Sperre von `act` (600 ms) muss vorbei sein, die Rückfrage (6 s) noch offen.
  await clock.advance(1_000)
  await pane.press({ key: 'git-discard' })
  for (let i = 0; i < 3000 && !calls.some(c => c[0] === 'restore'); i++) await Promise.resolve()
  expect(calls.find(c => c[0] === 'restore')).toEqual(['restore', '--worktree', '--', '.'])
})

test('Git-Panel: Klick auf einen Branch wechselt nicht, sondern zeigt Optionen; Wechseln erst über die Option', async ($, on) => {
  mock.clock(on, { now: 1_700_000_600_000 })
  const calls: string[][] = []
  wireGit(on, calls)
  // Git ist ein Reiter des einen Cockpit-Panels; /cockpit-git öffnet ihn.
  await $.command.run({ command: 'cockpit-git', args: '' } as never)
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: GIT_PROPS })
  expect(await waitFind(pane, 'Branches (2)')).not.toBe(undefined)
  expect(await pane.find({ type: 'Button', key: 'git-o-switch' })).toBe(undefined)
  await pane.press({ key: 'git-sel-main' })
  for (let i = 0; i < 3000 && !(await pane.find({ type: 'Button', key: 'git-o-switch' })); i++) await Promise.resolve()
  expect(await pane.find({ type: 'Button', key: 'git-o-rebase' })).not.toBe(undefined)
  expect(calls.some(c => c[0] === 'switch')).toBe(false)
  await pane.press({ key: 'git-o-switch' })
  for (let i = 0; i < 3000 && !calls.some(c => c[0] === 'switch'); i++) await Promise.resolve()
  expect(calls.find(c => c[0] === 'switch')).toEqual(['switch', 'main'])
})

test('Git-Panel: Schlüssel wie credential.helper sperren Aktionen, Lesen bleibt; Branch-Namen mit Minus werden abgelehnt', async ($, on) => {
  const clock = mock.clock(on, { now: 1_700_000_600_000 })
  const calls: string[][] = []
  wireGit(on, calls, false, true)
  // Git ist ein Reiter des einen Cockpit-Panels; /cockpit-git öffnet ihn.
  await $.command.run({ command: 'cockpit-git', args: '' } as never)
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: GIT_PROPS })
  expect(await waitFind(pane, '● 2 Dateien geändert')).not.toBe(undefined)
  await pane.press({ key: 'git-stageAll' })
  expect(await waitFind(pane, /Gesperrt: Die Repo-Konfiguration/)).not.toBe(undefined)
  expect(calls.some(c => c[0] === 'add')).toBe(false)
  await clock.advance(1_000)
  expect(isBranchName('--upload-pack=x')).toBe(false)
})

test('Git-Panel: riskante Repo-Konfiguration sperrt alles; Terminal zeichnet als Text', async ($, on) => {
  mock.clock(on, { now: 1_700_000_600_000 })
  const calls: string[][] = []
  wireGit(on, calls, true)
  // Git ist ein Reiter des einen Cockpit-Panels; /cockpit-git öffnet ihn.
  await $.command.run({ command: 'cockpit-git', args: '' } as never)
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'terminal', component: 'Pane', requestId: 'tenvan-cc', props: GIT_PROPS })
  expect(await waitFind(pane, /Git-Aktionen gesperrt/)).not.toBe(undefined)
  expect(calls.some(c => c[0] === 'status')).toBe(false)
})
