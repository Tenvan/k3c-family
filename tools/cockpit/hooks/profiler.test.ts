import { expect, test } from 'claude-code/testing'

import {
  MAX_RECORDS, PROFILE_TEXT,
  bandText, cmdCategory, maskSecrets, newProfiler, paneView, reportText, shortText, stepBegin, stepChunk, stepEnd, stopFailure,
  subagentStart, subagentStop, summarize, toolApproval, toolBegin, toolEnd, toolExec, trim, turnComplete, turnStart,
  type ProfilerState,
} from './profiler'

const usage = (i: number, o: number, r = 0, w = 0) => ({ input_tokens: i, output_tokens: o, cache_read_input_tokens: r, cache_creation_input_tokens: w })

// Turn t1: Request 100–2100 (erster Chunk 600, Thinking 700–1200, Output ab 1300), Bash-Test 2200–7200 mit Freigabe und
// exec 3000, zweiter Request 7300–9000, Ende 10000. Danach Leerlauf bis t2 (20000–21000).
function sessionWithTool(): ProfilerState {
  const st = newProfiler()
  turnStart(st, 't1', undefined, 0)
  stepBegin(st, 't1', 0, 'opus', undefined, 100)
  stepChunk(st, 't1', 0, { kind: 'engine' }, 600)
  stepChunk(st, 't1', 0, { kind: 'thinking' }, 700)
  stepChunk(st, 't1', 0, { kind: 'thinking' }, 1200)
  stepChunk(st, 't1', 0, { kind: 'text' }, 1300)
  stepChunk(st, 't1', 0, { kind: 'stop', usage: usage(1000, 200, 5000, 100) }, 2000)
  stepEnd(st, 't1', 0, 2100)
  toolApproval(st, 'tu1', 'ask')
  toolBegin(st, 'tu1', 'Bash', undefined, { tool: 'Bash', tool_use_id: 'tu1', command: 'npm test' }, 2200)
  toolExec(st, 'tu1', 3000)
  toolEnd(st, 'tu1', 7200, true)
  stepBegin(st, 't1', 1, 'opus', undefined, 7300)
  stepChunk(st, 't1', 1, { kind: 'tool' }, 7800)
  stepEnd(st, 't1', 1, 9000, usage(10, 20))
  turnComplete(st, 't1', 10_000)
  turnStart(st, 't2', undefined, 20_000)
  turnComplete(st, 't2', 21_000)
  return st
}

test('Profiler: Turn zerlegt in Modell, Tools und Overhead, Leerlauf ausgeschlossen', () => {
  const sum = summarize(sessionWithTool(), 30_000)
  expect(sum.session.share).toEqual({ total: 11_000, model: 3_700, tools: 5_000, subagents: 0, overhead: 2_300 })
  expect(sum.session.turns).toBe(2)
  expect(sum.turn?.share).toEqual({ total: 1_000, model: 0, tools: 0, subagents: 0, overhead: 1_000 })
  expect(sum.inflight).toEqual([])
})

test('Profiler: Request mit TTFT, Thinking-/Output-Stream und Usage-Tokens', () => {
  const r = summarize(sessionWithTool(), 30_000).session.requests
  expect(r).toMatchObject({ count: 2, ttftAvg: 500, ttftMax: 500, thinkMs: 500, outMs: 2_000, tokensIn: 1_010, tokensOut: 220, cacheRead: 5_000, cacheWrite: 100 })
  expect(r.cacheHit).toBe(5_000 / 6_110)
})

test('Profiler: Tool mit exec (gemessen) und wait (abgeleitet), Freigabe vor dem Aufruf', () => {
  const s = summarize(sessionWithTool(), 30_000).session
  expect(s.slowTools[0]).toEqual({ label: 'Bash npm test', tool: 'Bash', wallMs: 5_000, execMs: 3_000, waitMs: 2_000, wait: 'approval', ok: true, open: false })
  expect(s.approvalMs).toBe(2_000)
  expect(s.tests).toEqual([{ label: 'Bash npm test', wallMs: 5_000, execMs: 3_000, ok: true, open: false }])
  // ohne 'ask' ist die Wartezeit Overhead
  const st = newProfiler()
  toolBegin(st, 'x', 'Grep', undefined, { pattern: 'foo' }, 0)
  toolApproval(st, 'x', 'allow')
  toolExec(st, 'x', 400)
  toolEnd(st, 'x', 1_000, false)
  expect(summarize(st, 2_000).session.slowTools[0]).toMatchObject({ label: 'Grep foo', wait: 'overhead', waitMs: 600, ok: false })
})

