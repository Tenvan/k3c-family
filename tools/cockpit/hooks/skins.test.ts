import { MODES } from './ui'
import { expect, test } from 'claude-code/testing'

import { DEFAULT_SETTINGS, THEME, demoTable } from './core'
import { detailInput, detailOutput, diffCardSvg, diffOf, firstErrorLine, iconKind, inlineTokens, isReadOnlyShell, isStyled, mdLiteral, richBlock, quietLine, splitMessage, toolIconSvg } from './skins'

const ON = { cards: true, inline: true }
const F = '```'

// ── reine Funktionen ──

test('Tool-Symbole: Art je Werkzeug, stabile Quelle in der Farbe', () => {
  expect(iconKind('Read')).toBe('read')
  expect(iconKind('MultiEdit')).toBe('edit')
  expect(iconKind('PowerShell')).toBe('shell')
  expect(iconKind('Glob')).toBe('search')
  expect(iconKind('Agent')).toBe('agent')
  expect(iconKind('mcp__x__y')).toBe('other')
  expect(toolIconSvg('read', THEME.info)).toBe(toolIconSvg('read', THEME.info))
  expect(toolIconSvg('read', THEME.info)).toContain(`stroke="${THEME.info}"`)
})

test('Ruhig: lesende Shell-Befehle, Einzeiler und echte Fehlerzeile', () => {
  expect(isReadOnlyShell('git status --short && ls -la')).toBe(true)
  expect(isReadOnlyShell('cat a.txt | grep x 2>&1')).toBe(true)
  expect(isReadOnlyShell('Get-ChildItem . | Select-Object Name')).toBe(true)
  expect(isReadOnlyShell('echo x > out.txt')).toBe(false)
  expect(isReadOnlyShell('git commit -m x')).toBe(false)
  expect(isReadOnlyShell('rm -rf x')).toBe(false)
  expect(isReadOnlyShell('ls $(rm x)')).toBe(false)
  expect(quietLine('Read', { type: 'text', file: { filePath: 'C:/a/b/c.ts', content: '', numLines: 120, startLine: 1, totalLines: 120 } }, 'de')).toBe('Read · 120 Zeilen · b/c.ts')
  expect(quietLine('Glob', { numFiles: 3, filenames: [], durationMs: 1, truncated: false }, 'en')).toBe('Glob · 3 files')
  expect(quietLine('Grep', { mode: 'content', numFiles: 0, filenames: [], numLines: 7 }, 'de')).toBe('Grep · 7 Zeilen')
  expect(quietLine('Bash', { stdout: 'a\nb\n', stderr: '', interrupted: false }, 'de')).toBe('Bash · 2 Zeilen')
  expect(quietLine('Bash', { stdout: '', stderr: '', interrupted: false }, 'en')).toBe('Bash · no output')
  expect(quietLine('Read', { type: 'image', file: {} }, 'de')).toBe(null)
  expect(firstErrorLine('Exit code 1\n\nnpm ERR! missing script: build\nmore')).toBe('npm ERR! missing script: build')
  expect(firstErrorLine('<tool_use_error>File does not exist.</tool_use_error>')).toBe('File does not exist.')
  expect(firstErrorLine('Exit code 2\nsomething odd')).toBe('something odd')
})

test('Diff-Karte: Zeilen mit Nummern, Kappung mit Rest, stabile Svg-Quelle', () => {
  const out = { filePath: 'x/src/a.ts', structuredPatch: [{ oldStart: 3, oldLines: 2, newStart: 3, newLines: 2, lines: [' keep', '-old <b>', '+new'] }] }
  const d = diffOf(out)
  expect(d?.added).toBe(1)
  expect(d?.removed).toBe(1)
  expect(d?.rows.map(r => `${r.sign}${r.no}`)).toEqual([' 3', '-4', '+4'])
  const svg = diffCardSvg(d!, 'de')
  expect(svg).toBe(diffCardSvg(d!, 'de'))
  expect(svg).toContain('old &lt;b&gt;')
  expect(svg).toContain(THEME.success)
  expect(svg).toContain(THEME.error)
  expect(svg).not.toContain('weitere Zeilen')
  const created = diffOf({ type: 'create', filePath: 'n.md', content: Array.from({ length: 45 }, (_, i) => `z${i}`).join('\n'), structuredPatch: [], originalFile: null })
  expect(created?.added).toBe(45)
  expect(diffCardSvg(created!, 'de')).toContain('… 5 weitere Zeilen')
  expect(diffOf({ stdout: '' })).toBe(null)
})

