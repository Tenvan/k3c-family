import { expect, test } from 'claude-code/testing'

import { splitTool, toolText, parseTaskNotifications, backgroundOf, todosOf, wrapupOf, resetLeft, parseLinkedSessions, dirTarget, isForeignDir, withClaude, modelLabel, timelineSvg, summaryLines, summarySections, topRows, resetLabel, agentStats, agentFlags, DEFAULT_SETTINGS, SETTING_DEFS, settingHint, settingLabel, catchUpPrompt, catchUpSystem, fingerprint, formatMessage, nextChunk, settingsOf, startIndex } from './core'
import { STRINGS, t, type Key } from './i18n'

type Msg = Parameters<typeof fingerprint>[0]
const msg = (role: 'user' | 'assistant', text: string, ids: string[] = []): Msg => ({
  role,
  text,
  toolUses: ids.map(id => ({ tool_use_id: id, tool: 'Read', input: { path: id } })),
})
const history = [msg('user', 'eins'), msg('assistant', 'zwei', ['a']), msg('user', 'drei'), msg('assistant', 'vier')]

test('settingsOf: Standard, Teileingabe und leere Altzustände', () => {
  expect(settingsOf(undefined)).toEqual(DEFAULT_SETTINGS)
  expect(settingsOf(null)).toEqual(DEFAULT_SETTINGS)
  expect(settingsOf({})).toEqual(DEFAULT_SETTINGS)
  expect(settingsOf({ rings: false })).toEqual({ ...DEFAULT_SETTINGS, rings: false })
})

test('SETTING_DEFS: deckt alle Schalter außer lang, breathKey und agentTiles genau einmal ab', () => {
  const keys = SETTING_DEFS.map(d => d.key as string).sort()
  expect(keys).toEqual(Object.keys(DEFAULT_SETTINGS).filter(k => k !== 'lang' && k !== 'breathKey' && k !== 'agentTiles').sort())
  const groups = ['band', 'work', 'effort', 'panel', 'summary', 'mr', 'sidebar', 'skins', 'tools', 'input', 'marks']
  for (const d of SETTING_DEFS) {
    for (const lang of ['de', 'en'] as const) {
      expect(settingLabel(d.key, lang).trim()).not.toBe('')
      expect(settingHint(d.key, lang).trim()).not.toBe('')
    }
    expect(groups).toContain(d.group)
  }
  expect(t('de', 'settings.group.band')).toBe('Band über der Eingabe')
})

test('fingerprint: Rolle, Textanfang und Tool-IDs', () => {
  expect(fingerprint(msg('user', 'x'.repeat(300), ['a', 'b']))).toBe(`user\n${'x'.repeat(200)}\na,b`)
})

test('startIndex: Normalfall, verschoben, Marker fehlt, covered 0', () => {
  expect(startIndex(history, 2, fingerprint(history[1] as Msg))).toBe(2)
  expect(startIndex(history.slice(1), 2, fingerprint(history[1] as Msg))).toBe(1)
  expect(startIndex(history, 3, 'unbekannt')).toBe(0)
  expect(startIndex(history, 0, fingerprint(history[2] as Msg))).toBe(3)
  expect(startIndex(history, 0, 'unbekannt')).toBe(0)
})

test('nextChunk: mindestens eine Nachricht, Limit, monotones end, alles bei großem Limit', () => {
  expect(nextChunk(history, 0, 'de', 1).end).toBe(1)
  const big = [msg('user', 'a'.repeat(100)), msg('user', 'b'.repeat(100)), msg('user', 'c'.repeat(100))]
  expect(nextChunk(big, 0, 'de', 150).end).toBe(2)
  let from = 0
  while (from < history.length) {
    const { end } = nextChunk(history, from, 'en', 1)
    expect(end).toBeGreaterThan(from)
    from = end
  }
  expect(nextChunk(history, 0, 'en').end).toBe(history.length)
  expect(nextChunk(history, 4, 'en')).toEqual({ text: '', end: 4 })
})

