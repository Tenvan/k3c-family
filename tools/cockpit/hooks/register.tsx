import { atom, read, update } from 'claude-code'
import type { EngineInterface, Register, RenderChildren, RenderElement, SessionUsage } from 'claude-code'

import type { BgTask, About, ModelInfo, Agent, Breakdown, CallMark, CatchUp, Decision, LoopAlert, McpSchema, UiLang, Repo, Gate, LogEntry, LogSrc, ModSettings, Step, Turn, TurnRecord, Usage } from '../types'
import { THEME, splitTool, toolText, withClaude, parseTaskNotification, legendSvg, modelLabel, SPIN_MODE, spinnerText, timelineSvg, demoTable, agentStats, agentFlags, breakdownBarSvg, contextTip, limitTip, cacheTip, summaryLines, topRows, resetLabel, CATCH0, CATCH_KEEP, DEFAULT_SETTINGS, SETTING_DEFS, settingLabel, settingHint, catchUpPrompt, catchUpSystem, nextChunk, settingsOf, startIndex, fingerprint } from './core'
import type { SettingGroup, SettingKey } from './core'
import { CACHE_WARN_MS, WASTE_USD, compactFocus, detectLoop, isAgentHeavy, isWasteful, mcpDiet, mcpKey, turnBarsSvg } from './core'
import { RISKY_GIT_CONFIG, COMPACT_AT, DECISION_COLOR, RING_HEX, contextColor, LEGEND, LEGEND_NOTE, paceText, LIMIT_LABEL, LOG_MAX, SRC_COLOR, bar, cacheHitPct, cacheLeftMs, clockTime, duration, limitColor, pace, parseSteps, pushCapped, markdownStats, shortHash, statsLines, toolInfoLines, oneLine, parseRepo, repoRows, repoShort, ringSvg, svgSource, titleText, toBreakdown, breakdownTitle, categoryColor, spinner, title, toolTarget, toolKind, tokens, usd } from './core'
import { t } from './i18n'
import type { Key } from './i18n'
import { ALERT_COLOR, ICON_COLOR, TONE_COLOR, detailInput, detailOutput, diffCardSvg, diffOf, errorText, firstErrorLine, iconKind, inlineTokens, inputPath, isReadOnlyShell, isShell, isStyled, lineCountOf, mdLiteral, QUIET_TOOLS, quietLine, shellCommand, splitMessage, toolIconSvg } from './skins'
import { askSummary, mathSvg } from './skins'
import type { Segment, SplitOptions } from './skins'
import { diagramType, mermaidSvg } from './mermaid'

const PLUGIN = 'orgamax-cockpit'
// Slash-Command des Hauptpanels: kurz statt der Plugin-ID.
const COMMAND = 'cockpit'
// Panels mit Titel in der eingestellten Sprache. Geöffnet wird mit `focus`: der erste Klick in ein Panel ohne Fokus holt erst
// den Fokus, der Host zeichnet dabei neu und tauscht den Button unter dem Zeiger aus; der Klick verpufft. Der Host gewährt den
// Fokus nur bei leerem Eingabefeld mit Tastatur, nicht aus einem Knopf in Band oder Panel heraus.
const panes = (lang: UiLang) => ({
  main: { id: PLUGIN, title: t(lang, 'common.paneMain') },
  summary: { id: 'orgamax-cockpit-summary', title: t(lang, 'common.paneSummary') },
  settings: { id: 'orgamax-cockpit-settings', title: t(lang, 'common.paneSettings') },
})
const EDIT_TOOLS = new Set(['Edit', 'Write', 'NotebookEdit'])
const LIVE = new Set(['pending', 'running', 'waiting'])
const NL = String.fromCharCode(10)

const usage = atom({ plugin: 'orgamax-cockpit', key: 'usage' } as const, null)
const lastTurnAt = atom({ plugin: 'orgamax-cockpit', key: 'lastTurnAt' } as const, null)
const turn = atom({ plugin: 'orgamax-cockpit', key: 'turn' } as const, null)
const agents = atom({ plugin: 'orgamax-cockpit', key: 'agents' } as const, [] as Agent[])
const GATE0: Gate = { allow: 0, you: 0, ask: 0, deny: 0, file: 0, shell: 0, other: 0, trail: [], asked: [] }
const gate = atom({ plugin: 'orgamax-cockpit', key: 'gate' } as const, GATE0)
// $.state überlebt einen Reload: Zähler aus älteren Versionen haben neue Felder nicht und werden hier aufgefüllt.
const gateOf = (g: Partial<Gate>): Gate => ({ ...GATE0, ...g })
const log = atom({ plugin: 'orgamax-cockpit', key: 'log' } as const, [] as LogEntry[])
// Jede Änderung zeichnet Band und Panel neu – hält Cache-Countdown und Laufzeiten aktuell.
const tick = atom({ plugin: 'orgamax-cockpit', key: 'tick' } as const, 0)
const steps = atom({ plugin: 'orgamax-cockpit', key: 'steps' } as const, [] as Step[])
const cacheHit = atom({ plugin: 'orgamax-cockpit', key: 'cacheHit' } as const, null)
const about = atom({ plugin: 'orgamax-cockpit', key: 'about' } as const, null)
const bgTasks = atom({ plugin: 'orgamax-cockpit', key: 'bgTasks' } as const, [] as BgTask[])
const lastPrompt = atom({ plugin: 'orgamax-cockpit', key: 'lastPrompt' } as const, '')
// Claudes eigener Vorschlag nach dem Turn (prompt.suggest, origin suggestion); gilt bis zum nächsten Prompt.
const claudeHint = atom({ plugin: 'orgamax-cockpit', key: 'claudeHint' } as const, null as Step | null)
// Animationsbild des Panels; läuft nur, solange ein Turn läuft.
const frame = atom({ plugin: 'orgamax-cockpit', key: 'frame' } as const, 0)
const breakdown = atom({ plugin: 'orgamax-cockpit', key: 'breakdown' } as const, null)
// Warum es keine Aufteilung gibt (Fehlertext oder Hinweis); leer, solange sie vorliegt.
const breakdownNote = atom({ plugin: 'orgamax-cockpit', key: 'breakdownNote' } as const, '')
const logo = atom({ plugin: 'orgamax-cockpit', key: 'logo' } as const, null)
const modelInfo = atom({ plugin: 'orgamax-cockpit', key: 'modelInfo' } as const, null)
const repo = atom({ plugin: 'orgamax-cockpit', key: 'repo' } as const, null)
const settings = atom({ plugin: 'orgamax-cockpit', key: 'settings' } as const, DEFAULT_SETTINGS)
const catchUp = atom({ plugin: 'orgamax-cockpit', key: 'catchUp' } as const, CATCH0)
const expanded = atom({ plugin: 'orgamax-cockpit', key: 'expanded' } as const, [] as string[])
const touched = atom({ plugin: 'orgamax-cockpit', key: 'touched' } as const, [] as string[])
const quietIds = atom({ plugin: 'orgamax-cockpit', key: 'quietIds' } as const, [] as string[])
const collapsed = atom({ plugin: 'orgamax-cockpit', key: 'collapsed' } as const, [] as string[])
// Aufrufe des Haupt-Agents im laufenden Turn und die daraus erkannte Schleife; beides leert ein neuer Turn.
const marks = atom({ plugin: 'orgamax-cockpit', key: 'marks' } as const, [] as CallMark[])
const loopAlert = atom({ plugin: 'orgamax-cockpit', key: 'loopAlert' } as const, null as LoopAlert | null)
// Geladene MCP-Schemas aus dem Kontext-Breakdown und die Server, deren Tools die Session schon aufgerufen hat (Kontext-Diät).
const mcpTools = atom({ plugin: 'orgamax-cockpit', key: 'mcpTools' } as const, [] as McpSchema[])
const usedMcp = atom({ plugin: 'orgamax-cockpit', key: 'usedMcp' } as const, [] as string[])
const turnLog = atom({ plugin: 'orgamax-cockpit', key: 'turnLog' } as const, [] as TurnRecord[])
// Subagents, deren weitere Tool-Calls das Cockpit ablehnt (Agent-Bremse: die API kann einen Subagent nicht abbrechen).
const braked = atom({ plugin: 'orgamax-cockpit', key: 'braked' } as const, [] as string[])

const STEPS_SYSTEM =
  'Du schlägst die nächsten sinnvollen Schritte in einer Coding-Session vor. ' +
  'Antworte nur mit einem JSON-Array aus genau 5 Objekten {"label": string, "why": string, "prompt": string}, das wichtigste zuerst. ' +
  '"prompt": genau so formuliert, wie der Nutzer es selbst als nächsten Prompt ins Eingabefeld tippen würde: ' +
  'knapp (meist unter 15 Wörtern), konkret, als direkte Aufforderung, ohne Höflichkeitsfloskeln und ohne Erklärung. ' +
  '"label": Kurzfassung dieses Prompts, höchstens 6 Wörter. "why": kurze Begründung, höchstens 10 Wörter. ' +
  'Schlage nur vor, was sich direkt aus dem letzten Austausch ergibt; nichts wiederholen, was schon erledigt ist. ' +
  'Sprache der Unterhaltung verwenden. Gibt es nichts Sinnvolles, antworte mit [].'

type Ui = ReturnType<EngineInterface['ui']['resolve']>

// Farblegende: `hints` zeigt die Erklärung, sonst nur die Kurzbezeichnung.
// Kurze Legende als Text, nur wo die Oberfläche kein Svg zeichnet (sonst `legendSvg` mit Tooltip).
function legend({ Box, Text }: Ui, lang: UiLang) {
  return (
    <Box flexDirection="row" columnGap={2}>
      {LEGEND.map(l => (
        <Text><Text color={THEME[l.color]}>■</Text> <Text dimColor>{t(lang, l.label)}</Text></Text>
      ))}
    </Box>
  )
}

// Farbige Kästchen im Tooltip: ein <title> ist reiner Text, Emoji-Quadrate zeigen trotzdem die Farbe der Legende.
const TONE_EMOJI = { success: '🟩', warning: '🟨', error: '🟥', accent: '🟦' } as const

// Ganze Legende als Klartext für den Tooltip der kurzen Legende im Band.
const legendTip = (lang: UiLang): string =>
  [...LEGEND.map(l => `${TONE_EMOJI[l.color]} ${t(lang, l.label)} – ${t(lang, l.hint)}`), '', ...LEGEND_NOTE.map(n => t(lang, n))].join(NL)

const clip = (text: string, max: number) => (text.length > max ? `${text.slice(0, max)}…` : text)

function toUsage(u: Pick<SessionUsage, 'context' | 'rateLimits' | 'cost'>): Usage {
  return {
    contextTokens: u.context.tokens,
    contextWindow: u.context.window,
    contextPercent: u.context.percent,
    limits: u.rateLimits.map(({ kind, percentUsed, resetsAt }) => ({ kind, percentUsed, resetsAt })),
    usd: u.cost?.usd,
  }
}

const langOf = async ($: EngineInterface): Promise<UiLang> => (await read($, settings)).lang

// Schreibt nur, wenn sich der Wert ändert: jedes Neuzeichnen tauscht die Buttons in Band und Panel aus, ein Klick in dem
// Moment geht verloren. Messwerte (session.measure, Breakdown) kommen oft unverändert wieder.
// Lesen und Schreiben stehen am Aufrufort, weil der Modul-Scan nur Atome als Literal-Referenz akzeptiert.
const same = (a: unknown, b: unknown): boolean => JSON.stringify(a) === JSON.stringify(b)

// ponytail: Log-Einträge speichern fertigen Text in der Sprache beim Schreiben; nach einem Sprachwechsel bleiben alte Einträge so, bis sie aus dem Log fallen
async function note($: EngineInterface, src: LogSrc, text: string, tool?: { tool: string; target: string; isError: boolean }) {
  const at = await $.clock.now()
  await update($, log, list => pushCapped(list, { at, src, text, ...tool }, LOG_MAX))
}

// Tool-Name farbig: Server gedimmt, Tool in Akzentfarbe; Ziel gedimmt hinter „→“ (nur, wenn es eins gibt), Fehler rot.
function toolLabel({ Text }: Ui, name: string, target = '', isError = false) {
  const p = splitTool(name)
  return (
    <Text>
      {p.server !== null && <Text color={THEME.textMuted}>{p.server} › </Text>}
      <Text bold color={THEME.accent}>{p.tool}</Text>
      {target !== '' && <Text color={THEME.textSecondary}> → {target}</Text>}
      {isError && <Text color={THEME.error}> ✗</Text>}
    </Text>
  )
}