test('Details: Shell als Shell, sonst JSON; Read-Ergebnis mit Pfad', () => {
  expect(detailInput('Bash', { command: 'ls' })).toEqual({ source: 'ls', language: 'bash' })
  expect(detailInput('Grep', { pattern: 'x' }).language).toBe('json')
  expect(detailOutput({ type: 'text', file: { filePath: 'a.ts', content: 'let x' } })).toEqual({ source: 'let x', path: 'a.ts' })
  expect(detailOutput('Fehler')).toEqual({ source: 'Fehler' })
  expect(detailOutput(undefined)).toBe(null)
})

test('Prompt: Text wörtlich als Markdown', () => {
  expect(mdLiteral('# a *b*')).toBe('\\# a \\*b\\*')
  expect(mdLiteral('x\n  y')).toBe('x  \n\u00a0\u00a0y')
})

test('Antwort zerlegen: Code, Tabelle, Hinweis, Aufgaben, offener Fence, Mermaid bleibt', () => {
  const text = [
    'Vorher **fett**.', '', `${F}ts`, 'const a = 1', 'const b = 2', F, '', '| A | B |', '| --- | --- |', '| 1 | 2 |', '',
    '> [!WARNING]', '> Achtung', '', '- [x] eins', '- [ ] zwei', '', `${F}mermaid`, 'graph TD', F,
  ].join('\n')
  const segs = splitMessage(text, ON)
  expect(segs.map(s => s.kind)).toEqual(['rich', 'code', 'table', 'alert', 'tasks', 'md'])
  expect(segs[1]).toEqual({ kind: 'code', lang: 'ts', source: 'const a = 1\nconst b = 2' })
  expect(segs[2]).toMatchObject({ kind: 'table', rows: 1, cols: 2 })
  expect(segs[3]).toEqual({ kind: 'alert', alert: 'warning', text: 'Achtung' })
  expect(segs[4]).toMatchObject({ done: 1, total: 2 })
  // Hervorhebungen aus: Hinweise und Aufgaben bleiben Markdown
  expect(splitMessage(text, { cards: true, inline: false }).map(s => s.kind)).toEqual(['md', 'code', 'table', 'md'])
  // Streaming: offener Fence bleibt Markdown
  expect(isStyled(splitMessage(`a\n${F}ts\nconst`, ON))).toBe(false)
  expect(isStyled(splitMessage('nur Text ohne Zahl', ON))).toBe(false)
  expect(isStyled(splitMessage('Version 1.2.3 in 3 Dateien', ON))).toBe(true)
})

test('Hervorhebungen: Pfade, geänderte Dateien, Versionen, Zahlen', () => {
  const parts = inlineTokens('hooks/core.ts und register.tsx auf v0.6.0 mit 42 %', ['C:/x/hooks/register.tsx'])
  expect(parts.filter(p => p.tone).map(p => `${p.tone}:${p.text.trim()}`)).toEqual(['path:hooks/core.ts', 'touched:register.tsx', 'version:v0.6.0', 'number:42 %'])
})

test('Demo-Tabelle: gestylte Komponenten nennen ihre Chat-Skins', () => {
  const md = demoTable(DEFAULT_SETTINGS, 'de')
  expect(md).toContain('| `UserMessage` | Deine Nachricht im Chat | ✅ Chat-Skin (Prompt-Rahmen) |')
  expect(md).toContain('✅ Chat-Skin (Code- und Tabellenkarten, Mermaid-Karten, Formelkarten)')
  expect(md).toContain('| `AskUserQuestion` | Rückfrage-Dialog an dich | ✅ Chat-Skin (Rückfrage-Kopf) |')
  expect(md).toContain('| `ToolProgress` | Fortschritt eines laufenden Tools | ➖ nur Terminal')
  expect(demoTable({ ...DEFAULT_SETTINGS, skinPrompt: false }, 'de')).toContain('⏸ Chat-Skin aus')
})

// ── Zeichnung auf dem Desktop ──

const TOOL_USE = { tool_use_id: 'u1', tool: 'Read', input: { file_path: 'a.ts' }, isRunning: false, isErrored: false, isInterrupted: false, output: { type: 'text', file: { filePath: 'a.ts', content: 'let x = 1', numLines: 1, startLine: 1, totalLines: 1 } } }

