// Ein- und Ausgabe der MR/PR-Plattformen: GitLab über `glab`, GitHub über `gh`, Bitbucket Server über REST 1.0 mit
// `$.http.fetch`. Liefert nur Daten und Ergebnisse (`Done`); Zustand, Hinweise und Anzeige bleiben in register.tsx,
// das Übersetzen der Antworten in `Mr` in mr.ts.

import type { Forge, Mr, MrDraft, MrMine } from '../types'
import { bitbucketMine, fromBitbucket, fromGithub, githubMine, openThreads, toMine, toMr } from './mr'

export type Remote = { kind: Forge; host: string; project: string }
// `$` darf kein Modul verlassen (die Engine prüft das): register.tsx reicht Prozess und HTTP als `io` herein.
export type Io = {
  run: (argv: readonly string[], opts: { timeoutMs: number; stdin?: string; env?: Record<string, string> }) => Promise<{ exitCode: number; stdout: string; stderr: string }>
  fetch: (url: string, init: { method: string; headers: Record<string, string>; body?: string }) => Promise<{ status: number; text: string }>
}
export type Done = { ok: boolean; err: string }
type Thread = Mr['threadList'][number]

const firstLine = (s: string): string => (s.trim().split('\n')[0] ?? '').slice(0, 160)

async function cli(io: Io, argv: readonly string[], stdin?: string): Promise<Done & { out: string }> {
  const r = await io.run(argv, stdin === undefined ? { timeoutMs: 60_000 } : { timeoutMs: 60_000, stdin }).catch(() => null)
  if (r === null) return { ok: false, err: argv[0] ?? '', out: '' }
  return { ok: r.exitCode === 0, err: firstLine(r.stderr || r.stdout), out: r.stdout }
}

async function cliJson(io: Io, argv: readonly string[]): Promise<unknown> {
  const r = await cli(io, argv)
  if (!r.ok) return null
  try {
    return JSON.parse(r.out) as unknown
  } catch {
    return null
  }
}

// ── GitHub ──
const ghRepo = (r: Remote): string => `${r.host}/${r.project}`
const GH_VIEW = 'number,title,state,isDraft,url,headRefName,baseRefName,mergeable,mergeStateStatus,statusCheckRollup,latestReviews,reviews,reviewDecision,comments,author,body,updatedAt,changedFiles'
const GH_THREADS = 'query($o:String!,$n:String!,$p:Int!){repository(owner:$o,name:$n){pullRequest(number:$p){reviewThreads(first:100){nodes{isResolved path line comments(first:1){nodes{author{login} body}}}}}}}'

function ghThreadsArgs(r: Remote, iid: number): string[] {
  const [owner = '', name = ''] = r.project.split('/')
  return ['gh', 'api', 'graphql', '--hostname', r.host, '-f', `query=${GH_THREADS}`, '-F', `o=${owner}`, '-F', `n=${name}`, '-F', `p=${iid}`]
}

// ── Bitbucket Server ──
// Zugang aus dem Git-Credential-Store (dasselbe Token wie für `git push` über https); nie nachfragen, nie anzeigen.
let bbAuth: { host: string; header: string } | null = null
let bbMissing = ''
export const authMissing = (): string => bbMissing

const B64 = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/'
function base64(s: string): string {
  // UTF-8-Bytes als Zeichen 0–255, ohne TextEncoder/btoa (in der Hook-Umgebung nicht zugesichert).
  const b = encodeURIComponent(s).replace(/%([0-9A-F]{2})/g, (_, h: string) => String.fromCharCode(parseInt(h, 16)))
  let out = ''
  for (let i = 0; i < b.length; i += 3) {
    const n = (b.charCodeAt(i) << 16) | ((b.charCodeAt(i + 1) || 0) << 8) | (b.charCodeAt(i + 2) || 0)
    out += B64[(n >> 18) & 63]! + B64[(n >> 12) & 63]! + (i + 1 < b.length ? B64[(n >> 6) & 63]! : '=') + (i + 2 < b.length ? B64[n & 63]! : '=')
  }
  return out
}

