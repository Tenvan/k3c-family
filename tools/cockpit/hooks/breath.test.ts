import { expect, mock, test } from 'claude-code/testing'

import { BREATHS, breathFrameSvg, breathOf, breathSvg, cycleMs, fillAt, phaseAt, segments } from './breath'

const L = { in: 'Einatmen', sip: 'Nachatmen', hold: 'Halten', out: 'Ausatmen' }

test('Atemübung: Phasen, Restzeit und Zyklus je Übung', () => {
  const box = breathOf('box')
  expect(cycleMs(box)).toBe(16_000)
  expect(phaseAt(box, 1_000)).toEqual({ phase: 'in', leftMs: 3_000 })
  expect(phaseAt(box, 5_000).phase).toBe('hold')
  expect(phaseAt(box, 9_000).phase).toBe('out')
  expect(phaseAt(box, 13_000).phase).toBe('hold')
  expect(phaseAt(box, 16_500)).toEqual({ phase: 'in', leftMs: 3_500 })
  expect(segments(breathOf('sigh')).map(s => s.phase)).toEqual(['in', 'sip', 'out'])
  expect(segments(breathOf('hrv')).map(s => s.phase)).toEqual(['in', 'out'])
  expect(breathOf('unbekannt').key).toBe('hrv')
})

test('Atemübung: Svg animiert Kreis und Wörter über einen ganzen Zyklus', () => {
  for (const b of BREATHS) {
    const svg = breathSvg(b, L, b.key, '#8eadfd', '#8c929c')
    expect(svg).toContain(`dur="${cycleMs(b) / 1000}s"`)
    // keyTimes beginnen bei 0 und enden bei 1, eine Wortzeile je Abschnitt
    expect(svg).toMatch(/keyTimes="0;[^"]*;1"/)
    expect(svg.match(/<text /g)?.length).toBe(segments(b).length + 1)
  }
  expect(breathSvg(breathOf('sigh'), L, 'x', '#fff', '#999')).toContain('Nachatmen')
})

test('Atemübung: Kreis im Band und Phase im Spinner nur während eines Turns', async ($, on) => {
  mock.clock(on, { now: 3_600_000 })
  on('session.measure', () => ({ changed: [] }))
  on('prompt.submit', () => ({ text: 'los' }))
  await $.prompt.submit({ text: 'los' } as never)
  const props = { word: 'Creating notes.md', message: null, suffix: '…', mode: 'thinking' as const }
  const line = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Spinner', props })
  expect(await line.find({ type: 'Text', text: /^Einatmen \d+s ·$/ })).not.toBe(undefined)
  expect(await line.find({ type: 'Text', text: 'Creating notes.md' })).not.toBe(undefined)
})

test('Atemübung: Einzelbild im Turn wächst beim Einatmen und nennt die Restzeit', () => {
  const box = breathOf('box')
  expect(fillAt(box, 0)).toBe(0)
  expect(fillAt(box, 2_000)).toBeGreaterThan(0.5)
  expect(fillAt(box, 6_000)).toBe(1)
  expect(fillAt(box, 14_000)).toBe(0)
  const svg = breathFrameSvg(box, 1_000, L, 'Box', '#8eadfd', '#8c929c')
  expect(svg).toContain('Einatmen 3s')
  expect(svg).not.toContain('<animate')
})
