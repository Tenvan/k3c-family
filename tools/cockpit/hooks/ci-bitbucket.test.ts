import { expect, test } from 'claude-code/testing'

import { bbRollup, bbSafeUrl, bbStatus, toBbBuilds, toBbCommits } from './ci-bitbucket'

// Bitbucket Server, nur lesen: Übersetzung der Antworten von `commits` und `build-status` in Commits mit Builds, gegen Mock-Daten.
// Panel und Aufrufe: ci-bitbucket-panel.test.ts.

const ESC = String.fromCharCode(27)
const BEL = String.fromCharCode(7)
const SHA = '0123456789abcdef0123456789abcdef01234567'

test('bbStatus und bbRollup: Bitbucket-Vokabular auf Badges, ein roter Build färbt den Commit, ohne Builds leer', () => {
  expect([bbStatus('SUCCESSFUL'), bbStatus('FAILED'), bbStatus('INPROGRESS'), bbStatus('STOPPED'), bbStatus(''), bbStatus('weird')]).toEqual(['success', 'failed', 'running', 'canceled', 'created', 'weird'])
  const b = (state: string) => ({ state, key: 'k', name: 'n', url: '', description: '', at: '', durationS: 0 })
  expect(bbRollup([])).toBe('')
  expect(bbRollup([b('SUCCESSFUL'), b('SUCCESSFUL')])).toBe('success')
  expect(bbRollup([b('SUCCESSFUL'), b('INPROGRESS')])).toBe('running')
  expect(bbRollup([b('INPROGRESS'), b('FAILED'), b('SUCCESSFUL')])).toBe('failed')
})

test('toBbCommits: Felder, Autor-Anzeigename vor Login, erste Zeile der Meldung, ungültige Commit-Ids verworfen', () => {
  const raw = { values: [
    { id: SHA, displayId: '0123456789a', message: 'Export vereinfachen\n\nLangtext', author: { name: 'rstich', displayName: 'Ralf Stich' }, authorTimestamp: Date.parse('2026-11-02T10:00:00Z') },
    { id: 'abcdef1', message: 'ohne Anzeige', author: { name: 'anna' } },
    { id: '../../etc/passwd', message: 'böse' },
    { message: 'ohne Id' },
  ] }
  const c = toBbCommits(raw)
  expect(c).toHaveLength(2)
  expect(c[0]).toEqual({ sha: SHA, short: '0123456789a', title: 'Export vereinfachen', author: 'Ralf Stich', at: '2026-11-02T10:00:00.000Z', builds: [] })
  expect(c[1]).toMatchObject({ sha: 'abcdef1', short: 'abcdef1', author: 'anna', at: '' })
  expect(toBbCommits(null)).toEqual([])
})

test('toBbBuilds: Status, Name (sonst Key), Zeitpunkt, Dauer nur wenn gemeldet, neueste zuerst; Links nur http(s)', () => {
  const raw = { values: [
    { state: 'SUCCESSFUL', key: 'PLAN-1', name: 'Build', url: 'https://bamboo.deltra.de/browse/PLAN-1-5', description: 'Build #5 passed', dateAdded: Date.parse('2026-11-02T10:05:00Z'), duration: 65_000 },
    { state: 'FAILED', key: 'JOB-2', url: 'javascript:alert(1)', dateAdded: Date.parse('2026-11-02T10:10:00Z') },
    { key: 'ohne-state' },
  ] }
  const b = toBbBuilds(raw)
  expect(b.map(x => [x.state, x.name, x.durationS, x.url])).toEqual([['FAILED', 'JOB-2', 0, ''], ['SUCCESSFUL', 'Build', 65, 'https://bamboo.deltra.de/browse/PLAN-1-5']])
  expect(b[1]?.description).toBe('Build #5 passed')
  expect(toBbBuilds({})).toEqual([])
})

test('Sanitizing und Links: Escape-Folgen und Steuerzeichen aus Bitbucket-Text entfernt, fremde Schemata abgelehnt', () => {
  const evil = `${ESC}]0;gehackt${BEL}Titel${ESC}[31m rot${ESC}[0m\u0001`
  const [c] = toBbCommits({ values: [{ id: SHA, message: evil, author: { displayName: evil } }] })
  expect([c?.title, c?.author]).toEqual(['Titel rot', 'Titel rot'])
  const [b] = toBbBuilds({ values: [{ state: 'FAILED', key: evil, name: evil, description: evil }] })
  expect([b?.name, b?.key, b?.description]).toEqual(['Titel rot', 'Titel rot', 'Titel rot'])
  expect([bbSafeUrl('https://a.de/x'), bbSafeUrl('http://a.de'), bbSafeUrl('file:///c:/x'), bbSafeUrl('javascript:1'), bbSafeUrl('https://a.de/ x')]).toEqual(['https://a.de/x', 'http://a.de', '', '', ''])
})