async function bbHeader(io: Io, host: string): Promise<string> {
  if (bbAuth?.host === host) return bbAuth.header
  const r = await io.run(['git', 'credential', 'fill'], { stdin: `protocol=https\nhost=${host}\n\n`, env: { GIT_TERMINAL_PROMPT: '0', GCM_INTERACTIVE: 'never' }, timeoutMs: 15_000 }).catch(() => null)
  const kv = new Map((r?.exitCode === 0 ? r.stdout : '').split(/\r?\n/).map(l => [l.slice(0, l.indexOf('=')), l.slice(l.indexOf('=') + 1)] as const))
  const user = kv.get('username') ?? ''
  const pass = kv.get('password') ?? ''
  if (user === '' || pass === '') {
    bbMissing = host
    return ''
  }
  bbMissing = ''
  bbAuth = { host, header: `Basic ${base64(`${user}:${pass}`)}` }
  return bbAuth.header
}

async function bb(io: Io, r: Remote, method: string, url: string, body?: unknown): Promise<{ status: number; data: unknown }> {
  const auth = await bbHeader(io, r.host)
  if (auth === '') return { status: 401, data: null }
  const headers = { Authorization: auth, Accept: 'application/json', 'Content-Type': 'application/json', 'X-Atlassian-Token': 'no-check' }
  const res = await io.fetch(url, body === undefined ? { method, headers } : { method, headers, body: JSON.stringify(body) }).catch(() => null)
  if (res === null) return { status: 0, data: null }
  // Abgelaufenes Token: beim nächsten Mal neu aus dem Store lesen.
  if (res.status === 401) {
    bbAuth = null
    bbMissing = r.host
  }
  let data: unknown = null
  try {
    data = JSON.parse(res.text) as unknown
  } catch {
    // leerer oder kein JSON-Körper (204)
  }
  return { status: res.status, data }
}

const bbKey = (r: Remote) => {
  const [key = '', slug = ''] = r.project.split('/')
  return { key: key.toUpperCase(), slug }
}
const bbRepo = (r: Remote): string => `https://${r.host}/rest/api/1.0/projects/${bbKey(r).key}/repos/${bbKey(r).slug}`
const bbGet = async (io: Io, r: Remote, url: string): Promise<unknown> => {
  const x = await bb(io, r, 'GET', url)
  return x.status >= 200 && x.status < 300 ? x.data : null
}
function bbDone(x: { status: number; data: unknown }): Done {
  const errors = (x.data as { errors?: { message?: unknown }[] } | null)?.errors
  const msg = typeof errors?.[0]?.message === 'string' ? errors[0].message : `HTTP ${x.status}`
  return { ok: x.status >= 200 && x.status < 300, err: firstLine(msg) }
}

// ── Lesen ──

// Nummer des offenen MR/PR für den Branch, sonst null.
export async function forgeFind(io: Io, r: Remote, branch: string): Promise<number | null> {
  const b = encodeURIComponent(branch)
  if (r.kind === 'gitlab') {
    const list = await cliJson(io, ['glab', 'api', `projects/:id/merge_requests?state=opened&source_branch=${b}`])
    const iid = Array.isArray(list) ? (list[0] as { iid?: unknown } | undefined)?.iid : undefined
    return typeof iid === 'number' ? iid : null
  }
  if (r.kind === 'github') {
    const list = await cliJson(io, ['gh', 'pr', 'list', '-R', ghRepo(r), '--head', branch, '--state', 'open', '--json', 'number', '--limit', '1'])
    const n = Array.isArray(list) ? (list[0] as { number?: unknown } | undefined)?.number : undefined
    return typeof n === 'number' ? n : null
  }
  const page = (await bbGet(io, r, `${bbRepo(r)}/pull-requests?at=refs/heads/${b}&direction=OUTGOING&state=OPEN&limit=1`)) as { values?: { id?: unknown }[] } | null
  const id = page?.values?.[0]?.id
  return typeof id === 'number' ? id : null
}