test('Profiler: blockierender Subagent zählt als Subagent-Zeit, seine Requests nicht als Modell der Hauptschleife', () => {
  const st = newProfiler()
  turnStart(st, 't1', undefined, 0)
  toolBegin(st, 'a1', 'Agent', undefined, { subagent_type: 'explore', prompt: 'x' }, 1_000)
  subagentStart(st, 'ag1', 'explore', 1_100, 'a1')
  turnStart(st, 's1', 'ag1', 1_100)
  stepBegin(st, 's1', 0, 'haiku', 'ag1', 1_200)
  stepEnd(st, 's1', 0, 3_000)
  toolBegin(st, 'g1', 'Grep', 'ag1', { pattern: 'foo' }, 3_100)
  toolEnd(st, 'g1', 3_500, true)
  turnComplete(st, 's1', 4_900)
  subagentStop(st, 'ag1', 5_000)
  toolEnd(st, 'a1', 5_100, true)
  turnComplete(st, 't1', 6_000)
  const s = summarize(st, 7_000).session
  expect(s.share).toEqual({ total: 6_000, model: 0, tools: 200, subagents: 3_900, overhead: 1_900 })
  expect(s.subagents).toEqual([{ id: 'ag1', type: 'explore', ms: 3_900, open: false, blocking: true, failed: false, requests: 1, tools: 1 }])
  expect(st.tools.find(t => t.id === 'g1')?.turnId).toBe('s1')
})

test('Profiler: Hintergrund-Subagent blockiert nicht und läuft über den Turn hinaus', () => {
  const st = newProfiler()
  turnStart(st, 't', undefined, 0)
  toolBegin(st, 'a2', 'Agent', undefined, { subagent_type: 'bg', run_in_background: true }, 100)
  subagentStart(st, 'ag2', 'bg', 150, 'a2')
  toolEnd(st, 'a2', 200, true)
  turnComplete(st, 't', 1_000)
  const s = summarize(st, 3_000).session
  expect(s.share).toEqual({ total: 1_000, model: 0, tools: 100, subagents: 0, overhead: 900 })
  expect(s.subagents[0]).toMatchObject({ blocking: false, open: true, ms: 2_850 })
  subagentStop(st, 'ag2', 5_000)
  expect(summarize(st, 9_000).session.subagents[0]).toMatchObject({ open: false, ms: 4_850 })
})

test('Profiler: StopFailure schließt Turn, Requests und Tools des Kontexts; Warten danach ist Leerlauf', () => {
  const st = newProfiler()
  turnStart(st, 't1', undefined, 0)
  stepBegin(st, 't1', 0, 'opus', undefined, 100)
  toolBegin(st, 'x', 'Bash', undefined, { command: 'ls' }, 200)
  stopFailure(st, undefined, 1_000, 'rate_limit')
  expect(st.turns[0]).toEqual({ id: 't1', ctx: 'main', start: 0, end: 1_000, reason: 'error:rate_limit', failed: true })
  expect(st.requests[0]).toMatchObject({ end: 1_000, failed: true, cut: true })
  expect(st.tools[0]).toMatchObject({ end: 1_000, ok: false, cut: true })
  const s = summarize(st, 61_000)
  expect(s.session.share.total).toBe(1_000)
  expect(s.inflight).toEqual([])
  expect(s.session.findings).toContainEqual({ key: 'failed', vars: { n: 1 } })
})

