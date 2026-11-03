import { expect, mock, test } from 'claude-code/testing'
import type { TestBody } from 'claude-code/testing'

// Reiter Sessions im Panel: liest die Status-Datei (nur lesen), zeichnet Übersicht und Freigabe-Inbox, fängt Lesefehler ab und
// liest bei geänderter Datei neu. Parser und Sortierung: sessions.test.ts.

const PROPS = { title: 'Cockpit', isFocused: false, bodyColumns: 160, placement: 'dock', scroll: { offset: 0, bodyRows: 60 }, view: {} } as const
type Engine = Parameters<TestBody>[0]
type On = Parameters<TestBody>[1]
type View = { find(q: { type: 'Text'; text: string | RegExp } | { type: 'Button'; key: string }): Promise<unknown>; press(t: { key: string }): Promise<unknown> }

const SAMPLE = {
  updated: '2026-11-02',
  sessions: [
    { id: 'om-01', title: 'Cockpit', repo: 'orga-marketplace', branch: 'feature/cockpit-sessions-panel', state: 'läuft', waitingFor: '-', note: 'baut Panel' },
    { id: 'qs-01', title: 'EXE neu bauen', repo: 'qs-automation', branch: 'main', state: 'wartet', waitingFor: 'Ralf', note: 'build_runtime.py uncommittet' },
  ],
  approvals: [
    { id: 'qs-01', action: 'commit', detail: 'build_runtime.py committen', since: '2026-11-02' },
    { id: 'om-00', action: 'push', detail: '3 lokale Commits', since: '2026-11-02' },
  ],
}

// Datei als Zustand: Inhalt null = fehlt (stat und read werfen), mtime zählt bei jeder Änderung hoch.
function wireFile(on: On, file: { text: string | null; mtime: number; reads?: number; paths?: string[] }, home = 'C:\\Users\\anna') {
  const clock = mock.clock(on, { now: 1 })
  on('env.get', (_$, e) => ({ value: e.name === 'USERPROFILE' ? home : undefined }) as never)
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  on('ui.open', () => ({ value: { isPlaced: true as const } }))
  on('fs.stat', (_$, e) => {
    file.paths?.push(e.path)
    if (file.text === null) throw new Error('ENOENT')
    return { value: { kind: 'file' as const, size: file.text.length, mtimeMs: file.mtime, isLink: false } }
  })
  on('fs.read', () => {
    file.reads = (file.reads ?? 0) + 1
    if (file.text === null) throw new Error('ENOENT')
    return { value: file.text } as never
  })
  return clock
}

async function waitFind(view: View, text: string | RegExp) {
  for (let i = 0; i < 3000; i++) {
    const hit = await view.find({ type: 'Text', text })
    if (hit !== undefined) return hit
    await Promise.resolve()
  }
  return undefined
}

async function openSessions($: Engine, surface: 'desktop' | 'terminal' = 'desktop') {
  await $.command.run({ command: 'cockpit-sessions', args: '' } as never)
  return $.ui.mount({ plugin: 'tenvan-cc', surface, component: 'Pane', requestId: 'tenvan-cc', props: PROPS })
}

test('Sessions: Beispieldatei rendert Übersicht und Inbox, Wartende zuerst, Stand der Datei steht oben', async ($, on) => {
  wireFile(on, { text: JSON.stringify(SAMPLE), mtime: 1 })
  const pane = await openSessions($)
  expect(await waitFind(pane, 'Stand der Datei: 2026-11-02')).not.toBe(undefined)
  expect(await waitFind(pane, '1 warten auf dich')).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: 'Freigabe-Inbox' })).not.toBe(undefined)
  for (const word of ['qs-01', 'om-01', 'wartet', 'läuft', 'commit', 'push', 'om-00']) expect(await pane.find({ type: 'Text', text: new RegExp(word) })).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /Keine offenen Freigaben/ })).toBe(undefined)
  // Wartende Session oben: qs-01 steht vor om-01 (Reihenfolge der Spalten-IDs im gezeichneten Baum).
  const tree = JSON.stringify(await pane.find({ type: 'Box', key: 'sl-grid' }))
  expect(tree.indexOf('qs-01')).toBeGreaterThan(-1)
  expect(tree.indexOf('qs-01')).toBeLessThan(tree.indexOf('om-01'))
})

