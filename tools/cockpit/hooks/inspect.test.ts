import { expect, test } from 'claude-code/testing'

import {
  INSPECT_TEXT, IMAGE_TOKENS, MAX_ITEMS, TARGET_MAX,
  bandText, clipTarget, contentHash, editLines, hamming, ingestAppend, ingestCompaction, ingestToolCall, ingestToolResult, inputOf,
  maskSecrets, newInspect, num, paneView, reportText, sampleInput, simHash, summarize, targetOf, tx,
  type InspectState,
} from './inspect'

let seq = 0
// Ein Tool-Aufruf wie in der Engine: tool.call (vorher/nachher), dann die tool_result-Zeile per session.append.
function call(st: InspectState, tool: string, input: Record<string, unknown>, text: string, o: { agent?: string; answer?: Record<string, unknown> } = {}): string {
  const id = `toolu_${++seq}`
  ingestToolCall(st, id, tool, o.agent, { tool, tool_use_id: id, ...input })
  ingestToolResult(st, id, o.answer ?? { result: { type: 'text' }, text })
  ingestAppend(st, { door: 'tool-result', uuid: `row-${id}`, ...(o.agent === undefined ? {} : { agentId: o.agent }), message: { type: 'user', content: [{ type: 'tool_result', tool_use_id: id, content: text }] } })
  return id
}
const read = (st: InspectState, path: string, text: string, extra: Record<string, unknown> = {}, o: { agent?: string; answer?: Record<string, unknown> } = {}) =>
  call(st, 'Read', { file_path: path, ...extra }, text, o)
const prompt = (st: InspectState, text: string, uuid = `p${++seq}`) => ingestAppend(st, { door: 'prompt', uuid, message: { type: 'user', content: [{ type: 'text', text }] } })
const words = (n: number, tag = 'w') => Array.from({ length: n }, (_, i) => `${tag}${String.fromCharCode(97 + (i % 26))}${String.fromCharCode(97 + Math.floor(i / 26))}`).join(' ')

test('Maskierung: Authorization, Bearer, --password/--token, NAME=wert, URL-Passwort, curl -u', () => {
  expect(maskSecrets('curl -H "Authorization: Bearer abc.def" x')).toBe('curl -H "Authorization: Bearer ***" x')
  expect(maskSecrets('Bearer eyJhbGciOi.xyz')).toBe('Bearer ***')
  expect(maskSecrets('mysql --password=hunter2 --token abc db')).toBe('mysql --password=*** --token *** db')
  expect(maskSecrets('API_KEY=sk-123 GITHUB_TOKEN="g h" DB_PASSWORD=x npm run')).toBe('API_KEY=*** GITHUB_TOKEN=*** DB_PASSWORD=*** npm run')
  expect(maskSecrets('git clone https://ralf:geheim@host/repo')).toBe('git clone https://ralf:***@host/repo')
  expect(maskSecrets('curl -u ralf:geheim https://x')).toBe('curl -u ralf:*** https://x')
  expect(maskSecrets('ls -la src')).toBe('ls -la src')
  // erst maskieren, dann kürzen: das Secret am Ende darf nicht halb stehen bleiben
  const long = `${'x'.repeat(190)} --token=supersecretvalue`
  expect(clipTarget(long).includes('supersecret')).toBe(false)
  expect(clipTarget('a '.repeat(300)).length).toBe(TARGET_MAX)
})

