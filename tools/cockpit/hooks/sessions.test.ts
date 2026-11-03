import { expect, test } from 'claude-code/testing'

import { actionOf, isWaiting, parseSessions, sessionsFilePath, sessionsFindings, sessionsPromptKind, standOf } from './sessions'

// Reiter Sessions: Parser der Status-Datei der Schaltzentrale, Sortierung und Fehlerfälle.

const SAMPLE = JSON.stringify({
  updated: '2026-11-02',
  sessions: [
    { id: 'om-01', title: 'Cockpit', repo: 'orga-marketplace', branch: 'feature/x', state: 'läuft', waitingFor: '-', note: 'baut' },
    { id: 'qs-02', title: 'Alt', repo: 'qs', branch: 'main', state: 'archiviert', waitingFor: '', note: '' },
    { id: 'qs-01', title: 'EXE', repo: 'qs-automation', branch: 'main', state: 'wartet', waitingFor: 'Ralf', note: 'build_runtime.py uncommittet' },
    { id: 'xx-01', title: 'Fertig', repo: 'x', branch: 'main', state: 'fertig', waitingFor: '–', note: '' },
  ],
  approvals: [
    { id: 'qs-01', action: 'commit', detail: 'build_runtime.py', since: '2026-11-02' },
    { id: 'om-00', action: 'umbenennen', detail: 'unbekannte Aktion', since: '2026-11-02' },
  ],
})

test('Sessions: Beispieldatei wird gelesen, wartende Sessions stehen oben, dann läuft, fertig, archiviert', () => {
  const r = parseSessions(SAMPLE)
  expect(r.ok).toBe(true)
  if (!r.ok) return
  expect(r.view.updated).toBe('2026-11-02')
  expect(r.view.sessions.map(s => s.id)).toEqual(['qs-01', 'om-01', 'xx-01', 'qs-02'])
  expect(r.view.issues).toEqual([])
  expect(r.view.sessions[0]).toEqual({ id: 'qs-01', title: 'EXE', repo: 'qs-automation', branch: 'main', state: 'wartet', stateRaw: 'wartet', waitingFor: 'Ralf', note: 'build_runtime.py uncommittet' })
})

test('Sessions: Freigaben behalten die Dateireihenfolge, unbekannte Aktionen werden „sonstiges“ mit Originaltext', () => {
  const r = parseSessions(SAMPLE)
  if (!r.ok) throw new Error('parse')
  expect(r.view.approvals.map(a => [a.id, a.action, a.actionRaw])).toEqual([
    ['qs-01', 'commit', 'commit'],
    ['om-00', 'sonstiges', 'umbenennen'],
  ])
})

test('Sessions: gleiche Gruppe bleibt in Dateireihenfolge, wartet über waitingFor auch ohne Stand „wartet“', () => {
  const r = parseSessions(JSON.stringify({
    updated: 'x',
    sessions: [
      { id: 'a', repo: 'r', branch: 'b', state: 'läuft', waitingFor: '-' },
      { id: 'b', repo: 'r', branch: 'b', state: 'läuft', waitingFor: 'Ralf' },
      { id: 'c', repo: 'r', branch: 'b', state: 'läuft', waitingFor: '-' },
    ],
    approvals: [],
  }))
  if (!r.ok) throw new Error('parse')
  expect(r.view.sessions.map(s => s.id)).toEqual(['b', 'a', 'c'])
})

test('Sessions: isWaiting, standOf und actionOf', () => {
  for (const w of ['', '-', '–', '—', ' niemand ', 'None']) expect(isWaiting(w, 'läuft')).toBe(false)
  expect(isWaiting('Ralf', 'läuft')).toBe(true)
  expect(isWaiting('-', 'wartet')).toBe(true)
  expect(standOf(' Läuft ')).toBe('läuft')
  expect(standOf('kaputt')).toBe('')
  expect(actionOf('LÖSCHEN')).toBe('löschen')
  expect(actionOf('')).toBe('sonstiges')
})