// Log-Eintrag: strukturierte Tool-Einträge farbig, alles andere als Text.
function entryView(ui: Ui, l: LogEntry, dim: boolean) {
  const { Text } = ui
  return l.tool !== undefined ? toolLabel(ui, l.tool, l.target ?? '', l.isError === true) : <Text dimColor={dim}>{l.text}</Text>
}

// ponytail: kein Build-Schritt – „Build“ ist die jüngste Änderung der Hook-Dateien; echter Stempel erst mit Build-Pipeline
async function loadAbout($: EngineInterface): Promise<About | null> {
  try {
    const manifest = JSON.parse(await $.fs.read(`${$.plugin.root}/.claude-plugin/plugin.json`)) as { version?: string }
    const stats = await Promise.all(['register.tsx', 'core.ts'].map(f => $.fs.stat(`${$.plugin.root}/hooks/${f}`)))
    return { version: manifest.version ?? '?', builtAt: Math.max(...stats.map(s => s.mtimeMs)) }
  } catch {
    return null
  }
}

// Gleicht laufende Agents mit der Engine ab – fängt Hintergrund-Agents ohne eigenes turn.complete.
async function syncAgents($: EngineInterface) {
  if (!(await read($, agents)).some(a => LIVE.has(a.status))) return
  const status = new Map((await $.agent.list()).map(a => [a.id, a.status]))
  await update($, agents, list => list.map(a => (LIVE.has(a.status) && status.has(a.id) ? { ...a, status: status.get(a.id) ?? a.status } : a)))
}

// ponytail: ein Haiku-Call pro Antwort, kein Cache – abschaltbar über die Option nextSteps
async function suggest($: EngineInterface, asked: string, answer: string): Promise<Step[]> {
  const r = await $.model.complete({
    model: 'haiku',
    system: STEPS_SYSTEM,
    prompt: `Nutzer fragte:\n${clip(asked, 3000)}\n\nAssistent antwortete:\n${clip(answer, 6000)}\n\nNächste Schritte (JSON-Array):`,
    maxTokens: 1000,
    effort: 'low',
    timeoutMs: 15_000,
  })
  return r.isAnswered ? parseSteps(r.text) : []
}

// Claudes Vorschlag vor die Haiku-Vorschläge setzen; egal, welcher von beiden zuerst eintrifft.
async function mergeHint($: EngineInterface) {
  const hint = await read($, claudeHint)
  await update($, steps, list => withClaude(list, hint))
}

let spin: { cancel(): void } | undefined
let lastBreakdownAt = 0
// Für welchen Turn der Cache-Wächter schon einen Vorschlag gemacht hat: je Turn höchstens einmal.
let warnedFor: number | null = null

// Logo aus assets/logo.svg: Bildmarke aus ErpApi/design-system/brand/logo-symbol-dark.svg (Korpus weiß, Akzente #82b1ff).
async function loadLogo($: EngineInterface): Promise<string | null> {
  try {
    return svgSource(await $.fs.read(`${$.plugin.root}/assets/logo.svg`))
  } catch {
    return null
  }
}

// Ein git-Aufruf, rein lesend: fsmonitor aus, kein Neuschreiben von Index oder Cache; bei Fehler ''.
async function gitText($: EngineInterface, args: readonly string[]): Promise<string> {
  const r = await $.process.run(['git', '-c', 'core.fsmonitor=false', '--no-optional-locks', ...args])
  return r.exitCode === 0 ? r.stdout.trim() : ''
}

// Enthält die lokale Konfiguration Schlüssel, über die das Repo beim Lesen Befehle starten kann?
// Nur Exit 1 („kein Treffer“) gilt als unbedenklich; ein Treffer oder jeder Fehler (kaputte Konfiguration) als riskant.
async function isRiskyRepo($: EngineInterface): Promise<boolean> {
  const r = await $.process.run(['git', '-c', 'core.fsmonitor=false', '--no-optional-locks', 'config', '--local', '--includes', '--name-only', '--get-regexp', RISKY_GIT_CONFIG])
  return r.exitCode !== 1
}

// Git-Stand des Arbeitsverzeichnisses; rein lesend, ohne Netzwerk. Außerhalb eines Repos null.
// Ein fremdes Repo kann über seine Konfiguration Befehle starten (Filter, fsmonitor, textconv). Nur `git status`
// kann das hier auslösen; es läuft deshalb nur, wenn die lokale Konfiguration keinen solchen Schlüssel enthält.
async function loadRepo($: EngineInterface): Promise<Repo | null> {
  const git = (...args: string[]) => gitText($, args)
  const risky = await isRiskyRepo($)
  const [top, gitDir, commonDir, branch, sha, status, counts, prefix] = await Promise.all([
    git('rev-parse', '--show-toplevel'),
    git('rev-parse', '--absolute-git-dir'),
    git('rev-parse', '--path-format=absolute', '--git-common-dir'),
    git('branch', '--show-current'),
    git('rev-parse', '--short', 'HEAD'),
    risky ? Promise.resolve(null) : git('status', '--porcelain', '--ignore-submodules=all'),
    git('rev-list', '--left-right', '--count', '@{upstream}...HEAD'),
    git('rev-parse', '--show-prefix'),
  ])
  return top === '' ? null : parseRepo({ top, gitDir, commonDir, branch, sha, status, counts, prefix })
}

// Info-Zeile unter einem gezeichneten Chat-Element: orgaMAX-Bildmarke und die Angaben als gedimmter Text, immer sichtbar.
// Ohne Logo oder Svg steht „ⓘ“ davor. Umhüllt nur, was schon gezeichnet ist; das Element selbst bleibt unberührt.
function withInfo(ui: Ui, key: string, lines: readonly string[], theirs: RenderElement, mark: string | null): RenderElement {
  const { Box, Text } = ui
  const Svg = 'Svg' in ui ? ui.Svg : undefined
  return (
    <Box key={key} flexDirection="column">
      {theirs}
      <Box flexDirection="row" columnGap={1} alignItems="center">
        {mark !== null && Svg ? <Svg source={mark} alt="orgaMAX" width={11} height={14} /> : <Text color={THEME.textMuted}>ⓘ</Text>}
        <Text color={THEME.textMuted} wrap="truncate-end">{lines.join(' · ')}</Text>
      </Box>
    </Box>
  )
}

// ── Chat-Skins (Desktop): Zeichnung zu den reinen Funktionen in skins.ts ──
// Alle Svg hier sind einfache Bilder (nie interaktiv) mit stabiler Quelle; keine absoluten Boxen; Buttons nur mit Label.

function toolIcon(ui: Ui, tool: string, errored: boolean, key: string) {
  const Svg = 'Svg' in ui ? ui.Svg : undefined
  if (!Svg) return null
  const kind = iconKind(tool)
  return <Svg key={key} source={toolIconSvg(kind, errored ? THEME.error : ICON_COLOR[kind])} alt={tool} width={16} height={16} />
}

// Kennzeichen „hier greift ein Cockpit-Style“: orgaMAX-Bildmarke und/oder kurzer Style-Name, beides abschaltbar.
type Marker = { mark: string | null; label: boolean; lang: UiLang }
type StyleKey = Extract<Key, `style.${string}`>

async function markerOf($: EngineInterface): Promise<Marker> {
  const s = await read($, settings)
  return { mark: s.skinMarkIcon ? await read($, logo) : null, label: s.skinMarkLabel, lang: s.lang }
}

function skinMark(ui: Ui, mk: Marker, key: string, style: StyleKey) {
  const { Box, Text } = ui
  const Svg = 'Svg' in ui ? ui.Svg : undefined
  const icon = mk.mark !== null && Svg ? <Svg source={mk.mark} alt="orgaMAX" width={9} height={11} /> : null
  if (icon === null && !mk.label) return null
  return (
    <Box key={key} flexDirection="row" columnGap={1} alignItems="center">
      {icon}
      {mk.label && <Text color={THEME.textMuted} italic>{t(mk.lang, style)}</Text>}
    </Box>
  )
}

// Element mit vorangestelltem Kennzeichen in einer Zeile.
function marked(ui: Ui, mk: Marker, key: string, el: RenderElement, style: StyleKey, align: 'center' | 'flex-start' = 'center'): RenderElement {
  const m = skinMark(ui, mk, `${key}-m`, style)
  if (m === null) return el
  const { Box } = ui
  return (
    <Box key={key} flexDirection="row" columnGap={1} alignItems={align}>
      {m}
      {el}
    </Box>
  )
}

// Karte mit Kopfzeile; der Kopier-Knopf steht darunter im Fluss, nie als absolute Box.
function card(ui: Ui, key: string, head: RenderElement, body: RenderElement, copy: (() => void) | null, label: string, color: string, mk: Marker, style: StyleKey) {
  const { Box, Button } = ui
  return (
    <Box key={key} flexDirection="column">
      <Box flexDirection="column" borderStyle="round" borderColor={color} paddingX={1}>
        {marked(ui, mk, `${key}-h`, head, style)}
        {body}
      </Box>
      {copy !== null && (
        <Box flexDirection="row">
          <Button key={`${key}-copy`} label={label} plain dimColor onPress={copy} />
        </Box>
      )}
    </Box>
  )
}

const splitOptions = (s: ModSettings): SplitOptions => ({ cards: s.skinCards, inline: s.skinInline, mermaid: s.skinMermaid, math: s.skinMath })

// Abschnitte einer Antwort oder Command-Ausgabe: Prosa bleibt Markdown (markierbar), Karten und Hervorhebungen eigen.
function segmentsView($: EngineInterface, ui: Ui, segs: readonly Segment[], base: string, lang: UiLang, changed: readonly string[], mk: Marker): RenderElement {
  const { Box, Text, Markdown, Code } = ui
  const Svg = 'Svg' in ui ? ui.Svg : undefined
  const copy = (text: string) => () => void $.ui.copy({ text }).catch(() => undefined)
  const copyLabel = t(lang, 'skin.copy')
  return (
    <Box key={base} flexDirection="column" rowGap={1}>
      {segs.map((s, i) => {
        const k = `${base}-${i}`
        if (s.kind === 'md') return <Markdown key={k} text={s.text} />
        if (s.kind === 'plain')
          return (
            <Text key={k} wrap="wrap">
              {inlineTokens(s.text, changed).map(p => (p.tone === undefined ? p.text : <Text color={TONE_COLOR[p.tone]}>{p.text}</Text>))}
            </Text>
          )
        if (s.kind === 'code') {
          const head = <Text color={THEME.accent} bold>{t(lang, 'skin.codeHead', { lang: s.lang || t(lang, 'skin.code'), n: lineCountOf(s.source) })}</Text>
          const body = s.lang === 'diff' ? <Code source={s.source} format="diff" /> : <Code source={s.source} language={s.lang === '' ? undefined : s.lang} />
          return card(ui, k, head, body, copy(s.source), copyLabel, THEME.border, mk, s.lang === 'diff' ? 'style.diff' : 'style.code')
        }
        if (s.kind === 'table')
          return card(ui, k, <Text color={THEME.accent} bold>{t(lang, 'skin.tableHead', { rows: s.rows, cols: s.cols })}</Text>, <Markdown text={s.source} />, copy(s.source), copyLabel, THEME.border, mk, 'style.table')
        if (s.kind === 'alert')
          return card(ui, k, <Text color={ALERT_COLOR[s.alert]} bold>{t(lang, `skin.alert.${s.alert}`)}</Text>, <Markdown text={s.text} />, null, '', ALERT_COLOR[s.alert], mk, 'style.alert')
        // Diagramm und Formel als einfaches Bild (nie interaktiv, Quelle nur aus dem Text); ohne Svg bleibt das Original.
        if (s.kind === 'mermaid') {
          const type = diagramType(s.source)
          if (!Svg) return <Markdown key={k} text={['```mermaid', s.source, '```'].join(NL)} />
          const body = <Svg key={`${k}-svg`} source={mermaidSvg(s.diagram)} alt={t(lang, 'skin.mermaidAlt', { type })} />
          return card(ui, k, <Text color={THEME.accent} bold>{t(lang, 'skin.mermaidHead', { type })}</Text>, body, copy(s.source), t(lang, 'skin.copySource'), THEME.border, mk, 'style.mermaid')
        }
        if (s.kind === 'math') {
          const ok = s.text !== null && Svg
          const body = ok ? <Svg key={`${k}-svg`} source={mathSvg(s.text ?? '')} alt={s.text ?? ''} /> : <Code source={s.tex} language="latex" />
          return card(ui, k, <Text color={THEME.accent} bold>{t(lang, ok ? 'skin.mathHead' : 'skin.mathRaw')}</Text>, body, copy(s.tex), t(lang, 'skin.copySource'), THEME.border, mk, 'style.math')
        }
        return (
          <Box key={k} flexDirection="column">
            {marked(ui, mk, `${k}-t`, <Text color={s.done === s.total ? THEME.success : THEME.accent} bold>{t(lang, 'skin.tasks', { done: s.done, total: s.total })}</Text>, 'style.tasks')}
            <Markdown text={s.text} />
          </Box>
        )
      })}
    </Box>
  )
}

