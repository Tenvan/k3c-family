import { expect, mock, test } from 'claude-code/testing'

import { DEFAULT_SETTINGS, sidebarCalls } from './core'

const ON = { sidebarDone: true, sidebarUnread: true, sidebarPin: true }

test('sidebarCalls: Standard aus, nichts wird aufgerufen', () => {
  const s = { sidebarDone: DEFAULT_SETTINGS.sidebarDone, sidebarUnread: DEFAULT_SETTINGS.sidebarUnread, sidebarPin: DEFAULT_SETTINGS.sidebarPin }
  expect(sidebarCalls(s, ['merged', 'pipelineFailed', 'conflict'], true, '!7', null)).toEqual({ calls: [], pin: null })
})

test('sidebarCalls: gemergt → erledigt, rot oder Konflikt nur im Leerlauf → ungelesen', () => {
  expect(sidebarCalls(ON, ['merged'], false, null, null).calls).toEqual([{ tool: 'mark_completed' }])
  expect(sidebarCalls(ON, ['pipelineFailed'], true, '!7', { ref: '!7', mine: true }).calls).toEqual([{ tool: 'set_unread' }])
  expect(sidebarCalls(ON, ['conflict'], true, '!7', { ref: '!7', mine: true }).calls).toEqual([{ tool: 'set_unread' }])
  expect(sidebarCalls(ON, ['pipelineFailed', 'conflict'], false, '!7', { ref: '!7', mine: true }).calls).toEqual([])
})

test('sidebarCalls: offener MR pinnt einmal, Ende löst nur die eigene Nadel', () => {
  const first = sidebarCalls(ON, [], true, '!7', null)
  expect(first).toEqual({ calls: [{ tool: 'set_pinned', pinned: true }], pin: { ref: '!7', mine: true } })
  expect(sidebarCalls(ON, [], true, '!7', first.pin)).toEqual({ calls: [], pin: first.pin })
  expect(sidebarCalls(ON, ['merged'], true, null, first.pin)).toEqual({ calls: [{ tool: 'mark_completed' }, { tool: 'set_pinned', pinned: false }], pin: null })
  expect(sidebarCalls(ON, ['closed'], true, null, { ref: '!7', mine: false })).toEqual({ calls: [], pin: null })
  // Schalter aus, während die Nadel steht: das Cockpit räumt seine Nadel weg.
  expect(sidebarCalls({ ...ON, sidebarPin: false }, [], true, '!7', first.pin).calls).toEqual([{ tool: 'set_pinned', pinned: false }])
})

const ok = (stdout: string) => ({ value: { exitCode: 0, stdout, stderr: '', isStdoutTruncated: false, isStderrTruncated: false } })
const MR = {
  iid: 7, title: 'Export', draft: false, web_url: 'https://gitlab.example.com/g/p/-/merge_requests/7', source_branch: 'feature/export', target_branch: 'main',
  detailed_merge_status: 'mergeable', has_conflicts: false, user_notes_count: 0, head_pipeline: { id: 55, status: 'success', web_url: 'u' },
}
const TURN = { answer: 'Fertig.', reason: 'answer', durationMs: 1000, isAborted: false, turnId: 't1' } as const

test('Sitzungsleiste: offener MR pinnt die Session, Merge markiert sie erledigt und löst die Nadel', async ($, on) => {
  mock.clock(on, { now: 1 })
  let state = 'opened'
  const calls: { server: string; tool: string; args: Record<string, unknown> }[] = []
  on('store.get', (_, e) => ({ value: e.key === 'settings' ? { ...ON, mrBand: false, mrPane: false, mrNotify: false, mrFooter: false } : undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  on('session.measure', () => ({ changed: [] }))
  on('session.start', (_, e) => ({ cwd: e.cwd }))
  on('command.register', (_, e) => ({ value: { command: e.name } }))
  on('turn.complete', () => ({ text: 'Fertig.' }))
  on('process.run', (_, e) => {
    const a = e.argv.join(' ')
    if (a.endsWith('branch --show-current')) return ok('feature/export') as never
    if (a.endsWith(' remote')) return ok('origin') as never
    if (a.includes('remote get-url')) return ok('git@gitlab.example.com:g/p.git') as never
    if (a.includes('merge_requests?state=opened')) return ok(state === 'opened' ? '[{"iid":7}]' : '[]') as never
    if (a.endsWith('merge_requests/7')) return ok(JSON.stringify({ ...MR, state })) as never
    if (a.includes('merge_requests/7/')) return ok('[]') as never
    return { value: { exitCode: 1, stdout: '', stderr: '', isStdoutTruncated: false, isStderrTruncated: false } } as never
  })
  on('mcp.call', (_, e) => {
    calls.push(e)
    const text = e.tool === 'get_session' ? '{"pinned":false}' : 'ok'
    return { value: { content: [{ type: 'text', text }], isError: false } } as never
  })
  const settle = async (n: number) => {
    for (let i = 0; i < 5000 && calls.filter(c => c.server === 'ccd_sidebar').length < n; i++) await Promise.resolve()
  }

  await $.session.start({ cwd: '.', surface: 'desktop', isInteractive: true })
  await settle(1)
  expect(calls.filter(c => c.server === 'ccd_sidebar')).toEqual([{ server: 'ccd_sidebar', tool: 'set_pinned', args: { session_id: 'self', pinned: true } }])

  state = 'merged'
  await $.turn.complete(TURN as never)
  await settle(3)
  expect(calls.filter(c => c.server === 'ccd_sidebar').slice(1)).toEqual([
    { server: 'ccd_sidebar', tool: 'mark_completed', args: { session_id: 'self' } },
    { server: 'ccd_sidebar', tool: 'set_pinned', args: { session_id: 'self', pinned: false } },
  ])
})
