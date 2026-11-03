import { expect, mock, test } from 'claude-code/testing'
import type { TestBody } from 'claude-code/testing'

// Reiter CI/CD mit Bitbucket-Server-Remote gegen Mock-Daten (kein Netzwerk, kein glab/gh, nur GET): Commits des Branches mit dem von
// Bamboo/Jenkins gemeldeten Build-Status, Commit wählen, Zugang aus dem Git-Credential-Store (nie angezeigt), Fehlerfälle.
// Übersetzung der Antworten: ci-bitbucket.test.ts.

const PROPS = { title: 'Cockpit', isFocused: false, bodyColumns: 160, placement: 'dock', scroll: { offset: 0, bodyRows: 60 }, view: {} } as const
type Engine = Parameters<TestBody>[0]
type On = Parameters<TestBody>[1]
type View = { find(q: { type: 'Text'; text: string | RegExp } | { type: 'Button'; key: string }): Promise<unknown>; press(t: { key: string }): Promise<unknown> }

const SHA1 = '1111111111111111111111111111111111111111'
const SHA2 = '2222222222222222222222222222222222222222'
const SHA3 = '3333333333333333333333333333333333333333'
const SECRET = 'geheimes-token-123'

type Http = { method: string; url: string; auth: string }
type World = { calls: string[][]; http: Http[]; creds: boolean; status: number; builds: Record<string, unknown> }

const commit = (id: string, msg: string, who: string) => ({ id, displayId: id.slice(0, 11), message: msg, author: { name: who, displayName: `${who} Name` }, authorTimestamp: Date.parse('2026-11-02T10:00:00Z') })

function world(): World {
  return {
    calls: [],
    http: [],
    creds: true,
    status: 200,
    builds: {
      [SHA1]: { values: [{ state: 'FAILED', key: 'PLAN-1', name: 'Unit Tests', url: 'https://bamboo.deltra.de/browse/PLAN-1-7', description: '3 Tests fehlgeschlagen', dateAdded: Date.parse('2026-11-02T10:06:00Z'), duration: 125_000 }, { state: 'SUCCESSFUL', key: 'PLAN-2', name: 'Lint', url: 'https://bamboo.deltra.de/browse/PLAN-2-4', dateAdded: Date.parse('2026-11-02T10:05:00Z') }] },
      [SHA2]: { values: [{ state: 'SUCCESSFUL', key: 'PLAN-1', name: 'Unit Tests', dateAdded: Date.parse('2026-11-02T09:00:00Z') }] },
      [SHA3]: { values: [] },
    },
  }
}

const out = (stdout: string, exitCode = 0) => ({ value: { exitCode, stdout, stderr: '', isStdoutTruncated: false, isStderrTruncated: false } }) as never

function wire(on: On, w: World) {
  const clock = mock.clock(on, { now: 1 })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  on('ui.open', () => ({ value: { isPlaced: true as const } }))
  on('ui.toast', () => ({ value: undefined }) as never)
  on('process.run', (_$, e) => {
    const argv = (e as { argv: readonly string[] }).argv
    if (argv[0] === 'git') {
      if (argv.includes('get-url')) return out('https://git.deltra.de/scm/cor/orga-marketplace.git\n')
      if (argv.includes('--show-current')) return out('feature/x\n')
      if (argv.includes('credential')) return w.creds ? out(`protocol=https\nhost=git.deltra.de\nusername=rstich\npassword=${SECRET}\n`) : out('', 1)
      return out('')
    }
    w.calls.push([...argv])
    return out('', 1)
  })
  on('http.fetch', (_$, e) => {
    const x = e as { url: string; init?: { method?: string; headers?: Record<string, string> } }
    w.http.push({ method: x.init?.method ?? 'GET', url: x.url, auth: x.init?.headers?.Authorization ?? '' })
    const json = (v: unknown, status = 200) => ({ value: { status, headers: {}, text: JSON.stringify(v) } }) as never
    if (w.status !== 200) return json({ errors: [{ message: 'boom' }] }, w.status)
    if (/\/commits\?/.test(x.url)) return json({ values: [commit(SHA1, 'Export vereinfachen', 'Ralf'), commit(SHA2, 'Tests ergänzen', 'Anna'), commit(SHA3, 'Doku', 'Max')] })
    const b = /\/rest\/build-status\/1\.0\/commits\/([0-9a-f]+)$/.exec(x.url)
    if (b) return json(w.builds[b[1] ?? ''] ?? { values: [] })
    return json({})
  })
  on('session.model', () => ({ value: 'claude-opus-5-5' }) as never)
  on('session.version', () => ({ value: { version: '2.1.295' } }) as never)
  on('session.start', ($, e) => ({ cwd: e.cwd }))
  on('command.register', ($, e) => ({ value: { command: e.name } }))
  on('fs.read', () => ({ value: '{ "version": "1" }' }) as never)
  return clock
}