test('ToolUse: Symbol als einfaches Svg, Details klappen als Code auf; Terminal ohne Symbol, mit Details', async ($, on) => {
  on('ui.render', { component: 'ToolUse' }, () => ({ type: 'Box' as const }))
  const row = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'ToolUse', props: TOOL_USE })
  const icon = JSON.stringify(await row.find({ type: 'Svg' }))
  expect(icon).toContain(THEME.info)
  expect(icon).not.toContain('isInteractive')
  expect(await row.find({ type: 'Code' })).toBe(undefined)
  await row.press({ key: 'om-xb-u1' })
  for (let i = 0; i < 100 && !(await row.find({ type: 'Code' })); i++) await Promise.resolve()
  const codes = JSON.stringify(await row.findAll({ type: 'Code' }))
  expect(codes).toContain('let x = 1')
  expect(codes).toContain('"language":"json"')
  const term = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'terminal', component: 'ToolUse', props: TOOL_USE })
  expect(await term.find({ type: 'Svg' })).toBe(undefined)
  expect(await term.find({ type: 'Button', key: 'om-xb-u1' })).not.toBe(undefined)
  // Modus „skin“: orange Pille, auf dem Desktop abgerundet, im Terminal ohne Rahmen.
  const pillOf = async (v: { find(q: { key: string }): Promise<unknown> }) => JSON.stringify(await v.find({ key: 'om-xb-u1-pill' }))
  expect(await pillOf(row)).toContain(MODES.skin.fill)
  expect(await pillOf(row)).toContain('"borderStyle":"round"')
  expect(await pillOf(term)).not.toContain('borderStyle')
})

test('ToolResult: Diff-Karte, ruhige Zeile, echte Fehlerzeile', async ($, on) => {
  on('ui.render', { component: 'ToolResult' }, () => ({ type: 'Box' as const }))
  const base = { plugin: 'tenvan-cc', surface: 'desktop', component: 'ToolResult' } as const
  const edit = await $.ui.mount({ ...base, props: { tool_use_id: 'e1', tool: 'Edit', isErrored: false, output: { filePath: 'a.ts', oldString: 'a', newString: 'b', originalFile: 'a', structuredPatch: [{ oldStart: 1, oldLines: 1, newStart: 1, newLines: 1, lines: ['-a', '+b'] }], userModified: false, replaceAll: false } } })
  const card = JSON.stringify(await edit.find({ type: 'Svg' }))
  expect(card).toContain('+1')
  expect(card).not.toContain('isInteractive')
  const read = await $.ui.mount({ ...base, props: { tool_use_id: 'r1', tool: 'Read', isErrored: false, output: TOOL_USE.output } })
  expect(await read.find({ type: 'Text', text: /Read · 1 Zeilen · a\.ts/ })).not.toBe(undefined)
  const err = await $.ui.mount({ ...base, props: { tool_use_id: 'b1', tool: 'Bash', isErrored: true, output: 'Exit code 1\nerror: pathspec x did not match' } })
  const line = await err.find({ type: 'Text', text: /pathspec/ })
  expect(JSON.stringify(line)).toContain(THEME.error)
  expect(await err.find({ type: 'Text', text: /^Exit code/ })).toBe(undefined)
})

test('UserMessage: getippter Prompt rechtsbündig im Rahmen als Markdown; Benachrichtigungen bleiben', async ($, on) => {
  on('ui.render', { component: 'UserMessage' }, () => ({ type: 'Box' as const }))
  const msg = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'UserMessage', props: { text: 'Mach *das*', origin: { kind: 'composer' }, isExpanded: true } })
  expect(JSON.stringify(await msg.find({ type: 'Box' }))).toContain(THEME.accent)
  expect(JSON.stringify(await msg.find({ type: 'Markdown' }))).toContain('Mach \\\\*das\\\\*')
  const other = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'UserMessage', props: { text: 'x', origin: { kind: 'sdk' }, isExpanded: true } })
  expect(await other.find({ type: 'Markdown' })).toBe(undefined)
})

test('ToolGroup: je Tool Symbol, Name und Zähler; nur MCP-Aufrufe einzeln mit Ziel', async ($, on) => {
  on('ui.render', { component: 'ToolGroup' }, () => ({ type: 'Box' as const }))
  const call = (tool: string, input: unknown = {}) => ({ tool, input, isRunning: false, isErrored: false, isInterrupted: false })
  const calls = [call('Read', { file_path: 'C:/a/hooks/core.ts' }), call('Read'), call('Bash', { command: 'git status' }), call('mcp__serena__find_symbol', { query: 'register' })]
  const g = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'ToolGroup', props: { calls, isActive: false, isExpanded: false } })
  expect((await g.findAll({ type: 'Svg' })).length).toBe(3)
  expect(await g.find({ type: 'Text', text: 'Read ×2' })).not.toBe(undefined)
  expect(await g.find({ type: 'Text', text: 'serena ×1' })).not.toBe(undefined)
  expect(await g.find({ type: 'Text', text: /find_symbol → register/ })).not.toBe(undefined)
  expect(await g.find({ type: 'Text', text: /git status/ })).toBe(undefined)
})

