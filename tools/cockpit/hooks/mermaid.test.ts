import { expect, test } from 'claude-code/testing'

import { THEME } from './core'
import { diagramType, flowRanks, mermaidSvg, parseMermaid } from './mermaid'
import { askSummary, mathSvg, splitMessage, texToText } from './skins'

const F = '```'
const ALL = { cards: true, inline: false, mermaid: true, math: true }

// ── Mermaid ──

test('Mermaid: Flowchart mit Formen, Kanten, Labels, Ketten und &', () => {
  const d = parseMermaid('graph LR\n  A[Start] -->|ja| B{Prüfen?}\n  B -- nein --> C((Ende)); B -.-> A\n  C & B --- D(Rund)\n  %% Kommentar\n  classDef x fill:#fff')
  expect(d?.type).toBe('flowchart')
  if (d?.type !== 'flowchart') return
  expect(d.dir).toBe('LR')
  expect(d.nodes.map(n => `${n.id}:${n.shape}:${n.label}`)).toEqual(['A:rect:Start', 'B:diamond:Prüfen?', 'C:circle:Ende', 'D:round:Rund'])
  expect(d.edges[0]).toEqual({ from: 'A', to: 'B', label: 'ja', dashed: false, arrow: true })
  expect(d.edges[1]).toMatchObject({ from: 'B', to: 'C', label: 'nein', arrow: true })
  expect(d.edges[2]).toMatchObject({ from: 'B', to: 'A', dashed: true, arrow: true })
  expect(d.edges.slice(3).map(e => `${e.from}${e.to}${e.arrow}`)).toEqual(['CDfalse', 'BDfalse'])
  // Zyklus B → A zählt nicht als Vorwärtskante: A bleibt Rang 0
  expect([...flowRanks(d).entries()]).toEqual([['A', 0], ['B', 1], ['C', 2], ['D', 3]])
})

test('Mermaid: Sequenzdiagramm mit Teilnehmern, Pfeilen, Notiz und Block', () => {
  const d = parseMermaid('sequenceDiagram\n  participant U as Nutzer\n  U->>S: Anfrage\n  loop alle 5s\n  S-->>U: Antwort\n  end\n  Note over U,S: fertig\n  S->S: intern')
  expect(d?.type).toBe('sequence')
  if (d?.type !== 'sequence') return
  expect(d.actors).toEqual([{ id: 'U', label: 'Nutzer' }, { id: 'S', label: 'S' }])
  expect(d.steps.map(s => s.kind)).toEqual(['msg', 'block', 'msg', 'note', 'msg'])
  expect(d.steps[2]).toEqual({ kind: 'msg', from: 'S', to: 'U', text: 'Antwort', dashed: true, arrow: true })
  expect(d.steps[4]).toMatchObject({ from: 'S', to: 'S', arrow: false })
})

test('Mermaid: andere Typen und Parse-Fehler ergeben null; Typ für den Kopf', () => {
  expect(parseMermaid('pie title X\n "a": 1')).toBe(null)
  expect(parseMermaid('graph TD\n  A --> ')).toBe(null)
  expect(parseMermaid('graph TD')).toBe(null)
  expect(parseMermaid('sequenceDiagram\n  kaputt')).toBe(null)
  expect(diagramType('graph TD\nA-->B')).toBe('flowchart')
  expect(diagramType('%% x\nsequenceDiagram\nA->>B: x')).toBe('sequenceDiagram')
})

test('Mermaid-Svg: stabile Quelle, maskierte Texte, Breite gekappt, keine Animation', () => {
  const d = parseMermaid('graph TD\n  A["a < b"] --> B')
  if (d === null) throw new Error('parse')
  const svg = mermaidSvg(d)
  expect(svg).toBe(mermaidSvg(d))
  expect(svg).toContain('a &lt; b')
  expect(svg).toContain(THEME.accent)
  expect(svg).not.toContain('animation')
  const wide = parseMermaid(`graph LR\n  ${Array.from({ length: 12 }, (_, i) => `N${i}[Ein recht langer Knotentext ${i}]`).join(' --> ')}`)
  if (wide === null) throw new Error('parse')
  expect(Number(/width="(\d+)"/.exec(mermaidSvg(wide))?.[1])).toBeLessThan(721)
  const seq = parseMermaid('sequenceDiagram\n  A->>B: hallo')
  if (seq === null) throw new Error('parse')
  expect(mermaidSvg(seq)).toContain('hallo')
})

// ── Formeln ──

test('Formeln: LaTeX zu Unicode; Unbekanntes ergibt null', () => {
  expect(texToText('E = mc^2')).toBe('E = mc²')
  expect(texToText('\\frac{a+b}{2} \\leq \\sqrt{x_1}')).toBe('(a+b)/2 ≤ √(x₁)')
  expect(texToText('\\sum_{i=1}^{n} \\alpha_i')).toBe('∑ᵢ₌₁ⁿ αᵢ')
  expect(texToText('x^{ab}')).toBe('x^(ab)')
  // Formel aus assets/skins-demo.*.md
  expect(texToText('\\mu = \\frac{1}{n} \\sum_{i=1}^{n} x_i \\leq \\sqrt{x_1^2 + x_2^2}')).toBe('μ = 1/n ∑ᵢ₌₁ⁿ xᵢ ≤ √(x₁² + x₂²)')
  expect(parseMermaid('graph TD\n  A[Prompt] --> B{Freigabe?}\n  B -->|ja| C([Tool läuft])\n  B -.->|nein| D((Ende))')?.type).toBe('flowchart')
  expect(texToText('a \\\\ b')).toBe('a\nb')
  expect(texToText('\\unbekannt{x}')).toBe(null)
  expect(mathSvg('a\nb')).toBe(mathSvg('a\nb'))
  expect(mathSvg('a < b')).toContain('a &lt; b')
})