export async function forgeLoad(io: Io, r: Remote, iid: number): Promise<Mr | null> {
  if (r.kind === 'gitlab') {
    const [detail, approvals, discussions] = await Promise.all([
      cliJson(io, ['glab', 'api', `projects/:id/merge_requests/${iid}`]),
      cliJson(io, ['glab', 'api', `projects/:id/merge_requests/${iid}/approvals`]),
      cliJson(io, ['glab', 'api', `projects/:id/merge_requests/${iid}/discussions?per_page=100`]),
    ])
    return toMr(detail, approvals, discussions)
  }
  if (r.kind === 'github') {
    const [view, gql] = await Promise.all([cliJson(io, ['gh', 'pr', 'view', String(iid), '-R', ghRepo(r), '--json', GH_VIEW]), cliJson(io, ghThreadsArgs(r, iid))])
    return fromGithub(view, gql)
  }
  const base = `${bbRepo(r)}/pull-requests/${iid}`
  const pr = (await bbGet(io, r, base)) as { fromRef?: { latestCommit?: unknown } } | null
  if (pr === null) return null
  const commit = typeof pr.fromRef?.latestCommit === 'string' ? pr.fromRef.latestCommit : ''
  const [merge, builds, activities] = await Promise.all([
    bbGet(io, r, `${base}/merge`),
    commit === '' ? null : bbGet(io, r, `https://${r.host}/rest/build-status/1.0/commits/${commit}`),
    bbGet(io, r, `${base}/activities?limit=200`),
  ])
  return fromBitbucket(pr, merge, builds, activities)
}

// Standard-Branch der Plattform; leer, wenn sie ihn nicht nennt.
export async function forgeDefault(io: Io, r: Remote): Promise<string> {
  if (r.kind === 'gitlab') {
    const p = (await cliJson(io, ['glab', 'api', 'projects/:id'])) as { default_branch?: unknown } | null
    return typeof p?.default_branch === 'string' ? p.default_branch : ''
  }
  if (r.kind === 'github') {
    const p = (await cliJson(io, ['gh', 'repo', 'view', ghRepo(r), '--json', 'defaultBranchRef'])) as { defaultBranchRef?: { name?: unknown } } | null
    return typeof p?.defaultBranchRef?.name === 'string' ? p.defaultBranchRef.name : ''
  }
  const p = (await bbGet(io, r, `${bbRepo(r)}/default-branch`)) as { displayId?: unknown } | null
  return typeof p?.displayId === 'string' ? p.displayId : ''
}

let glabUser = ''

// Eigene offene MRs/PRs und die, die auf dein Review warten (plattformweit).
export async function forgeLists(io: Io, r: Remote): Promise<{ mine: MrMine[]; review: MrMine[] }> {
  if (r.kind === 'gitlab') {
    const mine = toMine(await cliJson(io, ['glab', 'api', 'merge_requests?scope=created_by_me&state=opened&per_page=20']))
    if (glabUser === '') glabUser = String(((await cliJson(io, ['glab', 'api', 'user'])) as { username?: unknown } | null)?.username ?? '')
    const review = glabUser === '' ? [] : toMine(await cliJson(io, ['glab', 'api', `merge_requests?scope=all&state=opened&reviewer_username=${encodeURIComponent(glabUser)}&per_page=20`]))
    return { mine, review }
  }
  if (r.kind === 'github') {
    const q = (who: string) => cliJson(io, ['gh', 'search', 'prs', who, '@me', '--state', 'open', '--json', 'number,title,url,isDraft,repository', '--limit', '20'])
    const [mine, review] = await Promise.all([q('--author'), q('--review-requested')])
    return { mine: githubMine(mine), review: githubMine(review) }
  }
  const dash = `https://${r.host}/rest/api/1.0/dashboard/pull-requests?state=OPEN&limit=20`
  const [mine, review] = await Promise.all([bbGet(io, r, `${dash}&role=AUTHOR`), bbGet(io, r, `${dash}&role=REVIEWER&participantStatus=UNAPPROVED`)])
  return { mine: bitbucketMine(mine), review: bitbucketMine(review) }
}

// Offene Threads für „Anmerkungen → Claude“: GitLab holt alle neu, sonst die (höchstens 20) aus der letzten Abfrage.
export async function forgeThreads(io: Io, r: Remote, m: Mr): Promise<Thread[]> {
  if (r.kind === 'gitlab') return openThreads(await cliJson(io, ['glab', 'api', `projects/:id/merge_requests/${m.iid}/discussions?per_page=100`]))
  return m.threadList
}

// ── Schreiben (nur per Klick) ──