test('ToolGroup: MCP-Aufrufe je Server zusammengefasst, Plugin-Präfix bereinigt, Überlauf als +n', async ($, on) => {
  on('ui.render', { component: 'ToolGroup' }, () => ({ type: 'Box' as const }))
  const call = (tool: string) => ({ tool, input: {}, isRunning: false, isErrored: false, isInterrupted: false })
  const calls = [
    ...Array.from({ length: 9 }, () => call('mcp__plugin_serena_serena__find_symbol')),
    call('mcp__serena__get_symbols_overview'),
    call('mcp__context7__query'),
  ]
  const g = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'ToolGroup', props: { calls, isActive: false, isExpanded: false } })
  // Beide Schreibweisen von serena landen in einem Eintrag, context7 bekommt einen eigenen.
  expect(await g.find({ type: 'Text', text: 'serena ×10' })).not.toBe(undefined)
  expect(await g.find({ type: 'Text', text: 'context7 ×1' })).not.toBe(undefined)
  expect(await g.find({ type: 'Text', text: /plugin_/ })).toBe(undefined)
  // 11 MCP-Aufrufe: 8 Einzelzeilen, Rest als +3.
  expect((await g.findAll({ type: 'Text', text: /find_symbol/ })).length).toBeGreaterThan(0)
  expect(await g.find({ type: 'Text', text: /\+3/ })).not.toBe(undefined)
})

test('AssistantMessage: Code- und Tabellenkarten mit Kopier-Knopf darunter; Prosa bleibt Markdown', async ($, on) => {
  on('ui.render', { component: 'AssistantMessage' }, () => ({ type: 'Box' as const }))
  const text = `Hier:\n\n${F}ts\nconst a = 1\n${F}\n\n| A | B |\n| --- | --- |\n| 1 | 2 |`
  const m = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'AssistantMessage', props: { text, isFirstOfReply: true } })
  expect(JSON.stringify(await m.find({ type: 'Code' }))).toContain('"language":"ts"')
  expect(await m.find({ type: 'Text', text: 'ts · 1 Zeilen' })).not.toBe(undefined)
  expect(await m.find({ type: 'Text', text: /Tabelle · 1 Zeilen × 2 Spalten/ })).not.toBe(undefined)
  expect((await m.findAll({ type: 'Button' })).length).toBe(2)
  expect(JSON.stringify(await m.find({ type: 'Markdown' }))).toContain('Hier:')
  // Ohne Karten-Inhalt bleibt die Engine-Zeichnung (nur Info-Zeile)
  const plain = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'AssistantMessage', props: { text: 'nur Text', isFirstOfReply: true } })
  expect(await plain.find({ type: 'Code' })).toBe(undefined)
})

test('Hervorhebungen eingeschaltet: Hinweis als Karte, geänderte Datei in Erfolgsfarbe, Aufgaben mit Zähler', async ($, on) => {
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  on('tool.call', () => ({ result: {} }) as never)
  on('ui.render', { component: 'AssistantMessage' }, () => ({ type: 'Box' as const }))
  const props = { title: 'x', isFocused: false, bodyColumns: 144, placement: 'dock', scroll: { offset: 0, bodyRows: 40 }, view: {} } as const
  const page = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc-settings', props })
  await page.press({ key: 'set-skinInline' })
  await $.tool.call({ tool: 'Edit', tool_use_id: 'e1', input: { file_path: 'C:/p/hooks/core.ts', old_string: 'a', new_string: 'b' } } as never)
  const text = 'Geändert: core.ts auf 0.7.0\n\n> [!TIP]\n> Gut\n\n- [x] a\n- [x] b'
  const m = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'AssistantMessage', props: { text, isFirstOfReply: true } })
  expect(JSON.stringify(await m.find({ type: 'Text', text: 'core.ts' }))).toContain(THEME.success)
  expect(JSON.stringify(await m.find({ type: 'Text', text: 'Tipp' }))).toContain(THEME.success)
  expect(await m.find({ type: 'Text', text: 'Aufgaben · 2/2 erledigt' })).not.toBe(undefined)
})

