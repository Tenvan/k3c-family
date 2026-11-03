// Ein- und Ausgabe der MR/PR-Plattformen: GitLab über `glab`, GitHub über `gh`, Bitbucket Server über REST 1.0 mit
// `$.http.fetch`. Liefert nur Daten und Ergebnisse (`Done`); Zustand, Hinweise und Anzeige bleiben in register.tsx,
// das Übersetzen der Antworten in `Mr` in mr.ts.

import type { BbList, CiJobs, CiPipeline, Forge, Mr, MrDraft, MrMine } from '../types'
import { clean, commitTitle, PIPELINES_MAX, toArtifacts, toJobs, toPipelines, withDetail } from './ci'
import { COMMITS_MAX, toBbBuilds, toBbCommits } from './ci-bitbucket'
import { ghLogText, ghWithDetail, ghWorkflowName, toGhArtifacts, toGhJobs, toGhRuns } from './ci-github'
import { bitbucketMine, fromBitbucket, fromGithub, githubMine, openThreads, toMine, toMr } from './mr'

export type Remote = { kind: Forge; host: string; project: string }
// `$` darf kein Modul verlassen (die Engine prüft das): register.tsx reicht Prozess und HTTP als `io` herein.
export type Io = {
  run: (argv: readonly string[], opts: { timeoutMs: number; stdin?: string; env?: Record<string, string> }) => Promise<{ exitCode: number; stdout: string; stderr: string }>
  fetch: (url: string, init: { method: string; headers: Record<string, string>; body?: string }) => Promise<{ status: number; text: string }>
}
export type Done = { ok: boolean; err: string }
type Thread = Mr['threadList'][number]