test('formatMessage: Hinweise, Kürzung, Tools, Sprache', () => {
  const clean = formatMessage(msg('user', 'Hallo <system-reminder>\ngeheim\n</system-reminder>'), 'en')
  expect(clean).toBe('USER: Hallo')
  expect(formatMessage(msg('user', '<system-reminder>nur Hinweis</system-reminder>'), 'en')).toBe('')
  expect(formatMessage(msg('assistant', 'y'.repeat(5000)), 'en')).toBe(`ASSISTANT: ${'y'.repeat(4000)} […]`)
  const tool = formatMessage({ role: 'assistant', text: '', toolUses: [{ tool_use_id: 't', tool: 'Bash', input: { cmd: 'z'.repeat(500) } }] }, 'en')
  expect(tool).toContain('Bash')
  expect(tool).toContain(' […]')
  expect(tool.length).toBeLessThan(400)
  expect(formatMessage(msg('user', 'hi'), 'de')).toBe('NUTZER: hi')
  expect(formatMessage(msg('assistant', 'hi'), 'de')).toBe('ASSISTENT: hi')
  expect(formatMessage(msg('assistant', 'hi'), 'en')).toBe('ASSISTANT: hi')
})

test('catchUpSystem: Sprache und Anweisungen', () => {
  const de = catchUpSystem('de', 'ANWEISUNG')
  expect(de).toContain('auf Deutsch')
  expect(de).toContain('<instructions>\nANWEISUNG\n</instructions>')
  const en = catchUpSystem('en', 'INSTR')
  expect(en).toContain('Write entirely in English')
  expect(en).toContain('<instructions>\nINSTR\n</instructions>')
})

test('catchUpPrompt: Abschnitte und Platzhalter je Sprache', () => {
  const p = catchUpPrompt('alt', 'neu\n', 'git', 'en')
  for (const tag of ['<previous_summary>', '<new_messages>', '<git_state>']) expect(p).toContain(tag)
  expect(p).toContain('alt')
  expect(catchUpPrompt('', 'x', 'g', 'de')).toContain('(noch keine)')
  expect(catchUpPrompt('', 'x', 'g', 'en')).toContain('(none yet)')
})

test('i18n: en hat genau die Schlüssel von de, kein Text leer, Platzhalter gleich', () => {
  // Typebene: `en` ist Record<Key, string>, ein fehlender oder zusätzlicher Schlüssel bricht schon den Typecheck.
  const sameKeys: Record<Key, string> = STRINGS.en
  expect(Object.keys(sameKeys).sort()).toEqual(Object.keys(STRINGS.de).sort())
  const vars = (s: string) => (s.match(/\{\w+\}/g) ?? []).sort()
  for (const k of Object.keys(STRINGS.de) as Key[]) {
    expect(STRINGS.de[k].trim()).not.toBe('')
    expect(STRINGS.en[k].trim()).not.toBe('')
    expect(vars(STRINGS.en[k])).toEqual(vars(STRINGS.de[k]))
  }
  expect(t('en', 'agents.steps', { n: 3 })).toBe('3 steps')
  expect(t('de', 'agents.steps')).toBe('{n} Schritte')
})

test('resetLabel englisch: Wochentag aus en-US, Datum als Monat/Tag', () => {
  const now = new Date(2026, 9, 12, 9, 0).getTime()
  expect(resetLabel(new Date(2026, 9, 13, 21, 0).toISOString(), now, 'en')).toBe('Tue 21:00')
  expect(resetLabel(new Date(2026, 9, 20, 8, 5).toISOString(), now, 'en')).toBe('Tue 08:05 (10/20)')
})