test('CommandOutput: Codeblock als Karte', async ($, on) => {
  on('ui.render', { component: 'CommandOutput' }, () => ({ type: 'Box' as const }))
  const out = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'CommandOutput', props: { command: 'x', args: '', text: `${F}sh\nls\n${F}`, isErrored: false } })
  expect(JSON.stringify(await out.find({ type: 'Code' }))).toContain('"source":"ls"')
})

test('Style-Kennzeichen: Name neben dem Symbol, über die Einstellungen abschaltbar', async ($, on) => {
  on('ui.render', { component: 'ToolResult' }, () => ({ type: 'Box' as const }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  const props = { tool_use_id: 'b2', tool: 'Bash', isErrored: true, output: 'Exit code 1\nerror: kaputt' }
  const base = { plugin: 'tenvan-cc', surface: 'desktop', component: 'ToolResult', props } as const
  expect(await (await $.ui.mount(base)).find({ type: 'Text', text: 'Fehlerzeile' })).not.toBe(undefined)
  const page = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc-settings', props: { title: 'S', isFocused: false, bodyColumns: 100, placement: 'dock', scroll: { offset: 0, bodyRows: 40 }, view: {} } })
  await page.press({ key: 'set-skinMarkLabel' })
  for (let i = 0; i < 50; i++) await Promise.resolve()
  expect(await (await $.ui.mount(base)).find({ type: 'Text', text: 'Fehlerzeile' })).toBe(undefined)
})

test('richBlock: Listen, Code und Fett selbst gezeichnet, Links und Kursiv bleiben Markdown', () => {
  const lines = richBlock('- Entfernt ist `markAfterDelivery` im **Runner**\n  - Unterpunkt 3\n1. Erstens')
  expect(lines?.map(l => l.bullet)).toEqual(['•', '•', '1.'])
  expect(lines?.[1]?.indent).toBe(1)
  expect(lines?.[0]?.parts).toEqual([{ text: 'Entfernt ist ' }, { text: 'markAfterDelivery', code: true }, { text: ' im ' }, { text: 'Runner', bold: true }])
  expect(richBlock('Siehe [Doku](https://x.de)')).toBe(null)
  expect(richBlock('ein _kursives_ Wort')).toBe(null)
  expect(richBlock('## Titel')).toBe(null)
  expect(richBlock('**Workflow `X`** neu')).toBe(null)
  expect(richBlock('`UPDATE_RECORD` bleibt')).not.toBe(null)
  const segs = splitMessage('Absatz mit `code`\n\nDatei core.ts geändert', { cards: true, inline: true })
  expect(segs.map(s => s.kind)).toEqual(['rich', 'plain'])
  expect(isStyled(segs.slice(0, 1))).toBe(false)
})

test('Aufgabenliste & Abschluss: Kopfband mit Fortschritt und Chip „new style“ über TodoWrite, update_plan und wrap_up', async ($, on) => {
  on('ui.render', { component: 'ToolUse' }, () => ({ type: 'Box' as const }))
  const row = (tool: string, input: unknown) =>
    $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'ToolUse', props: { tool_use_id: `t-${tool}`, tool, input, isRunning: false, isErrored: false, isInterrupted: false } })
  const todos = await row('TodoWrite', { todos: [{ content: 'A', status: 'completed', activeForm: 'A' }, { content: 'B', status: 'in_progress', activeForm: 'Baut B' }] })
  expect(await todos.find({ type: 'Text', text: /Aufgaben · 1\/2 erledigt/ })).not.toBe(undefined)
  expect(JSON.stringify(await todos.find({ type: 'Text', text: /läuft: Baut B/ }))).not.toBe(undefined)
  expect(await todos.find({ type: 'Text', text: ' new style ' })).not.toBe(undefined)
  const plan = await row('mcp__ccd_turn__update_plan', { items: [{ content: 'X', status: 'completed' }] })
  expect(await plan.find({ type: 'Text', text: /1\/1 erledigt/ })).not.toBe(undefined)
  const step = await row('mcp__ccd_turn__update_plan', { advance: true })
  expect(await step.find({ type: 'Text', text: ' new style ' })).toBe(undefined)
  const wrap = await row('mcp__ccd_turn__wrap_up', { recap: 'Fix ist drin.', ask: 'Commit?' })
  expect(await wrap.find({ type: 'Text', text: 'Abschluss-Box' })).not.toBe(undefined)
  expect(await wrap.find({ type: 'Text', text: ' new style ' })).not.toBe(undefined)
  // Alte Styles tragen keinen Chip.
  const read = await row('Read', { file_path: 'a.ts' })
  expect(await read.find({ type: 'Text', text: ' new style ' })).toBe(undefined)
})