test('Ziele: Datei, Befehl (maskiert), Muster, URL, Agent', () => {
  expect(targetOf('Read', { file_path: 'C:\\w\\a.ts' })).toEqual({ kind: 'file', value: 'C:/w/a.ts' })
  expect(targetOf('Bash', { command: 'TOKEN=abc deploy' })).toEqual({ kind: 'command', value: 'TOKEN=*** deploy' })
  expect(targetOf('Grep', { pattern: 'foo', path: 'src' })).toEqual({ kind: 'pattern', value: 'foo' })
  expect(targetOf('WebFetch', { url: 'https://u:pw@x.de/a' })).toEqual({ kind: 'url', value: 'https://u:***@x.de/a' })
  expect(targetOf('Agent', { description: 'Suche Bugs', prompt: 'lang' })).toEqual({ kind: 'agent', value: 'Suche Bugs' })
  expect(targetOf('TodoWrite', { todos: [] })).toBe(undefined)
  expect(editLines('Edit', { old_string: 'a\nb', new_string: 'c' })).toBe(2)
  expect(editLines('MultiEdit', { edits: [{ old_string: 'a', new_string: 'b\nc\nd' }, { old_string: 'x', new_string: 'y' }] })).toBe(4)
  expect(editLines('Write', { content: '1\n2\n3' })).toBe(3)
})

test('Ledger: tool.call und session.append ergeben ein Item je tool_use_id, Schätzung chars/4, Bild 1500', () => {
  const st = newInspect()
  prompt(st, 'x'.repeat(40))
  read(st, 'C:\\w\\a.ts', 'y'.repeat(400))
  expect(st.items.length).toBe(2)
  expect(st.items[1]).toMatchObject({ category: 'Read', door: 'tool-result', raw: 100, target: { kind: 'file', value: 'C:/w/a.ts' }, whole: true, live: true, turn: 1 })
  // Bild aus der Antwort, Zeile ohne Text
  const id = 'img'
  ingestToolCall(st, id, 'Read', undefined, { file_path: '/p.png' })
  ingestToolResult(st, id, { result: { type: 'image' } })
  expect(st.byId[id]?.raw).toBe(IMAGE_TOKENS)
  // abgelehnter Aufruf erzeugt kein Item aus der Antwort
  ingestToolCall(st, 'd', 'Bash', undefined, { command: 'rm -rf /' })
  ingestToolResult(st, 'd', { deny: 'nein' })
  expect(st.byId.d).toBe(undefined)
  expect(inputOf({ input_tokens: 10, cache_read_input_tokens: 200, cache_creation_input_tokens: 30 })).toBe(240)
  expect(inputOf(null)).toBe(undefined)
})

test('Deckel: höchstens 20.000 Items, Items außerhalb des Fensters gehen zuerst', () => {
  const st = newInspect()
  prompt(st, 'alt', 'old')
  ingestCompaction(st, { trigger: 'auto', messages: [{ text: 'alt', handle: 'old' }] }, { messages: [{ text: 'Zusammenfassung' }] })
  expect(st.items[0]?.live).toBe(false)
  for (let i = 0; i < MAX_ITEMS + 4; i++) ingestAppend(st, { door: 'response', uuid: `r${i}`, message: { type: 'assistant', content: [{ type: 'text', text: 'ok' }] } })
  expect(st.items.length).toBe(MAX_ITEMS)
  expect(st.dropped).toBe(5)
  expect(st.items.some(i => i.row === 'old')).toBe(false)
  expect(st.items[0]?.row).toBe('r4')
})

test('Kalibrierung: k aus gemessenem Input-Wachstum, begrenzt auf 0.5–2.0, Kompaktierung unterbricht', () => {
  const st = newInspect()
  expect(summarize(st).k).toBe(1)
  sampleInput(st, undefined, 10_000)
  prompt(st, 'x'.repeat(4000)) // 1000 geschätzt
  sampleInput(st, undefined, 11_500) // 1500 gemessen
  let s = summarize(st)
  expect(s.k).toBe(1.5)
  expect(s.calibrated).toBe(true)
  expect(s.kClamped).toBe(false)
  prompt(st, 'x'.repeat(4000))
  sampleInput(st, undefined, 30_000) // 18.500 gemessen → k > 2
  s = summarize(st)
  expect(s.k).toBe(2)
  expect(s.kClamped).toBe(true)
  // Intervall über eine Kompaktierung zählt nicht
  const before = st.calMeasured
  ingestCompaction(st, { trigger: 'auto', messages: [] }, { messages: [] })
  prompt(st, 'x'.repeat(4000))
  sampleInput(st, undefined, 40_000)
  expect(st.calMeasured).toBe(before)
})