test('Sessions: ohne Einstellung liest das Cockpit <Benutzerordner>/.deltra/claude/sessions.json und zeigt den Pfad', async ($, on) => {
  const file = { text: JSON.stringify(SAMPLE), mtime: 1, paths: [] as string[] }
  wireFile(on, file, 'C:\\Users\\anna')
  const pane = await openSessions($)
  expect(await waitFind(pane, 'Stand der Datei: 2026-11-02')).not.toBe(undefined)
  expect(file.paths[0]?.replace(/\\/g, '/')).toBe('C:/Users/anna/.deltra/claude/sessions.json')
  expect(await pane.find({ type: 'Text', text: 'C:/Users/anna/.deltra/claude/sessions.json' })).not.toBe(undefined)
})

test('Sessions: die Einstellung sessionsFile hat Vorrang vor dem Standardpfad', { options: { sessionsFile: 'D:/anders/s.json' } }, async ($, on) => {
  const file = { text: JSON.stringify(SAMPLE), mtime: 1, paths: [] as string[] }
  wireFile(on, file)
  const pane = await openSessions($)
  expect(await waitFind(pane, 'Stand der Datei: 2026-11-02')).not.toBe(undefined)
  expect(file.paths[0]?.replace(/\\/g, '/')).toBe('D:/anders/s.json')
})

test('Sessions: fehlender Ordner .deltra zeigt die Meldung mit dem Pfad im Benutzerordner', async ($, on) => {
  wireFile(on, { text: null, mtime: 1 }, 'C:\\Users\\anna')
  const pane = await openSessions($)
  expect(await waitFind(pane, /Status-Datei nicht lesbar: C:\/Users\/anna\/\.deltra\/claude\/sessions\.json \(ENOENT\)/)).not.toBe(undefined)
})

test('Sessions: leere Inbox zeigt die klare Meldung', async ($, on) => {
  wireFile(on, { text: JSON.stringify({ ...SAMPLE, approvals: [] }), mtime: 1 })
  const pane = await openSessions($)
  expect(await waitFind(pane, /Keine offenen Freigaben/)).not.toBe(undefined)
})

test('Sessions: fehlende Datei, leere Datei und kaputtes JSON zeigen eine lesbare Meldung statt eines Fehlers', async ($, on) => {
  const file = { text: null as string | null, mtime: 1 }
  wireFile(on, file)
  const pane = await openSessions($)
  expect(await waitFind(pane, /Status-Datei nicht lesbar: .*sessions\.json \(ENOENT\)/)).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: 'Freigabe-Inbox' })).toBe(undefined)
  // Neu lesen (Klick) nach dem Anlegen einer leeren, dann einer kaputten Datei.
  file.text = '   '
  file.mtime = 2
  await pane.press({ key: 'sessions-reload' })
  expect(await waitFind(pane, /Status-Datei ist leer/)).not.toBe(undefined)
  file.text = '{ "updated": '
  file.mtime = 3
  await pane.press({ key: 'sessions-reload' })
  expect(await waitFind(pane, /Status-Datei ist kein gültiges JSON/)).not.toBe(undefined)
})

test('Sessions: nach gutem Stand bleibt er bei kaputter Datei stehen, die Meldung kommt dazu; reparierte Datei ersetzt ihn', async ($, on) => {
  const file = { text: JSON.stringify(SAMPLE) as string | null, mtime: 1 }
  wireFile(on, file)
  const pane = await openSessions($)
  expect(await waitFind(pane, 'Stand der Datei: 2026-11-02')).not.toBe(undefined)
  file.text = '{ kaputt'
  file.mtime = 2
  await pane.press({ key: 'sessions-reload' })
  expect(await waitFind(pane, /kein gültiges JSON/)).not.toBe(undefined)
  expect(await waitFind(pane, /Der letzte lesbare Stand bleibt stehen/)).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /EXE neu bauen/ })).not.toBe(undefined)
  file.text = JSON.stringify({ ...SAMPLE, updated: '2026-11-03' })
  file.mtime = 3
  await pane.press({ key: 'sessions-reload' })
  expect(await waitFind(pane, 'Stand der Datei: 2026-11-03')).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /kein gültiges JSON/ })).toBe(undefined)
})

test('Sessions: fehlende Felder erscheinen als Hinweis, die übrigen Zeilen bleiben sichtbar', async ($, on) => {
  wireFile(on, { text: JSON.stringify({ sessions: [{ id: 'x-01', state: 'schläft' }], approvals: [] }), mtime: 1 })
  const pane = await openSessions($)
  expect(await waitFind(pane, 'Hinweise zur Datei (3)')).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /sessions\[0\]: repo, branch fehlt/ })).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /x-01/ })).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: 'schläft' })).not.toBe(undefined)
})