// Zeile unter dem Titel: Modell, Effort, Kontextfenster, Ausgabestil, Claude-Code-Version; nur, was bekannt ist.
function modelRow({ Text }: Ui, mi: ModelInfo | null, window: number | undefined, lang: UiLang) {
  if (mi === null) return null
  const parts = [
    `${modelLabel(mi.model)}${modelLabel(mi.model) !== mi.model ? ` (${mi.model})` : ''}`,
    mi.effort === undefined ? '' : t(lang, 'model.effort', { v: mi.effort }),
    window === undefined ? '' : t(lang, 'model.context', { v: tokens(window) }),
    mi.style ? t(lang, 'model.style', { v: mi.style }) : '',
    mi.cc ? t(lang, 'model.cc', { v: mi.cc }) : '',
  ].filter(x => x !== '')
  return <Text color={THEME.textSecondary} wrap="truncate-end">{parts.join(' · ')}</Text>
}

// Kopf der Panels: Titel, Modellzeile und eine Leerzeile Abstand zum Inhalt.
async function paneHead($: EngineInterface, ui: Ui, lang: UiLang) {
  const { Box } = ui
  // Liest bewusst nicht `usage`: das ändert sich bei jeder Messung, und jedes Neuzeichnen kostet einen Klick (siehe `same`).
  const mi = await read($, modelInfo)
  return (
    <Box flexDirection="column" marginBottom={1}>
      {titleRow(ui, await read($, about), await read($, logo))}
      {modelRow(ui, mi, mi?.window, lang)}
    </Box>
  )
}

// Titelzeile: Logo links, Text daneben; ohne Logo oder Svg bleibt „orgaMAX Claude …“ als Text.
function titleRow(ui: Ui, info: About | null, mark: string | null) {
  const { Box, Text } = ui
  const Svg = 'Svg' in ui ? ui.Svg : undefined
  if (mark === null || !Svg) return <Text bold color={THEME.accent}>{title(info)}</Text>
  return (
    <Box flexDirection="row" columnGap={1} alignItems="center">
      <Svg source={mark} alt="orgaMAX" width={16} height={20} />
      {/* Wortmarke als Text in den Markenfarben für dunklen Grund: „orga“ weiß, „MAX“ #82b1ff (brand/README.md) */}
      <Text italic>
        <Text color={THEME.orga}>orga</Text>
        <Text bold color={THEME.max}>MAX</Text>
      </Text>
      <Text bold color={THEME.accent}>{titleText(info)}</Text>
    </Box>
  )
}

function repoRow({ Text }: Ui, r: Repo | null) {
  return r === null ? null : <Text dimColor>{repoShort(r)}</Text>
}

// ponytail: höchstens alle 5 s (nach einem Turn immer); 'summary' schätzt lokal und kostet keinen Request
async function fetchBreakdown($: EngineInterface, force: boolean) {
  const now = await $.clock.now()
  if (!force && now - lastBreakdownAt < 5000) return
  lastBreakdownAt = now
  try {
    const b = (await $.session.usage({ breakdown: 'summary' })).context.breakdown
    // ponytail: der Hinweis wird wie das Log beim Schreiben übersetzt; nach einem Sprachwechsel erst beim nächsten Abruf neu
    if (b) {
      {
        const v = toBreakdown(b)
        if (!same(await read($, breakdown), v)) await update($, breakdown, () => v)
      }
      {
        const v = (b.mcpTools ?? []).map(({ name, serverName, tokens, isLoaded }) => ({ name, serverName, tokens, isLoaded }))
        if (!same(await read($, mcpTools), v)) await update($, mcpTools, () => v)
      }
      {
        const v = ''
        if (!same(await read($, breakdownNote), v)) await update($, breakdownNote, () => v)
      }
    } else {
      const lang = await langOf($)
      {
        const v = t(lang, 'panel.contextNone')
        if (!same(await read($, breakdownNote), v)) await update($, breakdownNote, () => v)
      }
    }
  } catch (err) {
    const lang = await langOf($)
    {
      const v = t(lang, 'panel.contextFailed', { err: err instanceof Error ? err.message : String(err) })
      if (!same(await read($, breakdownNote), v)) await update($, breakdownNote, () => v)
    }
  }
}

function stopSpin() {
  spin?.cancel()
  spin = undefined
}

// ponytail: 250 ms Takt nur während eines Turns; zeichnet Panel und Band je Frame neu
function startSpin($: EngineInterface) {
  if (spin) return
  spin = $.clock.every(250, async () => {
    const t = await read($, turn)
    if (!t || t.durationMs !== undefined) return stopSpin()
    await update($, frame, n => n + 1)
  })
}

// Einstellungen: aus dem $.store (gilt für alle Sessions) in den State laden; Standardwerte aus den Plugin-Optionen.
async function loadSettings($: EngineInterface, defaults: Partial<ModSettings>) {
  // Ohne lesbaren Speicher gelten die Standardwerte; der Start des Mods darf daran nicht scheitern.
  const saved = await $.store.get('settings').then(v => v as Partial<ModSettings> | undefined, () => undefined)
  await update($, settings, () => settingsOf({ ...defaults, ...saved }))
}

// Zusammenfassung nach einem Turn anstoßen: nur automatisch eingeschaltet und nur, wo eine Oberfläche das Panel zeigt (nicht in claude -p).
async function maybeCatchUp($: EngineInterface) {
  const s = await read($, settings)
  if (!s.catchUp || !s.cardCatchUp) return
  if ((await $.session.surfaces()).length > 0) await runCatchUp($, true)
}

async function setSettings($: EngineInterface, patch: Partial<ModSettings>) {
  await update($, settings, s => settingsOf({ ...s, ...patch }))
  // Gespeichert werden nur die geänderten Schlüssel; alles andere folgt weiter den Standardwerten und Plugin-Optionen.
  const saved = (await $.store.get('settings').then(v => v, () => undefined)) as Partial<ModSettings> | undefined
  await $.store.set('settings', { ...saved, ...patch })
}

// Zurück auf die Standardwerte; die Sprache bleibt, sie hat eine eigene Karte.
async function resetSettings($: EngineInterface) {
  const lang = await langOf($)
  await update($, settings, () => ({ ...DEFAULT_SETTINGS, lang }))
  await $.store.set('settings', lang === 'de' ? {} : { lang })
}

// Zusammenfassung der aktuellen Session aus dem $.store laden (bleibt über Neustart und /resume erhalten).
async function loadCatchUp($: EngineInterface) {
  const sessionId = await $.session.id()
  const saved = (await $.store.get(`catch:${sessionId}`)) as Partial<CatchUp> | undefined
  await update($, catchUp, () => ({ ...CATCH0, ...saved, sessionId, status: '' }))
}

async function saveCatchUp($: EngineInterface, c: CatchUp) {
  await $.store.set(`catch:${c.sessionId}`, { summary: c.summary, covered: c.covered, marker: c.marker, updatedAt: c.updatedAt })
  // Neueste zuerst; die Zusammenfassungen der ältesten Sessions fallen weg.
  const order = [c.sessionId, ...(((await $.store.get('catch-order')) as string[] | undefined) ?? []).filter(id => id !== c.sessionId)]
  for (const id of order.slice(CATCH_KEEP)) await $.store.delete(`catch:${id}`)
  await $.store.set('catch-order', order.slice(0, CATCH_KEEP))
}

// Git-Zustand für die Zusammenfassung: Status und Diff entfallen bei riskanter Repo-Konfiguration.
async function catchGitState($: EngineInterface): Promise<string> {
  const risky = await isRiskyRepo($)
  const cmds = [...(risky ? [] : [['status', '--short', '--ignore-submodules=all'], ['diff', '--stat', '--ignore-submodules=all']]), ['log', '--oneline', '--no-show-signature', '-n', '5']]
  const out: string[] = []
  for (const args of cmds) out.push(`$ git ${args.join(' ')}\n${(await gitText($, args)) || '(leer)'}`)
  if (risky) out.push('(git status und diff ausgelassen: Repo-Konfiguration enthält Befehle)')
  return out.join('\n\n')
}

let catching = false
// Die angeforderte Aktualisierung, die während einer laufenden kam: null oder deren onlyIfWork
let catchQueued: boolean | null = null

// Faltet die seit dem letzten Stand hinzugekommenen Nachrichten in die Zusammenfassung (rollierend, ein Haiku-Call je Block).
// Mit onlyIfWork nur, wenn darunter ein Tool-Aufruf war.
async function runCatchUp($: EngineInterface, onlyIfWork: boolean) {
  if (catching) {
    catchQueued = (catchQueued ?? true) && onlyIfWork
    return
  }
  catching = true
  let lang: UiLang = 'de'
  try {
    lang = await langOf($)
    const id = await $.session.id()
    if ((await read($, catchUp)).sessionId !== id) await loadCatchUp($)
    const messages = await $.session.messages()
    const c0 = await read($, catchUp)
    let i = startIndex(messages, c0.covered, c0.marker)
    if (i >= messages.length) return
    if (onlyIfWork && !messages.slice(i).some(m => m.toolUses.length > 0)) return

    await update($, catchUp, c => ({ ...c, status: t(lang, 'summary.updating') }))
    const system = catchUpSystem(lang, await $.fs.read(`${$.plugin.root}/assets/catchup.${lang}.md`))
    const git = await catchGitState($)

    while (i < messages.length) {
      const { text, end } = nextChunk(messages, i, lang)
      const previous = (await read($, catchUp)).summary
      const r = await $.model.complete({ model: 'haiku', system, prompt: catchUpPrompt(previous, text, git, lang), maxTokens: 3000 })
      if (!r.isAnswered) throw new Error(`${r.reason}${'error' in r ? ` (${String(r.error)})` : ''}`)
      // Eine leere Antwort würde die bisherige Zusammenfassung löschen: lieber Fehler als Datenverlust.
      if (r.text.trim() === '') throw new Error(t(lang, 'summary.emptyReply'))
      // /clear, /resume oder /branch haben die Session gewechselt, während das Modell lief
      if ((await $.session.id()) !== id) {
        await update($, catchUp, c => ({ ...c, status: '' }))
        return
      }
      const updatedAt = await $.clock.now()
      const next: CatchUp = { sessionId: id, summary: r.text.trim().slice(0, 10_000), covered: end, marker: fingerprint(messages[end - 1] as (typeof messages)[number]), updatedAt, status: t(lang, 'summary.updating') }
      await update($, catchUp, () => next)
      await saveCatchUp($, next)
      i = end
    }
    await update($, catchUp, c => ({ ...c, status: '' }))
  } catch (err) {
    await update($, catchUp, c => ({ ...c, status: t(lang, 'summary.failed', { err: err instanceof Error ? err.message : String(err) }) }))
  } finally {
    catching = false
    if (catchQueued !== null) {
      const again = catchQueued
      catchQueued = null
      void runCatchUp($, again).catch(() => undefined)
    }
  }
}

async function rebuildCatchUp($: EngineInterface) {
  if (catching) return
  const sessionId = await $.session.id()
  await update($, catchUp, () => ({ ...CATCH0, sessionId }))
  await runCatchUp($, false)
}

// Eine offene Rückfrage ist entschieden, sobald ihr tool.call zurückkommt: ein deny heißt abgelehnt, sonst freigegeben.
// ponytail: markiert in der Kachelzeile die älteste offene Rückfrage, nicht genau diese (Dialoge kommen nacheinander).
// ponytail: modulweit, beginnt nach einem Reload leer; nur gegen Mehrfachprüfungen desselben Aufrufs.
const checked = new Set<string>()

async function settleAsk($: EngineInterface, id: string, isDenied: boolean) {
  await update($, gate, old => {
    const g = gateOf(old)
    if (!g.asked.includes(id)) return old
    const at = g.trail.indexOf('ask')
    const mark: Decision = isDenied ? 'deny' : 'you'
    return {
      ...g,
      ask: g.asked.length - 1,
      [mark]: g[mark] + 1,
      asked: g.asked.filter(x => x !== id),
      trail: at < 0 ? g.trail : g.trail.map((d, i) => (i === at ? mark : d)),
    }
  })
}