test('Agent-Kachel: Kennzahlen und Merkmale, angefragtes Modell nur bei Abweichung', () => {
  const a = { id: 'a', type: 't', description: 'd', status: 'completed', since: 0, model: 'claude-opus-5-5', requested: 'opus', background: true, permission: 'auto', tokensIn: 43_000, tokensOut: 4000, cacheHit: 80, steps: 6 }
  expect(agentStats(a)).toBe('ctx 43k · out 4k · 6 Schritte')
  expect(agentFlags(a)).toBe('angefragt opus · Hintergrund · Rechte auto · Cache 80 %')
  expect(agentStats({ id: 'a', type: 't', description: 'd', status: 'running', since: 0 })).toBe('0 Schritte')
  expect(agentFlags({ id: 'a', type: 't', description: 'd', status: 'x', since: 0, model: 'm', requested: 'm' })).toBe('')
})

test('resetLabel: Wochentag und Uhrzeit, ab 7 Tagen auch das Datum', () => {
  const tue = new Date(2026, 9, 13, 21, 0).getTime() // Dienstag, 13.10.2026
  const now = new Date(2026, 9, 12, 9, 0).getTime()
  expect(resetLabel(new Date(tue).toISOString(), now)).toBe('Di 21:00')
  expect(resetLabel(new Date(2026, 9, 20, 8, 5).toISOString(), now)).toBe('Di 08:05 (20.10.)')
})

test('summaryLines: einfache Zeilen ohne Bilder, Links, HTML und Referenzen', () => {
  const lines = summaryLines('## Warum\n- ok **fett** und `code`\n![x](https://evil/?d=1) <img src="https://evil"> [Doku][1] [A](https://a.de)\n[1]: https://evil/ref\n\ntext')
  expect(lines).toEqual([
    { kind: 'heading', text: 'Warum' },
    { kind: 'bullet', text: 'ok fett und code' },
    { kind: 'text', text: 'x  Doku A' },
    { kind: 'text', text: 'text' },
  ])
})

test('topRows: die größten belegten Zeilen, ohne freien Platz', () => {
  const b = { total: 10, window: 100, percent: 10, rows: [
    { name: 'a', tokens: 1, percent: 1, kind: 'used' as const },
    { name: 'frei', tokens: 90, percent: 90, kind: 'free' as const },
    { name: 'b', tokens: 5, percent: 5, kind: 'used' as const },
    { name: 'c', tokens: 3, percent: 3, kind: 'used' as const },
  ] }
  expect(topRows(b, 2).map(r => r.name)).toEqual(['b', 'c'])
})

test('settingsOf: ungültige Sprache fällt auf Deutsch zurück; Zusammenfassung ist standardmäßig nicht automatisch', () => {
  expect(settingsOf({ lang: 'fr' as never }).lang).toBe('de')
  expect(settingsOf({ lang: 'en' }).lang).toBe('en')
  expect(DEFAULT_SETTINGS.catchUp).toBe(false)
})

test('timelineSvg: Balken an Start und Dauer auf der gemeinsamen Achse, mindestens 3 px', () => {
  expect(timelineSvg(50, 100, 0, 100, '#0f0', 100)).toContain('<rect x="50.0" y="0" width="50.0"')
  expect(timelineSvg(0, 0, 0, 100, '#0f0', 100)).toContain('width="3.0"')
})

test('modelLabel: lesbarer Name aus der Modell-ID, Unbekanntes bleibt', () => {
  expect(modelLabel('claude-opus-5-5')).toBe('Opus 5.5')
  expect(modelLabel('claude-sonnet-5-5[1m]')).toBe('Sonnet 5.5')
  expect(modelLabel('claude-haiku-4-5-20251001')).toBe('Haiku 4.5')
  expect(modelLabel('gpt-x')).toBe('gpt-x')
})

test('withClaude: Claudes Vorschlag vorn, gleicher Prompt nur einmal, höchstens 3', () => {
  const s = (p: string) => ({ label: p, prompt: p })
  const hint = { label: 'Tests ausführen', prompt: 'Tests ausführen', why: 'Vorschlag von Claude' }
  const list = [s('Commit erstellen'), s('tests ausführen'), s('A'), s('B'), s('C')]
  expect(withClaude(list, hint).map(x => x.prompt)).toEqual(['Tests ausführen', 'Commit erstellen', 'A'])
  expect(withClaude(list, null)).toHaveLength(3)
  expect(withClaude([], hint)).toEqual([hint])
})

