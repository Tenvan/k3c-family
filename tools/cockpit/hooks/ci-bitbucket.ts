// CI/CD auf Bitbucket Server/Data Center: nur lesen. Bitbucket hat kein eigenes CI; Bamboo, Jenkins & Co. melden ihren Build-Status per
// Build-Status-API an Commits. Das Panel zeigt deshalb die letzten Commits des Branches mit dem gemeldeten Status, nicht Pipelines und Jobs.
// Rein, ohne `$`; die Aufrufe stehen in forge.ts (`bitbucketBuilds`). Aller API-Text läuft durch `str` (ci.ts): Escape-Folgen und
// Steuerzeichen fliegen raus (Security-Fix 86e914e). Links ins Build-System nur als http(s), nie andere Schemata.

import type { BbBuild, BbCommit } from '../types'
import { num, obj, str } from './ci'

export const COMMITS_MAX = 15

// Build-Status-Vokabular von Bitbucket (SUCCESSFUL, FAILED, INPROGRESS) auf das der Badges (success, failed, running …).
export function bbStatus(state: string): string {
  const s = state.toUpperCase()
  if (s === 'SUCCESSFUL') return 'success'
  if (s === 'FAILED') return 'failed'
  if (s === 'INPROGRESS') return 'running'
  if (s === 'CANCELLED' || s === 'CANCELED' || s === 'STOPPED') return 'canceled'
  return s === '' ? 'created' : s.toLowerCase()
}

// Gesamtstand eines Commits: ein fehlgeschlagener Build färbt ihn rot, sonst läuft einer, sonst alle grün; ohne Builds leer.
export function bbRollup(builds: readonly BbBuild[]): string {
  if (builds.length === 0) return ''
  const st = builds.map(b => bbStatus(b.state))
  if (st.includes('failed')) return 'failed'
  if (st.includes('running')) return 'running'
  if (st.every(s => s === 'success')) return 'success'
  return st.find(s => s !== 'success') ?? ''
}

export const bbSafeUrl = (v: unknown): string => {
  const u = str(v)
  return /^https?:\/\/[^\s]+$/i.test(u) ? u : ''
}

const iso = (ms: unknown): string => (typeof ms === 'number' && Number.isFinite(ms) && ms > 0 ? new Date(ms).toISOString() : '')
const firstLine = (v: unknown): string => str(v).split('\n')[0]?.trim() ?? ''

// `…/commits?until=…` → `{ values: [{ id, displayId, message, author: { name, displayName }, authorTimestamp }] }`; Builds kommen danach je Commit.
export function toBbCommits(raw: unknown): BbCommit[] {
  const list = obj(raw).values
  if (!Array.isArray(list)) return []
  return list.map(obj).filter(x => /^[0-9a-f]{7,64}$/i.test(str(x.id))).map(x => {
    const a = obj(x.author)
    return { sha: str(x.id), short: str(x.displayId) || str(x.id).slice(0, 8), title: firstLine(x.message), author: str(a.displayName) || str(a.name), at: iso(x.authorTimestamp), builds: [] }
  })
}

// `/rest/build-status/1.0/commits/<sha>` → `{ values: [{ state, key, name, url, description, dateAdded, duration? }] }`; neueste zuerst.
// `duration` (ms) kennen nur neuere Versionen; fehlt es, bleibt die Dauer 0 (unbekannt).
export function toBbBuilds(raw: unknown): BbBuild[] {
  const list = obj(raw).values
  if (!Array.isArray(list)) return []
  return list.map(obj).filter(x => str(x.state) !== '').map(x => ({
    state: str(x.state),
    key: str(x.key),
    name: str(x.name) || str(x.key),
    url: bbSafeUrl(x.url),
    description: firstLine(x.description),
    at: iso(x.dateAdded),
    durationS: Math.round(num(x.duration) / 1000),
  })).sort((a, b) => Date.parse(b.at) - Date.parse(a.at) || 0)
}