// Shell-Aufruf, der im Hintergrund weiterläuft (run_in_background oder Strg+B): als laufende Aufgabe merken.
// TaskStop beendet sie sofort als „killed“.
async function trackBackground($: EngineInterface, e: { tool: string; [k: string]: unknown }, ran: unknown) {
  const now = await $.clock.now()
  const result = (ran as { result?: { backgroundTaskId?: unknown } }).result
  const id = typeof result?.backgroundTaskId === 'string' ? result.backgroundTaskId : ''
  if (id !== '' && isShell(e.tool)) {
    const command = shellCommand(e)
    const description = typeof e.description === 'string' && e.description.trim() !== '' ? e.description.trim() : oneLine(command, 80)
    await update($, bgTasks, list => pushCapped(list.filter(x => x.id !== id), { id, description, command, status: 'running', since: now }, 20))
  }
  const stop = e.tool === 'TaskStop' || e.tool === 'KillShell' ? (typeof e.task_id === 'string' ? e.task_id : typeof e.shell_id === 'string' ? e.shell_id : '') : ''
  if (stop !== '') await update($, bgTasks, list => list.map(x => (x.id === stop && x.status === 'running' ? { ...x, status: 'killed', durationMs: now - x.since } : x)))
}

// Benachrichtigung der Engine über das Ende einer Hintergrundaufgabe: Status, Dauer und Kurztext übernehmen.
async function finishBackground($: EngineInterface, text: string) {
  const n = parseTaskNotification(text)
  if (n === null) return
  const now = await $.clock.now()
  await update($, bgTasks, list => list.map(x => (x.id === n.id ? { ...x, status: n.status, durationMs: now - x.since, summary: n.summary } : x)))
}

const featuresText = async ($: EngineInterface) => $.fs.read(`${$.plugin.root}/assets/features.${await langOf($)}.md`)

// Cache-Wächter: kurz vor Ablauf, im Leerlauf und einmal je Turn, einen kurzen Prompt vorschlagen, der den Cache warm hält.
async function watchCache($: EngineInterface) {
  const s = await read($, settings)
  const lastAt = await read($, lastTurnAt)
  const u = await read($, usage)
  const tn = await read($, turn)
  if (!s.cache || lastAt === null || u === null || warnedFor === lastAt || (tn !== null && tn.durationMs === undefined)) return
  const left = cacheLeftMs(lastAt, u.limits.length > 0, await $.clock.now())
  if (left <= 0 || left > CACHE_WARN_MS) return
  warnedFor = lastAt
  await $.prompt.suggest({ text: t(s.lang, 'cache.keepWarm') })
}

// Neuer Turn: Zähler zurücksetzen, Vorschläge leeren, Prompt ins Log. Aus prompt.submit und aus den Folge-Schritt-Knöpfen.
async function startTurn($: EngineInterface, text: string) {
  const startedAt = await $.clock.now()
  const usdStart = (await read($, usage))?.usd
  await update($, turn, (): Turn => ({ startedAt, tools: 0, edits: 0, errors: 0, usdStart }))
  await update($, lastPrompt, () => text)
  await update($, touched, () => [])
  await update($, marks, () => [])
  await update($, loopAlert, () => null)
  if ((await read($, settings)).animation) startSpin($)
  await update($, steps, () => [])
  await update($, claudeHint, () => null)
  await note($, 'you', oneLine(text))
}

