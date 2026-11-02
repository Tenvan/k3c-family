import { expect, test } from 'claude-code/testing'

import type { LogEntry } from '../types'
import type { ErrEntry } from './errors'
import { protocolCounts, protocolRows } from './protocol'
import type { Step } from './timeline'

// Protokoll: Log, Fehler-Eingang, Freigaben und Zeitreise als eine Liste (Reiter Diagnose → Protokoll).

const H = Date.UTC(2026, 9, 28, 10)
const LOG: LogEntry[] = [
  { at: H, src: 'you', text: 'Baue das Band um' },
  { at: H + 1_000, src: 'main', text: 'Read', tool: 'Read', target: 'band.ts', isError: false },
  { at: H + 2_000, src: 'main', text: 'Edit', tool: 'Edit', target: 'band.ts', isError: false },
  { at: H + 3_000, src: 'main', text: 'Bash', tool: 'Bash', target: 'npm test', isError: true },
  { at: H + 4_000, src: 'gate', text: 'Bash rm -rf abgelehnt' },
]
const STEPS: Step[] = [
  { n: 1, at: H + 1_100, tool: 'Read', target: 'band.ts', kind: 'read', ok: true },
  { n: 2, at: H + 2_200, tool: 'Edit', target: 'band.ts', kind: 'edit', ok: true, snap: 'abc' },
]
const ERRS: ErrEntry[] = [
  { n: 1, at: H + 3_050, tool: 'Bash', target: 'npm test', kind: 'error', text: 'FAIL band.test.ts', agent: false, times: 2 },
  { n: 2, at: H + 5_000, tool: 'Grep', target: 'x', kind: 'error', text: 'no such dir', agent: true, times: 1 },
]

test('Protokoll: ein Eintrag je Aufruf, Snapshot und Fehlerzeile hängen am passenden Tool-Eintrag', () => {
  const rows = protocolRows(LOG, ERRS, STEPS, true, 'all')
  // 5 Log-Einträge + 1 Subagent-Fehler + Start-Snapshot; Edit-Snapshot und Bash-Fehler erzeugen keine eigene Zeile.
  expect(rows.length).toBe(7)
  // Neueste zuerst, der Start-Snapshot (#0) ganz unten.
  expect(rows[0]?.tool).toBe('Grep')
  expect(rows[0]?.agent).toBe(true)
  expect(rows[rows.length - 1]?.snap).toBe(0)
  const edit = rows.find(r => r.tool === 'Edit')
  expect(edit?.snap).toBe(2)
  const bash = rows.find(r => r.tool === 'Bash')
  expect([bash?.errText, bash?.times, bash?.isError]).toEqual(['FAIL band.test.ts', 2, true])
})

test('Protokoll: Filter Fehler, Freigaben und Snapshots, Zähler ohne Kappung', () => {
  expect(protocolRows(LOG, ERRS, STEPS, true, 'errors').map(r => r.tool)).toEqual(['Grep', 'Bash'])
  expect(protocolRows(LOG, ERRS, STEPS, true, 'gate').map(r => r.src)).toEqual(['gate'])
  expect(protocolRows(LOG, ERRS, STEPS, true, 'snaps').map(r => r.snap)).toEqual([2, 0])
  expect(protocolRows(LOG, ERRS, STEPS, false, 'snaps').map(r => r.snap)).toEqual([2])
  // Fehler zählen mit Wiederholungen (Bash ×2 + Grep) wie die Bilanz des Fehler-Eingangs.
  expect(protocolCounts(LOG, ERRS, STEPS, true)).toEqual({ all: 7, errors: 3, gate: 1, snaps: 2 })
  // Kappung: höchstens `max` Einträge, die neuesten.
  expect(protocolRows(LOG, ERRS, STEPS, false, 'all', 2).map(r => r.tool)).toEqual(['Grep', undefined])
})

test('Protokoll: Snapshot ohne Log-Eintrag (Log gekappt) und abgelehnter Aufruf stehen als eigene Zeilen', () => {
  const rows = protocolRows([], [{ n: 1, at: H, tool: 'Bash', target: 'rm', kind: 'denied', text: 'nicht erlaubt', agent: false, times: 1 }], [{ n: 7, at: H - 1_000, tool: 'Write', target: 'a.ts', kind: 'edit', ok: true, snap: 'def' }], false, 'all')
  expect(rows.map(r => [r.tool, r.snap, r.denied])).toEqual([['Bash', undefined, true], ['Write', 7, undefined]])
  expect(protocolRows([], [{ n: 1, at: H, tool: 'Bash', target: 'rm', kind: 'denied', text: 'nicht erlaubt', agent: false, times: 1 }], [], false, 'errors')).toEqual([])
  expect(protocolRows([], [{ n: 1, at: H, tool: 'Bash', target: 'rm', kind: 'denied', text: 'nicht erlaubt', agent: false, times: 1 }], [], false, 'gate').length).toBe(1)
})

test('Protokoll: tool_use_id verbindet auch lange Aufrufe und Subagent-Fehler, ohne Doppelzeilen', () => {
  // Echte Reihenfolge: Fehler-Eingang und Zeitreise stempeln beim Start, das Log beim Ende (hier 40 s später).
  const log: LogEntry[] = [
    { at: H + 40_000, src: 'main', text: 'Bash', tool: 'Bash', target: 'npm test', isError: true, id: 'u1' },
    { at: H + 41_000, src: 'agent', text: 'Grep', tool: 'Grep', target: 'x', isError: true, id: 'u2' },
    { at: H + 42_000, src: 'main', text: 'Bash', tool: 'Bash', target: 'git status', isError: false, id: 'u3' },
  ]
  const errs: ErrEntry[] = [
    { n: 1, at: H, tool: 'Bash', target: 'npm test', kind: 'error', text: 'FAIL', agent: false, times: 1, id: 'u1' },
    { n: 2, at: H + 40_500, tool: 'Grep', target: 'x', kind: 'error', text: 'no such dir', agent: true, times: 1, id: 'u2' },
  ]
  // Snapshot des ändernden Bash (u1) liegt zeitlich näher an „git status“ (u3) – die id gewinnt.
  const steps: Step[] = [{ n: 4, at: H, tool: 'Bash', target: 'npm test', kind: 'shell', ok: false, snap: 'abc', id: 'u1' }]
  const rows = protocolRows(log, errs, steps, false, 'all')
  expect(rows.length).toBe(3)
  const bash = rows.find(r => r.id === 'u1')
  expect([bash?.snap, bash?.errText]).toEqual([4, 'FAIL'])
  expect(rows.find(r => r.id === 'u3')?.snap).toBe(undefined)
  expect(rows.find(r => r.id === 'u2')?.errText).toBe('no such dir')
  expect(protocolCounts(log, errs, steps, false).errors).toBe(2)
})

test('Protokoll: der Start-Snapshot (#0) zählt in die Kappung', () => {
  const rows = protocolRows(LOG, ERRS, STEPS, true, 'all', 3)
  expect(rows.length).toBe(3)
  expect(rows[2]?.snap).toBe(0)
})