test('Sessions: Datei fehlt, leer, kein JSON, falsche Form liefern je einen Fehler mit Klartext-Art', () => {
  expect(parseSessions(null, 'ENOENT')).toEqual({ ok: false, kind: 'missing', detail: 'ENOENT' })
  expect(parseSessions('  \n')).toEqual({ ok: false, kind: 'empty', detail: '' })
  const bad = parseSessions('{ "updated": ')
  expect(bad.ok).toBe(false)
  if (!bad.ok) {
    expect(bad.kind).toBe('json')
    expect(bad.detail).not.toBe('')
  }
  expect(parseSessions('[1,2]')).toEqual({ ok: false, kind: 'shape', detail: 'Array' })
  expect(parseSessions('"text"')).toEqual({ ok: false, kind: 'shape', detail: 'string' })
  expect(parseSessions('null')).toEqual({ ok: false, kind: 'shape', detail: 'object' })
})

test('Sessions: UTF-8-BOM vor dem JSON stört nicht', () => {
  const r = parseSessions(String.fromCharCode(0xfeff) + SAMPLE)
  expect(r.ok).toBe(true)
})

test('Sessions: fehlende Felder und Blöcke werden als Hinweis gemeldet, die Zeilen bleiben erhalten', () => {
  const r = parseSessions(JSON.stringify({
    sessions: [{ id: 'a', state: 'schläft' }, 'quatsch', { repo: 'r', branch: 'b', state: 'läuft', waitingFor: 7 }],
    approvals: { id: 'x' },
  }))
  expect(r.ok).toBe(true)
  if (!r.ok) return
  expect(r.view.updated).toBe('')
  expect(r.view.issues).toEqual([
    'updated: fehlt',
    'sessions[0]: repo, branch fehlt',
    'sessions[0]: state „schläft“ unbekannt',
    'sessions[1]: kein Objekt',
    'sessions[2]: id fehlt',
    'approvals: kein Array',
  ])
  // 2 Zeilen (das Nicht-Objekt entfällt); unbekannter Stand bleibt mit Originaltext, Zahl wird Text.
  expect(r.view.sessions.length).toBe(2)
  const a = r.view.sessions.find(s => s.id === 'a')
  expect([a?.state, a?.stateRaw]).toEqual(['', 'schläft'])
  expect(r.view.sessions.find(s => s.id === '')?.waitingFor).toBe('7')
  expect(r.view.approvals).toEqual([])
})

test('Sessions: leeres Objekt ist lesbar, meldet aber alle fehlenden Blöcke', () => {
  const r = parseSessions('{}')
  expect(r.ok).toBe(true)
  if (r.ok) expect(r.view.issues).toEqual(['updated: fehlt', 'sessions: fehlt', 'approvals: fehlt'])
})

test('Sessions: Standardpfad liegt im Benutzerordner, der Override hat Vorrang', () => {
  expect(sessionsFilePath('', 'C:\\Users\\rstich')).toBe('C:/Users/rstich/.deltra/claude/sessions.json')
  expect(sessionsFilePath(undefined, '/home/anna/')).toBe('/home/anna/.deltra/claude/sessions.json')
  expect(sessionsFilePath('   ', 'C:/Users/x')).toBe('C:/Users/x/.deltra/claude/sessions.json')
  expect(sessionsFilePath(' D:/x/s.json ', 'C:/Users/x')).toBe('D:/x/s.json')
  expect(sessionsFilePath(42, 'C:/Users/x')).toBe('C:/Users/x/.deltra/claude/sessions.json')
  // Ohne Benutzerordner bleibt der relative Rest stehen und der Lesefehler wird als fehlende Datei gemeldet.
  expect(sessionsFilePath('', '')).toBe('.deltra/claude/sessions.json')
})

test('sessionsPromptKind/sessionsFindings: fehlt oder leer → einrichten, kaputt oder mit Hinweisen → anpassen, sonst kein Angebot', () => {
  expect(sessionsPromptKind({ kind: 'missing' }, [])).toBe('create')
  expect(sessionsPromptKind({ kind: 'empty' }, [])).toBe('create')
  expect(sessionsPromptKind({ kind: 'json' }, [])).toBe('fix')
  expect(sessionsPromptKind({ kind: 'shape' }, [])).toBe('fix')
  expect(sessionsPromptKind(null, ['sessions[0]: id fehlt'])).toBe('fix')
  expect(sessionsPromptKind(null, [])).toBe(null)
  expect(sessionsFindings({ kind: 'missing', detail: 'ENOENT' }, ['x'])).toEqual(['missing: ENOENT'])
  expect(sessionsFindings({ kind: 'empty', detail: '' }, [])).toEqual(['empty'])
  expect(sessionsFindings(null, Array.from({ length: 15 }, (_, i) => `h${i}`))).toHaveLength(10)
})