const firstLine = (s: string): string => clean(s.trim().split('\n')[0] ?? '').slice(0, 160)

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
// Ein fehlgeschlagener Abruf wirft: sonst hielte das Panel „kein MR“ für einen Befund und böte „MR anlegen“ an, obwohl er nur
// nicht erreichbar war. Bitbucket ohne Zugang bleibt `null` (der Hinweis dazu kommt aus authMissing).
export async function forgeFind(io: Io, r: Remote, branch: string): Promise<number | null> {
  const b = encodeURIComponent(branch)
  if (r.kind === 'gitlab' || r.kind === 'github') {
    const argv = r.kind === 'gitlab'
      ? ['glab', 'api', `projects/:id/merge_requests?state=opened&source_branch=${b}`]
      : ['gh', 'pr', 'list', '-R', ghRepo(r), '--head', branch, '--state', 'open', '--json', 'number', '--limit', '1']
    const x = await cli(io, argv)
    if (!x.ok) throw new Error(x.err || argv[0])
    let list: unknown = null
    try {
      list = JSON.parse(x.out) as unknown
    } catch {
      throw new Error(argv[0] ?? 'cli')
    }
    const first = Array.isArray(list) ? (list[0] as { iid?: unknown; number?: unknown } | undefined) : undefined
    const n = r.kind === 'gitlab' ? first?.iid : first?.number
    return typeof n === 'number' ? n : null
  }
  const x = await bb(io, r, 'GET', `${bbRepo(r)}/pull-requests?at=refs/heads/${b}&direction=OUTGOING&state=OPEN&limit=1`)
  if (x.status === 401 || x.status === 403) return null
  if (x.status < 200 || x.status >= 300) throw new Error(`HTTP ${x.status}`)
  const id = (x.data as { values?: { id?: unknown }[] } | null)?.values?.[0]?.id
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

// `squash`: Commits beim Mergen zu einem zusammenfassen (Knopf neben „Mergen“, Standard aus).
export async function forgeMerge(io: Io, r: Remote, m: Mr, auto: boolean, squash = false): Promise<Done> {
  if (r.kind === 'gitlab') return cli(io, ['glab', 'api', '-X', 'PUT', `projects/:id/merge_requests/${m.iid}/merge`, '-F', `squash=${squash}`, ...(auto ? ['-f', 'merge_when_pipeline_succeeds=true'] : [])])
  if (r.kind === 'github') {
    // gh verlangt eine Methode: die erste, die das Repo erlaubt (Merge-Commit, Squash, Rebase).
    const p = (await cliJson(io, ['gh', 'repo', 'view', ghRepo(r), '--json', 'mergeCommitAllowed,squashMergeAllowed,rebaseMergeAllowed'])) as Record<string, unknown> | null
    const how = squash ? '--squash' : p?.mergeCommitAllowed !== false ? '--merge' : p.squashMergeAllowed !== false ? '--squash' : '--rebase'
    return cli(io, ['gh', 'pr', 'merge', String(m.iid), '-R', ghRepo(r), how, ...(auto ? ['--auto'] : [])])
  }
  const pr = (await bbGet(io, r, `${bbRepo(r)}/pull-requests/${m.iid}`)) as { version?: unknown } | null
  return bbDone(await bb(io, r, 'POST', `${bbRepo(r)}/pull-requests/${m.iid}/merge?version=${String(pr?.version ?? 0)}`, squash ? { strategyId: 'squash' } : {}))
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

// ── CI/CD: ein Adapter je Plattform (GitLab über `glab api`, GitHub Actions über `gh api`), beide liefern dieselbe Form
// (`CiPipeline`, `CiJobs`, Roh-Log). Die vorhandene glab- bzw. gh-Anmeldung trägt den Zugang; das Cockpit sieht nie ein Token. ──

export const ciSupported = (r: Remote | null): r is Remote => r !== null && (r.kind === 'gitlab' || r.kind === 'github')

// Bitbucket Server (nur lesen): die letzten Commits des Branches und der Build-Status, den Bamboo/Jenkins je Commit melden
// (`/rest/build-status/1.0/commits/<sha>`). Zugang wie bei den MR-Funktionen aus dem Git-Credential-Store (`bbHeader`), nie ein Token
// in Datei oder Log. Fehler stehen im Ergebnis (`error`: `auth` ohne Zugang, sonst HTTP-Status), nie als Wurf.
export async function bitbucketBuilds(io: Io, r: Remote, branch: string): Promise<BbList> {
  const at = branch === '' ? '' : `&until=${encodeURIComponent(`refs/heads/${branch}`)}`
  const x = await bb(io, r, 'GET', `${bbRepo(r)}/commits?limit=${COMMITS_MAX}${at}`)
  if (x.status === 401 || x.status === 403) return { commits: [], error: 'auth', branch }
  if (x.status < 200 || x.status >= 300) return { commits: [], error: x.status === 0 ? 'net' : `HTTP ${x.status}`, branch }
  const commits = toBbCommits(x.data)
  const builds = await Promise.all(commits.map(c => bbGet(io, r, `https://${r.host}/rest/build-status/1.0/commits/${c.sha}`)))
  return { commits: commits.map((c, i) => ({ ...c, builds: toBbBuilds(builds[i]) })), error: '', branch }
}

// Artefakte: nur Anzeige (Name, Größe, Link zur Browser-Seite). Ein Download durch das Cockpit entfällt: `$.process.run`/`$.fs.write`/`$.http.fetch`
// sind textbasiert (Binärdaten gingen kaputt), `gh run download` und `glab job artifact` entpacken das Archiv, und der Weg über ein Token
// (`gh auth token`) widerspricht der Regel, dass das Cockpit nie ein Token sieht. Siehe SPEC.md, Abschnitt CI/CD.

type CiAdapter = {
  pipelines: (io: Io, r: Remote) => Promise<CiPipeline[]>
  jobs: (io: Io, r: Remote, p: CiPipeline) => Promise<CiJobs | null>
  log: (io: Io, r: Remote, job: number) => Promise<{ ok: boolean; err: string; text: string }>
  jobAction: (io: Io, r: Remote, job: number, action: 'retry' | 'cancel' | 'play') => Promise<Done>
  pipelineAction: (io: Io, r: Remote, id: number, action: 'retry' | 'cancel') => Promise<Done>
  newPipeline: (io: Io, r: Remote, ref: string) => Promise<Done>
}

const gitlabCi: CiAdapter = {
  async pipelines(io) {
    const x = await cli(io, ['glab', 'api', `projects/:id/pipelines?per_page=${PIPELINES_MAX}&order_by=id&sort=desc`])
    if (!x.ok) throw new Error(x.err || 'glab')
    try {
      return toPipelines(JSON.parse(x.out) as unknown)
    } catch {
      throw new Error('glab: ungültige Antwort')
    }
  },
  async jobs(io, _r, p) {
    const [detail, jobs] = await Promise.all([
      cliJson(io, ['glab', 'api', `projects/:id/pipelines/${p.id}`]),
      cliJson(io, ['glab', 'api', `projects/:id/pipelines/${p.id}/jobs?per_page=100`]),
    ])
    if (!Array.isArray(jobs)) return null
    return { pipeline: { ...withDetail(p, detail), title: commitTitle(jobs) }, jobs: toJobs(jobs), artifacts: toArtifacts(jobs, Date.now()) }
  },
  async log(io, _r, job) {
    const x = await cli(io, ['glab', 'api', `projects/:id/jobs/${job}/trace`])
    return { ok: x.ok, err: x.err, text: x.out }
  },
  jobAction: (io, _r, job, action) => cli(io, ['glab', 'api', '-X', 'POST', `projects/:id/jobs/${job}/${action}`]),
  pipelineAction: (io, _r, id, action) => cli(io, ['glab', 'api', '-X', 'POST', `projects/:id/pipelines/${id}/${action}`]),
  newPipeline: (io, _r, ref) => cli(io, ['glab', 'api', '-X', 'POST', 'projects/:id/pipeline', '-f', `ref=${ref}`]),
}

// GitHub: `owner/name` kommt aus dem Remote-URL und landet als ein Argument im API-Pfad (nie in einer Shell); trotzdem nur
// die übliche Form zulassen. Kein Äquivalent gibt es für „play“ (manuelle Jobs), das Abbrechen einzelner Jobs und einen Run ohne
// Workflow-Wahl; ciCaps (ci.ts) blendet sie aus, die Funktionen hier verweigern sie zusätzlich.
const ghBase = (r: Remote): string => {
  if (!/^[\w.-]+\/[\w.-]+$/.test(r.project)) throw new Error('gh: ungültiger Repo-Name')
  return `repos/${r.project}`
}
const ghApi = (r: Remote, path: string, post = false): string[] => ['gh', 'api', '--hostname', r.host, ...(post ? ['-X', 'POST'] : []), path]
const NOT_ON_GITHUB: Done = { ok: false, err: 'GitHub Actions' }

const githubCi: CiAdapter = {
  async pipelines(io, r) {
    const x = await cli(io, ghApi(r, `${ghBase(r)}/actions/runs?per_page=${PIPELINES_MAX}`))
    if (!x.ok) throw new Error(x.err || 'gh')
    try {
      return toGhRuns(JSON.parse(x.out) as unknown)
    } catch {
      throw new Error('gh: ungültige Antwort')
    }
  },
  async jobs(io, r, p) {
    const base = ghBase(r)
    const [detail, jobs, artifacts] = await Promise.all([
      cliJson(io, ghApi(r, `${base}/actions/runs/${p.id}`)),
      cliJson(io, ghApi(r, `${base}/actions/runs/${p.id}/jobs?per_page=100`)),
      cliJson(io, ghApi(r, `${base}/actions/runs/${p.id}/artifacts?per_page=100`)),
    ])
    if (jobs === null || !Array.isArray((jobs as { jobs?: unknown }).jobs)) return null
    return { pipeline: ghWithDetail(p, detail), jobs: toGhJobs(jobs, ghWorkflowName(detail)), artifacts: toGhArtifacts(artifacts, p.url) }
  },
  // `…/logs` antwortet mit einem Redirect auf das Log; `gh api` folgt ihm.
  async log(io, r, job) {
    const x = await cli(io, ghApi(r, `${ghBase(r)}/actions/jobs/${job}/logs`))
    return { ok: x.ok, err: x.err, text: x.ok ? ghLogText(x.out) : '' }
  },
  async jobAction(io, r, job, action) {
    return action === 'retry' ? cli(io, ghApi(r, `${ghBase(r)}/actions/jobs/${job}/rerun`, true)) : NOT_ON_GITHUB
  },
  // retry = nur die fehlgeschlagenen Jobs (und ihre Abhängigen); lehnt GitHub das ab (z. B. abgebrochener Run), alle Jobs neu.
  async pipelineAction(io, r, id, action) {
    const base = `${ghBase(r)}/actions/runs/${id}`
    if (action === 'cancel') return cli(io, ghApi(r, `${base}/cancel`, true))
    const failed = await cli(io, ghApi(r, `${base}/rerun-failed-jobs`, true))
    return failed.ok ? failed : cli(io, ghApi(r, `${base}/rerun`, true))
  },
  newPipeline: () => Promise.resolve(NOT_ON_GITHUB),
}

const ciAdapter = (r: Remote): CiAdapter => (r.kind === 'github' ? githubCi : gitlabCi)

// Neueste Pipelines bzw. Workflow-Runs. Ein fehlgeschlagener Abruf wirft, damit das Panel „nicht erreichbar“ statt „keine Pipelines“ zeigt.
export const ciPipelines = (io: Io, r: Remote): Promise<CiPipeline[]> => ciAdapter(r).pipelines(io, r)

// Jobs einer Pipeline samt Detail (Dauer, Auslöser) und Titel; null, wenn die Jobs nicht ladbar waren.
export const ciPipelineJobs = (io: Io, r: Remote, p: CiPipeline): Promise<CiJobs | null> => ciAdapter(r).jobs(io, r, p)

// Rohes Job-Log; die Aufbereitung (letzte Zeilen, ANSI) macht ci.ts.
export const ciJobLog = (io: Io, r: Remote, job: number): Promise<{ ok: boolean; err: string; text: string }> => ciAdapter(r).log(io, r, job)

// Schreibend, nur per Klick: Job wiederholen, abbrechen oder (manuell) starten.
export const ciJobAction = (io: Io, r: Remote, job: number, action: 'retry' | 'cancel' | 'play'): Promise<Done> => ciAdapter(r).jobAction(io, r, job, action)

export const ciPipelineAction = (io: Io, r: Remote, id: number, action: 'retry' | 'cancel'): Promise<Done> => ciAdapter(r).pipelineAction(io, r, id, action)

// Neue Pipeline für einen Branch (Ref) anstoßen (nur GitLab).
export const ciNewPipeline = (io: Io, r: Remote, ref: string): Promise<Done> => ciAdapter(r).newPipeline(io, r, ref)