test('Profiler: StopFailure eines Subagents schließt nur dessen Kontext', () => {
  const st = newProfiler()
  turnStart(st, 'm', undefined, 0)
  subagentStart(st, 'ag', 'x', 10)
  stepBegin(st, 's', 0, 'haiku', 'ag', 20)
  stopFailure(st, 'ag', 500, 'overloaded')
  expect(st.open).toEqual({ main: 'm' })
  expect(st.requests[0]).toMatchObject({ end: 500, failed: true })
  expect(st.subagents[0]).toMatchObject({ end: 500, failed: true })
})

test('Profiler: turn.complete schließt offene Requests und Tools des Turns', () => {
  const st = newProfiler()
  turnStart(st, 't', undefined, 0)
  stepBegin(st, 't', 0, 'opus', undefined, 10)
  toolBegin(st, 'z', 'Read', undefined, { file_path: '/a' }, 20)
  turnComplete(st, 't', 100, 'aborted')
  expect(st.requests[0]).toMatchObject({ end: 100, cut: true })
  expect(st.tools[0]).toMatchObject({ end: 100, cut: true, ok: false })
  expect(st.turns[0]?.reason).toBe('aborted')
  expect(st.open).toEqual({})
  // späte Enden ändern nichts mehr
  stepEnd(st, 't', 0, 999)
  toolEnd(st, 'z', 999, true)
  expect(st.tools[0]?.end).toBe(100)
})

test('Profiler: Deckel bei 20.000 Requests+Tools lässt den ältesten fertigen Turn fallen', () => {
  const st = newProfiler()
  const fill = (turn: string, n: number, at: number) => {
    turnStart(st, turn, undefined, at)
    for (let i = 0; i < n; i++) {
      toolBegin(st, `${turn}${i}`, 'Glob', undefined, { pattern: '*' }, at + i)
      toolEnd(st, `${turn}${i}`, at + i + 1, true)
    }
    turnComplete(st, turn, at + n + 10)
  }
  fill('A', 15_000, 0)
  fill('B', 6_000, 100_000)
  expect(st.tools.length + st.requests.length).toBeLessThanOrEqual(MAX_RECORDS)
  expect(st.tools.length).toBe(6_000)
  expect(st.turns.map(t => t.id)).toEqual(['B'])
  expect(st.droppedTurns).toBe(1)
  const sum = summarize(st, 200_000)
  expect(reportText(sum, 200_000, 'de', 'session')).toContain('deckt die letzten 1 Turns ab, 1 ältere verworfen')
  expect(paneView(sum, 200_000, 'en')[0]?.note).toContain('covers the last 1 turns, 1 older dropped')
})

test('Profiler: ohne fertige Turns fallen die ältesten fertigen Einzeldatensätze, offene bleiben', () => {
  const st = newProfiler()
  for (const id of ['a', 'b', 'c']) toolBegin(st, id, 'Glob', undefined, {}, 0)
  toolEnd(st, 'a', 1, true)
  toolEnd(st, 'b', 2, true)
  trim(st, 1)
  expect(st.tools.map(t => t.id)).toEqual(['c'])
})

test('Profiler: Secrets maskiert, bevor gekürzt wird', () => {
  expect(maskSecrets('curl -H "Authorization: Bearer abc.def" https://x')).toBe('curl -H "Authorization: Bearer ***" https://x')
  expect(maskSecrets('echo Bearer xyz123 done')).toBe('echo Bearer *** done')
  expect(maskSecrets('tool --password hunter2 --token=abc')).toBe('tool --password *** --token=***')
  expect(maskSecrets('API_KEY=xyz GITHUB_TOKEN="a b" db_password=p npm run build')).toBe('API_KEY=*** GITHUB_TOKEN=*** db_password=*** npm run build')
  expect(maskSecrets('mysql -u root -psecret db')).toBe('mysql -u root -p*** db')
  expect(maskSecrets('git clone https://user:pw@host/repo')).toBe('git clone https://user:***@host/repo')
  expect(maskSecrets('npm test')).toBe('npm test')
  const long = shortText(`${'x'.repeat(30)} --password hunter2xyz`)
  expect(long.length).toBeLessThanOrEqual(40)
  expect(long).not.toContain('hunter')
  const st = newProfiler()
  toolBegin(st, 'c', 'Bash', undefined, { command: 'curl -H "Authorization: Bearer sk-abc123" https://api.example.com/v1' }, 0)
  toolBegin(st, 'g', 'Grep', undefined, { pattern: 'SECRET_KEY=topsecret' }, 0)
  expect(st.tools[0]?.label).not.toContain('sk-abc123')
  expect(st.tools[0]?.label.length).toBeLessThanOrEqual(48)
  expect(st.tools[0]?.cat).toBe('other')
  expect(st.tools[1]?.label).toBe('Grep SECRET_KEY=***')
  expect(JSON.stringify(st)).not.toContain('topsecret')
})