test('parseTaskNotifications: alle gebündelten Benachrichtigungen; leer ohne Kennung', () => {
  const one = (id: string, status: string) => `<task-notification>
<task-id>${id}</task-id>
<status>${status}</status>
<summary>S ${id}</summary>
</task-notification>`
  expect(parseTaskNotifications(one('b1x', 'completed'))).toEqual([{ id: 'b1x', status: 'completed', summary: 'S b1x' }])
  expect(parseTaskNotifications(`${one('a', 'failed')}\nText\n${one('b', 'killed')}`).map(n => `${n.id}:${n.status}`)).toEqual(['a:failed', 'b:killed'])
  expect(parseTaskNotifications('<task-notification>\n<summary>Review fertig</summary>\n</task-notification>')).toEqual([])
  expect(parseTaskNotifications('normaler Prompt')).toEqual([])
})

test('backgroundOf: Shell (auch per Timeout), Monitor, Workflow, Cloud-Agent; lokale Agents und Vordergrund nicht', () => {
  expect(backgroundOf('PowerShell', { command: 'npm test', description: 'Tests' }, { backgroundTaskId: 'b1', timedOutAfterMs: 120000 })).toEqual({ id: 'b1', description: 'Tests', command: 'npm test' })
  expect(backgroundOf('Bash', { command: 'ls' }, { stdout: '' })).toBe(null)
  expect(backgroundOf('Monitor', { description: 'CI', command: 'gh run watch' }, { taskId: 'm1', timeoutMs: 0 })).toEqual({ id: 'm1', description: 'CI', command: 'gh run watch' })
  expect(backgroundOf('Workflow', { script: 'x' }, { status: 'async_launched', taskId: 'w1', workflowName: 'review' })).toEqual({ id: 'w1', description: 'review' })
  expect(backgroundOf('Agent', { description: 'Cloud' }, { status: 'remote_launched', taskId: 'r1', description: 'Cloud', sessionUrl: 'u' })?.id).toBe('r1')
  expect(backgroundOf('Agent', { description: 'lokal' }, { status: 'async_launched', agentId: 'a1' })).toBe(null)
})

test('todosOf/wrapupOf: Fortschritt aus TodoWrite und update_plan, Teil-Updates und fremde Tools null', () => {
  expect(todosOf('TodoWrite', { todos: [{ content: 'a', status: 'completed' }, { content: 'b', status: 'in_progress', activeForm: 'Baut b' }, { content: 'c', status: 'pending' }] })).toEqual({ done: 1, total: 3, current: 'Baut b' })
  expect(todosOf('mcp__ccd_turn__update_plan', { items: [{ content: 'x', status: 'in_progress' }] })).toEqual({ done: 0, total: 1, current: 'x' })
  expect(todosOf('mcp__ccd_turn__update_plan', { advance: true })).toBe(null)
  expect(todosOf('Read', { todos: [] })).toBe(null)
  expect(wrapupOf('mcp__ccd_turn__wrap_up', { recap: 'Fertig.', ask: 'Commit?' })).toEqual({ recap: 'Fertig.', ask: 'Commit?' })
  expect(wrapupOf('mcp__ccd_turn__wrap_up', {})).toBe(null)
})

test('splitTool: MCP-Namen in Server und Tool, Plugin-Präfix gekürzt, eingebaute Tools unverändert', () => {
  expect(splitTool('mcp__dev-workbench__plan_session')).toEqual({ server: 'dev-workbench', tool: 'plan_session' })
  expect(splitTool('mcp__plugin_context-mode_context-mode__ctx_search')).toEqual({ server: 'context-mode', tool: 'ctx_search' })
  expect(splitTool('mcp__plugin_oh-my-claudecode_t__lsp_hover')).toEqual({ server: 'oh-my-claudecode_t', tool: 'lsp_hover' })
  expect(splitTool('Bash')).toEqual({ server: null, tool: 'Bash' })
  expect(toolText('mcp__dev-workbench__plan_session')).toBe('dev-workbench › plan_session')
})