test('Zerlegen: Mermaid- und Formelkarten nur mit Schalter; offene $$ und unbekannte Diagramme bleiben Markdown', () => {
  const text = ['Text', '', `${F}mermaid`, 'graph TD', 'A-->B', F, '', '$$', 'E = mc^2', '$$', '', '$$x$$', '', `${F}mermaid`, 'pie', F].join('\n')
  const segs = splitMessage(text, ALL)
  expect(segs.map(s => s.kind)).toEqual(['md', 'mermaid', 'math', 'math', 'md'])
  expect(segs[2]).toEqual({ kind: 'math', tex: 'E = mc^2', text: 'E = mc²' })
  expect(segs[3]).toMatchObject({ tex: 'x' })
  expect(splitMessage(text, { cards: true, inline: false }).map(s => s.kind)).toEqual(['md'])
  expect(splitMessage('a\n\n$$\nE = mc^2', ALL).map(s => s.kind)).toEqual(['md'])
  expect(splitMessage('Inline $x$ bleibt', ALL).map(s => s.kind)).toEqual(['md'])
})

test('Rückfrage-Kopf: Anzahl und Kurztitel', () => {
  expect(askSummary([{ header: 'Auth' }, { header: 'DB' }], 'de')).toBe('2 Rückfragen von Claude · Auth, DB')
  expect(askSummary([{ question: 'x' }], 'en')).toBe('Question from Claude')
})

// ── Zeichnung auf dem Desktop ──

const D = { plugin: 'orgamax-cockpit', surface: 'desktop' } as const

test('AssistantMessage: Mermaid- und Formelkarte als einfaches Svg mit Kopf und Kopier-Knopf', async ($, on) => {
  on('ui.render', { component: 'AssistantMessage' }, () => ({ type: 'Box' as const }))
  const text = `${F}mermaid\ngraph TD\nA-->B\n${F}\n\n$$\n\\alpha^2\n$$`
  const m = await $.ui.mount({ ...D, component: 'AssistantMessage', props: { text, isFirstOfReply: true } })
  expect(await m.find({ type: 'Text', text: 'Mermaid · flowchart' })).not.toBe(undefined)
  expect(await m.find({ type: 'Text', text: 'Formel' })).not.toBe(undefined)
  const svgs = JSON.stringify(await m.findAll({ type: 'Svg' }))
  expect(svgs).toContain('α²')
  expect(svgs).not.toContain('isInteractive')
  expect(JSON.stringify(await m.findAll({ type: 'Button' }))).toContain('Quelle kopieren')
  const term = await $.ui.mount({ ...D, surface: 'terminal', component: 'AssistantMessage', props: { text, isFirstOfReply: true } })
  expect(await term.find({ type: 'Svg' })).toBe(undefined)
})

test('CommandOutput: Mermaid-Karte', async ($, on) => {
  on('ui.render', { component: 'CommandOutput' }, () => ({ type: 'Box' as const }))
  const out = await $.ui.mount({ ...D, component: 'CommandOutput', props: { command: 'x', args: '', text: `${F}mermaid\nsequenceDiagram\nA->>B: hi\n${F}`, isErrored: false } })
  expect(await out.find({ type: 'Text', text: 'Mermaid · sequenceDiagram' })).not.toBe(undefined)
})

test('AskUserQuestion: Kopfband über dem unveränderten Dialog', async ($, on) => {
  // Die Engine verlangt genau einen eigenen Knoten (den Dialog) im Baum. Ein Test-Stub ist keiner, deshalb lehnt der Test-Host
  // den Baum ab; die Meldung beweist, dass der Hook auf dem Desktop einen eigenen Baum um `next(e)` zeichnet.
  on('ui.render', { component: 'AskUserQuestion' }, () => ({ type: 'Box' as const }))
  let err = ''
  try {
    await $.ui.mount({ ...D, component: 'AskUserQuestion', props: { tool: 'AskUserQuestion', questions: [{ header: 'Auth' }] } })
  } catch (x) {
    err = String(x)
  }
  expect(err).toContain('AskUserQuestion is drawn by exactly one')
})

test('SessionMode: Labels in Akzentfarbe; leer bleibt die Engine', async ($, on) => {
  on('ui.render', { component: 'SessionMode' }, () => ({ type: 'Box' as const }))
  const m = await $.ui.mount({ ...D, component: 'SessionMode', props: { modes: ['focus', 'memory paused'] } })
  expect(JSON.stringify(await m.find({ type: 'Text', text: 'focus & memory paused' }))).toContain(THEME.accent)
  const empty = await $.ui.mount({ ...D, component: 'SessionMode', props: { modes: [] } })
  expect(await empty.find({ type: 'Text' })).toBe(undefined)
})

test('PromptHint: Kennzeichen vor der Zeile der Engine', async ($, on) => {
  on('ui.render', { component: 'PromptHint' }, () => ({ type: 'Code' as const, props: { source: 'esc to interrupt' } }) as never)
  const h = await $.ui.mount({ ...D, component: 'PromptHint', props: { isDraft: false, isWorking: true, hint: 'esc to interrupt' } })
  expect(await h.find({ type: 'Text', text: 'Hinweiszeile' })).not.toBe(undefined)
  expect(JSON.stringify(await h.find({ type: 'Code' }))).toContain('esc to interrupt')
})
