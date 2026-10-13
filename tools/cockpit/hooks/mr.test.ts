import { expect, test } from 'claude-code/testing'

import { bitbucketMine, canMergeNow, forgeRemote, fromBitbucket, fromGithub, githubChecks, githubMine, mrBadge, mrRef, mrEvents, openThreads, parseDraft, pipelineTone, pollDelayMs, threadsPrompt, toMine, toMr } from './mr'

const DETAIL = {
  iid: 7,
  title: 'Export vereinfachen',
  state: 'opened',
  draft: false,
  web_url: 'https://gitlab.example.com/consulting/x/-/merge_requests/7',
  source_branch: 'feature/export',
  target_branch: 'main',
  detailed_merge_status: 'mergeable',
  has_conflicts: false,
  user_notes_count: 3,
  head_pipeline: { id: 55, status: 'running', web_url: 'https://gitlab.example.com/p/55' },
}
const APPROVALS = { approvals_required: 2, approvals_left: 1, approved_by: [{ user: { username: 'kollegin' } }] }
const DISCUSSIONS = [
  { notes: [{ resolvable: true, resolved: false, body: 'Bitte Test ergänzen', author: { username: 'kollegin' }, position: { new_path: 'src/a.ts', new_line: 12 } }] },
  { notes: [{ resolvable: true, resolved: true, body: 'erledigt' }] },
  { notes: [{ resolvable: false, body: 'Systemnotiz' }] },
]

test('forgeRemote: GitLab, GitHub und Bitbucket Server aus ssh/https, sonst null', () => {
  expect(forgeRemote('git@gitlab.example.com:team/report-tool.git')).toEqual({ kind: 'gitlab', host: 'gitlab.example.com', project: 'team/report-tool' })
  expect(forgeRemote('https://gitlab.example.com/consulting/x.git')).toEqual({ kind: 'gitlab', host: 'gitlab.example.com', project: 'consulting/x' })
  expect(forgeRemote('git@github.com:Tenvan/tool.git')).toEqual({ kind: 'github', host: 'github.com', project: 'Tenvan/tool' })
  expect(forgeRemote('ssh://git@git.example.com:7999/cor/tools.git')).toEqual({ kind: 'bitbucket', host: 'git.example.com', project: 'cor/tools' })
  expect(forgeRemote('https://git.example.com/scm/OX/aide.git')).toEqual({ kind: 'bitbucket', host: 'git.example.com', project: 'OX/aide' })
  expect(forgeRemote('https://example.com/a/b.git')).toBe(null)
  expect(forgeRemote('')).toBe(null)
})

test('GitHub: PR aus gh-JSON, Checks als Pipeline, offene Review-Threads, eigene PRs', () => {
  const view = {
    number: 5, title: 'T', state: 'OPEN', isDraft: false, url: 'https://github.com/o/r/pull/5', headRefName: 'feat', baseRefName: 'main',
    mergeable: 'MERGEABLE', mergeStateStatus: 'CLEAN', reviewDecision: 'APPROVED', comments: [{}], reviews: [{}, {}],
    latestReviews: [{ state: 'APPROVED', author: { login: 'anna' } }, { state: 'COMMENTED', author: { login: 'bob' } }],
    statusCheckRollup: [{ status: 'COMPLETED', conclusion: 'SUCCESS', detailsUrl: 'https://github.com/o/r/actions/runs/11/job/1' }, { status: 'COMPLETED', conclusion: 'FAILURE', detailsUrl: 'https://github.com/o/r/actions/runs/22/job/2' }],
    author: { login: 'ralf' }, body: 'Beschreibung', updatedAt: '2026-10-13T10:00:00Z', changedFiles: 3,
  }
  const gql = { data: { repository: { pullRequest: { reviewThreads: { nodes: [
    { isResolved: false, path: 'a.ts', line: 4, comments: { nodes: [{ author: { login: 'anna' }, body: 'Bitte umbenennen' }] } },
    { isResolved: true, path: 'b.ts', line: 1, comments: { nodes: [] } },
  ] } } } } }
  const m = fromGithub(view, gql)!
  expect(m).toMatchObject({ forge: 'github', iid: 5, state: 'opened', mergeStatus: 'mergeable', notes: 3, threads: 1, changes: '3', pipeline: { id: 22, status: 'failed' } })
  expect(m.approvals).toEqual({ required: 1, left: 0, by: ['anna'] })
  expect(m.threadList[0]).toEqual({ path: 'a.ts', line: 4, author: 'anna', body: 'Bitte umbenennen' })
  expect(mrRef(m)).toBe('#5')
  expect(canMergeNow(m)).toBe(true)
  expect(githubChecks([{ __typename: 'StatusContext', state: 'PENDING', targetUrl: 'https://ci/x' }], 'u')).toEqual({ id: 0, status: 'running', url: 'u/checks' })
  expect(githubChecks([], 'u')).toBe(null)
  expect(githubMine([{ number: 9, title: 'X', url: 'u9', isDraft: true, repository: { nameWithOwner: 'o/r' } }])).toEqual([{ ref: 'o/r#9', title: 'X', url: 'u9', draft: true }])
})