test('Profiler: Befehle nur als grobe Kategorie', () => {
  expect(cmdCategory('git status')).toBe('git')
  expect(cmdCategory('cd x && git push')).toBe('git')
  expect(cmdCategory('npm test')).toBe('test')
  expect(cmdCategory('pytest -q')).toBe('test')
  expect(cmdCategory('claude plugin test tools/cockpit')).toBe('test')
  expect(cmdCategory('npm install')).toBe('install')
  expect(cmdCategory('npm ci')).toBe('install')
  expect(cmdCategory('npx tsc -p .')).toBe('build')
  expect(cmdCategory('npm run build')).toBe('build')
  expect(cmdCategory('rg foo src')).toBe('search')
  expect(cmdCategory('echo hi')).toBe('other')
})

test('Profiler: wiederholte Reads gleicher Datei/Range, Schreibzugriff setzt zurück', () => {
  const st = newProfiler()
  let at = 0
  const call = (tool: string, input: Record<string, unknown>) => {
    const id = `r${at}`
    toolBegin(st, id, tool, undefined, input, at++)
    toolEnd(st, id, at++, true)
  }
  for (let i = 0; i < 3; i++) call('Read', { file_path: '/src/a.ts' })
  call('Edit', { file_path: '/src/a.ts', old_string: 'x', new_string: 'y' })
  call('Read', { file_path: '/src/a.ts' })
  call('Read', { file_path: '/src/b.ts', offset: 10, limit: 5 })
  call('Read', { file_path: '/src/b.ts', offset: 10, limit: 5 })
  call('Read', { file_path: '/src/b.ts', offset: 20, limit: 5 })
  const s = summarize(st, at).session
  expect(s.repeatedReads).toEqual([{ file: '/src/a.ts', range: '', count: 3 }, { file: '/src/b.ts', range: '10+5', count: 2 }])
  expect(s.findings).toContainEqual({ key: 'reread', vars: { file: 'a.ts', n: 3 } })
  expect(reportText(summarize(st, at), at, 'de', 'session')).toContain('a.ts wurde 3× unverändert gelesen.')
})

// Turn ab 0: Request 0–6100, danach läuft `npm test` seit 6100.
function running(): ProfilerState {
  const st = newProfiler()
  turnStart(st, 't', undefined, 0)
  stepBegin(st, 't', 0, 'opus', undefined, 0)
  stepEnd(st, 't', 0, 6_100)
  toolBegin(st, 'b', 'Bash', undefined, { command: 'npm test' }, 6_100)
  return st
}

test('Profiler: bandText auf Deutsch und Englisch, mit und ohne laufende Aktivität', () => {
  const st = running()
  const now = 20_100
  expect(bandText(summarize(st, now), now, 'de')).toBe('Modell 30 % · Tools 70 % · läuft: Bash npm test 14s')
  expect(bandText(summarize(st, now), now, 'en')).toBe('Model 30% · Tools 70% · running: Bash npm test 14s')
  toolEnd(st, 'b', now, true)
  turnComplete(st, 't', now)
  expect(bandText(summarize(st, now + 5_000), now + 5_000, 'de')).toBe('Modell 30 % · Tools 70 %')
  expect(bandText(summarize(newProfiler(), 0), 0, 'de')).toBe('')
  // nur ein laufender Request im Subagent
  const sub = newProfiler()
  stepBegin(sub, 's', 0, 'haiku', 'ag', 0)
  stepChunk(sub, 's', 0, { kind: 'thinking' }, 500)
  expect(bandText(summarize(sub, 3_000), 3_000, 'de')).toBe('läuft: ↳ Modell denkt 3s')
})