test('Re-Reads: unchanged-live ist Verschwendung jetzt, after-edit, different-range, engine-deduped, changed-externally', () => {
  const st = newInspect()
  const body = 'z'.repeat(800) // 200 Tokens
  read(st, '/w/a.ts', body)
  read(st, '/w/a.ts', body) // unchanged-live
  read(st, '/w/a.ts', body, { offset: 10, limit: 5 }) // different-range
  call(st, 'Edit', { file_path: '/w/a.ts', old_string: 'a', new_string: 'b' }, 'ok')
  read(st, '/w/a.ts', body, { offset: 10, limit: 5 }) // after-edit
  read(st, '/w/a.ts', 'u', { offset: 10, limit: 5 }, { answer: { result: { type: 'file_unchanged', file: { filePath: '/w/a.ts' } }, text: 'u' } }) // engine-deduped
  read(st, '/w/b.ts', 'eins')
  read(st, '/w/b.ts', 'zwei') // changed-externally
  const s = summarize(st)
  expect(s.rereads.rows.map(r => r.kind)).toEqual(['unchanged-live', 'different-range', 'after-edit', 'engine-deduped', 'changed-externally'])
  expect(s.rereads.counts['unchanged-live']).toBe(1)
  expect(s.rereads.wasteNow).toBe(200)
  expect(s.rereads.wasteSession).toBe(200)
  expect(s.waste).toBe(200)
})

test('Re-Reads: nach Kompaktierung bzw. von der Engine wiederhergestellt, keine Verschwendung mehr', () => {
  const st = newInspect()
  read(st, '/w/a.ts', 'z'.repeat(800))
  read(st, '/w/a.ts', 'z'.repeat(800))
  expect(summarize(st).waste).toBe(200)
  // Kompaktierung wirft beide Kopien hinaus: Verschwendung jetzt 0, Sitzung bleibt
  ingestCompaction(st, { trigger: 'auto', messages: [] }, { messages: [{ text: 'Zusammenfassung' }] })
  read(st, '/w/a.ts', 'z'.repeat(800))
  ingestAppend(st, { door: 'attachment', uuid: 'att', message: { type: 'attachment', name: 'file', content: [{ type: 'text', text: '{"file_path": "/w/b.ts", "content": "x"}' }] } })
  ingestCompaction(st, { trigger: 'manual', messages: [] }, { messages: [] })
  ingestAppend(st, { door: 'attachment', uuid: 'att2', message: { type: 'attachment', name: 'file', content: [{ type: 'text', text: '{"file_path": "/w/b.ts", "content": "x"}' }] } })
  const s = summarize(st)
  expect(s.rereads.rows.map(r => r.kind)).toEqual(['unchanged-live', 'after-compaction', 'restored'])
  expect(s.rereads.wasteNow).toBe(0)
  expect(s.rereads.wasteSession).toBe(200)
  expect(s.waste).toBe(0)
  expect(s.compactions[0]?.reflow).toBe(200)
})