test('Bitbucket Server: PR, Merge-Veto, Builds, ungelöste Threads und Dashboard', () => {
  const pr = {
    id: 7, version: 4, title: 'T', state: 'OPEN', draft: false, description: 'D', updatedDate: Date.UTC(2026, 9, 13),
    fromRef: { displayId: 'feat', latestCommit: 'abc' }, toRef: { displayId: 'main', repository: { slug: 'app', project: { key: 'COR' } } },
    author: { user: { name: 'rstich' } }, reviewers: [{ approved: true, status: 'APPROVED', user: { name: 'lg' } }, { approved: false, status: 'UNAPPROVED', user: { name: 'x' } }],
    links: { self: [{ href: 'https://git.example.com/projects/COR/repos/app/pull-requests/7' }] },
  }
  const merge = { canMerge: false, conflicted: false, vetoes: [{ summaryMessage: 'Mindestens 2 Freigaben' }] }
  const builds = { values: [{ state: 'SUCCESSFUL', url: 'https://ci/1' }, { state: 'INPROGRESS', url: 'https://ci/2' }] }
  const activities = { values: [
    { action: 'COMMENTED', commentAction: 'ADDED', comment: { text: 'Bitte prüfen', threadResolved: false, severity: 'NORMAL', state: 'OPEN', author: { name: 'lg' } }, commentAnchor: { path: 'a.py', line: 12 } },
    { action: 'COMMENTED', commentAction: 'ADDED', comment: { text: 'erledigt', threadResolved: true, severity: 'NORMAL', state: 'OPEN', author: { name: 'lg' } } },
    { action: 'APPROVED' },
  ] }
  const m = fromBitbucket(pr, merge, builds, activities)!
  expect(m).toMatchObject({ forge: 'bitbucket', iid: 7, state: 'opened', source: 'feat', target: 'main', mergeStatus: 'Mindestens 2 Freigaben', notes: 2, threads: 1, pipeline: { id: 0, status: 'running', url: 'https://ci/2' }, author: 'rstich' })
  expect(m.approvals?.by).toEqual(['lg'])
  expect(m.threadList[0]).toEqual({ path: 'a.py', line: 12, author: 'lg', body: 'Bitte prüfen' })
  expect(mrBadge(m)).toBe('#7 · ● · 1 💬')
  expect(fromBitbucket({ errors: [] }, null, null, null)).toBe(null)
  expect(bitbucketMine({ values: [pr] })).toEqual([{ ref: 'COR/app#7', title: 'T', url: 'https://git.example.com/projects/COR/repos/app/pull-requests/7', draft: false }])
})

test('toMr: Status, Pipeline, Approvals und offene Threads aus der REST-Antwort', () => {
  const m = toMr(DETAIL, APPROVALS, DISCUSSIONS)
  expect(m).toMatchObject({ iid: 7, state: 'opened', mergeStatus: 'mergeable', notes: 3, threads: 1, pipeline: { id: 55, status: 'running' } })
  expect(m?.approvals).toEqual({ required: 2, left: 1, by: ['kollegin'] })
  expect(toMr({ error: 'x' }, null, null)).toBe(null)
  expect(pollDelayMs(m)).toBe(15_000)
  expect(pollDelayMs(null)).toBe(60_000)
  expect(pipelineTone('failed')).toBe('error')
  expect(canMergeNow(m!)).toBe(true)
  expect(canMergeNow({ ...m!, draft: true })).toBe(false)
})

test('mrEvents: Pipeline grün, neue Kommentare, Freigabe, Merge, Konflikt', () => {
  const a = toMr(DETAIL, APPROVALS, DISCUSSIONS)!
  const b = { ...a, notes: 4, pipeline: { ...a.pipeline!, status: 'success' }, approvals: { ...a.approvals!, left: 0 }, threads: 0 }
  expect(mrEvents(a, b).sort()).toEqual(['approved', 'notes', 'pipelineSuccess', 'threadsDone'])
  expect(mrEvents(a, { ...a, state: 'merged' })).toEqual(['merged'])
  expect(mrEvents(a, { ...a, conflicts: true })).toEqual(['conflict'])
  expect(mrEvents(null, a)).toEqual([])
})

test('Entwurf, eigene MRs und Review-Auftrag', () => {
  expect(parseDraft('Klar:\n```json\n{"title":"Export vereinfachen","description":"## Was"}\n```', 'fallback')).toEqual({ title: 'Export vereinfachen', description: '## Was' })
  expect(parseDraft('Unsinn', 'feat: erster Commit')).toEqual({ title: 'feat: erster Commit', description: '' })
  expect(toMine([{ iid: 3, title: 'T', web_url: 'u', references: { full: 'g/p!3' }, draft: true }])).toEqual([{ ref: 'g/p!3', title: 'T', url: 'u', draft: true }])
  const text = threadsPrompt(toMr(DETAIL, APPROVALS, DISCUSSIONS)!, openThreads(DISCUSSIONS), 'de')
  expect(text).toContain('in !7 (')
  expect(text).toContain('1. src/a.ts:12 (@kollegin): Bitte Test ergänzen')
})

test('toMr: ohne Approval-Regeln (GitLab Free) bleiben die Freigebenden sichtbar', () => {
  const m = toMr(DETAIL, { approved: true, approvals_required: null, approvals_left: null, approved_by: [{ user: { username: 'chef' } }] }, [])
  expect(m?.approvals).toEqual({ required: 0, left: 0, by: ['chef'] })
  expect(toMr(DETAIL, null, null)?.approvals).toBe(null)
})

test('mrBadge: Kurzform für den Fußzeilen-Knopf', () => {
  const m = toMr(DETAIL, APPROVALS, DISCUSSIONS)!
  expect(mrBadge(m)).toBe('!7 · ● · 1 💬')
  expect(mrBadge({ ...m, pipeline: { ...m.pipeline!, status: 'success' }, threads: 0 })).toBe('!7 · ✓')
  expect(mrBadge({ ...m, state: 'merged' })).toBe('!7 · ✓ merged')
  expect(m.threadList[0]).toEqual({ path: 'src/a.ts', line: 12, author: 'kollegin', body: 'Bitte Test ergänzen' })
})
