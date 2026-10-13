// Merge/Pull Requests auf GitLab, GitHub und Bitbucket Server: erkennen, beobachten, anlegen (Ideen u. a. aus
// alexjacobs08/guided-mr; eigene Umsetzung). Rein, ohne `$`: Remote erkennen, API-Antworten (`glab api`, `gh`, Bitbucket
// REST 1.0) in denselben kleinen Zustand `Mr` übersetzen (Zustände und Pipeline-Status im GitLab-Vokabular), Änderungen
// als Ereignisse melden, Prompts für Entwurf und Review-Anmerkungen bauen. Gefragt wird in register.tsx.

import type { Forge, Mr, MrMine } from '../types'

export type MrEvent = 'pipelineSuccess' | 'pipelineFailed' | 'notes' | 'approved' | 'merged' | 'closed' | 'conflict' | 'threadsDone'

// Remote (ssh, scp-artig oder https) → Plattform, Host und Projektpfad; unbekannte Hosts null.
// Bitbucket Server erkannt am Namen, am SSH-Port 7999 oder am https-Pfad /scm/; der Pfad ohne „scm/“ ist KEY/slug.
// ponytail: erkannt am Hostnamen; eine Instanz unter anderem Namen bräuchte eine Liste in den Einstellungen.
export function forgeRemote(url: string): { kind: Forge; host: string; project: string } | null {
  const u = url.trim()
  const scp = /^(?:[\w.-]+@)?([\w.-]+):(?!\/\/)(.+?)(?:\.git)?\/?$/.exec(u)
  const full = /^(?:https?|ssh):\/\/(?:[^@/]+@)?([\w.-]+)(?::(\d+))?\/(.+?)(?:\.git)?\/?$/.exec(u)
  const host = (scp?.[1] ?? full?.[1] ?? '').toLowerCase()
  const path = scp?.[2] ?? full?.[3] ?? ''
  if (host === '' || path === '') return null
  if (/gitlab/.test(host)) return { kind: 'gitlab', host, project: path }
  if (/github/.test(host)) return { kind: 'github', host, project: path }
  if (/bitbucket/.test(host) || full?.[2] === '7999' || path.startsWith('scm/')) return { kind: 'bitbucket', host, project: path.replace(/^scm\//, '') }
  return null
}

// Kurzzeichen wie auf der Plattform: GitLab „!12“, GitHub und Bitbucket „#12“.
export const mrRef = (m: Pick<Mr, 'forge' | 'iid'>): string => `${m.forge === 'gitlab' ? '!' : '#'}${m.iid}`

const RUNNING = new Set(['created', 'waiting_for_resource', 'preparing', 'pending', 'running', 'scheduled'])
export const isPipelineRunning = (m: Mr | null): boolean => m?.pipeline != null && RUNNING.has(m.pipeline.status)

// Polling: alle 15 s, solange eine Pipeline läuft, sonst jede Minute.
export const pollDelayMs = (m: Mr | null): number => (isPipelineRunning(m) ? 15_000 : 60_000)

type Obj = Record<string, unknown>
const obj = (v: unknown): Obj => (typeof v === 'object' && v !== null ? (v as Obj) : {})
const str = (v: unknown): string => (typeof v === 'string' ? v : '')
const num = (v: unknown): number => (typeof v === 'number' && Number.isFinite(v) ? v : 0)

// Offene Review-Threads: Diskussionen mit mindestens einer lösbaren, noch nicht gelösten Notiz.
export function openThreads(discussions: unknown): { path: string; line: number | null; author: string; body: string }[] {
  if (!Array.isArray(discussions)) return []
  const out: { path: string; line: number | null; author: string; body: string }[] = []
  for (const d of discussions) {
    const notes = Array.isArray(obj(d).notes) ? (obj(d).notes as unknown[]).map(obj) : []
    const open = notes.filter(n => n.resolvable === true && n.resolved !== true)
    const first = open[0]
    if (!first) continue
    const pos = obj(first.position)
    const line = typeof pos.new_line === 'number' ? pos.new_line : typeof pos.old_line === 'number' ? pos.old_line : null
    out.push({ path: str(pos.new_path) || str(pos.old_path), line, author: str(obj(first.author).username), body: str(first.body).trim() })
  }
  return out
}

// Ein MR aus der Detail-Antwort, den Approvals und den Diskussionen.
export function toMr(detail: unknown, approvals: unknown, discussions: unknown): Mr | null {
  const d = obj(detail)
  if (typeof d.iid !== 'number') return null
  const p = d.head_pipeline ?? d.pipeline
  const a = obj(approvals)
  return {
    forge: 'gitlab',
    iid: d.iid,
    title: str(d.title),
    state: str(d.state),
    draft: d.draft === true || d.work_in_progress === true,
    url: str(d.web_url),
    source: str(d.source_branch),
    target: str(d.target_branch),
    mergeStatus: str(d.detailed_merge_status) || str(d.merge_status),
    conflicts: d.has_conflicts === true,
    notes: num(d.user_notes_count),
    threads: openThreads(discussions).length,
    pipeline: p ? { id: num(obj(p).id), status: str(obj(p).status), url: str(obj(p).web_url) } : null,
    // Ohne Approval-Regeln (GitLab Free, z. B. auf selbst gehosteten Instanzen) fehlt `approvals_required`; `approved_by` gibt es trotzdem.
    description: str(d.description).trim(),
    author: str(obj(d.author).username),
    updatedAt: str(d.updated_at),
    changes: str(d.changes_count),
    threadList: openThreads(discussions).slice(0, 20),
    approvals: Array.isArray(a.approved_by) || typeof a.approvals_required === 'number'
      ? { required: num(a.approvals_required), left: num(a.approvals_left), by: (Array.isArray(a.approved_by) ? a.approved_by : []).map(x => str(obj(obj(x).user).username)).filter(Boolean) }
      : null,
  }
}

// Eigene offene MRs aus `merge_requests?scope=created_by_me` (instanzweit).
export function toMine(list: unknown): MrMine[] {
  if (!Array.isArray(list)) return []
  return list.map(obj).filter(x => typeof x.iid === 'number').map(x => ({ ref: str(obj(x.references).full) || `!${x.iid as number}`, title: str(x.title), url: str(x.web_url), draft: x.draft === true }))
}

type Thread = ReturnType<typeof openThreads>[number]

// ── GitHub (`gh pr view --json …`, `gh api graphql`, `gh search prs --json …`) ──

// Checks und Statuses eines PR → ein Pipeline-Zustand: rot vor laufend vor grün. `id` ist der Actions-Run eines roten
// (sonst ersten) Checks, für „erneut starten“; externe CI ohne Run-Id ergibt 0.
export function githubChecks(rollup: unknown, prUrl: string): Mr['pipeline'] {
  if (!Array.isArray(rollup) || rollup.length === 0) return null
  const items = rollup.map(obj).map(c => {
    const raw = (str(c.conclusion) || str(c.state) || str(c.status)).toUpperCase()
    const tone = /FAILURE|ERROR|TIMED_OUT|CANCELLED|ACTION_REQUIRED|STARTUP_FAILURE/.test(raw) ? 'failed'
      : /PENDING|QUEUED|IN_PROGRESS|WAITING|REQUESTED|EXPECTED/.test(raw) ? 'running'
      : 'success'
    return { tone, url: str(c.detailsUrl) || str(c.targetUrl) }
  })
  const status = items.some(i => i.tone === 'failed') ? 'failed' : items.some(i => i.tone === 'running') ? 'running' : 'success'
  const pick = items.find(i => i.tone === status) ?? items[0]
  const run = /\/actions\/runs\/(\d+)/.exec(pick?.url ?? '')
  return { id: run ? Number(run[1]) : 0, status, url: run ? (pick?.url ?? '') : `${prUrl}/checks` }
}

// Offene Review-Threads aus `reviewThreads` (GraphQL); nur ungelöste.
export function githubThreads(gql: unknown): Thread[] {
  const nodes = obj(obj(obj(obj(obj(gql).data).repository).pullRequest).reviewThreads).nodes
  if (!Array.isArray(nodes)) return []
  return nodes.map(obj).filter(n => n.isResolved === false).map(n => {
    const first = obj((obj(n.comments).nodes as unknown[] | undefined)?.[0])
    return { path: str(n.path), line: typeof n.line === 'number' ? n.line : null, author: str(obj(first.author).login), body: str(first.body).trim() }
  })
}

export function fromGithub(view: unknown, gql: unknown): Mr | null {
  const d = obj(view)
  if (typeof d.number !== 'number') return null
  const state = str(d.state)
  const reviews = Array.isArray(d.latestReviews) ? d.latestReviews.map(obj) : []
  const decision = str(d.reviewDecision)
  const threads = githubThreads(gql)
  const merge = str(d.mergeStateStatus)
  return {
    forge: 'github',
    iid: d.number,
    title: str(d.title),
    state: state === 'OPEN' ? 'opened' : state === 'MERGED' ? 'merged' : 'closed',
    draft: d.isDraft === true,
    url: str(d.url),
    source: str(d.headRefName),
    target: str(d.baseRefName),
    // CLEAN/HAS_HOOKS/UNSTABLE: GitHub lässt mergen; sonst der Grund (blocked, behind, dirty, draft …).
    mergeStatus: /^(CLEAN|HAS_HOOKS|UNSTABLE)$/.test(merge) ? 'mergeable' : merge.toLowerCase(),
    conflicts: str(d.mergeable) === 'CONFLICTING',
    notes: (Array.isArray(d.comments) ? d.comments.length : 0) + (Array.isArray(d.reviews) ? d.reviews.length : 0),
    threads: threads.length,
    pipeline: githubChecks(d.statusCheckRollup, str(d.url)),
    description: str(d.body).trim(),
    author: str(obj(d.author).login),
    updatedAt: str(d.updatedAt),
    changes: typeof d.changedFiles === 'number' ? String(d.changedFiles) : '',
    threadList: threads.slice(0, 20),
    // Ohne Branch-Schutz gibt es keine Pflicht; `reviewDecision` sagt nur, ob noch eine Freigabe fehlt.
    approvals: {
      required: decision === '' ? 0 : 1,
      left: decision === 'REVIEW_REQUIRED' || decision === 'CHANGES_REQUESTED' ? 1 : 0,
      by: reviews.filter(r => str(r.state) === 'APPROVED').map(r => str(obj(r.author).login)).filter(Boolean),
    },
  }
}

// `gh search prs --json number,title,url,isDraft,repository`
export function githubMine(list: unknown): MrMine[] {
  if (!Array.isArray(list)) return []
  return list.map(obj).filter(x => typeof x.number === 'number').map(x => ({ ref: `${str(obj(x.repository).nameWithOwner)}#${x.number as number}`, title: str(x.title), url: str(x.url), draft: x.isDraft === true }))
}

// ── Bitbucket Server / Data Center (REST 1.0, getestet gegen 10.2) ──

// Offene Threads: Kommentare (Wurzel) mit `threadResolved: false`, dazu offene Aufgaben (Severity BLOCKER).
export function bitbucketThreads(activities: unknown): Thread[] {
  const values = obj(activities).values
  if (!Array.isArray(values)) return []
  return values.map(obj).filter(a => a.action === 'COMMENTED' && a.commentAction === 'ADDED').flatMap(a => {
    const c = obj(a.comment)
    const open = c.threadResolved === false || (c.severity === 'BLOCKER' && c.state === 'OPEN')
    if (!open) return []
    const anchor = obj(a.commentAnchor ?? c.anchor)
    return [{ path: str(anchor.path), line: typeof anchor.line === 'number' ? anchor.line : null, author: str(obj(c.author).name), body: str(c.text).trim() }]
  })
}

// Build-Status des letzten Commits → Pipeline-Zustand; Bitbucket kennt kein „erneut starten“ (id 0).
export function bitbucketBuilds(builds: unknown): Mr['pipeline'] {
  const values = obj(builds).values
  if (!Array.isArray(values) || values.length === 0) return null
  const states = values.map(obj).map(b => str(b.state))
  const status = states.includes('FAILED') ? 'failed' : states.includes('INPROGRESS') ? 'running' : 'success'
  const pick = values.map(obj).find(b => str(b.state) === ({ failed: 'FAILED', running: 'INPROGRESS', success: 'SUCCESSFUL' } as const)[status]) ?? obj(values[0])
  return { id: 0, status, url: str(pick.url) }
}

const bbUrl = (x: Obj): string => str(obj((obj(x.links).self as unknown[] | undefined)?.[0]).href)

export function fromBitbucket(pr: unknown, merge: unknown, builds: unknown, activities: unknown): Mr | null {
  const d = obj(pr)
  if (typeof d.id !== 'number') return null
  const state = str(d.state)
  const m = obj(merge)
  const threads = bitbucketThreads(activities)
  const reviewers = Array.isArray(d.reviewers) ? d.reviewers.map(obj) : []
  const veto = Array.isArray(m.vetoes) ? str(obj(m.vetoes[0]).summaryMessage) : ''
  const comments = Array.isArray(obj(activities).values) ? (obj(activities).values as unknown[]).map(obj).filter(a => a.action === 'COMMENTED').length : 0
  return {
    forge: 'bitbucket',
    iid: d.id,
    title: str(d.title),
    state: state === 'OPEN' ? 'opened' : state === 'MERGED' ? 'merged' : 'closed',
    draft: d.draft === true,
    url: bbUrl(d),
    source: str(obj(d.fromRef).displayId),
    target: str(obj(d.toRef).displayId),
    mergeStatus: m.canMerge === true ? 'mergeable' : veto || (state === 'OPEN' ? 'blocked' : state.toLowerCase()),
    conflicts: m.conflicted === true,
    notes: comments,
    threads: threads.length,
    pipeline: bitbucketBuilds(builds),
    description: str(d.description).trim(),
    author: str(obj(obj(d.author).user).name),
    updatedAt: typeof d.updatedDate === 'number' ? new Date(d.updatedDate).toISOString() : '',
    changes: '',
    threadList: threads.slice(0, 20),
    approvals: { required: 0, left: 0, by: reviewers.filter(r => r.approved === true || r.status === 'APPROVED').map(r => str(obj(r.user).name)).filter(Boolean) },
  }
}

// `/rest/api/1.0/dashboard/pull-requests` → „KEY/slug#id“
export function bitbucketMine(page: unknown): MrMine[] {
  const values = obj(page).values
  if (!Array.isArray(values)) return []
  return values.map(obj).filter(x => typeof x.id === 'number').map(x => {
    const repo = obj(obj(x.toRef).repository)
    return { ref: `${str(obj(repo.project).key)}/${str(repo.slug)}#${x.id as number}`, title: str(x.title), url: bbUrl(x), draft: x.draft === true }
  })
}

// Was sich zwischen zwei Abfragen desselben MR geändert hat, als Anlass für einen Hinweis.
export function mrEvents(prev: Mr | null, next: Mr | null): MrEvent[] {
  if (prev === null || next === null || prev.iid !== next.iid) return []
  const out: MrEvent[] = []
  if (prev.state === 'opened' && next.state === 'merged') out.push('merged')
  if (prev.state === 'opened' && next.state === 'closed') out.push('closed')
  const was = prev.pipeline?.status ?? ''
  const now = next.pipeline?.status ?? ''
  if (was !== now && now === 'success') out.push('pipelineSuccess')
  if (was !== now && now === 'failed') out.push('pipelineFailed')
  if (next.notes > prev.notes) out.push('notes')
  // Freigabe: Regel erfüllt (left → 0) oder, ohne Regeln, ein neuer Freigebender.
  const allDone = (prev.approvals?.left ?? 0) > 0 && next.approvals !== null && next.approvals.left === 0
  if (allDone || (next.approvals?.by.length ?? 0) > (prev.approvals?.by.length ?? 0)) out.push('approved')
  if (!prev.conflicts && next.conflicts) out.push('conflict')
  if (prev.threads > 0 && next.threads === 0) out.push('threadsDone')
  return out
}

// Kurzform für den Fußzeilen-Knopf: „!12 · ✓“, „!12 · ●“ (läuft), „!12 · ✗“, dazu offene Threads und Konflikt.
export function mrBadge(m: Mr): string {
  if (m.state === 'merged') return `${mrRef(m)} · ✓ merged`
  if (m.state === 'closed') return `${mrRef(m)} · closed`
  const p = m.pipeline === null ? '' : m.pipeline.status === 'success' ? ' · ✓' : m.pipeline.status === 'failed' ? ' · ✗' : isPipelineRunning(m) ? ' · ●' : ''
  return `${mrRef(m)}${m.draft ? ' Draft' : ''}${p}${m.threads > 0 ? ` · ${m.threads} 💬` : ''}${m.conflicts ? ' · ⚠' : ''}`
}

export const pipelineTone = (status: string): 'success' | 'error' | 'warning' | 'accent' =>
  status === 'success' ? 'success' : status === 'failed' || status === 'canceled' ? 'error' : RUNNING.has(status) ? 'accent' : 'warning'

// Mergen ohne Wartezeit nur, wenn GitLab es freigibt; sonst „mergen, sobald die Pipeline durch ist“ (bei laufender Pipeline).
export const canMergeNow = (m: Mr): boolean => m.state === 'opened' && !m.draft && !m.conflicts && m.mergeStatus === 'mergeable'

// Entwurf für Titel und Beschreibung: Haiku bekommt Commits und Diffstat und antwortet als JSON.
export function draftPrompt(branch: string, target: string, log: string, stat: string, lang: 'de' | 'en'): string {
  const language = lang === 'de' ? 'Deutsch' : 'English'
  return [
    `Write a GitLab merge request for branch "${branch}" into "${target}". Language: ${language}.`,
    'Title: imperative, at most 72 characters, no trailing period. Description: Markdown with "## Was" / "## Warum" / "## Prüfen" (or English equivalents), short bullet points, no invented facts.',
    'Answer with JSON only: {"title": "...", "description": "..."}',
    '',
    'Commits:',
    log.slice(0, 6000),
    '',
    'Diffstat:',
    stat.slice(0, 3000),
  ].join('\n')
}

// Die Antwort des Modells; Prosa oder Fences drumherum werden ignoriert. Ohne brauchbares JSON: erster Commit als Titel.
export function parseDraft(text: string, fallbackTitle: string): { title: string; description: string } {
  const a = text.indexOf('{')
  const z = text.lastIndexOf('}')
  try {
    const o = obj(JSON.parse(text.slice(a, z + 1)))
    const title = str(o.title).trim().slice(0, 200)
    if (title !== '') return { title, description: str(o.description).trim() }
  } catch {
    // fällt auf den Commit-Titel zurück
  }
  return { title: fallbackTitle.slice(0, 200), description: '' }
}

// Offene Review-Anmerkungen als Auftrag an Claude.
export function threadsPrompt(m: Mr, threads: ReturnType<typeof openThreads>, lang: 'de' | 'en'): string {
  const head = lang === 'de'
    ? `Bearbeite die offenen Review-Anmerkungen in ${mrRef(m)} („${m.title}“). Prüfe jede am Code, setze sie um oder begründe, warum nicht:`
    : `Address the open review comments on ${mrRef(m)} ("${m.title}"). Check each against the code, apply it or explain why not:`
  const items = threads.map((x, i) => `${i + 1}. ${x.path}${x.line !== null ? `:${x.line}` : ''}${x.author ? ` (@${x.author})` : ''}: ${x.body.replace(/\s+/g, ' ').slice(0, 500)}`)
  return [head, ...items].join('\n')
}