test('Ähnliche Ausgaben: exakter Hash, SimHash mit Zahlen als #, je Kontext getrennt', () => {
  const st = newInspect()
  const out = `PASS ${words(80)} took 123ms`
  call(st, 'Bash', { command: 'npm test' }, out)
  call(st, 'Bash', { command: 'npm test' }, out) // exakt
  call(st, 'Bash', { command: 'npm run lint' }, `${words(80, 'q')} 1`)
  call(st, 'Bash', { command: 'npm run lint' }, `${words(80, 'q')} 2`) // nur Zahl anders → ähnlich
  call(st, 'Bash', { command: 'npm test' }, out, { agent: 'a1' }) // anderer Kontext
  const s = summarize(st)
  expect(s.similar.groups.map(g => [g.kind, g.count])).toEqual([['exact', 2], ['near', 2]])
  expect(s.similar.wasteNow).toBe(s.similar.groups.reduce((n, g) => n + g.wasteNow, 0))
  expect(s.similar.groups[0]?.label).toBe('Bash · npm test')
  const all = summarize(st, true)
  expect(all.similar.groups.length).toBe(2)
  expect(all.similar.groups.find(g => g.kind === 'exact')?.count).toBe(2)
  // SimHash selbst: gleich nach Normalisierung, entfernt bei anderem Text
  const a = simHash(`build 12 ${words(60)}`)
  const b = simHash(`build 99 ${words(60)}`)
  const c = simHash(words(60, 'k'))
  if (a === undefined || b === undefined || c === undefined) throw new Error('simHash fehlt')
  expect(hamming(a, b)).toBe(0)
  expect(hamming(a, c) > 3).toBe(true)
  expect(contentHash('a') === contentHash('b')).toBe(false)
})

test('Kompaktierungen: Items verlassen das Fenster, Entitäten überleben per Teilstring, precompute zählt nicht', () => {
  const st = newInspect()
  prompt(st, 'Bitte bauen', 'h1')
  const e1 = call(st, 'Edit', { file_path: 'C:\\w\\src\\keep.ts', old_string: 'a', new_string: 'b' }, 'ok')
  call(st, 'Edit', { file_path: '/w/src/lost.ts', old_string: 'a', new_string: 'b' }, 'ok')
  const messages = [
    { text: 'Bitte bauen', handle: 'h1', toolUses: [] },
    { text: '', handle: 'h2', toolUses: [
      { tool_use_id: e1, tool: 'Edit', input: { file_path: 'C:\\w\\src\\keep.ts' } },
      { tool_use_id: 'x2', tool: 'Edit', input: { file_path: '/w/src/lost.ts' } },
      { tool_use_id: 'x3', tool: 'Read', input: { file_path: '/w/README.md' } },
    ] },
    { text: '', handle: 'h3', toolUses: [], toolResults: [{ tool_use_id: 'x4', text: 'Error: ENOENT foo\nstack', isError: true }] },
  ]
  ingestCompaction(st, { trigger: 'precompute', messages }, { messages: [{ text: 'x' }] })
  expect(st.compactions.length).toBe(0)
  ingestCompaction(st, { trigger: 'auto', messages }, { skip: 'nein' })
  expect(st.compactions.length).toBe(0)
  ingestCompaction(st, { trigger: 'auto', messages }, { messages: [{ text: 'Zusammenfassung: keep.ts geändert, README.md gelesen', handle: 's' }, messages[0] as never], tokensBefore: 150_000, tokensAfter: 20_000 })
  const c = summarize(st).compactions[0]
  expect(c).toMatchObject({ trigger: 'auto', before: 150_000, after: 20_000, droppedItems: 2 })
  expect(c?.edited).toEqual({ all: ['C:/w/src/keep.ts', '/w/src/lost.ts'], lost: ['/w/src/lost.ts'] })
  expect(c?.read.lost).toEqual([])
  expect(c?.errors).toEqual({ all: ['Error: ENOENT foo'], lost: ['Error: ENOENT foo'] })
  expect(st.items.find(i => i.row === 'h1')?.live).toBe(true)
  const s = summarize(st)
  expect(s.advice[0]?.key).toBe('lostEdits')
})