export async function forgeRebase(io: Io, r: Remote, m: Mr): Promise<Done> {
  if (r.kind === 'gitlab') return cli(io, ['glab', 'api', '-X', 'PUT', `projects/:id/merge_requests/${m.iid}/rebase`])
  if (r.kind === 'github') return cli(io, ['gh', 'pr', 'update-branch', String(m.iid), '-R', ghRepo(r), '--rebase'])
  const pr = (await bbGet(io, r, `${bbRepo(r)}/pull-requests/${m.iid}`)) as { version?: unknown } | null
  const { key, slug } = bbKey(r)
  return bbDone(await bb(io, r, 'POST', `https://${r.host}/rest/git/latest/projects/${key}/repos/${slug}/pull-requests/${m.iid}/rebase`, { version: pr?.version ?? 0 }))
}

// Fehlgeschlagene Pipeline erneut starten; Bitbucket hat dafür keine API (der Knopf erscheint nur mit Run-Id).
export async function forgeRetry(io: Io, r: Remote, m: Mr): Promise<Done> {
  if (m.pipeline === null || m.pipeline.id === 0) return { ok: false, err: '–' }
  if (r.kind === 'gitlab') return cli(io, ['glab', 'api', '-X', 'POST', `projects/:id/pipelines/${m.pipeline.id}/retry`])
  if (r.kind === 'github') return cli(io, ['gh', 'run', 'rerun', String(m.pipeline.id), '-R', ghRepo(r), '--failed'])
  return { ok: false, err: 'Bitbucket' }
}

// Auto-Merge bei laufender Pipeline können GitLab und GitHub; Bitbucket Server nicht.
export const canAutoMerge = (m: Mr): boolean => m.forge !== 'bitbucket'

export async function forgeMerge(io: Io, r: Remote, m: Mr, auto: boolean): Promise<Done> {
  if (r.kind === 'gitlab') return cli(io, ['glab', 'api', '-X', 'PUT', `projects/:id/merge_requests/${m.iid}/merge`, ...(auto ? ['-f', 'merge_when_pipeline_succeeds=true'] : [])])
  if (r.kind === 'github') {
    // gh verlangt eine Methode: die erste, die das Repo erlaubt (Merge-Commit, Squash, Rebase).
    const p = (await cliJson(io, ['gh', 'repo', 'view', ghRepo(r), '--json', 'mergeCommitAllowed,squashMergeAllowed,rebaseMergeAllowed'])) as Record<string, unknown> | null
    const how = p?.mergeCommitAllowed !== false ? '--merge' : p.squashMergeAllowed !== false ? '--squash' : '--rebase'
    return cli(io, ['gh', 'pr', 'merge', String(m.iid), '-R', ghRepo(r), how, ...(auto ? ['--auto'] : [])])
  }
  const pr = (await bbGet(io, r, `${bbRepo(r)}/pull-requests/${m.iid}`)) as { version?: unknown } | null
  return bbDone(await bb(io, r, 'POST', `${bbRepo(r)}/pull-requests/${m.iid}/merge?version=${String(pr?.version ?? 0)}`, {}))
}

// MR/PR aus dem Entwurf anlegen; der Branch ist schon gepusht.
export async function forgeCreate(io: Io, r: Remote, d: MrDraft, asDraft: boolean): Promise<Done> {
  if (r.kind === 'gitlab') return cli(io, ['glab', 'mr', 'create', '--source-branch', d.source, '--target-branch', d.target, '--title', d.title, '--description-file', '-', '--yes', ...(asDraft ? ['--draft'] : [])], d.description)
  if (r.kind === 'github') return cli(io, ['gh', 'pr', 'create', '-R', ghRepo(r), '--head', d.source, '--base', d.target, '--title', d.title, '--body-file', '-', ...(asDraft ? ['--draft'] : [])], d.description)
  const { key, slug } = bbKey(r)
  const repo = { slug, project: { key } }
  return bbDone(
    await bb(io, r, 'POST', `${bbRepo(r)}/pull-requests`, {
      title: d.title,
      description: d.description,
      draft: asDraft,
      fromRef: { id: `refs/heads/${d.source}`, repository: repo },
      toRef: { id: `refs/heads/${d.target}`, repository: repo },
    }),
  )
}