test('summarySections: Abschnitt je Überschrift, Vorspann ohne Titel', () => {
  const s = summarySections(summaryLines('Vorspann\n## Ziel\n- a\n- b\n## Stand\ntext'))
  expect(s.map(x => x.heading)).toEqual([undefined, 'Ziel', 'Stand'])
  expect(s.map(x => x.lines.length)).toEqual([1, 2, 1])
})

test('parseLinkedSessions: Felder aus list_sessions, Unbrauchbares ergibt []', () => {
  const text = JSON.stringify([{ sessionId: 'local_1', title: 'T', name: 'n', branch: 'b', cwd: 'C:/x', isRunning: true, isArchived: false }, { title: 'ohne id' }])
  expect(parseLinkedSessions(text)).toEqual([{ id: 'local_1', title: 'T', name: 'n', branch: 'b', cwd: 'C:/x', running: true, archived: false }])
  expect(parseLinkedSessions('kein json')).toEqual([])
  expect(parseLinkedSessions(undefined)).toEqual([])
})

test('dirTarget/isForeignDir: Wechsel per cd, git -C, Desktop und Schreiben; Temp, AppData und Punkt-Ordner zählen nicht', () => {
  const cwd = 'C:\\WORKSPACE\\k3c'
  const home = 'C:\\Users\\r'
  expect(dirTarget('PowerShell', { command: 'cd C:\\WORKSPACE\\ErpApi; dotnet build' }, cwd, home)).toBe('C:\\WORKSPACE\\ErpApi')
  expect(dirTarget('Bash', { command: 'git -C /c/WORKSPACE/ErpApi status' }, cwd, home)).toBe('/c/WORKSPACE/ErpApi')
  expect(dirTarget('Bash', { command: 'cd src && ls' }, cwd, home)).toBe('C:\\WORKSPACE\\k3c/src')
  expect(dirTarget('mcp__ccd_directory__change_directory', { path: 'C:\\WORKSPACE\\ErpApi' }, cwd, home)).toBe('C:\\WORKSPACE\\ErpApi')
  expect(dirTarget('Write', { file_path: 'C:/WORKSPACE/ErpApi/a.cs' }, cwd, home)).toBe('C:/WORKSPACE/ErpApi')
  expect(dirTarget('Read', { file_path: 'C:/WORKSPACE/ErpApi/a.cs' }, cwd, home)).toBe(null)
  expect(dirTarget('Bash', { command: 'ls -la' }, cwd, home)).toBe(null)
  expect(isForeignDir('C:/WORKSPACE/ErpApi', cwd, home)).toBe(true)
  expect(isForeignDir('/c/WORKSPACE/ErpApi', cwd, home)).toBe(true)
  expect(isForeignDir('C:\\WORKSPACE\\k3c/src', cwd, home)).toBe(false)
  expect(isForeignDir('/c/workspace/K3C/.claude/worktrees/x', cwd, home)).toBe(false)
  expect(isForeignDir('C:/Users/r/.claude/projects', cwd, home)).toBe(false)
  expect(isForeignDir('C:/Users/r/AppData/Local/Temp/x', cwd, home)).toBe(false)
})

test('resetLeft: Minuten, Stunden, Tage; abgelaufen 0m', () => {
  const now = Date.UTC(2026, 9, 15, 10, 0)
  expect(resetLeft(new Date(now + 12 * 60_000).toISOString(), now)).toBe('12m')
  expect(resetLeft(new Date(now + 133 * 60_000).toISOString(), now)).toBe('2h 13m')
  expect(resetLeft(new Date(now + (3 * 24 + 4) * 3600_000).toISOString(), now)).toBe('3d 4h')
  expect(resetLeft(new Date(now - 60_000).toISOString(), now)).toBe('0m')
})