export const register: Register = (on, options) => {

  on('session.start', async ($, e, next) => {
    const info = await loadAbout($)
    await update($, about, () => info)
    await loadSettings($, { steps: options.nextSteps !== false, chatInfo: options.chatInfo !== false })
    void loadCatchUp($).catch(() => undefined)
    await update($, logo, () => null)
    void Promise.all([$.session.model(), $.session.version()])
      .then(([model, v]) => update($, modelInfo, mi => ({ ...mi, model, cc: v.version })))
      .catch(() => undefined)
    void Promise.all([loadLogo($).then(l => update($, logo, () => l)), loadRepo($).then(async r => (same(await read($, repo), r) ? undefined : update($, repo, () => r)))]).catch(() => undefined)
    // ponytail: Beschreibungen in der Sprache beim Session-Start; nach einem Sprachwechsel erst mit dem nächsten Start/Reload
    const lang = await langOf($)
    await $.command.register({ name: COMMAND, description: t(lang, 'common.cmdPanel') })
    await $.command.register({ name: 'cockpit-zusammenfassung', description: t(lang, 'common.cmdSummary') })
    await $.command.register({ name: 'cockpit-einstellungen', description: t(lang, 'common.cmdSettings') })
    await $.command.register({ name: 'cockpit-demo', description: t(lang, 'common.cmdDemo') })
    await $.command.register({ name: 'cockpit-features', description: t(lang, 'common.cmdFeatures') })
    void $.clock.every(30_000, () => void Promise.all([update($, tick, n => n + 1), syncAgents($), watchCache($)]).catch(() => undefined))
    return next(e)
  })

  // Hover-Infos über dem Chat: nur Desktop, abschaltbar in den Einstellungen.
  // Antworten: Code-/Tabellenkarten (skinCards) und Hervorhebungen (skinInline) zeichnen selbst, aber nur wenn es etwas zu
  // stylen gibt; Prosa bleibt das native Markdown. Zusammenfassungs-Blöcke bleiben unverändert.
  on('ui.render', { component: 'AssistantMessage' }, async ($, e, next) => {
    const theirs = await next(e)
    if (e.surface !== 'desktop') return theirs
    const s = await read($, settings)
    const ui = $.ui.resolve(e)
    let el = theirs
    const so = splitOptions(s)
    if (Object.values(so).some(Boolean) && e.props.isSummary !== true) {
      const segs = splitMessage(e.props.text, so)
      if (isStyled(segs)) el = segmentsView($, ui, segs, `om-s-${shortHash(e.props.text)}`, s.lang, s.skinInline ? await read($, touched) : [], await markerOf($))
    }
    const lines = s.chatInfo ? statsLines(markdownStats(e.props.text), s.lang) : []
    return lines.length === 0 ? el : withInfo(ui, `om-m-${shortHash(e.props.text)}`, lines, el, await read($, logo))
  })

  // Deine Nachricht rechtsbündig im Rahmen; der Text als wörtliches Markdown, damit er markierbar bleibt.
  // Nur getippte Prompts: Benachrichtigungen und Nachrichten anderer Agents zeichnet weiter die Engine.
  on('ui.render', { component: 'UserMessage' }, async ($, e, next) => {
    if (e.surface !== 'desktop') return next(e)
    const s = await read($, settings)
    const p = e.props
    const md = mdLiteral(p.text)
    if (!s.skinPrompt || p.origin.kind !== 'composer' || p.from !== undefined || p.task !== undefined || p.text.trim() === '' || md.length > 100_000) return next(e)
    const { Box, Markdown } = $.ui.resolve(e)
    return (
      <Box key={`om-p-${shortHash(p.text)}`} flexDirection="column" alignItems="flex-end">
        <Box borderStyle="round" borderColor={THEME.accent} paddingX={1} flexShrink={1} flexDirection="row" columnGap={1} alignItems="flex-start">
          {skinMark($.ui.resolve(e), await markerOf($), `om-pm-${shortHash(p.text)}`, 'style.prompt')}
          <Markdown text={md} />
        </Box>
      </Box>
    )
  })

  // Tool-Zeile: Symbol davor (skinIcons), darunter ▸/▾ Details mit Eingabe und Ergebnis als Code (skinExpand), dann die Info-Zeile.
  on('ui.render', { component: 'ToolUse' }, async ($, e, next) => {
    const theirs = await next(e)
    if (e.surface !== 'desktop') return theirs
    const s = await read($, settings)
    const ui = $.ui.resolve(e)
    const { Box, Text, Button, Code } = ui
    const p = e.props
    const id = p.tool_use_id
    let el = theirs
    const icon = s.skinIcons ? toolIcon(ui, p.tool, p.isErrored, `om-i-${id}`) : null
    if (icon !== null)
      el = (
        <Box key={`om-ir-${id}`} flexDirection="row" columnGap={1} alignItems="flex-start">
          {skinMark(ui, await markerOf($), `om-im-${id}`, 'style.icon')}
          {icon}
          {theirs}
        </Box>
      )
    if (s.skinExpand && !p.isRunning) {
      const open = (await read($, expanded)).includes(id)
      const toggle = () => void update($, expanded, ids => (ids.includes(id) ? ids.filter(x => x !== id) : pushCapped(ids, id, 50))).catch(() => undefined)
      const input = detailInput(p.tool, p.input)
      const output = open ? detailOutput(p.output) : null
      el = (
        <Box key={`om-x-${id}`} flexDirection="column">
          {el}
          <Box flexDirection="row" columnGap={1} alignItems="center">
            {skinMark(ui, await markerOf($), `om-xm-${id}`, 'style.expand')}
            <Button key={`om-xb-${id}`} label={t(s.lang, open ? 'skin.detailsOpen' : 'skin.details')} plain dimColor onPress={toggle} />
          </Box>
          {open && (
            <Box flexDirection="column" borderStyle="round" borderColor={THEME.border} paddingX={1}>
              <Text color={THEME.textMuted}>{t(s.lang, 'skin.input')}</Text>
              <Code source={input.source} language={input.language} />
              {output !== null && <Text color={THEME.textMuted}>{t(s.lang, 'skin.output')}</Text>}
              {output !== null && <Code source={output.source} language={output.language} path={output.path} />}
            </Box>
          )}
        </Box>
      )
    }
    if (!s.chatInfo) return el
    const status = t(s.lang, p.isErrored ? 'chat.statusError' : p.isRunning ? 'chat.statusRunning' : 'chat.statusDone')
    return withInfo(ui, `om-u-${id}`, toolInfoLines(p.tool, p.input, status, s.lang), el, await read($, logo))
  })

  // Gruppe zusammengefasster Aufrufe: je Werkzeugart ein Symbol vor der Zählzeile (nur zugeklappt).
  const GROUP_CALLS_MAX = 8
  on('ui.render', { component: 'ToolGroup' }, async ($, e, next) => {
    const theirs = await next(e)
    if (e.surface !== 'desktop' || e.props.isExpanded || !(await read($, settings)).skinIcons) return theirs
    const ui = $.ui.resolve(e)
    const { Box, Text } = ui
    const calls = e.props.calls
    // Je Tool (MCP: je Server) Symbol, Name und Zähler: „[icon] Bash ×3“.
    const nameOf = (tool: string) => splitTool(tool).server ?? tool
    const byName = new Map<string, (typeof calls)[number][]>()
    for (const c of calls) byName.set(nameOf(c.tool), [...(byName.get(nameOf(c.tool)) ?? []), c])
    const groups = [...byName].slice(0, 6)
    const base = calls[0]?.tool_use_id ?? shortHash(calls.map(c => c.tool).join(','))
    const icons = groups.flatMap(([name, cs]) => {
      const icon = toolIcon(ui, cs[0]!.tool, cs.some(c => c.isErrored), `om-gi-${base}-${name}`)
      return icon === null ? [] : [icon, <Text key={`om-gn-${base}-${name}`} dimColor>{name} ×{cs.length}</Text>]
    })
    if (icons.length === 0) return theirs
    // Nur MCP-Aufrufe einzeln mit Server › Tool → Ziel; der Rest bleibt in der Zählzeile. Lange Listen gekürzt.
    const mcp = calls.filter(c => splitTool(c.tool).server !== null)
    const shown = mcp.slice(0, GROUP_CALLS_MAX)
    return (
      <Box key={`om-gc-${base}`} flexDirection="column">
        <Box key={`om-g-${base}`} flexDirection="row" columnGap={1} alignItems="center">
          {skinMark(ui, await markerOf($), `om-gm-${base}`, 'style.group')}
          {icons}
          {theirs}
        </Box>
        {shown.map((c, i) => (
          <Text key={`om-gl-${base}-${i}`} wrap="truncate-end" dimColor={!c.isRunning}>
            {'  '}{toolLabel(ui, c.tool, toolTarget(c.input), c.isErrored)}
          </Text>
        ))}
        {mcp.length > shown.length && <Text key={`om-gx-${base}`} dimColor>{'  '}+{mcp.length - shown.length}</Text>}
      </Box>
    )
  })

  // Ergebnis: Diff-Karte für Edit/Write (skinDiff); Fehler als erste echte Fehlerzeile und lesende Aufrufe als eine Zeile (skinQuiet).
  on('ui.render', { component: 'ToolResult' }, async ($, e, next) => {
    const theirs = await next(e)
    if (e.surface !== 'desktop') return theirs
    const s = await read($, settings)
    const ui = $.ui.resolve(e)
    const { Text } = ui
    const Svg = 'Svg' in ui ? ui.Svg : undefined
    const p = e.props
    const id = p.tool_use_id
    let el = theirs
    const diff = s.skinDiff && !p.isErrored && Svg ? diffOf(p.output) : null
    if (diff !== null && Svg) {
      el = marked(ui, await markerOf($), `om-dm-${id}`, <Svg key={`om-d-${id}`} source={diffCardSvg(diff, s.lang)} alt={t(s.lang, 'skin.diffAlt', { file: diff.file, add: diff.added, del: diff.removed })} />, 'style.diff', 'flex-start')
    } else if (s.skinQuiet && p.isErrored) {
      el = marked(ui, await markerOf($), `om-qm-${id}`, <Text key={`om-q-${id}`} color={THEME.error} wrap="truncate-end">✖ {firstErrorLine(errorText(p.output))}</Text>, 'style.error')
    } else if (s.skinQuiet && (QUIET_TOOLS.has(p.tool) || (isShell(p.tool) && (await read($, quietIds)).includes(id)))) {
      const line = quietLine(p.tool, p.output, s.lang)
      if (line !== null) el = marked(ui, await markerOf($), `om-qm-${id}`, <Text key={`om-q-${id}`} color={THEME.textMuted} wrap="truncate-end">✓ {line}</Text>, 'style.quiet')
    }
    if (!s.chatInfo) return el
    const size = typeof p.output === 'string' ? t(s.lang, 'chat.outputLines', { n: p.output.split('\n').length }) : ''
    const lines = [t(s.lang, 'chat.result', { tool: p.tool, state: t(s.lang, p.isErrored ? 'chat.withError' : 'chat.ok') }), size].filter(l => l !== '')
    return withInfo(ui, `om-r-${id}`, lines, el, await read($, logo))
  })

  // Arbeitszeile: Phase in der Farbrolle des Design-Systems, Schritt, Laufzeit und Zähler des Turns. Ersetzt die Zeile der
  // Engine ganz (deren Token- und Effort-Angabe entfällt); der 250-ms-Takt von startSpin zeichnet sie neu.
  on('ui.render', { component: 'Spinner' }, async ($, e, next) => {
    const s = await read($, settings)
    if (!s.chatLines) return next(e)
    const tn = await read($, turn)
    const now = await $.clock.now()
    const { Box, Text } = $.ui.resolve(e)
    const mode = SPIN_MODE[e.props.mode]
    const step = spinnerText(e.props.word, e.props.message)
    return (
      <Box flexDirection="row" columnGap={1} alignItems="center">
        {skinMark($.ui.resolve(e), await markerOf($), 'om-spin-m', 'style.spinner')}
        <Text color={mode.color} bold>{spinner(await read($, frame))} {t(s.lang, mode.label)}</Text>
        {step !== '' && <Text color={THEME.textSecondary} wrap="truncate-end">{step}</Text>}
        {tn !== null && (
          <Text color={THEME.textMuted}>· {duration(now - tn.startedAt)} · {t(s.lang, 'chat.toolCalls', { n: tn.tools })}{tn.edits > 0 ? ` · ${t(s.lang, 'chat.edits', { n: tn.edits })}` : ''}</Text>
        )}
        {tn !== null && tn.errors > 0 && <Text color={THEME.error}>· {t(s.lang, 'chat.errors', { n: tn.errors })}</Text>}
      </Box>
    )
  })

  // Dauerzeile nach dem Turn: die Engine zeichnet sie nur im Terminal, der Desktop hat einen eigenen Fuß.
  on('ui.render', { component: 'TurnDuration' }, async ($, e, next) => {
    const s = await read($, settings)
    if (!s.chatLines) return next(e)
    const { Text } = $.ui.resolve(e)
    return (
      <Text>
        <Text color={THEME.success}>{t(s.lang, 'chat.done')}</Text>
        <Text color={THEME.textMuted}> {t(s.lang, 'chat.after', { d: duration(e.props.durationMs) })}</Text>
      </Text>
    )
  })

  // Rückfrage: nur ein Kopfband über dem Dialog der Engine; Fragen, Optionen und Eingabe bleiben unberührt (`theirs`).
  on('ui.render', { component: 'AskUserQuestion' }, async ($, e, next) => {
    const theirs = await next(e)
    if (e.surface !== 'desktop') return theirs
    const s = await read($, settings)
    if (!s.skinAsk) return theirs
    const ui = $.ui.resolve(e)
    const { Box, Text } = ui
    const key = `om-ask-${shortHash(JSON.stringify(e.props.questions))}`
    return (
      <Box key={key} flexDirection="column">
        <Box flexDirection="row" columnGap={1} alignItems="center" borderStyle="round" borderColor={THEME.accent} paddingX={1}>
          {skinMark(ui, await markerOf($), `${key}-m`, 'style.ask')}
          <Text color={THEME.accent} bold wrap="truncate-end">{askSummary(e.props.questions, s.lang)}</Text>
        </Box>
        {theirs}
      </Box>
    )
  })

  // Modus-Labels im Footer: dieselben Labels in Akzentfarbe; ohne Labels zeichnet die Engine.
  on('ui.render', { component: 'SessionMode' }, async ($, e, next) => {
    if (e.surface !== 'desktop' || e.props.modes.length === 0) return next(e)
    const s = await read($, settings)
    if (!s.skinMode) return next(e)
    const ui = $.ui.resolve(e)
    const { Box, Text } = ui
    return (
      <Box key="om-mode" flexDirection="row" columnGap={1} alignItems="center">
        {skinMark(ui, await markerOf($), 'om-mode-m', 'style.mode')}
        <Text color={THEME.accent}>{e.props.modes.join(' & ')}</Text>
      </Box>
    )
  })

  // Hinweiszeile unter dem Prompt: nur das Kennzeichen davor; die Zeile (samt Pills) bleibt die der Engine.
  on('ui.render', { component: 'PromptHint' }, async ($, e, next) => {
    const theirs = await next(e)
    if (e.surface !== 'desktop' || !(await read($, settings)).skinHint) return theirs
    const ui = $.ui.resolve(e)
    const m = skinMark(ui, await markerOf($), 'om-hint-m', 'style.hint')
    if (m === null) return theirs
    const { Box } = ui
    return (
      <Box key="om-hint" flexDirection="row" columnGap={1} alignItems="center">
        {m}
        {theirs}
      </Box>
    )
  })

  on('ui.render', { component: 'CommandOutput' }, async ($, e, next) => {
    const theirs = await next(e)
    if (e.surface !== 'desktop') return theirs
    const s = await read($, settings)
    const ui = $.ui.resolve(e)
    let el = theirs
    // Karten und Hervorhebungen auch hier (zeigt sie in /cockpit-demo); Fehlerzeilen zeichnet weiter die Engine.
    const so = splitOptions(s)
    if (Object.values(so).some(Boolean) && !e.props.isErrored) {
      const segs = splitMessage(e.props.text, so)
      if (isStyled(segs)) el = segmentsView($, ui, segs, `om-cs-${shortHash(e.props.command + e.props.text)}`, s.lang, s.skinInline ? await read($, touched) : [], await markerOf($))
    }
    if (!s.chatInfo) return el
    const lines = [
      `/${e.props.command} · ${t(s.lang, e.props.isErrored ? 'chat.withError' : 'chat.ok')}`,
      t(s.lang, 'chat.outputLines', { n: e.props.text.split('\n').length }),
      ...statsLines(markdownStats(e.props.text), s.lang),
    ]
    return withInfo(ui, `om-c-${shortHash(e.props.command + e.props.text)}`, lines, el, await read($, logo))
  })

  // Ausgabe als Markdown im Chat: zeigt jedes skinbare Element und die Hover-Info daran.
  on('command.run', { command: 'cockpit-demo' }, async $ => {
    const s = await read($, settings)
    const md = await $.fs.read(`${$.plugin.root}/assets/skins-demo.${s.lang}.md`)
    return { text: md.replace('<!-- KOMPONENTEN -->', demoTable(s, s.lang)) }
  })

  // Funktionsübersicht in Schlagworten, auch über den Knopf „Features“ im Band.
  on('command.run', { command: 'cockpit-features' }, async $ => ({ text: await featuresText($) }))

  on('command.run', { command: 'cockpit-zusammenfassung' }, async $ => {
    const lang = await langOf($)
    await $.ui.open({ ...panes(lang).summary, focus: true })
    return { text: t(lang, 'common.openedSummary') }
  })

  on('command.run', { command: 'cockpit-einstellungen' }, async $ => {
    const lang = await langOf($)
    await $.ui.open({ ...panes(lang).settings, focus: true })
    return { text: t(lang, 'common.openedSettings') }
  })

  on('command.run', { command: COMMAND }, async $ => {
    const lang = await langOf($)
    await $.ui.open({ ...panes(lang).main, focus: true })
    return { text: t(lang, 'common.openedPanel') }
  })

  on('session.measure', async ($, e, next) => {
    {
      const v = toUsage(e)
      if (!same(await read($, usage), v)) await update($, usage, () => v)
    }
    const mi = await read($, modelInfo)
    if (mi !== null && mi.window !== e.context.window) await update($, modelInfo, () => ({ ...mi, window: e.context.window }))
    void fetchBreakdown($, false).catch(() => undefined)
    return next(e)
  })

  on('prompt.submit', async ($, e, next) => {
    if (e.origin?.kind === 'task-notification') await finishBackground($, e.text)
    await startTurn($, e.text)
    return next(e)
  })

  on('tool.call', async ($, e, next) => {
    if (e.agentId !== undefined && (await read($, braked)).includes(e.agentId)) return { deny: t(await langOf($), 'agents.brakeDeny') }
    // Vor dem Lauf merken, damit das Ergebnis beim ersten Zeichnen schon als ruhig erkannt wird.
    if (isShell(e.tool) && isReadOnlyShell(shellCommand(e))) await update($, quietIds, ids => pushCapped(ids, e.tool_use_id, 200))
    const ran = await next(e)
    const isEdit = EDIT_TOOLS.has(e.tool)
    const isError = ran.isError === true
    const path = isEdit && !isError ? inputPath(e) : ''
    if (path !== '') await update($, touched, list => (list.includes(path) ? list : pushCapped(list, path, 100)))
    await update($, turn, t => t && { ...t, tools: t.tools + 1, edits: t.edits + (isEdit ? 1 : 0), errors: t.errors + (isError ? 1 : 0) })
    const target = toolTarget(e)
    const server = mcpKey(e.tool)
    if (server !== null) await update($, usedMcp, list => (list.includes(server) ? list : [...list, server]))
    if (e.agentId === undefined) {
      const mark: CallMark = { key: `${toolText(e.tool)}${target === '' ? '' : ` → ${target}`}`, isError, isEdit }
      const list = pushCapped(await read($, marks), mark, 60)
      await update($, marks, () => list)
      const alert = detectLoop(list)
      if (alert !== null) await update($, loopAlert, () => alert)
    }
    await note($, 'main', `${toolText(e.tool)}${target === '' ? '' : ` → ${target}`}${isError ? ' ✗' : ''}`, { tool: e.tool, target, isError })
    await settleAsk($, e.tool_use_id, ran.deny !== undefined)
    if (e.agentId !== undefined) await update($, agents, list => list.map(a => (a.id === e.agentId ? { ...a, steps: (a.steps ?? 0) + 1 } : a)))
    await trackBackground($, e, ran)
    return ran
  })

  // Nur beobachten: das Ergebnis geht unverändert durch, nie ein eigenes deny. Abfragen ohne echten Aufruf zählen nicht.
  on('tool.check', async ($, e, next) => {
    const verdict = await next(e)
    const id = e.tool_use_id
    // Die Engine prüft denselben Aufruf mitunter mehrfach: je tool_use_id nur einmal zählen.
    if (id === undefined || checked.has(id)) return verdict
    checked.add(id)
    const kind = toolKind(e.tool)
    await update($, gate, old => {
      const g = gateOf(old)
      const d = verdict.decision
      const asked = d === 'ask' ? pushCapped(g.asked, id, 30) : g.asked
      return { ...g, [d]: d === 'ask' ? asked.length : g[d] + 1, [kind]: g[kind] + 1, trail: pushCapped(g.trail, d, 30), asked }
    })
    if (verdict.decision === 'deny') await note($, 'gate', t(await langOf($), 'log.denied', { tool: toolText(e.tool), target: toolTarget(e) }))
    return verdict
  })

  // Nur beobachten: liest die Cache-Nutzung jedes Haupt-Requests, der Stream geht unverändert durch.
  // Nur beobachten: merkt sich den Ausgabestil für die Modellzeile, der Prompt geht unverändert durch.
  // Nur beobachten: Claudes eigener Vorschlag nach dem Turn wird wörtlich Folge-Schritt 1; die graue Vorgabe im Eingabefeld bleibt.
  on('prompt.suggest', async ($, e, next) => {
    const shown = await next(e)
    const s = await read($, settings)
    if (e.origin.kind === 'suggestion' && s.steps && e.text.trim() !== '') {
      const text = e.text.trim()
      // `why` ist unser Text: die Engine liefert zu ihrem Vorschlag keine Begründung.
      await update($, claudeHint, () => ({ label: text, prompt: text, why: t(s.lang, 'steps.claude'), fromClaude: true as const }))
      await mergeHint($)
    }
    return shown
  })

  on('prompt.compose', async ($, e, next) => {
    const mi = await read($, modelInfo)
    if (mi !== null) {
      const v = { ...mi, style: e.outputStyle?.name ?? null }
      if (!same(await read($, modelInfo), v)) await update($, modelInfo, () => v)
    }
    return next(e)
  })

  on('turn.step', async function* ($, e, next) {
    const result = yield* next(e)
    if (!e.agentId) {
      const v = { ...(await read($, modelInfo)), model: e.model, effort: e.effort }
      if (!same(await read($, modelInfo), v)) await update($, modelInfo, () => v)
    }
    if (!e.agentId && result.usage) {
      const hit = cacheHitPct(result.usage)
      const now = await $.clock.now()
      {
        const v = hit
        if (!same(await read($, cacheHit), v)) await update($, cacheHit, () => v)
      }
      await update($, lastTurnAt, () => now)
    }
    return result
  })

  on('agent.spawn', async ($, e, next) => {
    const spawned = await next(e)
    if (spawned.agentId) {
      const agent: Agent = {
        id: spawned.agentId,
        type: e.subagentType,
        description: e.description,
        status: 'running',
        since: await $.clock.now(),
        model: spawned.model,
        ...(e.model === undefined ? {} : { requested: e.model }),
        background: e.background,
        fork: e.fork,
        teammate: e.isTeammate === true,
        ...(e.permissionMode === undefined ? {} : { permission: e.permissionMode }),
      }
      await update($, agents, list => pushCapped(list, agent, 20))
      await note($, 'agent', t(await langOf($), 'log.agentStarted', { type: e.subagentType }))
    }
    return spawned
  })

  on('turn.complete', async ($, e, next) => {
    const done = await next(e)
    if (e.agentId) {
      const status = e.isAborted ? 'killed' : 'completed'
      const u = e.usage
      const end = {
        status,
        durationMs: e.durationMs,
        ...(u === undefined ? {} : { model: u.model, tokensIn: u.input_tokens + u.cache_read_input_tokens + u.cache_creation_input_tokens, tokensOut: u.output_tokens, cacheHit: cacheHitPct(u) }),
      }
      await update($, agents, list => list.map(a => (a.id === e.agentId ? { ...a, ...end } : a)))
      return done
    }
    stopSpin()
    // Nach dem Turn ist keine Rückfrage mehr offen (beantwortet oder abgebrochen): Zähler leeren.
    await update($, gate, old => ({ ...gateOf(old), ask: 0, asked: [] }))
    await syncAgents($)
    const fresh = toUsage(await $.session.usage())
    await update($, usage, () => fresh)
    void fetchBreakdown($, true).catch(() => undefined)
    void loadRepo($).then(async r => (same(await read($, repo), r) ? undefined : update($, repo, () => r))).catch(() => undefined)
    const now = await $.clock.now()
    await update($, lastTurnAt, () => now)
    await update($, turn, t => t && { ...t, durationMs: e.durationMs, usdDelta: fresh.usd !== undefined && t.usdStart !== undefined ? fresh.usd - t.usdStart : undefined })
    const done1 = await read($, turn)
    if (done1 !== null) await update($, turnLog, list => pushCapped(list, { usd: done1.usdDelta, tools: done1.tools, edits: done1.edits, durationMs: e.durationMs }, 30))
    const s = await read($, settings)
    await note($, 'main', t(s.lang, 'log.turnDone', { d: duration(e.durationMs) }))
    if (!e.agentId) void maybeCatchUp($).catch(() => undefined)
    if (s.steps && e.reason === 'answer' && !e.isAborted && e.answer.trim()) {
      const asked = await read($, lastPrompt)
      // Nur übernehmen, wenn inzwischen kein neuer Prompt gesendet wurde.
      void suggest($, asked, e.answer)
        .then(async list => ((await read($, lastPrompt)) === asked ? update($, steps, () => list).then(() => mergeHint($)) : undefined))
        .catch(() => undefined)
    }
    return done
  })

  on('ui.render', { component: 'AbovePrompt' }, async ($, e, next) => {
    const s = await read($, settings)
    const u = await read($, usage)
    const next3 = e.props.isWorking || !s.steps ? [] : await read($, steps)
    if (e.surface !== 'desktop' || e.props.hasSurvey || (u === null && next3.length === 0 && (await read($, loopAlert)) === null)) return next(e)
    await read($, tick)
    const now = await $.clock.now()
    const running = (await read($, agents)).filter(a => LIVE.has(a.status)).length
    const ui = $.ui.resolve(e)
    const { Box, Text, Button } = ui
    // Ringe nur, wo die Oberfläche Svg zeichnet; sonst bleibt der Textbalken.
    const Svg = 'Svg' in ui ? ui.Svg : undefined
    // Mit Tooltip ist das Ring-Svg interaktiv (Sandbox-Frame, `<title>` wird schwebend angezeigt). Der Frame flackert bei jedem
    // Neuzeichnen: darum nur im Leerlauf: weder Turn noch laufender Subagent; und der Text ändert sich nur minutengenau
    // (sonst baut die Oberfläche ihn neu auf).
    const tipNow = Math.floor(now / 60_000) * 60_000
    const L = s.lang
    const P = panes(L)
    const ring = (percent: number, color: keyof typeof RING_HEX, tip = '') =>
      !Svg || !s.rings ? null : s.ctxTip && tip !== '' && !e.props.isWorking && running === 0 ? (
        <Svg source={ringSvg(percent, RING_HEX[color], 28, tip)} alt={tip.split(NL)[0] ?? ''} width={28} height={28} isInteractive />
      ) : (
        <Svg source={ringSvg(percent, RING_HEX[color])} alt={`${Math.round(percent)} %`} width={28} height={28} />
      )
    const head = titleRow(ui, await read($, about), await read($, logo))
    const where = s.branch ? repoRow(ui, await read($, repo)) : null
    const alert = await read($, loopAlert)
    const loopText = alert === null ? '' : t(L, alert.kind === 'error' ? 'loop.error' : 'loop.edit', { target: alert.target, n: alert.n })
    const fillCompact = async () => {
      const focus = compactFocus(await read($, lastPrompt), await read($, touched), await read($, steps), L)
      await $.prompt.fill({ text: focus === '' ? '/compact' : `/compact ${focus}` })
    }
    const card = (...children: RenderChildren[]) => (
      <Box flexDirection="row" columnGap={3} alignItems="center" borderStyle="round" borderColor={THEME.border} paddingX={2}>
        {children}
      </Box>
    )

    return (
      <Box flexDirection="column">
        <Box flexDirection="row" justifyContent="space-between">
          {head}
          {/* Legende und Features-Knopf zusammen rechts, sonst verteilt space-between sie über die Zeile. */}
          <Box flexDirection="row" columnGap={2} alignItems="center">
          {s.legend && (Svg ? (
            // Kurze Legende als Bild; die ganze Legende steht im Tooltip. Interaktiv nur im Leerlauf (wie die Ringe), sonst Bild.
            <Svg
              key="leg"
              source={legendSvg(LEGEND.map(l => ({ color: THEME[l.color], label: t(L, l.label) })), legendTip(L), THEME.textMuted)}
              alt={LEGEND.map(l => t(L, l.label)).join(' · ')}
              height={16}
              isInteractive={!e.props.isWorking && running === 0}
            />
          ) : (
            legend(ui, L)
          ))}
          {/* $.command.run aus dem Knopf kam im Desktop nicht an: Befehl ins Eingabefeld, Enter führt ihn aus. */}
          <Button key="features" label={t(L, 'common.features')} plain onPress={() => void $.prompt.fill({ text: '/cockpit-features' }).catch(() => undefined)} />
          </Box>
        </Box>
        <Box flexDirection="row" marginBottom={1}>
          {where}
        </Box>
        {u !== null && metrics(u, await read($, lastTurnAt), await read($, cacheHit), await read($, breakdown))}
        {alert !== null && (
          <Box key="loop" flexDirection="row" columnGap={2} alignItems="center" borderStyle="round" borderColor={THEME.warning} paddingX={1}>
            <Text color={THEME.warning} wrap="truncate-end">⚠ {loopText}</Text>
            <Button key="loop-fix" label={t(L, 'loop.action')} plain onPress={() => void $.prompt.fill({ text: t(L, 'loop.prompt', { what: loopText }) }).catch(() => undefined)} />
          </Box>
        )}
        {next3.length > 0 && (
          // Je Vorschlag eine Zeile: „1: Label“ (Taste 1–5, auch im leeren Eingabefeld) und die Begründung gedimmt daneben.
          <Box flexDirection="column">
            <Text dimColor>{t(L, 'band.next')}</Text>
            {next3.map((s, i) => (
              <Box key={`step-row-${i}`} flexDirection="row" columnGap={2}>
                {s.fromClaude && <Text key={`step-cc-${i}`} backgroundColor={THEME.chip}> CC </Text>}
                <Button
                  key={`step-${i}`}
                  label={clip(s.label, Math.max(24, Math.floor(e.props.bodyColumns * 0.4)))}
                  hotkey={String(i + 1)}
                  plain
                  // Nur ins Eingabefeld: Enter sendet ihn als Eingabe der Person, damit andere Mods (effortless) ihn wie getippt bewerten; startTurn läuft dann im eigenen prompt.submit-Hook.
                  onPress={() => void $.prompt.fill({ text: s.prompt }).catch(() => undefined)}
                />
                {s.why !== undefined && <Text color={THEME.textMuted} wrap="truncate-end">{s.why}</Text>}
              </Box>
            ))}
          </Box>
        )}
      </Box>
    )

    function metrics(u: Usage, lastAt: number | null, hit: number | null, bd: Breakdown | null) {
      const ttl = u.limits.length > 0 ? 3600_000 : 300_000
      const cache = cacheLeftMs(lastAt, u.limits.length > 0, now)
      const isFull = (u.contextPercent ?? 0) >= COMPACT_AT
      return (
        <Box flexDirection="column">
        <Box flexDirection="row" flexWrap="wrap" columnGap={2} alignItems="center">
          {card(
            <Box key="ctx" flexDirection="row" columnGap={1} alignItems="center">
              {ring(u.contextPercent ?? 0, contextColor(u.contextPercent ?? 0), contextTip(u, bd, L))}
              <Text color={isFull ? THEME.warning : undefined}>{t(L, 'common.context')} {Svg && s.rings ? '' : `${bar(u.contextPercent ?? 0, 8)} `}{Math.round(u.contextPercent ?? 0)}%</Text>
            </Box>,
            // Schreibt /compact mit dem, was erhalten bleiben muss, ins Eingabefeld; Enter startet es (anpassbar vor dem Senden).
            <Button key="compact" label={t(L, 'band.compact')} {...(isFull ? { variant: 'primary' as const } : { plain: true })} onPress={() => void fillCompact().catch(() => undefined)} />,
          )}
          {s.limits && u.limits.length > 0 &&
            card(
              ...u.limits.map(l => (
                <Box key={`lim-${l.kind}`} flexDirection="row" columnGap={1} alignItems="center">
                  {ring(l.percentUsed, limitColor(l, now), limitTip(l, tipNow, L))}
                  <Text color={THEME[limitColor(l, now)]}>{LIMIT_LABEL[l.kind] ?? l.kind} {Math.round(l.percentUsed)}%</Text>
                </Box>
              )),
            )}
          {s.cache && card(
            <Box key="cache" flexDirection="row" columnGap={1} alignItems="center">
              {ring((100 * cache) / ttl, 'accent', cacheTip(cacheLeftMs(lastAt, u.limits.length > 0, tipNow), ttl, hit, lastAt, tipNow, L))}
              <Text dimColor={cache === 0 && u.contextTokens === undefined} color={cache === 0 && u.contextTokens !== undefined ? THEME.warning : cache > 0 && cache <= CACHE_WARN_MS ? THEME.warning : undefined}>
                {t(L, 'band.cache')} {cache === 0 ? (u.contextTokens !== undefined ? t(L, 'band.coldCost', { n: tokens(u.contextTokens) }) : t(L, 'band.cold')) : duration(cache)}
                {cache > 0 && hit !== null ? ` · ${hit}%` : ''}
              </Text>
            </Box>,
            u.usd !== undefined && <Text>{usd(u.usd)}</Text>,
            running > 0 && <Text color={THEME.accent}>{t(L, 'band.agents', { n: running })}</Text>,
          )}
          <Button key="details" label={t(L, 'common.details')} plain onPress={() => void $.ui.open({ ...P.main, focus: true })} />
          {s.cardCatchUp && <Button key="summary" label={t(L, 'common.summary')} plain onPress={() => void $.ui.open({ ...P.summary, focus: true })} />}
          <Button key="settings" label={t(L, 'common.settings')} plain onPress={() => void $.ui.open({ ...P.settings, focus: true })} />
          {/* $.command.run aus einem Knopf kam im Desktop nicht an: Befehl ins Eingabefeld, Enter führt ihn aus. */}
          <Button key="demo" label={t(L, 'common.demo')} plain onPress={() => void $.prompt.fill({ text: '/cockpit-demo' }).catch(() => undefined)} />
        </Box>
        </Box>
      )
    }
  })

  // Zusammenfassung in eigenem Panel rechts: Knöpfe oben, darunter die Abschnitte als einfacher Text (kein Markdown-Rendering).
  on('ui.render', { component: 'Pane', requestId: 'orgamax-cockpit-summary' }, async ($, e, next) => {
    if (e.surface !== 'desktop') return next(e)
    const s = await read($, settings)
    const cu = await read($, catchUp)
    const L = s.lang
    const ui = $.ui.resolve(e)
    const { Box, Text, Button } = ui

    return (
      <Box flexDirection="column" gap={1}>
        {await paneHead($, ui, L)}
        <Box flexDirection="row" columnGap={2} alignItems="center">
          <Button key="cu-refresh" label={t(L, 'summary.refresh')} variant="primary" onPress={() => void runCatchUp($, false).catch(() => undefined)} />
          <Button key="cu-rebuild" label={t(L, 'summary.rebuild')} plain onPress={() => void rebuildCatchUp($).catch(() => undefined)} />
          <Text dimColor>{cu.status !== '' ? cu.status : cu.updatedAt > 0 ? t(L, 'summary.updated', { time: clockTime(new Date(cu.updatedAt).toISOString()) }) : t(L, 'summary.none')}</Text>
        </Box>
        <Text dimColor>{t(L, 'summary.status', { auto: t(L, s.catchUp ? 'common.on' : 'common.off'), lang: t(L, L === 'de' ? 'common.langDe' : 'common.langEn') })}</Text>
        {cu.summary === '' && <Text dimColor>{t(L, 'summary.empty')}</Text>}
        {summaryLines(cu.summary).map(l =>
          l.kind === 'heading' ? (
            <Box paddingTop={1}>
              <Text bold color={THEME.accent}>{l.text}</Text>
            </Box>
          ) : l.kind === 'bullet' ? (
            <Text>• {l.text}</Text>
          ) : (
            <Text>{l.text}</Text>
          ),
        )}
      </Box>
    )
  })

  // Einstellungsseite: jedes Detail von Band, Panel und Chat ein- oder ausblenden; gilt sofort und für alle Sessions.
  on('ui.render', { component: 'Pane', requestId: 'orgamax-cockpit-settings' }, async ($, e, next) => {
    if (e.surface !== 'desktop') return next(e)
    const s = await read($, settings)
    const L = s.lang
    const ui = $.ui.resolve(e)
    const { Box, Text, Button } = ui
    const groups = [...new Set(SETTING_DEFS.map(d => d.group))]
    const card = (...children: RenderChildren[]) => (
      <Box flexDirection="column" borderStyle="round" borderColor={THEME.border} paddingX={1}>
        {children}
      </Box>
    )
    const keysOf = (group: SettingGroup): SettingKey[] => SETTING_DEFS.filter(d => d.group === group).map(d => d.key)
    const setAll = (group: SettingGroup, value: boolean) =>
      void setSettings($, Object.fromEntries(keysOf(group).map(k => [k, value])) as Partial<ModSettings>).catch(() => undefined)
    // Segment-Paar: die gewählte Sprache als Hauptknopf, die andere gedimmt.
    const langButton = (lang: UiLang, name: string) =>
      L === lang ? (
        <Button key={`lang-${lang}`} label={`● ${name}`} variant="primary" onPress={() => void setSettings($, { lang }).catch(() => undefined)} />
      ) : (
        <Button key={`lang-${lang}`} label={`○ ${name}`} dimColor onPress={() => void setSettings($, { lang }).catch(() => undefined)} />
      )

    return (
      <Box flexDirection="column">
        {await paneHead($, ui, L)}
        <Text dimColor>{t(L, 'settings.intro')}</Text>
        {card(
          <Text bold>{t(L, 'settings.langTitle')}</Text>,
          <Box flexDirection="row" columnGap={1} alignItems="center">
            {langButton('de', t(L, 'common.langDe'))}
            {langButton('en', t(L, 'common.langEn'))}
          </Box>,
          <Text dimColor wrap="wrap">{t(L, 'settings.langHint')}</Text>,
        )}
        {groups.map((group, i) => {
          const keys = keysOf(group)
          return card(
            <Box flexDirection="row" flexWrap="wrap" justifyContent="space-between" alignItems="center" columnGap={2}>
              <Text bold>{t(L, `settings.group.${group}`)}</Text>
              <Box flexDirection="row" flexWrap="wrap" columnGap={2} alignItems="center">
                <Text dimColor>{t(L, 'settings.count', { n: keys.filter(k => s[k]).length, m: keys.length })}</Text>
                <Button key={`grp-on-${i}`} label={t(L, 'settings.allOn')} plain onPress={() => setAll(group, true)} />
                <Button key={`grp-off-${i}`} label={t(L, 'settings.allOff')} plain onPress={() => setAll(group, false)} />
              </Box>
            </Box>,
            ...keys.map(k => {
              const flip = () => void setSettings($, { [k]: !s[k] } as Partial<ModSettings>).catch(() => undefined)
              // Der Desktop zeichnet keine Text-Kinder eines Buttons, nur das Label: der Schalterzustand steht deshalb im Label.
              return (
                <Box key={`row-${k}`} flexDirection="column">
                  <Box flexDirection="row" columnGap={1} alignItems="center">
                    {s[k] ? (
                      <Button key={`set-${k}`} label={`● ${t(L, 'settings.on')}`} variant="primary" onPress={flip} />
                    ) : (
                      <Button key={`set-${k}`} label={`○ ${t(L, 'settings.off')}`} dimColor onPress={flip} />
                    )}
                    <Text bold wrap="truncate-end">{settingLabel(k, L)}</Text>
                  </Box>
                  {/* Hinweis in eigener Zeile, eingerückt: bleibt auch in schmalen Panels sichtbar */}
                  <Box paddingLeft={2}>
                    <Text dimColor wrap="wrap">{settingHint(k, L)}</Text>
                  </Box>
                </Box>
              )
            }),
          )
        })}
        <Box flexDirection="row" paddingTop={1}>
          <Button key="set-reset" label={t(L, 'settings.reset')} onPress={() => void resetSettings($).catch(() => undefined)} />
        </Box>
      </Box>
    )
  })

  on('ui.render', { component: 'Pane', requestId: 'orgamax-cockpit' }, async ($, e, next) => {
    if (e.surface !== 'desktop') return next(e)
    await read($, tick)
    const now = await $.clock.now()
    const s = await read($, settings)
    const bdPanel = await read($, breakdown)
    const bdNote = await read($, breakdownNote)
    const [mark, place] = await Promise.all([read($, logo), read($, repo)])
    const [u, tn, g, list, entries, last, hit, info] = await Promise.all([
      read($, usage), read($, turn), read($, gate).then(gateOf), read($, agents), read($, log), read($, lastTurnAt), read($, cacheHit), read($, about),
    ])
    const cache = u ? cacheLeftMs(last, u.limits.length > 0, now) : 0
    const L = s.lang
    const P = panes(L)
    const ui = $.ui.resolve(e)
    const { Box, Text, Button } = ui
    const Svg = 'Svg' in ui ? ui.Svg : undefined
    // Kopf jeder Karte: ▾/▸ klappt zu (nur Session, Atom `collapsed`), ✕ schaltet die Karte in den Einstellungen ab.
    const folded = await read($, collapsed)
    const isOpen = (key: SettingKey) => !folded.includes(key)
    const headButtons = (key: SettingKey) => (
      <Box flexDirection="row" columnGap={1}>
        <Button key={`card-fold-${key}`} label={isOpen(key) ? '▾' : '▸'} plain onPress={() => void update($, collapsed, c => (c.includes(key) ? c.filter(x => x !== key) : [...c, key])).catch(() => undefined)} />
        <Button key={`card-close-${key}`} label="✕" plain onPress={() => void setSettings($, { [key]: false } as Partial<ModSettings>).catch(() => undefined)} />
      </Box>
    )
    // Halbe Zeile zwischen den Karten: Box rechnet nur in ganzen Zeilen, Svg in CSS-Pixeln.
    const gap = () => (Svg ? <Svg source={'<svg xmlns="http://www.w3.org/2000/svg" width="1" height="8"/>'} alt="" height={8} /> : <Box height={1} />)
    // Jede Karte trägt ihren Abstand nach oben selbst: abgeschaltete Karten hinterlassen keine doppelte Lücke.
    const spaced = (inner: RenderElement) => (
      <Box flexDirection="column" flexGrow={1}>
        {gap()}
        {inner}
      </Box>
    )
    const card = (key: SettingKey, name: string, ...children: RenderChildren[]) => spaced(
      <Box flexDirection="column" flexGrow={1} borderStyle="round" borderColor={THEME.border} paddingX={1}>
        <Box flexDirection="row" justifyContent="space-between">
          <Text bold>{name}</Text>
          {headButtons(key)}
        </Box>
        {isOpen(key) && children}
      </Box>
    )

    // Verlauf der Subagents wie in Flightdeck: Name links, Balken auf der gemeinsamen Zeitachse (ältester Start bis jetzt).
    const timeline = (all: readonly Pick<Agent, 'id' | 'description' | 'since' | 'status' | 'durationMs'>[]) => {
      const t0 = Math.min(...all.map(a => a.since))
      const endOf = (a: Pick<Agent, 'since' | 'status' | 'durationMs'>) => (LIVE.has(a.status) || a.durationMs === undefined ? now : a.since + a.durationMs)
      const t1 = Math.max(now, ...all.map(endOf))
      return all.map(a => {
        const color = LIVE.has(a.status) ? RING_HEX.accent : a.status === 'killed' || a.status === 'failed' ? RING_HEX.error : RING_HEX.success
        return (
          <Box key={`tl-${a.id}`} flexDirection="row" columnGap={1} alignItems="center">
            <Box width={22}><Text dimColor wrap="truncate-end">{a.description}</Text></Box>
            {Svg ? <Svg source={timelineSvg(a.since, endOf(a), t0, t1, color)} alt={`${a.description} ${duration(endOf(a) - a.since)}`} height={8} /> : <Text dimColor>{duration(endOf(a) - a.since)}</Text>}
          </Box>
        )
      })
    }
    const tasks = await read($, bgTasks)
    // Kontext-Diät: nie aufgerufene MCP-Server mit ihren Schema-Tokens; der Knopf schreibt /mcp ins Eingabefeld zum Abschalten.
    const dietRows = () =>
      diet.length === 0
        ? []
        : [
            <Box key="diet-head" flexDirection="row" columnGap={2} marginTop={1}>
              <Text color={THEME.warning}>{t(L, 'diet.title', { tokens: tokens(diet.reduce((n, r) => n + r.tokens, 0)) })}</Text>
              <Button key="diet-mcp" label={t(L, 'diet.action')} plain onPress={() => void $.prompt.fill({ text: '/mcp' }).catch(() => undefined)} />
            </Box>,
            ...diet.slice(0, 6).map(r => <Text key={`diet-${r.server}`} dimColor wrap="truncate-end">  {t(L, 'diet.row', { server: r.server, tokens: tokens(r.tokens), n: r.tools })}</Text>),
          ]
    const diet = mcpDiet(await read($, mcpTools), await read($, usedMcp))
    const turns = await read($, turnLog)
    const brakedIds = await read($, braked)
    const brake = async (a: Agent) => {
      await update($, braked, ids => (ids.includes(a.id) ? ids : [...ids, a.id]))
      await note($, 'agent', t(L, 'log.braked', { name: a.description }))
    }
    const working = s.animation && tn !== null && tn.durationMs === undefined

    return (
      <Box flexDirection="column">
        <Box flexDirection="row" justifyContent="space-between" alignItems="center">
          {await paneHead($, ui, L)}
          <Box flexDirection="row" columnGap={2}>
            {s.cardCatchUp && <Button key="open-summary" label={t(L, 'common.summary')} plain onPress={() => void $.ui.open({ ...P.summary, focus: true })} />}
            <Button key="open-settings" label={t(L, 'common.settings')} plain onPress={() => void $.ui.open({ ...P.settings, focus: true })} />
          </Box>
        </Box>
        {working && tn !== null && spaced(
          <Box flexDirection="column" borderStyle="round" borderColor={THEME.accent} paddingX={1}>
            <Text bold color={THEME.accent}>● {t(L, 'panel.working', { d: duration(now - tn.startedAt), tools: tn.tools, edits: tn.edits })}</Text>
            {entries.length > 0 && (
              <Text wrap="truncate-end">
                <Text dimColor>{t(L, 'panel.lastEvent', { text: '' })}</Text>
                {entryView(ui, entries[entries.length - 1] as LogEntry, true)}
              </Text>
            )}
          </Box>
        )}
        {s.cardWorkdir && place !== null &&
          card(
            'cardWorkdir',
            t(L, 'panel.workdir'),
            ...repoRows(place, L).map(([label, value]) => (
              <Box flexDirection="row" columnGap={1}>
                <Box width={12}><Text dimColor>{label}</Text></Box>
                <Text wrap="truncate-end">{value}</Text>
              </Box>
            )),
          )}
        {s.cardContext && bdPanel === null && card('cardContext', t(L, 'panel.context'), <Text dimColor>{bdNote !== '' ? bdNote : t(L, 'panel.contextPending')}</Text>)}
        {s.cardContext && bdPanel !== null &&
          card(
            'cardContext',
            t(L, 'panel.context'),
            <Text dimColor>{breakdownTitle(bdPanel, L)}</Text>,
            Svg && <Svg source={breakdownBarSvg(bdPanel)} alt={breakdownTitle(bdPanel, L)} height={14} />,
            ...bdPanel.rows.map((r, i) => (
              <Box flexDirection="row" columnGap={1}>
                <Text color={categoryColor(r, i)}>■</Text>
                <Box width={24}><Text>{r.name}</Text></Box>
                <Box width={7} justifyContent="flex-end"><Text dimColor>{tokens(r.tokens)}</Text></Box>
                <Box width={7} justifyContent="flex-end"><Text bold>{r.percent.toFixed(1)}%</Text></Box>
                <Text color={categoryColor(r, i)}>{bar(r.percent, 12)}</Text>
              </Box>
            )),
            ...dietRows(),
          )}
        {s.cardOverview && card(
          'cardOverview',
          t(L, 'panel.overview'),
          u === null && <Text dimColor>{t(L, 'panel.noMeasure')}</Text>,
          u !== null && (
            <Text color={(u.contextPercent ?? 0) >= COMPACT_AT ? THEME.warning : undefined}>
              {t(L, 'common.context')} {bar(u.contextPercent ?? 0)} {Math.round(u.contextPercent ?? 0)}% ({tokens(u.contextTokens ?? 0)} / {tokens(u.contextWindow)})
            </Text>
          ),
          ...(u?.limits.map(l => {
            const p = pace(l, now)
            return (
              <Text color={THEME[limitColor(l, now)]}>
                {LIMIT_LABEL[l.kind] ?? l.kind} {bar(l.percentUsed)} {Math.round(l.percentUsed)}%
                {p !== undefined ? ` · ${paceText(p, L)}` : ''}
                {l.resetsAt ? ` · ${t(L, 'panel.reset', { at: resetLabel(l.resetsAt, now, L) })}` : ''}
              </Text>
            )
          }) ?? []),
          u?.usd !== undefined && <Text>{t(L, 'panel.cost', { usd: usd(u.usd) })}</Text>,
          u !== null && (
            <Text dimColor={cache === 0}>
              {cache === 0 ? t(L, 'panel.cacheCold') : t(L, 'panel.cacheLeft', { d: duration(cache) })}
              {hit !== null ? ` · ${t(L, 'panel.cacheHit', { n: hit })}` : ''}
            </Text>
          ),
        )}
        <Box flexDirection="row">
          {s.cardTurn && card(
            'cardTurn',
            t(L, 'panel.turn'),
            tn === null && <Text dimColor>{t(L, 'panel.noTurn')}</Text>,
            tn !== null && (
              <Text>
                {tn.durationMs === undefined ? t(L, 'panel.runningSince', { d: duration(now - tn.startedAt) }) : duration(tn.durationMs)} ·{' '}
                {t(L, 'panel.turnStats', { tools: tn.tools, edits: tn.edits, errors: tn.errors })}
                {tn.usdDelta !== undefined ? ` · +${usd(tn.usdDelta)}` : ''}
              </Text>
            ),
            turns.length > 1 && <Text dimColor>{t(L, 'turn.bars', { usd: usd(WASTE_USD) })}</Text>,
            turns.length > 1 && Svg && <Svg source={turnBarsSvg(turns)} alt={t(L, 'turn.bars', { usd: usd(WASTE_USD) })} height={24} />,
            turns.some(isWasteful) && <Text color={THEME.warning}>{t(L, 'turn.waste', { n: turns.filter(isWasteful).length })}</Text>,
          )}
          {s.cardGate && spaced(
            <Box flexDirection="column" flexGrow={1} borderStyle="round" borderColor={THEME[g.deny > 0 ? 'error' : g.ask > 0 ? 'warning' : 'success']} paddingX={1}>
              <Box flexDirection="row" justifyContent="space-between">
                <Text bold color={THEME[g.deny > 0 ? 'error' : g.ask > 0 ? 'warning' : 'success']}>{t(L, 'gate.title')}</Text>
                <Box flexDirection="row" columnGap={1}>
                  <Text dimColor>{t(L, 'gate.checks', { n: g.allow + g.you + g.ask + g.deny })}</Text>
                  {headButtons('cardGate')}
                </Box>
              </Box>
              {isOpen('cardGate') && (
                <Box flexDirection="column">
                  <Text>{g.trail.length === 0 ? <Text dimColor>{t(L, 'gate.none')}</Text> : g.trail.map(d => <Text color={DECISION_COLOR[d]}>■</Text>)}</Text>
                  <Box flexDirection="row" flexWrap="wrap" columnGap={1}>
                    <Text><Text color={DECISION_COLOR.allow}>■</Text> {t(L, 'gate.allow', { n: g.allow })}</Text>
                    <Text><Text color={DECISION_COLOR.you}>■</Text> {t(L, 'gate.you', { n: g.you })}</Text>
                    <Text color={g.ask > 0 ? THEME.warning : undefined}><Text color={DECISION_COLOR.ask}>■</Text> {t(L, 'gate.ask', { n: g.ask })}</Text>
                    <Text color={g.deny > 0 ? THEME.error : undefined}><Text color={DECISION_COLOR.deny}>✗</Text> {t(L, 'gate.deny', { n: g.deny })}</Text>
                  </Box>
                  <Box flexDirection="row" flexWrap="wrap" columnGap={2}>
                    {([['file', t(L, 'gate.chipFile'), t(L, 'common.file'), g.file], ['shell', t(L, 'gate.chipShell'), t(L, 'common.shell'), g.shell], ['other', t(L, 'gate.chipOther'), t(L, 'gate.other'), g.other]] as const).map(([id, k, label, n]) => (
                      <Text key={`gk-${id}`}>
                        <Text backgroundColor={THEME.chip}> {k} </Text> <Text bold>{label} {n}</Text>
                      </Text>
                    ))}
                  </Box>
                </Box>
              )}
            </Box>
          )}
        </Box>
        {s.cardAgents && card(
          'cardAgents',
          t(L, 'agents.title', { live: list.filter(a => LIVE.has(a.status)).length, all: list.length }),
          list.length === 0 && <Text dimColor>{t(L, 'agents.none')}</Text>,
          // Ältere als kompakte Liste, nur die letzten beiden als Kachel.
          ...list.slice(0, -2).map((a, i) => (
            <Text key={`ago-${a.id}`} wrap="truncate-end" dimColor={!LIVE.has(a.status)}>
              <Text backgroundColor={THEME.chip}> {i + 1} </Text> {a.description} · {a.type}{a.model !== undefined ? ` · ${a.model}` : ''} ·{' '}
              {LIVE.has(a.status) ? t(L, 'agents.running', { d: duration(now - a.since) }) : `${a.status === 'killed' || a.status === 'failed' ? '✗' : '✓'} ${a.durationMs !== undefined ? duration(a.durationMs) : ''}`}
            </Text>
          )),
          <Box flexDirection="row" flexWrap="wrap" columnGap={1}>
            {list.slice(-2).map((a, k) => {
              const i = Math.max(0, list.length - 2) + k
              const live = LIVE.has(a.status)
              const isKilled = a.status === 'killed' || a.status === 'failed'
              const isBraked = brakedIds.includes(a.id)
              const heavy = live && isAgentHeavy(a, now)
              const tone = heavy || (live && isBraked) ? THEME.warning : live ? THEME.accent : isKilled ? THEME.error : THEME.success
              return (
                <Box key={`ag-${a.id}`} flexDirection="column" width={42} borderStyle="round" borderColor={tone} paddingX={1}>
                  <Text wrap="truncate-end">
                    <Text backgroundColor={THEME.chip}> {i + 1} </Text> <Text bold>{a.description}</Text>
                  </Text>
                  <Text wrap="truncate-end">{a.type}{a.model !== undefined ? ` · ${a.model}` : ''}</Text>
                  <Text dimColor wrap="truncate-end">{agentStats(a, L)}</Text>
                  {agentFlags(a, L) !== '' && <Text dimColor wrap="truncate-end">{agentFlags(a, L)}</Text>}
                  {heavy && <Text color={THEME.warning} wrap="truncate-end">{t(L, 'agents.heavy', { steps: a.steps ?? 0, d: duration(now - a.since) })}</Text>}
                  {live && isBraked && <Text color={THEME.warning}>{t(L, 'agents.braked')}</Text>}
                  {heavy && !isBraked && <Button key={`brake-${a.id}`} label={t(L, 'agents.brake')} plain onPress={() => void brake(a)} />}
                  <Text color={live ? THEME.accent : isKilled ? THEME.error : THEME.success}>
                    {live
                      ? t(L, 'agents.running', { d: duration(now - a.since) })
                      : t(L, isKilled ? 'agents.killed' : 'agents.done', { d: a.durationMs !== undefined ? duration(a.durationMs) : '' })}
                  </Text>
                </Box>
              )
            })}
          </Box>,
          list.length > 0 && <Text dimColor>{t(L, 'agents.timeline')}</Text>,
          ...timeline(list),
        )}
        {s.cardTasks && card(
          'cardTasks',
          t(L, 'tasks.title', { live: tasks.filter(x => x.status === 'running').length, all: tasks.length }),
          tasks.length === 0 && <Text dimColor>{t(L, 'tasks.none')}</Text>,
          ...tasks.map((x, i) => {
            const live = x.status === 'running'
            const bad = x.status === 'failed' || x.status === 'killed'
            const tone = live ? THEME.accent : bad ? THEME.error : THEME.success
            const d = duration(live || x.durationMs === undefined ? now - x.since : x.durationMs)
            return (
              <Box key={`bg-${x.id}`} flexDirection="column">
                <Text wrap="truncate-end">
                  <Text backgroundColor={THEME.chip}> {i + 1} </Text> <Text bold>{x.description}</Text> <Text color={tone}>{live ? `● ${d}` : `${bad ? '✗' : '✓'} ${x.status} ${d}`}</Text>
                </Text>
                {x.command !== undefined && x.command !== x.description && <Text color={THEME.textMuted} wrap="truncate-end">  $ {x.command}</Text>}
                {x.summary !== undefined && x.summary !== '' && <Text color={THEME.textMuted} wrap="truncate-end">  {x.summary}</Text>}
              </Box>
            )
          }),
          tasks.length > 0 && <Text dimColor>{t(L, 'agents.timeline')}</Text>,
          ...timeline(tasks),
        )}
        {s.cardLog && card(
          'cardLog',
          t(L, 'log.title'),
          entries.length === 0 && <Text dimColor>{t(L, 'log.empty')}</Text>,
          ...entries.slice(-LOG_MAX).map(l => {
            const src = l.src ?? 'main'
            return (
              <Text wrap="truncate-end">
                <Text dimColor>{clockTime(new Date(l.at).toISOString())}</Text> <Text bold color={SRC_COLOR[src]}>{src.padEnd(5)}</Text> {l.tool !== undefined ? entryView(ui, l, false) : <Text bold={src === 'you'} dimColor={src !== 'you'}>{l.text}</Text>}
              </Text>
            )
          }),
        )}
      </Box>
    )
  })
}