async function waitFor(done: () => Promise<boolean>) {
  for (let i = 0; i < 3000 && !(await done()); i++) await Promise.resolve()
}
const waitText = async (v: View, text: string | RegExp) => {
  await waitFor(async () => (await v.find({ type: 'Text', text })) !== undefined)
  return v.find({ type: 'Text', text })
}
const waitButton = async (v: View, key: string) => {
  await waitFor(async () => (await v.find({ type: 'Button', key })) !== undefined)
  return v.find({ type: 'Button', key })
}

async function openCi($: Engine, surface: 'desktop' | 'terminal' = 'desktop') {
  await $.command.run({ command: 'cockpit-ci', args: '' } as never)
  return $.ui.mount({ plugin: 'tenvan-cc', surface, component: 'Pane', requestId: 'tenvan-cc', props: PROPS })
}

test('CI/CD Bitbucket: Kopfzeile, Commits des Branches mit Gesamtstand, erster Commit mit seinen Builds', async ($, on) => {
  const w = world()
  wire(on, w)
  const pane = await openCi($)
  expect(await waitText(pane, /Commits von feature\/x/)).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: 'Bitbucket' })).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: 'git.deltra.de/cor/orga-marketplace' })).not.toBe(undefined)
  for (const word of ['Export vereinfachen', 'Tests ergänzen', 'Doku', 'failed', 'success', 'kein Build']) expect(await waitText(pane, new RegExp(word))).not.toBe(undefined)
  // Gewählt ist der neueste Commit: seine Builds samt Dauer 2:05, Beschreibung und Zeitpunkt.
  expect(await waitText(pane, /Commit 11111111111/)).not.toBe(undefined)
  for (const word of ['Unit Tests', 'Lint', '3 Tests fehlgeschlagen', '2:05', 'PLAN-1']) expect(await waitText(pane, new RegExp(word))).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /Nur lesen: Bamboo/ })).not.toBe(undefined)
  // Nur lesende Aufrufe gegen Bitbucket, kein glab/gh.
  expect(w.http.every(h => h.method === 'GET')).toBe(true)
  expect(w.http.some(h => h.url.includes('/rest/api/1.0/projects/COR/repos/orga-marketplace/commits?limit=15&until=refs%2Fheads%2Ffeature%2Fx'))).toBe(true)
  expect(w.calls).toEqual([])
})

test('CI/CD Bitbucket: Commit wählen zeigt dessen Builds; ohne Status die Meldung „kein Build-System“', async ($, on) => {
  const w = world()
  wire(on, w)
  const pane = await openCi($)
  await waitText(pane, /Commit 11111111111/)
  await pane.press({ key: `bb-pick-${SHA2}` })
  expect(await waitText(pane, /Commit 22222222222/)).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /3 Tests fehlgeschlagen/ })).toBe(undefined)
  await pane.press({ key: `bb-pick-${SHA3}` })
  expect(await waitText(pane, /Für diesen Commit hat kein Build-System einen Status gemeldet/)).not.toBe(undefined)
})

test('CI/CD Bitbucket: Zugang nur aus dem Credential-Store, das Geheimnis erscheint nie im Panel; ohne Zugang der Hinweis', async ($, on) => {
  const w = world()
  wire(on, w)
  const pane = await openCi($)
  await waitText(pane, /Commit 11111111111/)
  expect(w.http.length).toBeGreaterThan(0)
  expect(w.http.every(h => h.auth.startsWith('Basic '))).toBe(true)
  expect(JSON.stringify(await pane.find({ type: 'Box', key: 'card-bb-list' }))).not.toContain(SECRET)
})

test('CI/CD Bitbucket: ohne Zugangsdaten steht der Hinweis im Reiter, ohne einen Abruf', async ($, on) => {
  const w = world()
  w.creds = false
  wire(on, w)
  const pane = await openCi($)
  expect(await waitText(pane, /Kein Zugang zu git\.deltra\.de/)).not.toBe(undefined)
  expect(w.http).toEqual([])
})

test('CI/CD Bitbucket: HTTP-Fehler steht im Reiter, die letzten Commits bleiben; Aktualisieren lädt neu; Terminal ohne Svg', async ($, on) => {
  const w = world()
  wire(on, w)
  const pane = await openCi($, 'terminal')
  await waitText(pane, /Commit 11111111111/)
  expect(await pane.find({ type: 'Svg' })).toBe(undefined)
  w.status = 500
  await waitButton(pane, 'ci-reload')
  await pane.press({ key: 'ci-reload' })
  expect(await waitText(pane, /Build-Status nicht ladbar: HTTP 500/)).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /Der letzte Stand bleibt stehen/ })).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /Export vereinfachen/ })).not.toBe(undefined)
})