test('Profiler: paneView mit Abschnitten und Balkenanteilen in beiden Sprachen', () => {
  const sum = summarize(running(), 20_100)
  const de = paneView(sum, 20_100, 'de')
  expect(de.map(s => s.title)).toEqual(['Zeitverteilung', 'Läuft gerade', 'Langsamste Tools', 'Modell-Requests'])
  expect(de[0]?.rows.map(r => [r.label, r.pct])).toEqual([['Modell', 30], ['Tools', 70], ['Subagents', 0], ['Engine-Overhead', 0]])
  expect(de[0]?.note).toBe('1 Turns · Agentenzeit 20s · Leerlauf ausgeschlossen')
  expect(de[1]?.rows).toEqual([{ label: 'Bash npm test', value: '14s' }])
  expect(de[2]?.rows[0]).toEqual({ label: 'Bash npm test', value: '14s · läuft' })
  expect(de[3]?.note).toContain('keine Inferenzzeit')
  const en = paneView(sum, 20_100, 'en')
  expect(en.map(s => s.title)).toEqual(['Time split', 'Running now', 'Slowest tools', 'Model requests'])
  expect(en[0]?.rows[0]?.value).toBe('6s · 30%')
  // Subagents und Funde erscheinen nur mit Inhalt
  const st = newProfiler()
  stopFailure(st, undefined, 0, 'x')
  turnStart(st, 'f', undefined, 0)
  subagentStart(st, 'ag', 'explore', 10)
  stopFailure(st, 'ag', 20, 'overloaded')
  stopFailure(st, undefined, 50, 'rate_limit')
  const pane = paneView(summarize(st, 100), 100, 'de')
  expect(pane.map(s => s.title)).toEqual(['Zeitverteilung', 'Subagents', 'Funde'])
  expect(pane[1]?.rows[0]).toMatchObject({ label: 'explore', tone: 'bad' })
  expect(pane[2]?.rows[0]?.label).toBe('1 Turn(s) mit API-Fehler abgebrochen.')
})

test('Profiler: reportText für Session und letzten Turn mit Beobachtungsgraden', () => {
  const st = sessionWithTool()
  const sum = summarize(st, 30_000)
  const de = reportText(sum, 30_000, 'de', 'session')
  expect(de).toContain('## Profil · Session')
  expect(de).toContain('- Modell: 34 % (4s) [beobachtet]')
  expect(de).toContain('- Engine-Overhead: 21 % (2s) [abgeleitet]')
  expect(de).toContain('- Bash npm test: 5s · Ausführung 3s · Freigabe 2s')
  expect(de).toContain('### Testläufe')
  expect(de).toContain('[API-Angabe]')
  expect(de).toContain('Trefferquote 82 %')
  expect(de).toContain('Keine Auffälligkeiten.')
  const turn = reportText(sum, 30_000, 'de', 'turn')
  expect(turn).toContain('## Profil · letzter Turn')
  expect(turn).toContain('- Engine-Overhead: 100 % (1s) [abgeleitet]')
  expect(turn).not.toContain('npm test')
  expect(reportText(sum, 30_000, 'en', 'session')).toContain('- Model: 34% (4s) [observed]')
  expect(reportText(summarize(newProfiler(), 0), 0, 'de', 'session')).toBe('Noch keine Profildaten.')
  expect(reportText(summarize(newProfiler(), 0), 0, 'en', 'turn')).toBe('No profile data yet.')
})

test('Profiler: Texte de/en mit identischen Schlüsseln, Präfix profile., echte Umlaute', () => {
  const de = Object.keys(PROFILE_TEXT.de).sort()
  expect(Object.keys(PROFILE_TEXT.en).sort()).toEqual(de)
  expect(de.every(k => k.startsWith('profile.'))).toBe(true)
  const all = Object.values(PROFILE_TEXT.de).join('\n')
  expect(all).toContain('läuft')
  expect(all).toContain('Läuft gerade')
  expect(all).not.toMatch(/laeuft|Laeuft|pruefen|aeltere/)
})