test('Sessions: der Takt liest bei geänderter Datei neu und lässt sie sonst in Ruhe', async ($, on) => {
  const file = { text: JSON.stringify(SAMPLE) as string | null, mtime: 1, reads: 0 }
  const clock = wireFile(on, file)
  on('session.model', () => ({ value: 'claude-opus-5-5' }) as never)
  on('session.version', () => ({ value: { version: '2.1.295' } }) as never)
  on('session.start', ($, e) => ({ cwd: e.cwd }))
  on('command.register', ($, e) => ({ value: { command: e.name } }))
  await $.session.start({ cwd: '.', surface: 'desktop', isInteractive: true })
  const pane = await openSessions($)
  expect(await waitFind(pane, 'Stand der Datei: 2026-11-02')).not.toBe(undefined)
  const first = file.reads
  await clock.advance(5_000)
  await clock.advance(5_000)
  expect(file.reads).toBe(first)
  file.text = JSON.stringify({ ...SAMPLE, updated: '2026-11-09' })
  file.mtime = 2
  await clock.advance(5_000)
  expect(await waitFind(pane, 'Stand der Datei: 2026-11-09')).not.toBe(undefined)
})

test('Sessions: Terminal zeichnet dieselben Zeilen ohne Svg', async ($, on) => {
  wireFile(on, { text: JSON.stringify(SAMPLE), mtime: 1 })
  const pane = await openSessions($, 'terminal')
  expect(await waitFind(pane, '1 warten auf dich')).not.toBe(undefined)
  expect(await pane.find({ type: 'Svg' })).toBe(undefined)
})

// Prompt-Angebot: `prompt.fill` sammelt den Text, geschrieben wird nie (kein fs.write).
function wirePrompt(on: On): string[] {
  const filled: string[] = []
  on('prompt.fill', (_$, e) => {
    filled.push(e.text)
    return { isFilled: true, box: { text: e.text, cursor: e.text.length } } as never
  })
  return filled
}
const waitPress = async (pane: View, key: string) => {
  for (let i = 0; i < 3000 && (await pane.find({ type: 'Button', key })) === undefined; i++) await Promise.resolve()
  await pane.press({ key })
}

test('Sessions: fehlt die Datei, bietet der Reiter einen Prompt zum Einrichten einer Schaltzentrale mit dem Pfad', async ($, on) => {
  const filled = wirePrompt(on)
  wireFile(on, { text: null, mtime: 1 }, 'C:\\Users\\anna')
  const pane = await openSessions($)
  await waitPress(pane, 'sessions-prompt')
  for (let i = 0; i < 3000 && filled.length === 0; i++) await Promise.resolve()
  expect(filled).toHaveLength(1)
  expect(filled[0]).toContain('C:/Users/anna/.deltra/claude/sessions.json')
  expect(filled[0]).toMatch(/Richte auf diesem Rechner eine Schaltzentrale ein/)
  expect(filled[0]).toContain('"updated"')
  expect(filled[0]).toContain('ENOENT')
})

test('Sessions: kaputtes JSON oder Hinweise des Parsers bieten einen Prompt zum Anpassen; eine gültige Datei ohne Hinweise keinen', async ($, on) => {
  const filled = wirePrompt(on)
  const file = { text: '{ "updated": ' as string | null, mtime: 1 }
  wireFile(on, file)
  const pane = await openSessions($)
  await waitPress(pane, 'sessions-prompt')
  for (let i = 0; i < 3000 && filled.length === 0; i++) await Promise.resolve()
  expect(filled[0]).toMatch(/passt nicht zum Schema/)
  expect(filled[0]).toContain('json:')
  expect(filled[0]).not.toMatch(/Richte auf diesem Rechner/)
  // Gültig, aber mit Hinweis (Session ohne Branch): weiter Anpassen.
  file.text = JSON.stringify({ updated: '2026-11-02', sessions: [{ id: 'a', repo: 'r', state: 'läuft' }], approvals: [] })
  file.mtime = 2
  await pane.press({ key: 'sessions-reload' })
  expect(await waitFind(pane, /Hinweise zur Datei \(1\)/)).not.toBe(undefined)
  expect(await pane.find({ type: 'Button', key: 'sessions-prompt' })).not.toBe(undefined)
  // Sauber: kein Angebot mehr.
  file.text = JSON.stringify(SAMPLE)
  file.mtime = 3
  await pane.press({ key: 'sessions-reload' })
  expect(await waitFind(pane, 'Stand der Datei: 2026-11-02')).not.toBe(undefined)
  for (let i = 0; i < 300 && (await pane.find({ type: 'Button', key: 'sessions-prompt' })) !== undefined; i++) await Promise.resolve()
  expect(await pane.find({ type: 'Button', key: 'sessions-prompt' })).toBe(undefined)
})