test('Effizienz: geänderte Zeilen gegen Input-Wachstum je Turn, Erkundungs-Turns markiert', () => {
  const st = newInspect()
  prompt(st, 'eins')
  sampleInput(st, undefined, 10_000)
  call(st, 'Write', { file_path: '/w/a.ts', content: 'a\nb\nc\nd' }, 'ok')
  sampleInput(st, undefined, 12_000)
  prompt(st, 'zwei')
  call(st, 'Grep', { pattern: 'x' }, 'treffer')
  sampleInput(st, undefined, 15_000)
  prompt(st, 'drei')
  // fehlgeschlagene Bearbeitung zählt nicht
  call(st, 'Edit', { file_path: '/w/a.ts', old_string: 'a', new_string: 'b' }, 'Fehler', { answer: { isError: true, result: 'nope' } })
  sampleInput(st, undefined, 15_000)
  const e = summarize(st).efficiency
  expect(e.turns).toEqual([
    { turn: 1, lines: 4, growth: 2000, exploring: false },
    { turn: 2, lines: 0, growth: 3000, exploring: true },
    { turn: 3, lines: 0, growth: 0, exploring: false },
  ])
  expect(e).toMatchObject({ lines: 4, growth: 5000, exploring: 1 })
})

test('Belegung und Hinweise: Kategorien, Ziele, große Reads, lange Befehle', () => {
  const st = newInspect()
  prompt(st, 'x'.repeat(400))
  read(st, '/w/big.ts', 'b'.repeat(40_000)) // 10k
  call(st, 'Bash', { command: 'npm ci' }, 'n'.repeat(20_000)) // 5k
  const s = summarize(st)
  expect(s.occupancy.byCategory.map(c => c.name)).toEqual(['Read', 'Bash', 'prompt'])
  expect(s.occupancy.byTarget[0]).toEqual({ name: 'file\u0000/w/big.ts', est: 10_000, count: 1 })
  expect(s.advice.map(a => [a.key, a.saving])).toEqual([['largeRead', 8000], ['largeShell', 4000]])
})

test('Texte: de und en mit denselben Schlüsseln, Präfix inspect., echte Umlaute', () => {
  const de = Object.keys(INSPECT_TEXT.de).sort()
  expect(Object.keys(INSPECT_TEXT.en).sort()).toEqual(de)
  expect(de.every(k => k.startsWith('inspect.'))).toBe(true)
  expect(tx('de', 'inspect.sec.overview')).toBe('Überblick')
  expect(tx('en', 'inspect.adv.saving', { n: '1k' })).toBe('−≈1k')
  expect(num(8123, 'de')).toBe('8,1k')
  expect(num(8123, 'en')).toBe('8.1k')
  expect(num(512, 'de')).toBe('512')
  expect(num(2_500_000, 'en')).toBe('2.5M')
})

test('bandText und paneView in beiden Sprachen, höchstens 8 Zeilen je Abschnitt, reportText', () => {
  const st = newInspect()
  prompt(st, 'hallo')
  for (let i = 0; i < 12; i++) read(st, `/w/f${i % 3}.ts`, 'z'.repeat(32_400)) // je 8,1k
  const s = summarize(st)
  expect(s.waste > 0).toBe(true)
  expect(bandText({ ...s, waste: 8100 }, 62.4, 'de')).toBe('Kontext 62 % · Verschwendung ≈8,1k')
  expect(bandText({ ...s, waste: 8100 }, 62.4, 'en')).toBe('ctx 62% · waste≈8.1k')
  expect(bandText({ ...s, waste: 0 }, undefined, 'en')).toBe('waste≈0')
  for (const L of ['de', 'en'] as const) {
    const v = paneView(s, L)
    expect(v.map(x => x.title)).toEqual(['overview', 'occupancy', 'rereads', 'similar', 'compactions', 'efficiency', 'advice'].map(k => tx(L, `inspect.sec.${k}` as never)))
    expect(v.every(x => x.rows.length >= 1 && x.rows.length <= 8)).toBe(true)
    const report = reportText(s, L)
    expect(report.includes(tx(L, 'inspect.report.title'))).toBe(true)
    expect(report.includes(tx(L, 'inspect.sec.advice'))).toBe(true)
    expect(report.includes('<svg')).toBe(false)
  }
  expect(paneView(s, 'de')[2]?.rows[0]).toMatchObject({ label: 'unverändert, noch im Fenster', tone: 'bad' })
  expect(paneView(summarize(newInspect()), 'en')[1]?.rows[0]?.label).toBe('nothing recorded yet')
})
