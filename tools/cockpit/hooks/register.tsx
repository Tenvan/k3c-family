import { atom, read, update } from 'claude-code'
import type { EngineInterface, PressedLink, Register, RenderChildren, RenderElement, RenderSurface, ResolveInput, SessionUsage } from 'claude-code'

import type { BgTask, LinkedSession, About, ModelInfo, Agent, Breakdown, CallMark, CatchUp, Decision, LoopAlert, McpSchema, UiLang, Repo, Gate, LogEntry, LogSrc, ModSettings, Step, Turn, TurnRecord, Usage } from '../types'
import { mockData } from './mock'
import { BREATHS, breathFrameSvg, breathOf, breathSvg, phaseAt } from './breath'
import type { BreathKey } from './breath'
// Jede Atemübung in eigener Farbe aus dem Design-System, damit der Wechsel auf einen Blick erkennbar ist.
const BREATH_COLOR: Record<BreathKey, string> = { hrv: THEME.accent, sigh: THEME.success, box: THEME.violet, '478': THEME.warning }
import { canMergeNow, conflictPrompt, draftPrompt, inRepo, refOf, forgeRemote, mrBadge, mrRef, isPipelineRunning, mrEvents, parseDraft, pipelineTone, pollDelayMs, threadsPrompt } from './mr'
import { authMissing, canAutoMerge, forgeCreate, forgeDefault, forgeFind, forgeLists, forgeLoad, forgeMerge, forgeRebase, forgeRetry, forgeThreads } from './forge'
import type { Io, Remote } from './forge'
import type { Mr, MrDraft, MrMine } from '../types'
import { THEME, TINT, CARD_TINT, cycleTint, summarySections, splitTool, toolText, withClaude, parseTaskNotifications, notificationText, backgroundOf, NEW_STYLES, todosOf, wrapupOf, resetLeft, dirTarget, isForeignDir, nativeDir, parseLinkedSessions, infoSvg, legendSvg, modelLabel, SPIN_MODE, spinnerText, timelineSvg, demoTable, agentStats, agentFlags, breakdownBarSvg, contextTip, limitTip, cacheTip, summaryLines, topRows, resetLabel, CATCH0, CATCH_KEEP, DEFAULT_SETTINGS, SETTING_DEFS, settingLabel, settingHint, catchUpPrompt, catchUpSystem, nextChunk, settingsOf, startIndex, fingerprint } from './core'
import type { SettingGroup, SettingKey, Tone } from './core'
import { TILES, EFFORT_LEVELS, effortColor, effectiveEffort, effortJudgeText, handoffMarkdown, handoffPays, isTrivialPrompt, limitAlert } from './core'
import type { EffortLevel } from './core'
import { CACHE_WARN_MS, WASTE_USD, compactFocus, detectLoop, isAgentHeavy, isWasteful, mcpDiet, mcpKey, turnBarsSvg, turnBarsText } from './core'
import { RISKY_GIT_CONFIG, COMPACT_AT, DECISION_COLOR, RING_HEX, contextColor, LEGEND, LEGEND_NOTE, paceText, LIMIT_LABEL, LOG_MAX, SRC_COLOR, bar, cacheHitPct, cacheLeftMs, clockTime, duration, limitColor, pace, parseSteps, questionsOf, pushCapped, markdownStats, shortHash, statsLines, toolInfoLines, oneLine, parseRepo, trustText, trustEntry, isTrusted, isSafeTrustEntry, repoGitRows, repoPathRows, repoShort, ringSvg, svgSource, titleText, toBreakdown, breakdownTitle, categoryColor, spinner, title, toolTarget, toolAction, toolKind, tokens, usd } from './core'
import { t } from './i18n'
import type { Key } from './i18n'
import { ALERT_COLOR, CODE_BG, CODE_FG, ICON_COLOR, TONE_COLOR, detailInput, detailOutput, diffCardSvg, diffOf, errorText, firstErrorLine, iconKind, inlineTokens, inputPath, isReadOnlyShell, isShell, isStyled, lineCountOf, mdLiteral, QUIET_TOOLS, quietLine, shellCommand, splitMessage, toolIconSvg } from './skins'
import { askSummary, mathSvg } from './skins'
import type { RichPart, Segment, SplitOptions } from './skins'
import { diagramType, mermaidSvg } from './mermaid'
import { chip, labelRow, pill, tile } from './ui'
import type { Ui } from './ui'
import { addRecord, avgStepMs, completeAll, continuePrompt, dayKey, dayReport, etaOf, finishJob, isWorthShowing, newJob, noteAct, noteFile, planUpdate, progressOf, recordsOf, shiftDay, shownPhase, staleKeys, stepGuessMs, storeKey, toRecord, touch, visibleSteps, weekReport } from './glance'
import type { ShownPhase } from './glance'
import type { Job, JobRecord, JobStep } from '../types'
import { bandText as inspectBand, ingestAppend, ingestCompaction, ingestToolCall, ingestToolResult, inputOf, newInspect, paneView as inspectView, reportText as inspectReport, sampleInput, summarize as summarizeInspect } from './inspect'
import type { InspectSummary } from './inspect'
import { bandText as profileBand, newProfiler, paneView as profileView, reportText as profileReport, stepBegin, stepChunk, stepEnd, stopFailure, subagentStart, subagentStop, summarize as summarizeProfile, toolApproval, toolBegin, toolEnd, toolExec, turnComplete, turnStart } from './profiler'
import type { ProfileSummary } from './profiler'
import { bandText as blastBand, blastScan, normalizeTarget, paneView as blastView, parseCommand, reportText as blastReport } from './blast'
import type { BlastIo, BlastResult } from './blast'

const PLUGIN = 'tenvan-cc'
// Slash-Command des Hauptpanels: kurz statt der Plugin-ID.
const COMMAND = 'cockpit'
// Panels mit Titel in der eingestellten Sprache. Geöffnet wird mit `focus`: der erste Klick in ein Panel ohne Fokus holt erst
// den Fokus, der Host zeichnet dabei neu und tauscht den Button unter dem Zeiger aus; der Klick verpufft. Der Host gewährt den
// Fokus nur bei leerem Eingabefeld mit Tastatur, nicht aus einem Knopf in Band oder Panel heraus.
const panes = (lang: UiLang) => ({
  main: { id: PLUGIN, title: t(lang, 'common.paneMain') },
  summary: { id: 'tenvan-cc-summary', title: t(lang, 'common.paneSummary') },
  settings: { id: 'tenvan-cc-settings', title: t(lang, 'common.paneSettings') },
  mr: { id: 'tenvan-cc-mr', title: t(lang, 'common.paneMr') },
  history: { id: 'tenvan-cc-history', title: t(lang, 'common.paneHistory') },
  profiling: { id: 'tenvan-cc-profiling', title: t(lang, 'common.paneProfiling') },
})
const EDIT_TOOLS = new Set(['Edit', 'Write', 'NotebookEdit'])
const LIVE = new Set(['pending', 'running', 'waiting'])
const NL = String.fromCharCode(10)

// Oberflächen, auf denen das Cockpit zeichnet; überall sonst zeichnet die Engine (`next(e)`). Die Zeichnungen fragen nicht
// nach dem Namen der Oberfläche, sondern nach ihren Elementen: `'Svg' in ui` entscheidet zwischen Grafik (Desktop) und
// Text-Fallback (Terminal). Desktop führt, das Terminal bekommt dieselben Daten und Texte.
const DRAWS_ON: ReadonlySet<RenderSurface> = new Set(['desktop', 'terminal'])
const drawsOn = (surface: RenderSurface): boolean => DRAWS_ON.has(surface)

const usage = atom({ plugin: 'tenvan-cc', key: 'usage' } as const, null)
const lastTurnAt = atom({ plugin: 'tenvan-cc', key: 'lastTurnAt' } as const, null)
const turn = atom({ plugin: 'tenvan-cc', key: 'turn' } as const, null)
const agents = atom({ plugin: 'tenvan-cc', key: 'agents' } as const, [] as Agent[])
const GATE0: Gate = { allow: 0, you: 0, ask: 0, deny: 0, file: 0, shell: 0, other: 0, trail: [], asked: [] }
const gate = atom({ plugin: 'tenvan-cc', key: 'gate' } as const, GATE0)
// $.state überlebt einen Reload: Zähler aus älteren Versionen haben neue Felder nicht und werden hier aufgefüllt.
const gateOf = (g: Partial<Gate>): Gate => ({ ...GATE0, ...g })
const log = atom({ plugin: 'tenvan-cc', key: 'log' } as const, [] as LogEntry[])
// Jede Änderung zeichnet Band und Panel neu – hält Cache-Countdown und Laufzeiten aktuell.
const tick = atom({ plugin: 'tenvan-cc', key: 'tick' } as const, 0)
const steps = atom({ plugin: 'tenvan-cc', key: 'steps' } as const, [] as Step[])
const asks = atom({ plugin: 'tenvan-cc', key: 'asks' } as const, [] as Step[])
const cacheHit = atom({ plugin: 'tenvan-cc', key: 'cacheHit' } as const, null)
const about = atom({ plugin: 'tenvan-cc', key: 'about' } as const, null)
const bgTasks = atom({ plugin: 'tenvan-cc', key: 'bgTasks' } as const, [] as BgTask[])
const lastPrompt = atom({ plugin: 'tenvan-cc', key: 'lastPrompt' } as const, '')
// Letzte Antwort und ob gerade neue Folge-Schritte geholt werden: für den Knopf „↻“ hinter den Vorschlägen.
const lastAnswer = atom({ plugin: 'tenvan-cc', key: 'lastAnswer' } as const, '')
const stepsBusy = atom({ plugin: 'tenvan-cc', key: 'stepsBusy' } as const, false)
// Claudes eigener Vorschlag nach dem Turn (prompt.suggest, origin suggestion); gilt bis zum nächsten Prompt.
const claudeHint = atom({ plugin: 'tenvan-cc', key: 'claudeHint' } as const, null as Step | null)
// Animationsbild des Panels; läuft nur, solange ein Turn läuft.
const frame = atom({ plugin: 'tenvan-cc', key: 'frame' } as const, 0)
const breakdown = atom({ plugin: 'tenvan-cc', key: 'breakdown' } as const, null)
// Warum es keine Aufteilung gibt (Fehlertext oder Hinweis); leer, solange sie vorliegt.
const breakdownNote = atom({ plugin: 'tenvan-cc', key: 'breakdownNote' } as const, '')
const logo = atom({ plugin: 'tenvan-cc', key: 'logo' } as const, null)
const modelInfo = atom({ plugin: 'tenvan-cc', key: 'modelInfo' } as const, null)
const repo = atom({ plugin: 'tenvan-cc', key: 'repo' } as const, null)
const linked = atom({ plugin: 'tenvan-cc', key: 'linked' } as const, [] as LinkedSession[])
const settings = atom({ plugin: 'tenvan-cc', key: 'settings' } as const, DEFAULT_SETTINGS)
const catchUp = atom({ plugin: 'tenvan-cc', key: 'catchUp' } as const, CATCH0)
const expanded = atom({ plugin: 'tenvan-cc', key: 'expanded' } as const, [] as string[])
const touched = atom({ plugin: 'tenvan-cc', key: 'touched' } as const, [] as string[])
const quietIds = atom({ plugin: 'tenvan-cc', key: 'quietIds' } as const, [] as string[])
const collapsed = atom({ plugin: 'tenvan-cc', key: 'collapsed' } as const, [] as string[])
// Aufrufe des Haupt-Agents im laufenden Turn und die daraus erkannte Schleife; beides leert ein neuer Turn.
const marks = atom({ plugin: 'tenvan-cc', key: 'marks' } as const, [] as CallMark[])
const loopAlert = atom({ plugin: 'tenvan-cc', key: 'loopAlert' } as const, null as LoopAlert | null)
// Verzeichniswächter: Session-Ordner (root), Prozess-Ordner (cwd), Home; letzter Wechsel nach außerhalb im laufenden Turn.
const dirs = atom({ plugin: 'tenvan-cc', key: 'dirs' } as const, { cwd: '', root: '', home: '' })
const dirAlert = atom({ plugin: 'tenvan-cc', key: 'dirAlert' } as const, null as { dir: string; tool: string } | null)
const loopIgnored = atom({ plugin: 'tenvan-cc', key: 'loopIgnored' } as const, [] as string[])
const loopKey = (a: LoopAlert) => `${a.kind}:${a.target}`
// Geladene MCP-Schemas aus dem Kontext-Breakdown und die Server, deren Tools die Session schon aufgerufen hat (Kontext-Diät).
const mcpTools = atom({ plugin: 'tenvan-cc', key: 'mcpTools' } as const, [] as McpSchema[])
const usedMcp = atom({ plugin: 'tenvan-cc', key: 'usedMcp' } as const, [] as string[])
const turnLog = atom({ plugin: 'tenvan-cc', key: 'turnLog' } as const, [] as TurnRecord[])
// Höchstens so viele Zeilen Befehl und Ergebnis je Kachel einer Hintergrundaufgabe.
const TASK_LINES = 8
// Breite der Bezeichnungs-Spalte in der Übersicht („Kontext“ plus Luft).
const OVERVIEW_LABEL_COLS = 9
// Breite der Schalter-Spalte in den Einstellungen (Button „○ Aus“/„○ Off“ plus Luft).
const SWITCH_COLS = 9
const mrHost = atom({ plugin: 'tenvan-cc', key: 'mrHost' } as const, '')
const mrCur = atom({ plugin: 'tenvan-cc', key: 'mrCur' } as const, null as Mr | null)
const mrMine = atom({ plugin: 'tenvan-cc', key: 'mrMine' } as const, [] as MrMine[])
const mrDetails = atom({ plugin: 'tenvan-cc', key: 'mrDetails' } as const, {} as Record<string, Mr>)
const mrReview = atom({ plugin: 'tenvan-cc', key: 'mrReview' } as const, [] as MrMine[])
const mrAhead = atom({ plugin: 'tenvan-cc', key: 'mrAhead' } as const, 0)
const mrDraft = atom({ plugin: 'tenvan-cc', key: 'mrDraft' } as const, null as MrDraft | null)
const mrBusy = atom({ plugin: 'tenvan-cc', key: 'mrBusy' } as const, '')
// Knopf, dessen Aktion gerade läuft: zeichnet „◌ … “ statt seines Labels, bis die Aktion fertig ist.
const pressed = atom({ plugin: 'tenvan-cc', key: 'pressed' } as const, '')
const mrConfirm = atom({ plugin: 'tenvan-cc', key: 'mrConfirm' } as const, '')
// MRs (Schlüssel `merge-<iid>`), bei denen „Squash“ angehakt ist; Standard aus, nach dem Mergen wieder aus.
const mrSquash = atom({ plugin: 'tenvan-cc', key: 'mrSquash' } as const, [] as string[])
const devMock = atom({ plugin: 'tenvan-cc', key: 'devMock' } as const, false)
// Subagents, deren weitere Tool-Calls das Cockpit ablehnt (Agent-Bremse: die API kann einen Subagent nicht abbrechen).
const braked = atom({ plugin: 'tenvan-cc', key: 'braked' } as const, [] as string[])
const effortPick = atom({ plugin: 'tenvan-cc', key: 'effortPick' } as const, null as EffortLevel | null)
const effortBase = atom({ plugin: 'tenvan-cc', key: 'effortBase' } as const, null as string | number | null)
const handoffNote = atom({ plugin: 'tenvan-cc', key: 'handoffNote' } as const, '')
const effortNote = atom({ plugin: 'tenvan-cc', key: 'effortNote' } as const, '')
const effortShift = atom({ plugin: 'tenvan-cc', key: 'effortShift' } as const, null as { from: string | null; to: string; step: number } | null)
// Animation alt ─→ neu: EFFORT_STEPS Bilder im Abstand von EFFORT_FRAME_MS, danach bleibt „Haiku: neu“ stehen.
// Checkliste (Idee: glanceflow): aktueller Job der Session, angezeigter Verlaufstag (JJJJ-MM-TT) mit seinen Einträgen und
// die mittlere Schrittdauer aus dem Verlauf (für die Restzeit, solange der Job selbst noch keine zwei Schritte gemessen hat).
const glanceJob = atom({ plugin: 'tenvan-cc', key: 'job' } as const, null as Job | null)
const glanceDay = atom({ plugin: 'tenvan-cc', key: 'historyDay' } as const, '')
const glanceList = atom({ plugin: 'tenvan-cc', key: 'historyList' } as const, [] as JobRecord[])
const glanceAvg = atom({ plugin: 'tenvan-cc', key: 'stepAvg' } as const, null as number | null)

const EFFORT_STEPS = 10
const EFFORT_FRAME_MS = 140
let effortTimer: { cancel(): void } | undefined

const STEPS_SYSTEM =
  'Du schlägst die nächsten sinnvollen Schritte in einer Coding-Session vor. ' +
  'Antworte nur mit einem JSON-Array aus genau 3 Objekten {"label": string, "why": string, "prompt": string}, das wichtigste zuerst. ' +
  '"prompt": der nächste Prompt, den der Nutzer an Claude schickt, also immer ein Auftrag AN CLAUDE: ' +
  'Imperativ in der Du-Form an Claude gerichtet (z. B. "Führe die Tests aus", "Committe die Änderungen"), knapp (meist unter 15 Wörtern), ' +
  'konkret, ohne Höflichkeitsfloskeln und ohne Erklärung. Nie eine Anweisung an den Nutzer selbst ("Prüfe im Browser", "Starte die App neu"); ' +
  'braucht es seine Mitwirkung, formuliere den Auftrag an Claude, der sie vorbereitet oder abfragt. ' +
  '"label": Kurzfassung dieses Auftrags an Claude im selben Imperativ, höchstens 6 Wörter. ' +
  '"why": kurze Begründung aus Sicht des Nutzers, höchstens 10 Wörter. ' +
  'Schlage nur vor, was sich direkt aus dem letzten Austausch ergibt; nichts wiederholen, was schon erledigt ist. ' +
  'Sprache der Unterhaltung verwenden. Gibt es nichts Sinnvolles, antworte mit [].'


// Oberflächen, die Svg zeichnen. Die Engine ergänzt jede Elementtabelle um alle Namen; im Terminal steht deshalb auch ein
// `Svg` darin, das nur ein leeres Fragment zeichnet. `elementsOf` nimmt es dort heraus, damit `'Svg' in ui` stimmt.
const SVG_SURFACES: ReadonlySet<RenderSurface> = new Set(['desktop', 'vscode', 'mobile'])

// Die Elementtabelle für einen Render-Hook; jede Zeichnung holt sie hier statt direkt über `$.ui.resolve`.
function elementsOf($: EngineInterface, e: ResolveInput): Ui {
  const ui = $.ui.resolve(e)
  if (SVG_SURFACES.has(e.surface)) return ui
  const { Svg: _drawsNothing, ...rest } = ui as Ui & { Svg?: unknown }
  return rest as Ui
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
async function note($: EngineInterface, src: LogSrc, text: string, tool?: { tool: string; target: string; action?: string; isError: boolean }) {
  const at = await $.clock.now()
  await update($, log, list => pushCapped(list, { at, src, text, ...tool }, LOG_MAX))
}

// Tool-Name farbig: Server gedimmt, Tool in Akzentfarbe; Ziel in Gelb hinter gedimmtem „→“ (nur, wenn es eins gibt), Fehler rot.
function toolLabel({ Text }: Ui, name: string, target = '', isError = false, action = '') {
  const p = splitTool(name)
  return (
    <Text>
      {p.server !== null && <Text color={THEME.textMuted}>{p.server} › </Text>}
      <Text bold color={THEME.accent}>{p.tool}</Text>
      {action !== '' && <Text color={THEME.success}> ({action})</Text>}
      {target !== '' && <Text color={THEME.textMuted}> →</Text>}
      {target !== '' && <Text color={THEME.warning}> {target}</Text>}
      {isError && <Text color={THEME.error}> ✗</Text>}
    </Text>
  )
}

// Log-Eintrag: strukturierte Tool-Einträge farbig, alles andere als Text.
function entryView(ui: Ui, l: LogEntry, dim: boolean) {
  const { Text } = ui
  return l.tool !== undefined ? toolLabel(ui, l.tool, l.target ?? '', l.isError === true, l.action ?? '') : <Text dimColor={dim}>{l.text}</Text>
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

const ASKS_SYSTEM =
  'Du formulierst Rückfragen eines Coding-Assistenten (Claude) an den Nutzer in Aufträge um, die der Nutzer Claude als Antwort schickt. ' +
  'Antworte nur mit einem JSON-Array, je Frage in derselben Reihenfolge ein Objekt {"label": string, "prompt": string}. ' +
  '"prompt": die zustimmende Antwort als Auftrag AN CLAUDE im Imperativ, Du-Form (aus "Soll Claude die Tests fahren?" wird "Fahre die Tests"), ' +
  'knapp, ohne Höflichkeitsfloskeln. Bei einer Auswahlfrage ("A oder B?") die erstgenannte Option. ' +
  '"label": derselbe Auftrag in höchstens 6 Wörtern. Sprache der Fragen verwenden. Ist eine Frage keine Rückfrage an den Nutzer, lass sie weg.'

// Fragen aus der letzten Antwort als Aufträge; ohne Fragen kein Model-Call.
async function questionSteps($: EngineInterface, answer: string): Promise<Step[]> {
  const qs = questionsOf(answer)
  if (qs.length === 0) return []
  const r = await $.model.complete({
    model: 'haiku',
    system: ASKS_SYSTEM,
    prompt: `Fragen:\n${qs.map(q => `- ${q}`).join('\n')}\n\nAufträge (JSON-Array):`,
    maxTokens: 600,
    effort: 'low',
    timeoutMs: 15_000,
  })
  return r.isAnswered ? parseSteps(r.text) : []
}

// ponytail: ein Haiku-Call pro Antwort, kein Cache – abschaltbar über die Option nextSteps
async function suggest($: EngineInterface, asked: string, answer: string, avoid: readonly string[] = []): Promise<Step[]> {
  const r = await $.model.complete({
    model: 'haiku',
    system: STEPS_SYSTEM,
    prompt: `Nutzer fragte:\n${clip(asked, 3000)}\n\nAssistent antwortete:\n${clip(answer, 6000)}\n\n${avoid.length > 0 ? `Schon vorgeschlagen, wähle andere:\n${avoid.map(a => `- ${a}`).join('\n')}\n\n` : ''}Nächste Schritte (JSON-Array):`,
    maxTokens: 1000,
    effort: 'low',
    timeoutMs: 15_000,
  })
  return r.isAnswered ? parseSteps(r.text) : []
}

// Knopf „↻“: neue Folge-Schritte zum letzten Austausch, ohne die gerade gezeigten zu wiederholen.
async function refreshSteps($: EngineInterface) {
  const asked = await read($, lastPrompt)
  const answer = await read($, lastAnswer)
  if (await read($, stepsBusy)) return
  if (answer.trim() === '') {
    $.ui.toast(t((await read($, settings)).lang, 'band.noAnswerYet'))
    return
  }
  await update($, stepsBusy, () => true)
  try {
    const avoid = (await read($, steps)).filter(s => s.fromClaude !== true).map(s => s.label)
    const list = await suggest($, asked, answer, avoid)
    if ((await read($, lastPrompt)) !== asked) return
    if (list.length === 0) {
      $.ui.toast(t((await read($, settings)).lang, 'band.noNewSteps'))
      return
    }
    await update($, steps, () => list)
    await mergeHint($)
  } finally {
    await update($, stepsBusy, () => false)
  }
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
async function gitText($: EngineInterface, args: readonly string[], cwd?: string): Promise<string> {
  const r = await $.process.run(['git', '-c', 'core.fsmonitor=false', '--no-optional-locks', ...args], cwd === undefined ? undefined : { cwd })
  return r.exitCode === 0 ? r.stdout.trim() : ''
}

// Enthält die lokale Konfiguration Schlüssel, über die das Repo beim Lesen Befehle starten kann?
// Nur Exit 1 („kein Treffer“) gilt als unbedenklich; ein Treffer oder jeder Fehler (kaputte Konfiguration) als riskant.
// Ausnahme: der Nutzer hat das Repo freigegeben (Knopf „Entsperren“): sein Git-Ordner steht mit dem Hash der riskanten
// Schlüssel samt Werten in der globalen Git-Konfiguration (`cockpit.trustedRepo`, siehe `isTrusted`). Die Marke liegt
// bewusst nicht im Repo, und ein später geänderter Eintrag passt nicht mehr zum Hash: dann ist das Repo wieder gesperrt.
const GIT_BASE = ['git', '-c', 'core.fsmonitor=false', '--no-optional-locks'] as const

// Git-Ordner und Hash der riskanten Konfiguration; null, wenn etwas davon nicht zu lesen ist (dann gilt das Repo als gesperrt).
async function repoTrust($: EngineInterface, cwd?: string): Promise<{ common: string; hash: string } | null> {
  const opts = cwd === undefined ? undefined : { cwd }
  const run = (...args: string[]) => $.process.run([...GIT_BASE, ...args], opts).catch(() => null)
  const [common, local, worktree] = await Promise.all([
    run('rev-parse', '--path-format=absolute', '--git-common-dir'),
    run('config', '--local', '--includes', '--get-regexp', RISKY_GIT_CONFIG),
    run('config', '--worktree', '--includes', '--get-regexp', RISKY_GIT_CONFIG),
  ])
  if (common === null || common.exitCode !== 0 || common.stdout.trim() === '' || local === null || worktree === null) return null
  const text = trustText({ code: local.exitCode, out: local.stdout }, { code: worktree.exitCode, out: worktree.stdout })
  const hashed = await $.process.run([...GIT_BASE, 'hash-object', '--stdin'], { ...(opts ?? {}), stdin: text }).catch(() => null)
  const hash = hashed !== null && hashed.exitCode === 0 ? hashed.stdout.trim() : ''
  return hash === '' ? null : { common: common.stdout.trim(), hash }
}

async function isRiskyRepo($: EngineInterface, cwd?: string): Promise<boolean> {
  const opts = cwd === undefined ? undefined : { cwd }
  const r = await $.process.run([...GIT_BASE, 'config', '--local', '--includes', '--name-only', '--get-regexp', RISKY_GIT_CONFIG], opts)
  if (r.exitCode === 1) return false
  const [entries, fp] = await Promise.all([
    $.process.run([...GIT_BASE, 'config', '--global', '--no-includes', '--get-all', 'cockpit.trustedRepo'], opts).catch(() => null),
    repoTrust($, cwd),
  ])
  if (entries === null || entries.exitCode !== 0 || fp === null) return true
  return !isTrusted(entries.stdout, fp.common, fp.hash)
}

// Prompt für den Knopf „Entsperren“ mit dem fertigen Befehl samt Hash dieses Repos; null, wenn der Hash nicht zu bilden ist.
async function trustPrompt($: EngineInterface, lang: UiLang): Promise<string | null> {
  const cwd = await $.session.root().catch(() => undefined)
  const fp = await repoTrust($, cwd)
  return fp === null || !isSafeTrustEntry(fp.common, fp.hash) ? null : t(lang, 'repo.trustPrompt', { regex: RISKY_GIT_CONFIG, entry: trustEntry(fp.common, fp.hash) })
}

// Git-Stand des Arbeitsverzeichnisses; rein lesend, ohne Netzwerk. Außerhalb eines Repos null.
// Ein fremdes Repo kann über seine Konfiguration Befehle starten (Filter, fsmonitor, textconv). Nur `git status`
// kann das hier auslösen; es läuft deshalb nur, wenn die lokale Konfiguration keinen solchen Schlüssel enthält.
// Nur „kein Repository“ ergibt null. Jeder andere Fehler (Timeout, Sperre, Ordner kurz weg beim Worktree-Wechsel) ergibt
// undefined: der zuletzt bekannte Stand bleibt stehen, statt die Kachel bis zum nächsten Turn verschwinden zu lassen.
// Git läuft im Session-Ordner, nicht im Prozess-Ordner: ein `cd` nach Temp oder ~/.claude ließe das Repo sonst verschwinden.
async function loadRepo($: EngineInterface): Promise<Repo | null | undefined> {
  const cwd = await $.session.root().catch(() => undefined)
  const git = (...args: string[]) => gitText($, args, cwd)
  const probe = await $.process.run(['git', '-c', 'core.fsmonitor=false', '--no-optional-locks', 'rev-parse', '--show-toplevel'], cwd === undefined ? undefined : { cwd }).catch(() => null)
  if (probe === null) return undefined
  if (probe.exitCode !== 0) return /not a git repository/i.test(probe.stderr) ? null : undefined
  const top = probe.stdout.trim()
  const risky = await isRiskyRepo($, cwd)
  const [gitDir, commonDir, branch, sha, status, counts, prefix] = await Promise.all([
    git('rev-parse', '--absolute-git-dir'),
    git('rev-parse', '--path-format=absolute', '--git-common-dir'),
    git('branch', '--show-current'),
    git('rev-parse', '--short', 'HEAD'),
    risky ? Promise.resolve(null) : git('status', '--porcelain', '--ignore-submodules=all'),
    git('rev-list', '--left-right', '--count', '@{upstream}...HEAD'),
    git('rev-parse', '--show-prefix'),
  ])
  return top === '' ? undefined : parseRepo({ top, gitDir, commonDir, branch, sha, status, counts, prefix })
}

async function refreshDirs($: EngineInterface) {
  const [cwd, root, home] = await Promise.all([
    $.session.cwd(),
    $.session.root(),
    $.env.get('USERPROFILE').then(v => v ?? $.env.get('HOME')).then(v => v ?? ''),
  ])
  const next = { cwd, root, home }
  if (!same(await read($, dirs), next)) await update($, dirs, () => next)
  return next
}

// Ziel eines Aufrufs außerhalb des Session-Ordners, sonst null. Immer vor dem Lauf und mit frischen Ordnern aufrufen:
// ein `cd` verschiebt den Prozess-Ordner, gegen den relative Ziele aufgelöst werden.
async function foreignTarget($: EngineInterface, tool: string, input: Record<string, unknown>): Promise<string | null> {
  const d = await refreshDirs($)
  // Argumente liegen bei tool.call im Event, bei tool.check unter `input`.
  const args = (typeof input.input === 'object' && input.input !== null ? input.input : input) as Record<string, unknown>
  const dir = dirTarget(tool, args, d.cwd, d.home)
  return dir !== null && isForeignDir(dir, d.root, d.home) ? nativeDir(dir, d.root) : null
}

async function refreshRepo($: EngineInterface) {
  const r = await loadRepo($)
  if (r !== undefined && !same(await read($, repo), r)) await update($, repo, () => r)
}

// Info-Zeile unter einem gezeichneten Chat-Element: Tenvan-Bildmarke und die Angaben als gedimmter Text, immer sichtbar.
// Ohne Logo oder Svg steht „ⓘ“ davor. Umhüllt nur, was schon gezeichnet ist; das Element selbst bleibt unberührt.
function withInfo(ui: Ui, key: string, lines: readonly string[], theirs: RenderElement, mark: string | null): RenderElement {
  const { Box, Text } = ui
  const Svg = 'Svg' in ui ? ui.Svg : undefined
  return (
    <Box key={key} flexDirection="column">
      {theirs}
      <Box flexDirection="row" columnGap={1} alignItems="center">
        {mark !== null && Svg ? <Svg source={mark} alt="Tenvan" width={11} height={14} /> : <Text color={THEME.textMuted}>ⓘ</Text>}
        <Text color={THEME.textMuted} wrap="truncate-end">{lines.join(' · ')}</Text>
      </Box>
    </Box>
  )
}

// ── Chat-Skins: Zeichnung zu den reinen Funktionen in skins.ts ──
// Alle Svg hier sind einfache Bilder (nie interaktiv) mit stabiler Quelle; keine absoluten Boxen; Buttons nur mit Label.
// Ohne Svg (Terminal) fällt jede Grafik auf Text oder auf die Zeichnung der Engine zurück.

function toolIcon(ui: Ui, tool: string, errored: boolean, key: string) {
  const Svg = 'Svg' in ui ? ui.Svg : undefined
  if (!Svg) return null
  const kind = iconKind(tool)
  return <Svg key={key} source={toolIconSvg(kind, errored ? THEME.error : ICON_COLOR[kind])} alt={tool} width={16} height={16} />
}

// Kennzeichen „hier greift ein Cockpit-Style“: Tenvan-Bildmarke und/oder kurzer Style-Name, beides abschaltbar.
type Marker = { mark: string | null; label: boolean; lang: UiLang }
type StyleKey = Extract<Key, `style.${string}`>

async function markerOf($: EngineInterface): Promise<Marker> {
  const s = await read($, settings)
  return { mark: s.skinMarkIcon ? await read($, logo) : null, label: s.skinMarkLabel, lang: s.lang }
}

function skinMark(ui: Ui, mk: Marker, key: string, style: StyleKey) {
  const { Box, Text } = ui
  const Svg = 'Svg' in ui ? ui.Svg : undefined
  const icon = mk.mark !== null && Svg ? <Svg source={mk.mark} alt="Tenvan" width={9} height={11} /> : null
  if (icon === null && !mk.label) return null
  return (
    <Box key={key} flexDirection="row" columnGap={1} alignItems="center">
      {icon}
      {mk.label && <Text color={THEME.textMuted} italic>{t(mk.lang, style)}</Text>}
      {NEW_STYLES.has(style) && chip(ui, 'skin', `${key}-new`, t(mk.lang, 'skin.newStyle'), THEME.accent)}
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
          {pill(ui, 'skin', `${key}-copy`, label, copy)}
        </Box>
      )}
    </Box>
  )
}

// Mermaid-Karten nur mit Svg: ohne Grafik bliebe die Karte der Markdown-Block der Engine, die Übernahme brächte nichts.
const splitOptions = (s: ModSettings, hasSvg: boolean): SplitOptions => ({ cards: s.skinCards, inline: s.skinInline, mermaid: s.skinMermaid && hasSvg, math: s.skinMath })

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
        if (s.kind === 'rich') {
          // Hängender Einzug je Listenpunkt; Code als Chip, Fett fett, übriger Text mit den Farbrollen der Hervorhebungen.
          const runs = (parts: RichPart[]) =>
            parts.map(p =>
              p.code ? (
                <Text color={CODE_FG} backgroundColor={CODE_BG}> {p.text} </Text>
              ) : (
                <Text bold={p.bold === true}>{inlineTokens(p.text, changed).map(x => (x.tone === undefined ? x.text : <Text color={TONE_COLOR[x.tone]}>{x.text}</Text>))}</Text>
              ),
            )
          return (
            <Box key={k} flexDirection="column">
              {s.lines.map((l, j) =>
                l.bullet === undefined ? (
                  <Text key={`${k}-${j}`} wrap="wrap">{runs(l.parts)}</Text>
                ) : (
                  <Box key={`${k}-${j}`} flexDirection="row" paddingLeft={2 + l.indent * 2}>
                    <Box width={l.bullet.length + 1} flexShrink={0}>
                      <Text>{l.bullet}</Text>
                    </Box>
                    <Text wrap="wrap">{runs(l.parts)}</Text>
                  </Box>
                ),
              )}
            </Box>
          )
        }
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
          // Umgesetzte Formel als Svg-Bild, ohne Svg derselbe Unicode-Text; nur der Rest bleibt LaTeX-Quelltext.
          const ok = s.text !== null
          const body = !ok ? <Code source={s.tex} language="latex" /> : Svg ? <Svg key={`${k}-svg`} source={mathSvg(s.text ?? '')} alt={s.text ?? ''} /> : <Text key={`${k}-txt`} wrap="wrap">{s.text}</Text>
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
async function paneHead($: EngineInterface, ui: Ui, lang: UiLang, extra: RenderElement | null = null) {
  const { Box } = ui
  // Liest bewusst nicht `usage`: das ändert sich bei jeder Messung, und jedes Neuzeichnen kostet einen Klick (siehe `same`).
  const mi = await read($, modelInfo)
  return (
    <Box flexDirection="column" marginBottom={1}>
      {titleRow(ui, await read($, about), await read($, logo))}
      {modelRow(ui, mi, mi?.window, lang)}
      {extra}
    </Box>
  )
}

// Titelzeile: Logo links, Text daneben; ohne Logo oder Svg bleibt „Tenvan CC …“ als Text.
function titleRow(ui: Ui, info: About | null, mark: string | null) {
  const { Box, Text } = ui
  const Svg = 'Svg' in ui ? ui.Svg : undefined
  if (mark === null || !Svg) return <Text bold color={THEME.accent}>{title(info)}</Text>
  return (
    <Box flexDirection="row" columnGap={1} alignItems="center">
      <Svg source={mark} alt="Tenvan" width={16} height={20} />
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
let settingsDefaults: Partial<ModSettings> = {}
async function loadSettings($: EngineInterface, defaults: Partial<ModSettings>) {
  settingsDefaults = defaults
  // Ohne lesbaren Speicher gelten die Standardwerte; der Start des Mods darf daran nicht scheitern.
  const saved = await $.store.get('settings').then(v => v as Partial<ModSettings> | undefined, () => undefined)
  await update($, settings, () => settingsOf({ ...defaults, ...saved }))
}

// Abgleich mit dem Speicher: Änderungen aus einer anderen Session kommen so an. Ein Lesefehler lässt den Stand unberührt
// (anders als beim Start, wo Standardwerte gelten); gezeichnet wird nur bei echter Änderung.
async function syncSettings($: EngineInterface): Promise<boolean> {
  const saved = (await $.store.get('settings')) as Partial<ModSettings> | undefined
  const next = settingsOf({ ...settingsDefaults, ...saved })
  const folded = foldedOf(await $.store.get('collapsed'))
  let changed = false
  if (!same(await read($, settings), next)) {
    await update($, settings, () => next)
    changed = true
  }
  if (!same(await read($, collapsed), folded)) {
    await update($, collapsed, () => folded)
    changed = true
  }
  return changed
}

// Klapp-Status der Panel-Karten: je User im Speicher, nicht je Session.
const foldedOf = (v: unknown): string[] => (Array.isArray(v) ? v.filter((x): x is string => typeof x === 'string') : [])

// Mehrere Karten auf einmal zu- oder aufklappen (Einstellungsseite: „Alle einklappen“/„Alle ausklappen“).
async function foldCards($: EngineInterface, keys: readonly string[], fold: boolean) {
  await update($, collapsed, c => (fold ? [...new Set([...c, ...keys])] : c.filter(x => !keys.includes(x))))
  await $.store.set('collapsed', await read($, collapsed))
}

// Klick-Rückmeldung: der Knopf zeigt „◌ …“, solange seine Aktion läuft, mindestens 600 ms (sonst sieht man bei schnellen
// Aktionen wie „ins Eingabefeld“ nichts). Ein zweiter Klick auf denselben Knopf während des Laufs wird ignoriert.
async function act($: EngineInterface, key: string, run: () => Promise<unknown>) {
  if ((await read($, pressed)) === key) return
  const start = await $.clock.now()
  await update($, pressed, () => key)
  try {
    await run()
  } finally {
    const left = 600 - ((await $.clock.now()) - start)
    const clear = () => void update($, pressed, k => (k === key ? '' : k)).catch(() => undefined)
    if (left > 0) $.clock.after(left, clear)
    else clear()
  }
}


async function toggleCard($: EngineInterface, key: string) {
  await update($, collapsed, c => (c.includes(key) ? c.filter(x => x !== key) : [...c, key]))
  await $.store.set('collapsed', await read($, collapsed))
}

// Knopf „Neu laden“: gleicht ab und sagt, was dabei herauskam.
async function reloadSettings($: EngineInterface) {
  const L = (await read($, settings)).lang
  try {
    $.ui.toast(t(L, (await syncSettings($)) ? 'settings.reloaded' : 'settings.upToDate'))
  } catch (err) {
    $.ui.toast(t(L, 'settings.reloadFailed', { why: err instanceof Error ? err.message : String(err) }))
  }
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

// Aufruf, der im Hintergrund weiterläuft (Shell, Monitor, Workflow, Cloud-Agent): als laufende Aufgabe merken.
// TaskStop beendet sie sofort als „killed“.
async function trackBackground($: EngineInterface, e: { tool: string; [k: string]: unknown }, ran: unknown) {
  const now = await $.clock.now()
  const bg = backgroundOf(e.tool, e, (ran as { result?: unknown }).result)
  if (bg !== null) await update($, bgTasks, list => pushCapped(list.filter(x => x.id !== bg.id), { ...bg, status: 'running', since: now }, 20))
  const stop = e.tool === 'TaskStop' || e.tool === 'KillShell' ? (typeof e.task_id === 'string' ? e.task_id : typeof e.shell_id === 'string' ? e.shell_id : '') : ''
  if (stop !== '') await update($, bgTasks, list => list.map(x => (x.id === stop && x.status === 'running' ? { ...x, status: 'killed', durationMs: now - x.since } : x)))
}

// Benachrichtigung der Engine über das Ende einer Hintergrundaufgabe: Status, Dauer und Kurztext übernehmen.
async function finishBackground($: EngineInterface, text: string) {
  const ns = parseTaskNotifications(text)
  if (ns.length === 0) return
  const now = await $.clock.now()
  const byId = new Map(ns.map(n => [n.id, n]))
  await update($, bgTasks, list =>
    list.map(x => {
      const n = byId.get(x.id)
      return n === undefined ? x : { ...x, status: n.status, durationMs: x.durationMs ?? now - x.since, summary: n.summary }
    }),
  )
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

// Auto-Effort: Haiku bewertet den Prompt vor dem Senden (blockiert ihn rund 1 s); triviale Prompts behalten die letzte Stufe.
async function judgeEffort($: EngineInterface, text: string) {
  const s = await read($, settings)
  if (!s.autoEffort) return
  if (isTrivialPrompt(text)) {
    await update($, effortNote, () => t(s.lang, 'effort.kept'))
    return
  }
  await update($, effortNote, () => t(s.lang, 'effort.judging'))
  const prev = (await read($, effortPick)) ?? (await read($, effortBase))
  const level = await $.model.classify(effortJudgeText(text), EFFORT_LEVELS).catch(() => undefined)
  if (level === 'low' || level === 'medium' || level === 'high') {
    await update($, effortPick, () => level)
    await animateEffort($, prev === null ? null : String(prev), level)
    await update($, effortNote, () => t(s.lang, 'effort.judged', { v: level }))
    await note($, 'main', t(s.lang, 'effort.judged', { v: level }))
  } else await update($, effortNote, () => t(s.lang, 'effort.failed'))
}

// Kurzer Übergang im Band, damit der Wechsel auffällt; ein neuer Wechsel ersetzt einen laufenden.
async function animateEffort($: EngineInterface, from: string | null, to: string) {
  effortTimer?.cancel()
  await update($, effortShift, () => ({ from, to, step: 0 }))
  effortTimer = $.clock.every(EFFORT_FRAME_MS, async () => {
    const cur = await read($, effortShift)
    if (cur === null || cur.step >= EFFORT_STEPS) {
      effortTimer?.cancel()
      effortTimer = undefined
      return
    }
    await update($, effortShift, x => (x === null ? x : { ...x, step: x.step + 1 }))
  })
}

// Stufe für einen Haupt-Request oder undefined (nichts umschreiben). Weicht der Effort der Session vom zuletzt gesehenen ab,
// hat die Person ihn von Hand geändert (oder ein anderes Mod): Auto-Effort geht aus.
async function effortFor($: EngineInterface, base: string | number | undefined): Promise<EffortLevel | undefined> {
  const s = await read($, settings)
  if (!s.autoEffort || base === undefined) return undefined
  const known = await read($, effortBase)
  if (known !== null && known !== base) {
    await update($, effortBase, () => null)
    await setSettings($, { autoEffort: false })
    await note($, 'main', t(s.lang, 'effort.manualOff'))
    return undefined
  }
  if (known === null) await update($, effortBase, () => base)
  const pick = await read($, effortPick)
  return pick === null ? undefined : effectiveEffort(pick, s.saveMode, (await read($, usage))?.limits ?? [])
}

// Handoff: HANDOFF.md im Arbeitsverzeichnis plus Zwischenablage. Zusammenfassung aus catch-me-up, wenn vorhanden (kein Call),
// sonst ein Fork über das eigene Transcript.
async function writeHandoff($: EngineInterface) {
  const s = await read($, settings)
  await update($, handoffNote, () => t(s.lang, 'handoff.busy'))
  try {
    let summary = (await read($, catchUp)).summary
    if (summary.trim() === '') {
      const r = await $.model.fork({ prompt: t(s.lang, 'handoff.prompt') })
      if (!r.isAnswered) throw new Error(r.reason)
      summary = r.text
    }
    const text = handoffMarkdown({ summary, lastPrompt: await read($, lastPrompt), touched: await read($, touched), steps: await read($, steps), git: await catchGitState($) }, s.lang)
    await $.fs.write(`${await $.session.cwd()}/HANDOFF.md`, text)
    await $.ui.copy({ text })
    await update($, handoffNote, () => t(s.lang, 'handoff.done'))
  } catch (err) {
    await update($, handoffNote, () => t(s.lang, 'handoff.failed', { why: err instanceof Error ? err.message : String(err) }))
  }
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

// ── Checkliste (Idee: glanceflow, kein Code übernommen) ──
// Rein beobachtend: liest Prompt, Tool-Aufrufe und Turn-Ende mit; kein Prompt-Einschub, keine eigenen Tools, kein deny.
// Pause bricht nur den laufenden Turn ab, „Weiter“ sendet einen sichtbaren Auftrag wie getippt.
// Der Job selbst liegt im Atom und übersteht einen Reload; „Weiter“ funktioniert danach normal, weil sein Merker erst beim
// Klick entsteht.
// ponytail: die turnId für „Pause“ liegt in einer Modul-Variablen; nach einem Reload mitten im Turn meldet Pause bis zum
// nächsten Turn „nicht möglich“ (Esc geht immer). Ausbau: turnId ins Atom `job`
let mainTurnId: string | null = null
let pauseAsked = false
let lastContinue = ''
// Laufende Tool-Aufrufe der Hauptschleife: solange einer läuft, ist Claude nicht „hängt?“.
let toolsInFlight = 0

const updJob = ($: EngineInterface, fn: (j: Job) => Job) => update($, glanceJob, j => (j === null ? null : fn(j)))

// Projekt des Jobs: Session-Ordner, sonst Prozess-Ordner; ohne beides (Test, frischer Start) der Session-Ordner der Engine.
async function projectCwd($: EngineInterface): Promise<string> {
  const d = await read($, dirs)
  if (d.root !== '') return d.root
  if (d.cwd !== '') return d.cwd
  try {
    return await $.session.root()
  } catch {
    return ''
  }
}

// Erste Frage einer AskUserQuestion (Argumente liegen bei tool.call im Event).
function askText(args: Record<string, unknown>): string {
  const qs = Array.isArray(args.questions) ? args.questions : []
  const q = (qs[0] ?? {}) as Record<string, unknown>
  return typeof q.question === 'string' ? oneLine(q.question, 160) : ''
}

// Neuer Prompt: neuer Job. Kommt genau der Auftrag zurück, den „Weiter“ gesendet hat, läuft der alte Job weiter.
async function startJob($: EngineInterface, text: string) {
  const now = await $.clock.now()
  pauseAsked = false
  // Prompt aus einem Slash-Befehl (Skill, Command mit Prompt): gehört zum Befehls-Job, der Titel bleibt „/name“.
  const cur = await read($, glanceJob)
  if (cur !== null && cur.cmd === true && cur.endedAt === undefined && now - cur.startedAt < CMD_ADOPT_MS) return
  const resumed = lastContinue !== '' && text.trim() === lastContinue.trim()
  lastContinue = ''
  const old = await read($, glanceJob)
  if (resumed && old !== null) {
    // Ohne endedAt gilt der Job wieder als offen; finishGlance schließt ihn am Ende dieses Turns erneut.
    await updJob($, ({ endedAt: _ended, ...j }) => ({ ...j, phase: 'working', question: '', ask: '', signAt: now }))
    return
  }
  // Ein noch offener Job (neuer Prompt mitten im Turn, verlorenes Turn-Ende) landet als gestoppt im Verlauf.
  if (old !== null && old.endedAt === undefined) void saveJob($, { ...old, phase: 'stopped', endedAt: now }, now).catch(() => undefined)
  const cwd = await projectCwd($)
  await update($, glanceJob, () => newJob(String(now), text, cwd, now))
}

// Slash-Befehl als eigener Job „/name args“. Druckt der Befehl nur Text (oder öffnet ein Panel), ist er gleich fertig;
// startet er einen Turn, übernimmt startJob den Prompt und turn.complete schließt den Job.
const CMD_ADOPT_MS = 10_000
let cmdTurnStarted = false
async function startCommand($: EngineInterface, command: string, args: string) {
  await startJob($, `/${command}${args.trim() === '' ? '' : ` ${args.trim()}`}`)
  cmdTurnStarted = false
  await updJob($, j => ({ ...j, cmd: true }))
}
async function endCommand($: EngineInterface) {
  if (cmdTurnStarted) return
  const now = await $.clock.now()
  await updJob($, j => (j.cmd === true && j.endedAt === undefined ? { ...j, phase: 'done', endedAt: now } : j))
}

// Lebenszeichen (Model-Request, Subagent): nur ein laufender Job braucht es, sonst kein Neuzeichnen.
async function touchJob($: EngineInterface) {
  // Ein Lebenszeichen (Subagent startet oder arbeitet, Model-Request) heißt: eine offene Rückfrage ist entschieden. Ohne das
  // bliebe ein „ask“, das der Modus (auto) selbst freigibt, bis zum Ende des Tools offen, beim Agent-Tool minutenlang.
  // ponytail: ein langes Shell-Tool ohne Lebenszeichen bleibt bis zu seinem Ende „wartet auf Freigabe“; Ausbau, sobald die
  // Engine ein Ereignis für die Entscheidung einer Rückfrage liefert.
  for (const id of gateOf(await read($, gate)).asked) await settleAsk($, id, false)
  const j = await read($, glanceJob)
  if (j === null || j.phase !== 'working') return
  const now = await $.clock.now()
  await update($, glanceJob, () => touch(j, now))
}

// Ende des Haupt-Turns: Phase setzen, Verlauf schreiben, bei Rückfrage oder langem Job Ton/Hinweis.
async function finishGlance($: EngineInterface, answer: string, isAborted: boolean) {
  mainTurnId = null
  const paused = pauseAsked
  pauseAsked = false
  const j = await read($, glanceJob)
  // Ein Turn ohne eigenen Prompt (Benachrichtigung eines Hintergrund-Tasks) schließt einen abgeschlossenen Job nicht noch
  // einmal: sonst würde aus „braucht dich“ ein „fertig“, mit zweitem Ton und zweitem Verlaufseintrag.
  if (j === null || j.endedAt !== undefined) return
  const now = await $.clock.now()
  const done = finishJob(j, answer, isAborted, paused, now)
  await update($, glanceJob, () => done)
  void saveJob($, done, now).catch(() => undefined)
}

// Verlauf: ein Eintrag je Job im Tagesschlüssel, Tage älter als 30 Tage löschen, danach die mittlere Schrittdauer neu.
async function saveJob($: EngineInterface, j: Job, now: number) {
  if (!(await read($, settings)).glanceHistory || !isWorthShowing(j)) return
  const day = dayKey(now)
  const key = storeKey(day)
  const list = addRecord(recordsOf(await $.store.get(key)), toRecord(j, now))
  await $.store.set(key, list)
  if ((await read($, glanceDay)) === day) await update($, glanceList, () => list)
  for (const k of staleKeys(await $.store.keys(), day)) await $.store.delete(k)
  await loadAvg($, j.cwd, now, list)
}

// Mittlere Schrittdauer dieses Projekts aus den letzten sieben Tagen (heute ggf. schon geladen).
async function loadAvg($: EngineInterface, cwd: string, now: number, today?: JobRecord[]) {
  const days = Array.from({ length: 7 }, (_, i) => shiftDay(dayKey(now), -i))
  const lists = await Promise.all(days.map((d, i) => (i === 0 && today !== undefined ? today : $.store.get(storeKey(d)).then(recordsOf))))
  const avg = avgStepMs(lists.flat(), cwd)
  if ((await read($, glanceAvg)) !== avg) await update($, glanceAvg, () => avg)
}

// Verlaufstag ins Panel laden.
async function loadHistory($: EngineInterface, day: string) {
  await update($, glanceDay, () => day)
  const list = recordsOf(await $.store.get(storeKey(day)))
  // Nur übernehmen, wenn inzwischen kein anderer Tag gewählt wurde.
  if ((await read($, glanceDay)) === day) await update($, glanceList, () => list)
}

async function openHistory($: EngineInterface) {
  const L = await langOf($)
  await loadHistory($, dayKey(await $.clock.now())).catch(() => undefined)
  await $.ui.open({ ...panes(L).history, focus: true })
}

async function copyReport($: EngineInterface, kind: 'day' | 'week', surface: RenderSurface) {
  const L = await langOf($)
  const day = (await read($, glanceDay)) || dayKey(await $.clock.now())
  let text: string
  if (kind === 'day') text = dayReport(recordsOf(await $.store.get(storeKey(day))), day, await projectCwd($), L)
  else {
    const days = Array.from({ length: 7 }, (_, i) => shiftDay(day, i - 6))
    const lists = await Promise.all(days.map(d => $.store.get(storeKey(d)).then(recordsOf)))
    text = weekReport(Object.fromEntries(days.map((d, i) => [d, lists[i] ?? []])), day, L)
  }
  await $.ui.copy({ text, surface })
  $.ui.toast(t(L, 'glance.copied'))
}

// Pause: laufenden Haupt-Turn abbrechen; turn.complete wertet den Abbruch dann als „pausiert“.
async function pauseJob($: EngineInterface) {
  const L = await langOf($)
  const id = mainTurnId
  if (id === null) return $.ui.toast(t(L, 'glance.pauseFail'))
  pauseAsked = true
  try {
    await $.turn.abort({ turnId: id })
    $.ui.toast(t(L, 'glance.pausedToast'))
  } catch {
    pauseAsked = false
    $.ui.toast(t(L, 'glance.pauseFail'))
  }
}

// Weiter: Auftrag mit den offenen Schritten wie getippt senden (aus einem Knopf erst nach dem Klick, sonst wartet der Prompt
// auf den eigenen Hook); klappt das nicht, steht er im Eingabefeld.
async function continueJob($: EngineInterface) {
  const j = await read($, glanceJob)
  if (j === null) return
  const L = await langOf($)
  const text = continuePrompt(j, L)
  lastContinue = text
  $.clock.after(50, () =>
    void $.prompt
      .submit({ text, asUser: true })
      .catch(() => $.prompt.fill({ text }).then(() => $.ui.toast(t(L, 'glance.continueFail'))))
      .catch(() => undefined),
  )
}

// Phase, wie Band und Plan-Karte sie zeigen: eine offene Freigabe ist „braucht dich“, ein laufendes Tool nie „hängt?“.
const glancePhase = (j: Job, now: number, asking: number): ShownPhase =>
  asking > 0 && j.phase === 'working' ? 'needsYou' : shownPhase(j, now, toolsInFlight > 0)

// Farbe und Wort je Phase, wie das Band sie zeigt.
const PHASE_COLOR: Record<ShownPhase, string> = { working: THEME.accent, needsYou: THEME.warning, stuck: THEME.warning, done: THEME.success, stopped: THEME.textMuted, paused: THEME.textMuted }
const actText = (j: Job, L: UiLang): string => (j.act === null ? '' : `${t(L, `glance.act.${j.act.kind}`)}${j.act.n > 1 ? ` (${j.act.n})` : ''}`)
const stepMs = (s: JobStep, now: number): number | null => (s.startedAt === undefined ? null : (s.doneAt ?? now) - s.startedAt)

// Checkliste ganz oben im Band: Kopfzeile mit Status, Titel, Fortschritt, Restzeit und Knöpfen, darunter die Schritte.
// Nur Text (im Terminal identisch); auf dem Desktop zusätzlich ein kleiner Fortschrittsring.
function glanceBand($: EngineInterface, ui: Ui, j: Job, o: { L: UiLang; now: number; avg: number | null; asking: number; live: readonly Agent[] }): RenderElement {
  const { Box, Text } = ui
  const Svg = 'Svg' in ui ? ui.Svg : undefined
  const { L, now } = o
  const shown = glancePhase(j, now, o.asking)
  // Offene Freigabe: „braucht dich“ ohne Weiter (Claude arbeitet ja noch), mit dem Grund als Fragezeile.
  const waiting = o.asking > 0 && j.phase === 'working'
  const { done, total } = progressOf(j)
  const eta = etaOf(j, now, o.avg)
  const open = j.steps.some(s => s.status !== 'done')
  const canContinue = !waiting && (shown === 'paused' || shown === 'stopped' || shown === 'stuck' || (shown === 'needsYou' && open))
  const muted = THEME.textMuted
  const when =
    eta !== null ? t(L, eta.longer ? 'glance.longer' : 'glance.eta', { d: duration(eta.ms) })
    : j.endedAt !== undefined && shown !== 'working' && shown !== 'stuck' ? t(L, 'glance.took', { d: duration(j.endedAt - j.startedAt) })
    : ''
  // Helfer gebündelt: gleiche Art und gleiches Modell als „3× planner · Opus 5.5“, Plugin-Präfix weg; spart Zeilen,
  // denn der Desktop begrenzt die Höhe des Bands.
  const helpers = (() => {
    const groups = new Map<string, number>()
    for (const a of o.live) {
      const name = `${a.type.replace(/^[^:]+:/, '')}${a.model !== undefined ? ` · ${modelLabel(a.model)}` : ''}`
      groups.set(name, (groups.get(name) ?? 0) + 1)
    }
    return [...groups].map(([name, n]) => (n > 1 ? `${n}× ${name}` : name)).join(', ')
  })()
  // Eine Zeile unter dem laufenden Schritt: was Claude tut und welche Helfer laufen.
  const nowLine = (key: string) => {
    const now = j.act !== null && j.phase === 'working' ? t(L, 'glance.now', { what: actText(j, L) }) : ''
    const text = [now, helpers === '' ? '' : `↳ ${helpers}`].filter(x => x !== '').join(' · ')
    return text === '' ? null : <Text key={key} color={muted} wrap="truncate-end">{'    '}{text}</Text>
  }
  const hasActive = j.steps.some(s => s.status === 'active')
  return (
    <Box key="glance" flexDirection="column">
      <Box key="glance-head" flexDirection="row" justifyContent="space-between" alignItems="center" columnGap={2}>
        <Box flexDirection="row" columnGap={1} alignItems="center" flexShrink={1}>
          {Svg && total > 0 && <Svg key="glance-ring" source={ringSvg((100 * done) / total, PHASE_COLOR[shown], 14)} alt={t(L, 'glance.progress', { done, total })} width={14} height={14} />}
          <Text key="glance-phase" bold color={PHASE_COLOR[shown]}>{t(L, `glance.${shown}`)}</Text>
          <Box flexShrink={1}><Text key="glance-title" bold wrap="truncate-end">{j.title}</Text></Box>
          <Text key="glance-progress" color={muted}>{total > 0 ? `· ${t(L, 'glance.progress', { done, total })}` : `· ${t(L, 'glance.noSteps')}`}{when !== '' ? ` · ${when}` : ''}</Text>
        </Box>
        <Box flexDirection="row" columnGap={1} alignItems="center" flexShrink={0}>
          {pill(ui, 'band', 'glance-history', t(L, 'glance.history'), () => void openHistory($).catch(() => undefined))}
          {shown === 'working' && pill(ui, 'band', 'glance-pause', t(L, 'glance.pause'), () => void pauseJob($).catch(() => undefined))}
          {canContinue && pill(ui, 'band', 'glance-continue', t(L, 'glance.continue'), () => void continueJob($).catch(() => undefined))}
        </Box>
      </Box>
      {shown === 'stuck' && <Text key="glance-stuck" color={THEME.warning} wrap="truncate-end">{t(L, 'glance.stuckNote', { d: duration(now - j.signAt) })}</Text>}
      {waiting && <Text key="glance-perm" color={THEME.warning} wrap="truncate-end">? {t(L, 'glance.permission')}</Text>}
      {!waiting && shown === 'needsYou' && j.question !== '' && <Text key="glance-question" color={THEME.warning} wrap="truncate-end">? {j.question}</Text>}
      {visibleSteps(j.steps).map(r => {
        if (r.kind === 'fold') return <Text key={`glance-fold-${r.isDone ? 'done' : 'more'}`} color={r.isDone ? THEME.success : muted} dimColor={r.isDone}>{t(L, r.isDone ? 'glance.foldDone' : 'glance.foldMore', { n: r.n })}</Text>
        const s = r.step
        const ms = stepMs(s, now)
        if (s.status === 'done') return <Text key={`glance-s-${r.index}`} wrap="truncate-end"><Text color={THEME.success}>✓ {s.name}</Text>{ms !== null && <Text color={muted}> {duration(ms)}</Text>}</Text>
        if (s.status === 'upcoming') return <Text key={`glance-s-${r.index}`} color={muted} wrap="truncate-end">○ {s.name}</Text>
        return (
          <Box key={`glance-s-${r.index}`} flexDirection="column">
            <Text wrap="truncate-end"><Text bold color={THEME.accent}>▸ {s.name}</Text>{ms !== null && <Text color={muted}> {duration(ms)}</Text>}</Text>
            {nowLine(`glance-now-${r.index}`)}
          </Box>
        )
      })}
      {/* Ohne laufenden Schritt (kein Plan oder alles abgehakt) stehen Tätigkeit und Helfer unter der Kopfzeile. */}
      {!hasActive && nowLine('glance-now')}
    </Box>
  )
}

// ── Merge/Pull Requests (GitLab über glab, GitHub über gh, Bitbucket Server über REST; Ein-/Ausgabe in forge.ts) ──
// Lesend im Takt; schreibend nur per Klick.
let mrPolledAt = 0
let mrInflight: Promise<void> | null = null
// Letzte Zeichnung des MR-Panels und letzter Abruf der MR-Status: Status laden nur, solange das Panel offen ist (es zeichnet im
// 30-s-Takt neu), höchstens alle 5 Minuten.
let mrPaneAt = 0
let mrDetailAt = 0
const MINE_DETAIL_MAX = 10
let mrRemote: Remote | null = null
let authWarned = ''

// Remote des aktuellen Branches (git config branch.<b>.remote), sonst „origin“, sonst der erste Remote.
// Nicht jedes Repo nennt ihn „origin“ (ErpApi heißt etwa „ErpApi“); mit fest „origin“ blieb GitLab unerkannt.
async function gitRemote($: EngineInterface): Promise<string> {
  const branch = await gitText($, ['branch', '--show-current'])
  const own = branch === '' ? '' : await gitText($, ['config', '--get', `branch.${branch}.remote`])
  if (own !== '' && own !== '.') return own
  const all = (await gitText($, ['remote'])).split(/\r?\n/).filter(Boolean)
  return all.includes('origin') ? 'origin' : (all[0] ?? 'origin')
}

// Ziel-Branch: Standard-Branch der Plattform, sonst <remote>/HEAD, sonst „main“.
async function defaultTarget($: EngineInterface, remote: string): Promise<string> {
  const own = mrRemote === null ? '' : await forgeDefault(forgeIo($), mrRemote)
  if (own !== '') return own
  const head = await gitText($, ['symbolic-ref', '--short', `refs/remotes/${remote}/HEAD`])
  return head !== '' ? head.slice(remote.length + 1) : 'main'
}

// Prozess und HTTP für forge.ts (dort gibt es kein `$`).
function forgeIo($: EngineInterface): Io {
  return { run: (argv, opts) => $.process.run(argv, opts), fetch: (url, init) => $.http.fetch(url, init) }
}

// MR/PR des Branches abfragen; im Leerlauf jede Minute, bei laufender Pipeline alle 15 s. `force` wartet eine laufende
// Abfrage ab und fragt danach selbst sofort neu; Fehler wirft nur `force`, der Takt bleibt still beim letzten Stand.
async function refreshMr($: EngineInterface, force = false) {
  if (force && mrInflight !== null) await mrInflight.catch(() => undefined)
  const s = await read($, settings)
  if (!s.mrBand && !s.mrPane && !s.mrNotify && !s.mrFooter) return
  const prev = await read($, mrCur)
  const now = await $.clock.now()
  if (mrInflight !== null || (!force && now - mrPolledAt < pollDelayMs(prev))) return
  mrPolledAt = now
  const run = pollMr($, s, prev).finally(() => {
    mrInflight = null
  })
  mrInflight = run
  await run
}

// Verknüpfte Sessions der Desktop-App (lesend, geht auch im Auto-Modus). Ohne Desktop-Tools (CLI) oder bei Fehlern bleibt der Stand.
async function refreshLinked($: EngineInterface) {
  const r = await $.mcp.call('ccd_session_mgmt', 'list_sessions', { linked: true, include_archived: true, limit: 10 }).catch(() => null)
  if (r === null || r.isError === true) return
  const block = r.content.find(b => b.type === 'text')
  const list = parseLinkedSessions(block !== undefined && 'text' in block ? block.text : undefined)
  if (!same(await read($, linked), list)) await update($, linked, () => list)
}

async function pollMr($: EngineInterface, s: ModSettings, prev: Mr | null) {
  {
    const remoteName = await gitRemote($)
    const remote = forgeRemote(await gitText($, ['remote', 'get-url', remoteName]))
    mrRemote = remote
    { const v = remote?.host ?? ''; if (!same(await read($, mrHost), v)) await update($, mrHost, () => v) }
    if (remote === null) {
      { const v = null; if (!same(await read($, mrCur), v)) await update($, mrCur, () => v) }
      { const v = 0; if (!same(await read($, mrAhead), v)) await update($, mrAhead, () => v) }
      return
    }
    const branch = await gitText($, ['branch', '--show-current'])
    const found = branch === '' ? null : await forgeFind(forgeIo($), remote, branch)
    // Ein eben gemergter oder geschlossener MR desselben Branches wird weiter abgefragt, damit sein Ende gemeldet wird.
    const iid = found ?? (prev !== null && prev.source === branch && prev.forge === remote.kind ? prev.iid : null)
    const next = iid === null ? null : await forgeLoad(forgeIo($), remote, iid)
    if (iid !== null && next === null) throw new Error(t(s.lang, 'mr.loadFailed', { ref: `!${iid}` }))
    // Bitbucket ohne Zugang im Git-Credential-Store: einmal je Host sagen, warum nichts erscheint.
    if (authMissing() !== '' && authWarned !== authMissing()) {
      authWarned = authMissing()
      $.ui.toast(t(s.lang, 'mr.noAuth', { host: authWarned }))
    }
    if (s.mrNotify) {
      for (const ev of mrEvents(prev, next)) {
        const text = t(s.lang, `mr.ev.${ev}`, { ref: next === null ? '' : mrRef(next) })
        $.ui.toast(text)
        await note($, 'main', text)
      }
    }
    { const v = next; if (!same(await read($, mrCur), v)) await update($, mrCur, () => v) }
    // Ein Entwurf gilt nur für seinen Branch und nur, solange es dafür keinen offenen MR gibt (etwa weil er anderswo angelegt wurde).
    const draft = await read($, mrDraft)
    if (draft !== null && (draft.source !== branch || (next !== null && next.state === 'opened'))) await update($, mrDraft, () => null)
    const target = await defaultTarget($, remoteName)
    const ahead = branch !== '' && branch !== target && (next === null || next.state !== 'opened') ? Number(await gitText($, ['rev-list', '--count', `${remoteName}/${target}..HEAD`])) || 0 : 0
    { const v = ahead; if (!same(await read($, mrAhead), v)) await update($, mrAhead, () => v) }
    if (s.mrPane) {
      const lists = await forgeLists(forgeIo($), remote)
      { const v = lists.mine; if (!same(await read($, mrMine), v)) await update($, mrMine, () => v) }
      { const v = lists.review; if (!same(await read($, mrReview), v)) await update($, mrReview, () => v) }
    }
  }
}

// Status der eigenen MRs im selben Repo: höchstens MINE_DETAIL_MAX, je drei gleichzeitig. MRs anderer Projekte bleiben ohne Status,
// weil die Abfragen (`projects/:id`) am Projekt des Arbeitsverzeichnisses hängen.
async function loadMineDetails($: EngineInterface, force = false) {
  const r = mrRemote
  if (r === null || !(await read($, settings)).mrPane) return
  const now = await $.clock.now()
  if (!force && (now - mrPaneAt > 90_000 || now - mrDetailAt < 300_000)) return
  mrDetailAt = now
  const own = (await read($, mrMine)).filter(x => inRepo(x.ref, r.project)).slice(0, MINE_DETAIL_MAX)
  const out: Record<string, Mr> = {}
  for (let i = 0; i < own.length; i += 3) {
    await Promise.all(
      own.slice(i, i + 3).map(async x => {
        const iid = refOf(x.ref)?.iid
        const m = iid === undefined ? null : await forgeLoad(forgeIo($), r, iid).catch(() => null)
        if (m !== null) out[x.ref] = m
      }),
    )
  }
  if (!same(await read($, mrDetails), out)) await update($, mrDetails, () => out)
}

// Globale Sperre für schreibende MR-Aktionen: solange eine läuft, nimmt keine zweite (auch kein anderer Knopf) einen Klick an.
// Zustand außerhalb der Atome, damit zwei Klicks im selben Moment sicher nur einen durchlassen.
let mrRunning = false

// „Aktualisieren“: Panel aus dem tatsächlichen Stand neu aufbauen (MR, Branch, Listen, Repo-Infos), alte Rückfragen verwerfen.
async function reloadMrPane($: EngineInterface) {
  if (mrRunning) return
  mrRunning = true
  const L = (await read($, settings)).lang
  await update($, mrBusy, () => t(L, 'mr.refreshing'))
  try {
    await update($, mrConfirm, () => '')
    await refreshMr($, true)
    await loadMineDetails($, true)
    await refreshRepo($)
  } catch (err) {
    $.ui.toast(t(L, 'mr.failed', { what: t(L, 'mr.refresh'), why: err instanceof Error ? err.message : String(err) }))
  } finally {
    await update($, mrBusy, () => '')
    mrRunning = false
  }
}

// Eine schreibende Aktion mit Statuszeile, Ergebnis als Toast und anschließender Abfrage.
async function mrStep($: EngineInterface, what: string, run: (r: Remote, m: Mr) => Promise<{ ok: boolean; err: string }>, ref?: string) {
  const m = ref === undefined ? await read($, mrCur) : ((await read($, mrDetails))[ref] ?? null)
  if (m === null || mrRemote === null || mrRunning) return
  mrRunning = true
  const r = mrRemote
  const L = (await read($, settings)).lang
  await update($, mrBusy, () => `${what} …`)
  try {
    const x = await run(r, m)
    $.ui.toast(x.ok ? t(L, 'mr.done', { what }) : t(L, 'mr.failed', { what, why: x.err }))
    await note($, 'main', x.ok ? t(L, 'mr.done', { what }) : t(L, 'mr.failed', { what, why: x.err }))
  } finally {
    await update($, mrBusy, () => '')
    mrRunning = false
    await refreshMr($, true)
    await loadMineDetails($, true).catch(() => undefined)
  }
}

const rebaseMr = async ($: EngineInterface, ref?: string) => mrStep($, t((await read($, settings)).lang, 'mr.rebase'), (r, m) => forgeRebase(forgeIo($), r, m), ref)
const retryPipeline = async ($: EngineInterface) => mrStep($, t((await read($, settings)).lang, 'mr.retry'), (r, m) => forgeRetry(forgeIo($), r, m))

// Mergen in zwei Schritten: erster Klick fragt („Wirklich mergen?“, 6 s gültig), zweiter mergt. Läuft die Pipeline noch,
// merget GitLab bzw. GitHub, sobald sie grün ist.
async function mergeMr($: EngineInterface, ref?: string) {
  const m = ref === undefined ? await read($, mrCur) : ((await read($, mrDetails))[ref] ?? null)
  if (m === null || mrRunning) return
  const key = `merge-${m.iid}`
  if ((await read($, mrConfirm)) !== key) {
    await update($, mrConfirm, () => key)
    $.clock.after(6_000, () => void update($, mrConfirm, c => (c === key ? '' : c)).catch(() => undefined))
    return
  }
  await update($, mrConfirm, () => '')
  const auto = isPipelineRunning(m) && canAutoMerge(m)
  const squash = (await read($, mrSquash)).includes(key)
  await update($, mrSquash, l => l.filter(k => k !== key))
  await mrStep($, t((await read($, settings)).lang, auto ? 'mr.mergeAuto' : 'mr.merge'), (r, cur) => forgeMerge(forgeIo($), r, cur, auto, squash), ref)
}

// Squash-Haken je MR umschalten.
async function toggleSquash($: EngineInterface, iid: number) {
  const key = `merge-${iid}`
  await update($, mrSquash, l => (l.includes(key) ? l.filter(k => k !== key) : [...l, key]))
}

// Entwurf für einen neuen MR: Haiku schreibt Titel und Beschreibung aus Commits und Diffstat; angelegt wird erst per Klick.
async function draftMr($: EngineInterface) {
  if (mrRunning) return
  mrRunning = true
  const L = (await read($, settings)).lang
  await update($, mrBusy, () => t(L, 'mr.drafting'))
  try {
    const branch = await gitText($, ['branch', '--show-current'])
    const remoteName = await gitRemote($)
    const target = await defaultTarget($, remoteName)
    const [log, stat, first] = await Promise.all([
      gitText($, ['log', '--no-merges', '--format=- %s%n%b', `${remoteName}/${target}..HEAD`]),
      gitText($, ['diff', '--stat', `${remoteName}/${target}...HEAD`]),
      gitText($, ['log', '-1', '--format=%s']),
    ])
    const r = await $.model.complete({ model: 'haiku', prompt: draftPrompt(branch, target, log, stat, L), maxTokens: 1500, effort: 'low', timeoutMs: 30_000 }).catch(() => null)
    const d = parseDraft(r !== null && r.isAnswered ? r.text : '', first || branch)
    await update($, mrDraft, () => ({ ...d, source: branch, target }))
  } finally {
    await update($, mrBusy, () => '')
    mrRunning = false
  }
  await $.ui.open({ ...panes(L).mr, focus: true }).catch(() => undefined)
}

// Branch pushen, dann den MR/PR aus dem Entwurf anlegen (Beschreibung über stdin bzw. REST, kein Editor).
async function createMr($: EngineInterface, asDraft: boolean) {
  const d = await read($, mrDraft)
  if (d === null || mrRemote === null || mrRunning) return
  mrRunning = true
  const L = (await read($, settings)).lang
  await update($, mrBusy, () => t(L, 'mr.creating'))
  try {
    const push = await $.process.run(['git', 'push', '-u', await gitRemote($), d.source], { timeoutMs: 120_000 }).catch(() => null)
    if (push === null || push.exitCode !== 0) {
      $.ui.toast(t(L, 'mr.failed', { what: 'git push', why: (push?.stderr ?? '').trim().split('\n')[0] ?? '' }))
      return
    }
    const r = await forgeCreate(forgeIo($), mrRemote, d, asDraft)
    if (!r.ok) {
      $.ui.toast(t(L, 'mr.failed', { what: t(L, 'mr.create'), why: r.err }))
      return
    }
    await update($, mrDraft, () => null)
    $.ui.toast(t(L, 'mr.created', { ref: d.source }))
    await note($, 'main', t(L, 'mr.created', { ref: d.source }))
  } finally {
    await update($, mrBusy, () => '')
    mrRunning = false
    await refreshMr($, true)
  }
}

// Offene Review-Threads als Auftrag ins Eingabefeld; gesendet wird erst mit Enter.
async function threadsToClaude($: EngineInterface) {
  const m = await read($, mrCur)
  if (m === null || mrRemote === null) return
  const threads = await forgeThreads(forgeIo($), mrRemote, m)
  if (threads.length === 0) return
  const L = (await read($, settings)).lang
  await $.prompt.fill({ text: threadsPrompt(m, threads, L) })
  $.ui.toast(t(L, 'mr.filled'))
}

// Konflikt als Auftrag ins Eingabefeld; gesendet wird erst mit Enter. Mit `ref` ein eigener MR eines anderen Branches (Worktree).
async function conflictToClaude($: EngineInterface, ref?: string) {
  const m = ref === undefined ? await read($, mrCur) : ((await read($, mrDetails))[ref] ?? null)
  if (m === null || !m.conflicts) return
  const L = (await read($, settings)).lang
  await $.prompt.fill({ text: conflictPrompt(m, L, ref !== undefined) })
  $.ui.toast(t(L, 'mr.filled'))
}

// Statusteile eines MR für Band und Panel: Nummer, Zustand, Pipeline, Approvals, Threads, Konflikt.
function mrParts(ui: Ui, m: Mr, L: UiLang): RenderElement[] {
  const { Text } = ui
  const parts: RenderElement[] = [<Text key="mr-ref" bold color={THEME.accent}>{mrRef(m)}</Text>]
  if (m.state === 'merged') parts.push(<Text key="mr-st" color={THEME.success}>{t(L, 'mr.merged')}</Text>)
  if (m.state === 'closed') parts.push(<Text key="mr-st" color={THEME.textMuted}>{t(L, 'mr.closed')}</Text>)
  if (m.draft) parts.push(<Text key="mr-dr" color={THEME.textMuted}>{t(L, 'mr.draft')}</Text>)
  if (m.state !== 'opened') return parts
  parts.push(
    m.pipeline !== null ? (
      <Text key="mr-pl" color={THEME[pipelineTone(m.pipeline.status)]}>{t(L, 'mr.pipeline', { status: m.pipeline.status })}</Text>
    ) : (
      <Text key="mr-pl" color={THEME.textMuted}>{t(L, 'mr.noPipeline')}</Text>
    ),
  )
  if (m.approvals !== null && m.approvals.required > 0) {
    parts.push(<Text key="mr-ap" color={m.approvals.left === 0 ? THEME.success : THEME.warning}>{t(L, 'mr.approvals', { have: m.approvals.required - m.approvals.left, need: m.approvals.required })}</Text>)
  }
  if (m.threads > 0) parts.push(<Text key="mr-th" color={THEME.warning}>{t(L, 'mr.threads', { n: m.threads })}</Text>)
  if (m.conflicts) parts.push(<Text key="mr-cf" color={THEME.error}>{t(L, 'mr.conflict')}</Text>)
  return parts
}

const withDots = (ui: Ui, parts: readonly RenderElement[], base: string): RenderElement[] =>
  parts.flatMap((p, i) => (i === 0 ? [p] : [<ui.Text key={`${base}-sep-${i}`} color={THEME.border}>·</ui.Text>, p]))

// Zeile im Band: MR mit Status, sonst Entwurf oder „n Commits ohne MR“ mit dem Knopf zum Anlegen.
async function mrBandRow($: EngineInterface, ui: Ui, L: UiLang, devMr: Mr | null = null): Promise<RenderElement | null> {
  const { Box, Text, Button } = ui
  const [cur, ahead, draft, busy, down] = await Promise.all([read($, mrCur), read($, mrAhead), read($, mrDraft), read($, mrBusy), read($, pressed)])
  const btn = (key: string, label: string, run: () => Promise<unknown>) => pill(ui, 'band', key, label, () => void act($, key, run).catch(() => undefined), down === key)
  // Im Dev-Mode der Mock-MR, damit die Zeile auch ohne GitLab-Repo sichtbar ist.
  const m = devMr ?? cur
  const details = btn('mr-details', t(L, 'mr.details'), () => $.ui.open({ ...panes(L).mr, focus: true }))
  if (busy !== '') return <Box key="mr" flexDirection="row" columnGap={1}><Text color={THEME.accent}>⇄ {busy}</Text></Box>
  if (m !== null) {
    return (
      <Box key="mr" flexDirection="row" columnGap={1} alignItems="center" flexWrap="wrap">
        <Text color={THEME.textMuted}>⇄</Text>
        {withDots(ui, mrParts(ui, m, L), 'mr-b')}
        {m.state === 'opened' && m.threads > 0 && btn('mr-claude', t(L, 'mr.toClaude'), () => threadsToClaude($))}
        {m.state === 'opened' && m.conflicts && btn('mr-fix', t(L, 'mr.fixConflict'), () => conflictToClaude($))}
        {details}
      </Box>
    )
  }
  if (draft !== null) {
    return (
      <Box key="mr" flexDirection="row" columnGap={1} alignItems="center">
        <Text color={THEME.accent}>⇄ {t(L, 'mr.draftHead', { target: draft.target })}</Text>
        {details}
      </Box>
    )
  }
  if (ahead === 0) return null
  return (
    <Box key="mr" flexDirection="row" columnGap={1} alignItems="center">
      <Text color={THEME.textMuted}>⇄ {t(L, 'mr.ahead', { n: ahead })}</Text>
      {btn('mr-create', t(L, 'mr.create'), () => draftMr($))}
    </Box>
  )
}

const mdText = (s: string): string => s.replace(/[[\]()<>]/g, ' ').trim()

// Öffnet http(s)-Links im Standard-Browser des Systems: Windows über rundll32 (ohne cmd, damit „&“ in URLs heil bleibt),
// sonst macOS `open`, sonst `xdg-open`.
// Unter Windows mit vollem Pfad (der Prozess der App muss System32 nicht im PATH haben); explorer.exe meldet auch bei
// Erfolg Exit 1. Schlägt alles fehl, sagt ein Toast warum, statt still nichts zu tun.
async function openExternal($: EngineInterface, href: string) {
  if (!/^https?:\/\//i.test(href)) return
  const root = (await $.env.get('SystemRoot').catch(() => undefined)) ?? ''
  const tries: { argv: string[]; ok: (code: number) => boolean }[] = root !== ''
    ? [
        { argv: [`${root}\\System32\\rundll32.exe`, 'url.dll,FileProtocolHandler', href], ok: c => c === 0 },
        { argv: [`${root}\\explorer.exe`, href], ok: c => c === 0 || c === 1 },
      ]
    : [
        { argv: ['open', href], ok: c => c === 0 },
        { argv: ['xdg-open', href], ok: c => c === 0 },
      ]
  let why = ''
  for (const x of tries) {
    const r = await $.process.run(x.argv, { timeoutMs: 10_000 }).catch((err: unknown) => {
      why = String(err).slice(0, 120)
      return null
    })
    if (r !== null && x.ok(r.exitCode)) return
    if (r !== null) why = `${x.argv[0]?.split(/[\\/]/).pop()}: exit ${r.exitCode} ${r.stderr.trim().slice(0, 80)}`
  }
  $.ui.toast(t((await read($, settings)).lang, 'mr.openFailed', { why }))
}

// Das MR-Panel: je Abschnitt eine Karte (Branch-MR mit Beschreibung, Threads und Aktionen, Entwurf, eigene MRs, Reviews).
// `mock` füllt es im Dev-Mode.
async function mrPaneBody($: EngineInterface, ui: Ui, L: UiLang, mock: { mr: Mr; mrMine: MrMine[]; mrDetails: Record<string, Mr> } | null, now: number, marg: number): Promise<RenderElement[]> {
  const { Box, Text, Button, Markdown } = ui
  // Links im externen Standard-Browser, nicht im Browser der App.
  const ext = (link: PressedLink) => void openExternal($, link.href).catch(() => undefined)
  const section = (i: number, key: string, ...children: RenderChildren[]) => tile(ui, 'pane', { key, tint: cycleTint(i), marg }, ...children)
  const host = mock !== null ? 'gitlab' : await read($, mrHost)
  if (host === '') return [section(0, 'mr-s-none', <Text color={THEME.textMuted}>{t(L, 'mr.noGitlab')}</Text>)]
  const m = mock !== null ? mock.mr : await read($, mrCur)
  const mine = mock !== null ? mock.mrMine : await read($, mrMine)
  const details = mock !== null ? mock.mrDetails : await read($, mrDetails)
  if (mock === null) mrPaneAt = now
  const reviews = mock !== null ? [] : await read($, mrReview)
  const [ahead, draft, busy, confirm, down, squashed] = await Promise.all([read($, mrAhead), read($, mrDraft), read($, mrBusy), read($, mrConfirm), read($, pressed), read($, mrSquash)])
  // Läuft eine MR-Aktion, sind alle Knöpfe grau und nehmen keinen Klick an; der gedrückte zeigt „◌ … “.
  const lock = busy !== ''
  const btn = (key: string, label: string, run: () => Promise<unknown>) =>
    pill(ui, 'pane', key, label, () => void act($, key, run).catch(() => undefined), down === key ? true : lock ? 'locked' : false)
  const out: RenderElement[] = []
  const branch: RenderChildren[] = [<Text key="mr-bh" bold>{t(L, 'mr.branchHead')}</Text>]
  const refreshBtn = btn('mr-a-refresh', t(L, 'mr.refresh'), () => reloadMrPane($))
  if (busy !== '') branch.push(<Text key="mr-busy" color={THEME.accent}>◌ {busy}</Text>)
  if (m !== null) {
    branch.push(<Markdown onLinkPress={ext} key="mr-link" text={`[${mrRef(m)} ${mdText(m.title)}](${m.url})`} />)
    branch.push(<Text key="mr-br" color={THEME.textMuted}>{m.source} → {m.target}</Text>)
    if (m.author !== '') {
      const when = m.updatedAt === '' ? '–' : duration(Math.max(0, now - Date.parse(m.updatedAt)))
      branch.push(<Text key="mr-meta" color={THEME.textMuted}>{t(L, 'mr.meta', { author: m.author, when, changes: m.changes || '?' })}</Text>)
    }
    branch.push(<Box key="mr-parts" flexDirection="row" columnGap={1} flexWrap="wrap">{withDots(ui, mrParts(ui, m, L), 'mr-p')}</Box>)
    if (m.pipeline !== null && m.pipeline.url !== '') branch.push(<Markdown onLinkPress={ext} key="mr-pl" text={`[${t(L, 'mr.pipelineLink')}${m.pipeline.id > 0 ? ` #${m.pipeline.id}` : ''}](${m.pipeline.url})`} />)
    if (m.approvals !== null && m.approvals.by.length > 0) branch.push(<Text key="mr-by" color={THEME.textMuted}>{t(L, 'mr.approvedBy', { who: m.approvals.by.join(', ') })}</Text>)
    if (m.state === 'opened') {
      branch.push(<Text key="mr-ms" color={THEME.textMuted}>{t(L, 'mr.status', { s: m.mergeStatus })}</Text>)
      const asking = confirm === `merge-${m.iid}`
      const mergeLabel = asking ? t(L, 'mr.confirm') : t(L, canMergeNow(m) || !canAutoMerge(m) ? 'mr.merge' : 'mr.mergeAuto')
      branch.push(
        <Box key="mr-actions" flexDirection="row" columnGap={2} flexWrap="wrap" marginTop={1}>
          {m.threads > 0 && btn('mr-a-claude', t(L, 'mr.toClaude'), () => threadsToClaude($))}
          {m.conflicts && btn('mr-a-fix', t(L, 'mr.fixConflict'), () => conflictToClaude($))}
          {btn('mr-a-rebase', t(L, 'mr.rebase'), () => rebaseMr($))}
          {(canMergeNow(m) || (isPipelineRunning(m) && canAutoMerge(m))) &&
            (asking && down !== 'mr-a-merge' && !lock ? (
              <Button key="mr-a-merge" label={mergeLabel} variant="primary" onPress={() => void act($, 'mr-a-merge', () => mergeMr($)).catch(() => undefined)} />
            ) : (
              btn('mr-a-merge', mergeLabel, () => mergeMr($))
            ))}
          {(canMergeNow(m) || (isPipelineRunning(m) && canAutoMerge(m))) &&
            pill(ui, 'pane', 'mr-a-squash', t(L, squashed.includes(`merge-${m.iid}`) ? 'mr.squashOn' : 'mr.squashOff'), () => { if (!lock) void toggleSquash($, m.iid).catch(() => undefined) }, lock ? 'locked' : false)}
          {m.pipeline !== null && m.pipeline.status === 'failed' && m.pipeline.id > 0 && btn('mr-a-retry', t(L, 'mr.retry'), () => retryPipeline($))}
          {refreshBtn}
        </Box>,
      )
    }
  } else if (draft === null) {
    branch.push(<Text key="mr-none" color={THEME.textMuted}>{t(L, 'mr.none')}</Text>)
    if (ahead > 0) {
      branch.push(
        <Box key="mr-ahead" flexDirection="row" columnGap={2} alignItems="center">
          <Text>{t(L, 'mr.ahead', { n: ahead })}</Text>
          {btn('mr-a-create', t(L, 'mr.create'), () => draftMr($))}
        </Box>,
      )
    }
  }
  if (!(m !== null && m.state === 'opened')) branch.push(<Box key="mr-refresh-row" flexDirection="row" marginTop={1}>{refreshBtn}</Box>)
  out.push(section(0, 'mr-s-branch', ...branch))
  if (m !== null && m.description !== '') out.push(section(1, 'mr-s-desc', <Markdown onLinkPress={ext} key="mr-desc" text={m.description} />))
  if (m !== null && m.threadList.length > 0) {
    out.push(
      section(
        2,
        'mr-s-threads',
        <Text key="mr-th-h" bold>{t(L, 'mr.threadsHead')}</Text>,
        ...m.threadList.map((x, i) => (
          <Box key={`mr-th-${i}`} flexDirection="column" marginTop={1}>
            <Text color={THEME.accent}>{x.path}{x.line !== null ? `:${x.line}` : ''}{x.author !== '' ? <Text color={THEME.textMuted}> · @{x.author}</Text> : null}</Text>
            <Text wrap="wrap">{x.body}</Text>
          </Box>
        )),
      ),
    )
  }
  if (draft !== null && !(m !== null && m.state === 'opened')) {
    out.push(
      section(
        3,
        'mr-s-draft',
        <Text key="mr-dh" color={THEME.textMuted}>{t(L, 'mr.draftHead', { target: draft.target })}</Text>,
        <Text key="mr-dt" bold color={THEME.accent}>{draft.title}</Text>,
        draft.description !== '' && <Markdown onLinkPress={ext} key="mr-dd" text={draft.description} />,
        <Box key="mr-dactions" flexDirection="row" columnGap={2} flexWrap="wrap" marginTop={1}>
          {down !== 'mr-d-create' && !lock ? (
            <Button key="mr-d-create" label={t(L, 'mr.submit')} variant="primary" onPress={() => void act($, 'mr-d-create', () => createMr($, false)).catch(() => undefined)} />
          ) : (
            btn('mr-d-create', t(L, 'mr.submit'), () => createMr($, false))
          )}
          {btn('mr-d-draft', t(L, 'mr.submitDraft'), () => createMr($, true))}
          {btn('mr-d-redraft', t(L, 'mr.redraft'), () => draftMr($))}
          {btn('mr-d-discard', t(L, 'mr.discard'), () => update($, mrDraft, () => null))}
        </Box>,
      ),
    )
  }
  const list = (items: readonly MrMine[]) => items.map(x => `- [${mdText(x.ref)}](${x.url}) ${mdText(x.title)}${x.draft ? ' · Draft' : ''}`).join('\n')
  // Je eigener MR: Link, darunter (wenn geladen) Status und Aktionen; ein Konflikt geht als Auftrag an Claude (Lösung im Worktree).
  const mineRows = mine.map(x => {
    const d = details[x.ref]
    const asking = d !== undefined && confirm === `merge-${d.iid}`
    const mergeable = d !== undefined && d.state === 'opened' && (canMergeNow(d) || (isPipelineRunning(d) && canAutoMerge(d)))
    return (
      <Box key={`mine-${x.ref}`} flexDirection="column">
        <Markdown onLinkPress={ext} key={`mine-l-${x.ref}`} text={list([x])} />
        {d !== undefined && (
          <Box key={`mine-s-${x.ref}`} flexDirection="row" columnGap={1} flexWrap="wrap" marginLeft={2}>
            {withDots(ui, mrParts(ui, d, L).slice(1), `mine-p-${d.iid}`)}
            {d.state === 'opened' && d.conflicts && btn(`mine-fix-${d.iid}`, t(L, 'mr.fixConflict'), () => conflictToClaude($, x.ref))}
            {d.state === 'opened' && btn(`mine-rebase-${d.iid}`, t(L, 'mr.rebase'), () => rebaseMr($, x.ref))}
            {mergeable && d !== undefined &&
              (asking && down !== `mine-merge-${d.iid}` && !lock ? (
                <Button key={`mine-merge-${d.iid}`} label={t(L, 'mr.confirm')} variant="primary" onPress={() => void act($, `mine-merge-${d.iid}`, () => mergeMr($, x.ref)).catch(() => undefined)} />
              ) : (
                btn(`mine-merge-${d.iid}`, asking ? t(L, 'mr.confirm') : t(L, canMergeNow(d) || !canAutoMerge(d) ? 'mr.merge' : 'mr.mergeAuto'), () => mergeMr($, x.ref))
              ))}
            {mergeable && d !== undefined &&
              pill(ui, 'pane', `mine-squash-${d.iid}`, t(L, squashed.includes(`merge-${d.iid}`) ? 'mr.squashOn' : 'mr.squashOff'), () => { if (!lock) void toggleSquash($, d.iid).catch(() => undefined) }, lock ? 'locked' : false)}
          </Box>
        )}
      </Box>
    )
  })
  if (mine.length > 0) out.push(section(4, 'mr-s-mine', <Text key="mr-mine-h" bold>{t(L, 'mr.mine')}</Text>, ...mineRows))
  if (reviews.length > 0) out.push(section(5, 'mr-s-review', <Text key="mr-rev-h" bold>{t(L, 'mr.review')}</Text>, <Markdown onLinkPress={ext} key="mr-rev" text={list(reviews)} />))
  return out
}

// ---- Diagnose (Idee: YeonwooSung/my-claude-code-mods: context-inspector, agent-profiler, blast-radius; kein Code übernommen) ----
// Zustand der drei Module liegt in Modul-Variablen, nicht in Atomen: er hält bis zu 20.000 Datensätze, und ein Reload beginnt
// von vorn (// ponytail: kein Wiederaufbau aus $.session.messages(); Ausbaupfad: backfill wie im Original). Rein beobachtend.
let inspectSt = newInspect()
let inspectVer = 0
let inspectAll = false
let inspectMemo: { ver: number; all: boolean; sec: number; sum: InspectSummary } | null = null
let profileSt = newProfiler()
let profileVer = 0
let profileMemo: { ver: number; sec: number; sum: ProfileSummary } | null = null
let blastRes: BlastResult | null = null
let blastWatched: string[] = []
let blastRun: Promise<void> | null = null
let blastLocked = false
let lastRepaint = 0

// Eine Beobachtung stört die Session nie: jeder Eintrag läuft hierdurch und zählt die Version hoch (Cache der Auswertung).
const observeInspect = (fn: () => void): void => {
  try {
    fn()
    inspectVer++
  } catch {
    // verworfen
  }
}
const observeProfile = (fn: () => void): void => {
  try {
    fn()
    profileVer++
  } catch {
    // verworfen
  }
}

// Das Band zeichnet während eines Turns im 250-ms-Takt, jede Beobachtung zählt die Version hoch: im Band darum höchstens
// einmal je Sekunde neu rechnen (`throttled`); Panel und Bericht rechnen frisch.
const inspectSum = (now = 0, throttled = false): InspectSummary => {
  const sec = Math.floor(now / 1000)
  const stale = inspectMemo === null || inspectMemo.all !== inspectAll || (inspectMemo.ver !== inspectVer && (!throttled || inspectMemo.sec !== sec))
  if (stale) inspectMemo = { ver: inspectVer, all: inspectAll, sec, sum: summarizeInspect(inspectSt, inspectAll) }
  return inspectMemo!.sum
}
const profileSum = (now: number, throttled = false): ProfileSummary => {
  const sec = Math.floor(now / 1000)
  const stale = profileMemo === null || (profileMemo.ver !== profileVer && (!throttled || profileMemo.sec !== sec)) || (!throttled && profileMemo.sec !== sec)
  if (stale) profileMemo = { ver: profileVer, sec, sum: summarizeProfile(profileSt, now) }
  return profileMemo!.sum
}

// Uhr für die Beobachtung: fehlt sie (Test-Host), zählt die Beobachtung nicht, die Session läuft weiter.
const stamp = async ($: EngineInterface): Promise<number> => {
  try {
    return await $.clock.now()
  } catch {
    return 0
  }
}

// Zeichnet Band und Panels neu, höchstens einmal je Sekunde.
async function repaint($: EngineInterface) {
  const now = await stamp($)
  if (now - lastRepaint < 1_000) return
  lastRepaint = now
  $.ui.invalidate('ui.render')
}

const blastKey = (cwd: string) => `blast:watch:${cwd}`

async function loadBlast($: EngineInterface) {
  const cwd = await projectCwd($)
  if (cwd === '') return
  const saved = await $.store.get(blastKey(cwd))
  blastWatched = Array.isArray(saved) ? saved.filter((x): x is string => typeof x === 'string').slice(0, 200) : []
  if ((await read($, settings)).blastBand) await refreshBlast($)
}

async function saveBlast($: EngineInterface, cwd: string) {
  await $.store.set(blastKey(cwd), blastWatched)
}

// Git-Aufrufe wie überall im Cockpit: ohne fsmonitor und Locks, und nie in einem Repo, dessen Konfiguration Befehle starten kann.
async function blastIo($: EngineInterface): Promise<BlastIo | null> {
  const cwd = await projectCwd($)
  if (cwd === '') return null
  blastLocked = await isRiskyRepo($, cwd)
  if (blastLocked) return null
  return {
    cwd,
    read: path => $.fs.read(path),
    run: argv => $.process.run(argv[0] === 'git' ? [...GIT_BASE, ...argv.slice(1)] : argv, { cwd, timeoutMs: 10_000 }),
  }
}

// Läuft schon eine Berechnung, kehrt der Aufruf sofort zurück; `wait` (Befehle) wartet sie ab und rechnet danach neu.
async function refreshBlast($: EngineInterface, wait = false) {
  if (blastRun !== null) {
    if (!wait) return
    await blastRun
  }
  const run = (async () => {
    try {
      const io = await blastIo($)
      blastRes = io === null ? null : await blastScan(io, blastWatched)
    } catch {
      // die alten Zahlen bleiben
    }
  })()
  blastRun = run
  try {
    await run
  } finally {
    blastRun = null
    await repaint($).catch(() => undefined)
  }
}

async function blastCommand($: EngineInterface, args: string): Promise<string> {
  const L = await langOf($)
  const cmd = parseCommand(args)
  if (cmd.kind === 'help') return t(L, 'diag.blastHelp')
  const cwd = await projectCwd($)
  if (cmd.kind === 'reset') {
    blastWatched = []
    await saveBlast($, cwd)
    await refreshBlast($, true)
    return blastReport(blastRes, blastWatched, L)
  }
  if (cmd.kind === 'auto') {
    await $.ui.open({ ...panes(L).profiling, focus: true })
    void refreshBlast($).catch(() => undefined)
    return ''
  }
  const path = normalizeTarget(cwd, cmd.path)
  if (path === null) return t(L, 'diag.blastBad')
  if (cmd.kind === 'watch') blastWatched = [...new Set([...blastWatched, path])].slice(-200)
  if (cmd.kind === 'unwatch') blastWatched = blastWatched.filter(p => p !== path)
  if (cmd.kind === 'target') {
    const io = await blastIo($)
    return io === null ? t(L, 'diag.blastLocked') : blastReport(await blastScan(io, [path]), blastWatched, L)
  }
  await saveBlast($, cwd)
  await refreshBlast($, true)
  return blastReport(blastRes, blastWatched, L)
}

type DiagSection = { title: string; rows: readonly { label: string; value: string; pct?: number; tone?: 'ok' | 'warn' | 'bad' | 'dim' }[]; note?: string }

// Zeilen eines Moduls im Profiling-Panel: Abschnittsüberschrift gedimmt, Zeilen als Bezeichnung und Wert (kurze Bezeichnungen
// im Raster, lange – Dateipfade – gedimmt davor), Anteile als Textbalken, zwischen den Abschnitten eine Leerzeile.
function diagRows(ui: Ui, key: string, sections: readonly DiagSection[]): RenderElement[] {
  const { Box, Text } = ui
  const TONE = { ok: THEME.success, warn: THEME.warning, bad: THEME.error, dim: THEME.textMuted } as const
  return sections.flatMap((sec, i) => [
    <Text key={`${key}-${i}-h`} bold color={THEME.textMuted}>{sec.title}</Text>,
    ...sec.rows.map((r, j) => {
      const value = (
        <Text key={`${key}-${i}-${j}-v`} wrap="truncate-end" color={r.tone === undefined ? undefined : TONE[r.tone]}>
          {r.pct === undefined ? '' : `${bar(r.pct, 12)} `}
          {r.value}
        </Text>
      )
      return r.label.length <= 12 ? (
        labelRow(ui, 'pane', `${key}-${i}-${j}`, r.label, value)
      ) : (
        <Box key={`${key}-${i}-${j}`} flexDirection="row" columnGap={1}>
          <Text dimColor wrap="truncate-end">{r.label}</Text>
          {value}
        </Box>
      )
    }),
    ...(sec.note === undefined ? [] : [<Text key={`${key}-${i}-note`} dimColor wrap="wrap">{sec.note}</Text>]),
    ...(i < sections.length - 1 ? [<Box key={`${key}-${i}-gap`} height={1} />] : []),
  ])
}

// Profiling-Panel im Schema des Cockpit-Panels: Kopf mit Modellzeile, darunter je Modul eine Karte mit ▾/▸ (Atom `collapsed`).
async function profilingPane($: EngineInterface, e: ResolveInput): Promise<RenderElement> {
  const s = await read($, settings)
  const L = s.lang
  const marg = s.paneMargins ? 1 : 0
  const ui = elementsOf($, e)
  const { Box, Text } = ui
  const now = await $.clock.now()
  await read($, tick)
  const folded = await read($, collapsed)
  const cards = [
    {
      id: 'inspect',
      title: t(L, 'diag.cardInspect'),
      sections: () => inspectView(inspectSum(), L),
      action: pill(ui, 'pane', 'diag-agents', t(L, inspectAll ? 'diag.mainOnly' : 'diag.withAgents'), () => {
        inspectAll = !inspectAll
        $.ui.invalidate('ui.render')
      }),
    },
    { id: 'profile', title: t(L, 'diag.cardProfile'), sections: () => profileView(profileSum(now), now, L), action: null },
    {
      id: 'blast',
      title: t(L, 'diag.cardBlast'),
      sections: () => blastView(blastRes, blastWatched, L),
      action: pill(ui, 'pane', 'diag-refresh', t(L, 'diag.refresh'), () => void refreshBlast($, true).catch(() => undefined), blastRun !== null),
    },
  ]
  const tiles = cards.map((c, i) => {
    const key = `diag-${c.id}`
    const open = !folded.includes(key)
    let rows: RenderElement[]
    try {
      rows = diagRows(ui, key, c.sections())
    } catch (error) {
      rows = [<Text key={`${key}-fail`} color={THEME.error} wrap="wrap">{t(L, 'diag.failed', { why: String(error) })}</Text>]
    }
    return tile(
      ui,
      'pane',
      {
        key: `card-${key}`,
        tint: cycleTint(i + 1),
        marg,
        grow: true,
        open,
        head: (
          <Box flexDirection="row" justifyContent="space-between">
            <Text bold>{c.title}</Text>
            <Box flexDirection="row" columnGap={1}>
              {open && c.action}
              {pill(ui, 'pane', `card-fold-${key}`, open ? '▾' : '▸', () => void toggleCard($, key).catch(() => undefined))}
            </Box>
          </Box>
        ),
      },
      ...(c.id === 'blast' && blastLocked ? [<Text key="diag-locked" color={THEME.warning} wrap="wrap">{t(L, 'diag.blastLocked')}</Text>] : []),
      ...rows,
    )
  })
  return (
    <Box flexDirection="column">
      {await paneHead($, ui, L)}
      {...tiles}
    </Box>
  )
}

// Kurzinfos im Band: je ein Knopf, der das zugehörige Panel öffnet; leere Kurzinfos (noch keine Daten) fehlen.
function diagChips($: EngineInterface, ui: Ui, s: ModSettings, percent: number | undefined, now: number): RenderElement | null {
  const L = s.lang
  const open = (p: { id: string; title: string }) => () => void $.ui.open({ ...p, focus: true }).catch(() => undefined)
  const safe = (fn: () => string): string => {
    try {
      return fn()
    } catch {
      return ''
    }
  }
  const items = [
    { key: 'diag-inspect', on: s.inspectBand, text: () => inspectBand(inspectSum(now, true), percent, L), pane: panes(L).profiling },
    { key: 'diag-profile', on: s.profileBand, text: () => profileBand(profileSum(now, true), now, L), pane: panes(L).profiling },
    { key: 'diag-blast', on: s.blastBand, text: () => blastBand(blastRes, L), pane: panes(L).profiling },
  ]
  const chips = items.flatMap(i => {
    const text = i.on ? safe(i.text) : ''
    return text === '' ? [] : [pill(ui, 'band', i.key, text, open(i.pane), false, true)]
  })
  if (chips.length === 0) return null
  return (
    <ui.Box key="diag-row" flexDirection="row" flexWrap="wrap" columnGap={2} alignItems="center">
      {chips}
    </ui.Box>
  )
}

export const register: Register = (on, options) => {
  // Checkliste: jeder Slash-Befehl wird ein Job (nur beobachten, Ausgabe unverändert). Als erster Hook registriert, damit er
  // auch die eigenen Befehle sieht, die ohne next antworten.
  on('command.run', async ($, e, next) => {
    // Nur getippte Befehle (Herkunft composer, Bridge, SDK); Aufrufe von Plugins und ohne Herkunft (Test-Host) gehen durch.
    if (e.origin === undefined || e.origin.kind === 'plugin' || !(await read($, settings)).glance) return next(e)
    await startCommand($, e.command, e.args)
    const out = await next(e)
    $.clock.after(1_500, () => void endCommand($).catch(() => undefined))
    return out
  }).catch(($, e, next) => next(e))

  on('session.start', async ($, e, next) => {
    const info = await loadAbout($)
    await update($, about, () => info)
    // Diagnose: neue Session, neue Messung (der Zustand liegt in Modul-Variablen und überlebt keinen Reload).
    inspectSt = newInspect()
    profileSt = newProfiler()
    inspectVer++
    profileVer++
    blastRes = null
    await loadSettings($, { steps: options.nextSteps !== false, chatInfo: options.chatInfo !== false })
    void $.store.get('collapsed').then(v => update($, collapsed, () => foldedOf(v))).catch(() => undefined)
    void loadCatchUp($).catch(() => undefined)
    await update($, logo, () => null)
    void Promise.all([$.session.model(), $.session.version()])
      .then(([model, v]) => update($, modelInfo, mi => ({ ...mi, model, cc: v.version })))
      .catch(() => undefined)
    void Promise.all([loadLogo($).then(l => update($, logo, () => l)), refreshRepo($), refreshLinked($), refreshDirs($)]).catch(() => undefined)
    // ponytail: Beschreibungen in der Sprache beim Session-Start; nach einem Sprachwechsel erst mit dem nächsten Start/Reload
    const lang = await langOf($)
    await $.command.register({ name: COMMAND, description: t(lang, 'common.cmdPanel') })
    await $.command.register({ name: 'cockpit-zusammenfassung', description: t(lang, 'common.cmdSummary') })
    await $.command.register({ name: 'cockpit-einstellungen', description: t(lang, 'common.cmdSettings') })
    await $.command.register({ name: 'cockpit-demo', description: t(lang, 'common.cmdDemo') })
    await $.command.register({ name: 'cockpit-dev', description: t(lang, 'common.cmdDev') })
    await $.command.register({ name: 'cockpit-mr', description: t(lang, 'common.cmdMr') })
    await $.command.register({ name: 'cockpit-features', description: t(lang, 'common.cmdFeatures') })
    await $.command.register({ name: 'cockpit-verlauf', description: t(lang, 'common.cmdHistory') })
    await $.command.register({ name: 'cockpit-profiling', description: t(lang, 'common.cmdProfiling') })
    await $.command.register({ name: 'cockpit-kontext', description: t(lang, 'common.cmdInspect') })
    await $.command.register({ name: 'cockpit-profil', description: t(lang, 'common.cmdProfile') })
    await $.command.register({ name: 'cockpit-blast', description: t(lang, 'common.cmdBlast') })
    void refreshDirs($).then(() => loadBlast($)).catch(() => undefined)
    // Checkliste: mittlere Schrittdauer aus dem Verlauf, sobald der Session-Ordner bekannt ist.
    void refreshDirs($).then(() => projectCwd($)).then(async cwd => loadAvg($, cwd, await $.clock.now())).catch(() => undefined)
    // ponytail: Git-Stand alle 30 s neu (auch nach Worktree-Wechsel oder Neustart ohne neuen Turn), classic.CwdChanged wäre sofort
    // Im selben Takt prüft die Checkliste „hängt?“ (das Band zeichnet über `tick` ohnehin neu).
    void $.clock.every(30_000, () => void Promise.all([update($, tick, n => n + 1), syncAgents($), watchCache($), refreshRepo($), refreshLinked($), refreshDirs($)]).catch(() => undefined))
    void $.clock.every(15_000, () => void refreshMr($).then(() => loadMineDetails($)).catch(() => undefined))
    void $.clock.every(10_000, () => void syncSettings($).catch(() => undefined))
    void refreshMr($, true).catch(() => undefined)
    // Jede Speicherung im Plugin-Ordner lädt das Mod in allen Sessions neu; ein Band, das dabei nichts zeichnete, hat keine
    // Atome abonniert und bliebe leer, bis ein neuer Prompt neu zeichnet.
    $.clock.after(1_500, () => $.ui.invalidate('ui.render'))
    return next(e)
  })

  // Info-Zeilen und Karten im Chat, abschaltbar in den Einstellungen.
  // Antworten: Code-/Tabellenkarten (skinCards) und Hervorhebungen (skinInline) zeichnen selbst, aber nur wenn es etwas zu
  // stylen gibt; Prosa bleibt das native Markdown. Zusammenfassungs-Blöcke bleiben unverändert.
  on('ui.render', { component: 'AssistantMessage' }, async ($, e, next) => {
    const theirs = await next(e)
    if (!drawsOn(e.surface)) return theirs
    const s = await read($, settings)
    const ui = elementsOf($, e)
    let el = theirs
    const so = splitOptions(s, 'Svg' in ui)
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
    if (!drawsOn(e.surface)) return next(e)
    const s = await read($, settings)
    const p = e.props
    const md = mdLiteral(p.text)
    if (!s.skinPrompt || p.origin.kind !== 'composer' || p.from !== undefined || p.task !== undefined || p.text.trim() === '' || md.length > 100_000) return next(e)
    const { Box, Markdown } = elementsOf($, e)
    return (
      <Box key={`om-p-${shortHash(p.text)}`} flexDirection="column" alignItems="flex-end">
        <Box borderStyle="round" borderColor={THEME.accent} paddingX={1} flexShrink={1} flexDirection="row" columnGap={1} alignItems="flex-start">
          {skinMark(elementsOf($, e), await markerOf($), `om-pm-${shortHash(p.text)}`, 'style.prompt')}
          <Markdown text={md} />
        </Box>
      </Box>
    )
  })

  // Tool-Zeile: Symbol davor (skinIcons), darunter ▸/▾ Details mit Eingabe und Ergebnis als Code (skinExpand), dann die Info-Zeile.
  on('ui.render', { component: 'ToolUse' }, async ($, e, next) => {
    const theirs = await next(e)
    if (!drawsOn(e.surface)) return theirs
    const s = await read($, settings)
    const ui = elementsOf($, e)
    const { Box, Text, Button, Code } = ui
    const p = e.props
    const id = p.tool_use_id
    let el = theirs
    // Aufgabenliste und Abschluss-Box: nur ein Kopfband mit Fortschritt bzw. Rückblick darüber, die Zeile bleibt die der Engine.
    const todo = s.skinTodos ? todosOf(p.tool, p.input) : null
    const wrap = s.skinTodos ? wrapupOf(p.tool, p.input) : null
    if (todo !== null || wrap !== null) {
      const head =
        todo !== null ? (
          <Text color={todo.done === todo.total ? THEME.success : THEME.accent} bold wrap="truncate-end">
            {t(s.lang, 'skin.todosHead', { done: todo.done, total: todo.total })}
            {todo.current !== '' ? ` · ${t(s.lang, 'skin.todosNow', { what: todo.current })}` : ''}
          </Text>
        ) : (
          <Text color={THEME.accent} wrap="truncate-end">
            <Text bold>{t(s.lang, 'skin.wrapupHead')}</Text> · {wrap?.recap}
            {wrap?.ask ? ` · ${t(s.lang, 'skin.wrapupAsk', { what: wrap.ask })}` : ''}
          </Text>
        )
      el = (
        <Box key={`om-td-${id}`} flexDirection="column">
          <Box flexDirection="row" columnGap={1} alignItems="center" borderStyle="round" borderColor={THEME.accent} paddingX={1}>
            {skinMark(ui, await markerOf($), `om-tdm-${id}`, todo !== null ? 'style.todos' : 'style.wrapup')}
            {head}
          </Box>
          {theirs}
        </Box>
      )
    }
    const icon = s.skinIcons ? toolIcon(ui, p.tool, p.isErrored, `om-i-${id}`) : null
    if (icon !== null)
      el = (
        <Box key={`om-ir-${id}`} flexDirection="row" columnGap={1} alignItems="flex-start">
          {skinMark(ui, await markerOf($), `om-im-${id}`, 'style.icon')}
          {icon}
          {el}
        </Box>
      )
    // Einzelner MCP-Aufruf (nicht gruppiert): dieselbe farbige Zeile „Server › Tool (Aktion) → Ziel“ wie in der Tool-Gruppe.
    if (icon !== null && splitTool(p.tool).server !== null)
      el = (
        <Box key={`om-ml-${id}`} flexDirection="column">
          {el}
          <Text wrap="truncate-end" dimColor={!p.isRunning}>{'  '}{toolLabel(ui, p.tool, toolTarget(p.input), p.isErrored, toolAction(p.input))}</Text>
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
            {pill(ui, 'skin', `om-xb-${id}`, t(s.lang, open ? 'skin.detailsOpen' : 'skin.details'), toggle, false, true)}
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
    if (!drawsOn(e.surface) || e.props.isExpanded || !(await read($, settings)).skinIcons) return theirs
    const ui = elementsOf($, e)
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
            {'  '}{toolLabel(ui, c.tool, toolTarget(c.input), c.isErrored, toolAction(c.input))}
          </Text>
        ))}
        {mcp.length > shown.length && <Text key={`om-gx-${base}`} dimColor>{'  '}+{mcp.length - shown.length}</Text>}
      </Box>
    )
  })

  // Ergebnis: Diff-Karte für Edit/Write (skinDiff); Fehler als erste echte Fehlerzeile und lesende Aufrufe als eine Zeile (skinQuiet).
  on('ui.render', { component: 'ToolResult' }, async ($, e, next) => {
    const theirs = await next(e)
    if (!drawsOn(e.surface)) return theirs
    const s = await read($, settings)
    const ui = elementsOf($, e)
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
    const { Box, Text } = elementsOf($, e)
    const mode = SPIN_MODE[e.props.mode]
    // Atemübung: der Spinner nennt zusätzlich die Phase mit Restsekunden, im selben Takt wie der Kreis im Band.
    const breathing = s.breath && tn !== null && tn.durationMs === undefined
    const ph = breathing ? phaseAt(breathOf(s.breathKey), now - tn.startedAt) : null
    const step = spinnerText(e.props.word, e.props.message)
    return (
      <Box flexDirection="row" columnGap={1} alignItems="center">
        {skinMark(elementsOf($, e), await markerOf($), 'om-spin-m', 'style.spinner')}
        <Text color={mode.color} bold>{spinner(await read($, frame))} {t(s.lang, mode.label)}</Text>
        {ph !== null && <Text key="breath-phase" color={BREATH_COLOR[breathOf(s.breathKey).key]}>{t(s.lang, `breath.${ph.phase}`)} {Math.ceil(ph.leftMs / 1000)}s ·</Text>}
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
    const { Text } = elementsOf($, e)
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
    if (!drawsOn(e.surface)) return theirs
    const s = await read($, settings)
    if (!s.skinAsk) return theirs
    const ui = elementsOf($, e)
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
  // Fußzeile rechts unter dem Prompt: Modus-Labels (skinMode in Akzentfarbe, sonst gedimmt wie die Engine) und die Cockpit-Minibuttons.
  on('ui.render', { component: 'SessionMode' }, async ($, e, next) => {
    if (!drawsOn(e.surface)) return next(e)
    const s = await read($, settings)
    const ui = elementsOf($, e)
    const { Box, Text, Button } = ui
    const L = s.lang
    const P = panes(L)
    const modes = e.props.modes
    // MR des Branches als kleiner Statusknopf (wie der PR-Knopf der Desktop-App); im Dev-Mode der Mock-MR.
    const footMr = s.mrFooter ? ((await read($, devMock)) ? mockData(await $.clock.now()).mr : await read($, mrCur)) : null
    // Fußzeilen-Muster aus effortless: mehrere sehr kurze Knöpfe, Leerzeichen als Innenrand, Hover-Hintergrund wie die
    // Pillen der App. Lange Labels sprengen die schmale Fußzeile, dann landet der Rest hinter „…“ (so war es mit
    // „Tenvan · Details“). Svg zeichnet die Fußzeile nicht; Text-Kinder eines Buttons lehnt die Engine ab.
    const mrFoot = footMr !== null ? ` ${mrBadge(footMr).replace(/ · /g, ' ')} ` : null
    const pill = (scope: string) => ({ scope, backgroundColor: THEME.chip })
    return (
      <Box key="om-mode" flexDirection="row" gap={1} alignItems="center">
        {modes.length > 0 && s.skinMode && skinMark(ui, await markerOf($), 'om-mode-m', 'style.mode')}
        {modes.length > 0 && (s.skinMode ? <Text color={THEME.accent}>{modes.join(' & ')}</Text> : <Text dimColor>{modes.join(' & ')}</Text>)}
        {mrFoot !== null && <Button key="mr-foot" plain label={mrFoot} hover={pill('om-mr')} onPress={() => void $.ui.open({ ...P.mr, focus: true })} />}
        <Button key="details" plain dimColor label=" Tenvan " hover={pill('om-details')} onPress={() => void $.ui.open({ ...P.main, focus: true })} />
      </Box>
    )
  })

  // Hinweiszeile unter dem Prompt: nur das Kennzeichen davor; die Zeile (samt Pills) bleibt die der Engine.
  on('ui.render', { component: 'PromptHint' }, async ($, e, next) => {
    const theirs = await next(e)
    if (!drawsOn(e.surface) || !(await read($, settings)).skinHint) return theirs
    const ui = elementsOf($, e)
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
    if (!drawsOn(e.surface)) return theirs
    const s = await read($, settings)
    const ui = elementsOf($, e)
    let el = theirs
    // Karten und Hervorhebungen auch hier (zeigt sie in /cockpit-demo); Fehlerzeilen zeichnet weiter die Engine.
    const so = splitOptions(s, 'Svg' in ui)
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

  // Dev-Mode: schaltet Mockdaten im Panel um und öffnet es.
  on('command.run', { command: 'cockpit-dev' }, async $ => {
    const L = (await read($, settings)).lang
    const isOn = !(await read($, devMock))
    await update($, devMock, () => isOn)
    if (isOn) await $.ui.open({ ...panes(L).main, focus: true })
    return { text: t(L, isOn ? 'dev.on' : 'dev.off') }
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

  // Verlauf der Aufgaben: heutigen Tag laden und das Panel öffnen.
  on('command.run', { command: 'cockpit-verlauf' }, async $ => {
    await openHistory($)
    return {}
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

  // Nach /compact misst die Engine erst mit dem nächsten Request neu: Kontext, Aufschlüsselung und Cache selbst nachlesen.
  on('session.compact', async ($, e, next) => {
    const done = await next(e)
    observeInspect(() => ingestCompaction(inspectSt, e as never, done as never))
    if (e.agentId === undefined && e.trigger !== 'precompute' && done.skip === undefined) {
      const reread = async () => {
        const v = toUsage(await $.session.usage())
        if (!same(await read($, usage), v)) await update($, usage, () => v)
        await fetchBreakdown($, true)
      }
      await update($, cacheHit, () => null)
      await reread().catch(() => undefined)
      // Die Zählung danach kann einen Moment hinterherhängen: einmal kurz später nachlesen.
      $.clock.after(1500, () => void reread().catch(() => undefined))
    }
    return done
  })

  // Mitten im Turn kommt das Ende einer Hintergrundaufgabe als Anhang in die Unterhaltung, nicht als Prompt.
  on('session.append', async ($, e, next) => {
    observeInspect(() => ingestAppend(inspectSt, e as never))
    const text = notificationText(e.door, e.message.content)
    if (text.includes('<task-notification>')) await finishBackground($, text).catch(() => undefined)
    return next(e)
  })

  on('prompt.submit', async ($, e, next) => {
    // Benachrichtigungen kommen auch gebündelt oder mit einem getippten Prompt zusammen: jeden Prompt danach absuchen.
    await finishBackground($, e.text)
    await update($, dirAlert, () => null)
    if (e.origin?.kind !== 'task-notification') await judgeEffort($, e.text)
    // Checkliste: jeder eigene Prompt ist ein neuer Job; eine Benachrichtigung eines Hintergrund-Tasks nicht.
    if (e.origin?.kind !== 'task-notification') await startJob($, e.text)
    await startTurn($, e.text)
    return next(e)
  })

  on('tool.call', async ($, e, next) => {
    if (e.agentId !== undefined && (await read($, braked)).includes(e.agentId)) return { deny: t(await langOf($), 'agents.brakeDeny') }
    // Vor dem Lauf merken, damit das Ergebnis beim ersten Zeichnen schon als ruhig erkannt wird.
    if (isShell(e.tool) && isReadOnlyShell(shellCommand(e))) await update($, quietIds, ids => pushCapped(ids, e.tool_use_id, 200))
    const out = await foreignTarget($, e.tool, e).catch(() => null)
    // Argumente liegen bei tool.call im Event selbst (ältere Aufrufer und Tests: unter `input`).
    const ev = e as unknown as Record<string, unknown>
    const args = (typeof ev.input === 'object' && ev.input !== null ? ev.input : ev) as Record<string, unknown>
    // Checkliste: solange eine AskUserQuestion offen ist, „braucht dich“ mit der Frage.
    if (e.agentId === undefined && e.tool === 'AskUserQuestion') {
      const question = askText(args)
      await updJob($, j => ({ ...j, phase: 'needsYou', question }))
    }
    // Ein laufender Aufruf der Hauptschleife (langer Build, offene Freigabe) ist kein „hängt?“.
    const isMain = e.agentId === undefined
    if (isMain) toolsInFlight++
    const startedAt = await stamp($)
    observeInspect(() => ingestToolCall(inspectSt, e.tool_use_id, e.tool, e.agentId, args))
    observeProfile(() => toolBegin(profileSt, e.tool_use_id, e.tool, e.agentId, args, startedAt))
    void repaint($).catch(() => undefined)
    let ran: Awaited<ReturnType<typeof next>>
    let ranOk = false
    try {
      ran = await next(e)
      ranOk = ran.deny === undefined && ran.isError !== true
    } finally {
      if (isMain) toolsInFlight = Math.max(0, toolsInFlight - 1)
      const endedAt = await stamp($)
      observeProfile(() => toolEnd(profileSt, e.tool_use_id, endedAt, ranOk))
    }
    observeInspect(() => ingestToolResult(inspectSt, e.tool_use_id, ran))
    const isEdit = EDIT_TOOLS.has(e.tool)
    const isError = ran.isError === true
    const path = isEdit && !isError ? inputPath(e) : ''
    if (path !== '') await update($, touched, list => (list.includes(path) ? list : pushCapped(list, path, 100)))
    {
      // Checkliste: Hauptschleife zählt Tätigkeit, Plan und geänderte Dateien; ein Subagent-Aufruf ist nur ein Lebenszeichen.
      const now = await $.clock.now()
      if (e.agentId !== undefined) await touchJob($)
      else
        await updJob($, j => {
          let nj = noteAct(j, e.tool, now)
          // Ein fehlgeschlagener Aufruf der Aufgabenliste hat den Plan nicht geändert.
          if (!isError) nj = planUpdate(nj, e.tool, args, ran.result, now) ?? nj
          if (path !== '') nj = noteFile(nj, path)
          if (e.tool === 'AskUserQuestion' && nj.phase === 'needsYou') nj = { ...nj, phase: 'working', question: '' }
          // Desktop-Abschlussbox: ihre Frage hat am Turn-Ende Vorrang vor der Frage aus der Antwort.
          if (e.tool === 'mcp__ccd_turn__wrap_up' && !isError) nj = completeAll(nj, now)
          if (e.tool === 'mcp__ccd_turn__wrap_up' && typeof args.ask === 'string' && args.ask.trim() !== '') nj = { ...nj, ask: oneLine(args.ask, 160) }
          return nj
        })
    }
    await update($, turn, t => t && { ...t, tools: t.tools + 1, edits: t.edits + (isEdit ? 1 : 0), errors: t.errors + (isError ? 1 : 0) })
    const target = toolTarget(e)
    const server = mcpKey(e.tool)
    if (server !== null) await update($, usedMcp, list => (list.includes(server) ? list : [...list, server]))
    if (e.agentId === undefined) {
      const mark: CallMark = { key: `${toolText(e.tool)}${target === '' ? '' : ` → ${target}`}`, isError, isEdit }
      const list = pushCapped(await read($, marks), mark, 60)
      await update($, marks, () => list)
      const alert = detectLoop(list)
      // Eine ignorierte Warnung (gleiche Art, gleiches Ziel) kommt in dieser Session nicht wieder.
      if (alert !== null && !(await read($, loopIgnored)).includes(loopKey(alert))) await update($, loopAlert, () => alert)
    }
    await note($, 'main', `${toolText(e.tool)}${target === '' ? '' : ` → ${target}`}${isError ? ' ✗' : ''}`, { tool: e.tool, target, action: toolAction(e), isError })
    await settleAsk($, e.tool_use_id, ran.deny !== undefined)
    if (e.agentId !== undefined) await update($, agents, list => list.map(a => (a.id === e.agentId ? { ...a, steps: (a.steps ?? 0) + 1 } : a)))
    await trackBackground($, e, ran)
    if (!isError && ran.deny === undefined) {
      if (out !== null && (await read($, dirAlert))?.dir !== out) {
        const L = await langOf($)
        await update($, dirAlert, () => ({ dir: out, tool: e.tool }))
        $.ui.toast(t(L, 'dir.alert', { dir: out }))
        await note($, 'main', t(L, 'log.dirOutside', { dir: out }))
      }
    }
    // Neue Session aus dieser gestartet: gleich in der Arbeitsverzeichnis-Kachel zeigen, nicht erst im 30-s-Takt.
    if (e.tool === 'mcp__ccd_session__start_session') void refreshLinked($).catch(() => undefined)
    return ran
  })

  // Nur beobachten: das Ergebnis geht unverändert durch, nie ein eigenes deny. Abfragen ohne echten Aufruf zählen nicht.
  on('tool.check', async ($, e, next) => {
    const engine = await next(e)
    // Verzeichniswächter: ein erlaubter Wechsel nach außerhalb wird zur Rückfrage; ein deny von unten bleibt.
    const out = engine.decision === 'allow' && (await read($, settings)).dirGuard ? await foreignTarget($, e.tool, e).catch(() => null) : null
    const verdict = out === null ? engine : { decision: 'ask' as const, reason: t(await langOf($), 'dir.ask', { root: (await read($, dirs)).root, dir: out }) }
    const id = e.tool_use_id
    if (id !== undefined) observeProfile(() => toolApproval(profileSt, id, verdict.decision))
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
    // Auto-Effort nur für die Hauptschleife; Subagents behalten ihren Effort.
    const want = e.agentId ? undefined : await effortFor($, e.effort)
    // Checkliste: jeder Model-Request (auch eines Subagents) ist ein Lebenszeichen.
    await touchJob($)
    // Diagnose: den Stream von Hand durchreichen, damit Zeiten (erster Chunk, Thinking, Ausgabe) und Usage beobachtbar sind.
    const stepAt = await stamp($)
    observeProfile(() => stepBegin(profileSt, e.turnId, e.index, e.model, e.agentId, stepAt))
    const gen = next(want === undefined ? e : { ...e, effort: want })
    let step: Awaited<ReturnType<typeof gen.next>> | undefined
    let stepOk = false
    let prevKind = ''
    try {
      step = await gen.next()
      while (step.done !== true) {
        const chunk = step.value
        // Die Uhr nur bei Wechsel der Chunk-Art fragen: ein Host-Aufruf je Chunk würde den Stream bremsen.
        if (chunk.kind !== prevKind) {
          prevKind = chunk.kind
          const at = await stamp($)
          observeProfile(() => stepChunk(profileSt, e.turnId, e.index, { kind: chunk.kind as never, usage: chunk.kind === 'stop' ? chunk.usage : undefined }, at))
        }
        if (chunk.kind === 'stop') observeInspect(() => sampleInput(inspectSt, e.agentId, inputOf(chunk.usage)))
        yield chunk
        step = await gen.next()
      }
      stepOk = true
    } finally {
      // Wie `yield*`: bricht der Aufrufer ab (Esc), geht das Ende auch an den Stream darunter.
      if (step === undefined || step.done !== true) await gen.return(undefined as never).catch(() => undefined)
      const endAt = await stamp($)
      const stepUsage = step !== undefined && step.done === true ? step.value.usage : undefined
      observeProfile(() => stepEnd(profileSt, e.turnId, e.index, endAt, stepUsage, stepOk))
    }
    const result = step.value
    if (!e.agentId) {
      const v = { ...(await read($, modelInfo)), model: e.model, effort: want ?? e.effort }
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
    await touchJob($)
    return spawned
  })

  // Checkliste: turnId des Haupt-Turns für „Pause“ merken; der Start eines Subagent-Laufs ist ein Lebenszeichen.
  on('turn.start', async ($, e, next) => {
    const started = await next(e)
    const turnAt = await stamp($)
    observeProfile(() => turnStart(profileSt, e.turnId, e.agentId, turnAt))
    if (e.agentId === undefined) {
      mainTurnId = e.turnId
      cmdTurnStarted = true
    }
    else await touchJob($)
    return started
  })

  on('turn.complete', async ($, e, next) => {
    const done = await next(e)
    const doneAt = await stamp($)
    observeProfile(() => turnComplete(profileSt, e.turnId, doneAt, e.reason))
    if (!e.agentId) void refreshMr($, true).catch(() => undefined)
    if (!e.agentId && (await read($, settings)).blastBand) void refreshBlast($).catch(() => undefined)
    void repaint($).catch(() => undefined)
    if (e.agentId) {
      const status = e.isAborted ? 'killed' : 'completed'
      const u = e.usage
      const end = {
        status,
        durationMs: e.durationMs,
        ...(u === undefined ? {} : { model: u.model, tokensIn: u.input_tokens + u.cache_read_input_tokens + u.cache_creation_input_tokens, tokensOut: u.output_tokens, cacheHit: cacheHitPct(u) }),
      }
      await update($, agents, list => list.map(a => (a.id === e.agentId ? { ...a, ...end } : a)))
      await touchJob($)
      return done
    }
    stopSpin()
    await finishGlance($, e.answer, e.isAborted)
    // Nach dem Turn ist keine Rückfrage mehr offen (beantwortet oder abgebrochen): Zähler leeren.
    await update($, gate, old => ({ ...gateOf(old), ask: 0, asked: [] }))
    await syncAgents($)
    const fresh = toUsage(await $.session.usage())
    await update($, usage, () => fresh)
    void fetchBreakdown($, true).catch(() => undefined)
    void refreshRepo($).catch(() => undefined)
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
      await update($, lastAnswer, () => e.answer)
      // Nur übernehmen, wenn inzwischen kein neuer Prompt gesendet wurde.
      void suggest($, asked, e.answer)
        .then(async list => ((await read($, lastPrompt)) === asked ? update($, steps, () => list).then(() => mergeHint($)) : undefined))
        .catch(() => undefined)
      await update($, asks, () => [])
      void questionSteps($, e.answer)
        .then(async list => ((await read($, lastPrompt)) === asked ? update($, asks, () => list) : undefined))
        .catch(() => undefined)
    }
    return done
  })

  // Diagnose (Profiler): gemessene Ausführungszeit der Tools und Lebenslauf der Subagents aus den klassischen Hook-Events.
  on('classic.PostToolUse', async ($, e, next) => {
    if (e.duration_ms !== undefined) observeProfile(() => toolExec(profileSt, e.tool_use_id, e.duration_ms ?? 0))
    return next(e)
  }).catch(($, e, next) => (next.called ? undefined : next(e)))
  on('classic.PostToolUseFailure', async ($, e, next) => {
    if (e.duration_ms !== undefined) observeProfile(() => toolExec(profileSt, e.tool_use_id, e.duration_ms ?? 0))
    return next(e)
  }).catch(($, e, next) => (next.called ? undefined : next(e)))
  on('classic.SubagentStart', async ($, e, next) => {
    const at = await stamp($)
    observeProfile(() => subagentStart(profileSt, e.agent_id, e.agent_type, at))
    return next(e)
  }).catch(($, e, next) => (next.called ? undefined : next(e)))
  on('classic.SubagentStop', async ($, e, next) => {
    const at = await stamp($)
    observeProfile(() => subagentStop(profileSt, e.agent_id, at))
    return next(e)
  }).catch(($, e, next) => (next.called ? undefined : next(e)))
  on('classic.StopFailure', async ($, e, next) => {
    const at = await stamp($)
    observeProfile(() => stopFailure(profileSt, e.agent_id, at, String(e.error)))
    return next(e)
  }).catch(($, e, next) => (next.called ? undefined : next(e)))

  on('ui.render', { component: 'AbovePrompt' }, async ($, e, next) => {
    const s = await read($, settings)
    const u = await read($, usage)
    // Kappung auch hier: $.state überlebt Reloads und kann noch Listen älterer Versionen (fünf Schritte) halten.
    const next3 = e.props.isWorking || !s.steps ? [] : (await read($, steps)).slice(0, 3)
    const asks3 = e.props.isWorking || !s.steps ? [] : (await read($, asks)).slice(0, 3)
    // Die Zeile mit „↻ Neue“ steht immer, wenn Folge-Schritte an sind und Claude nicht arbeitet, auch ohne Vorschläge.
    const showSteps = s.steps && !e.props.isWorking
    const busySteps = await read($, stepsBusy)
    // Checkliste: zeigt sich auch ohne Messung, sobald der Job Plan, Tool-Arbeit oder eine Rückfrage hat.
    const job = s.glance ? await read($, glanceJob) : null
    const showJob = job !== null && isWorthShowing(job)
    if (!drawsOn(e.surface) || e.props.hasSurvey || (u === null && !showJob && next3.length === 0 && (await read($, loopAlert)) === null && (await read($, dirAlert)) === null)) return next(e)
    await read($, tick)
    const now = await $.clock.now()
    const liveAgents = (await read($, agents)).filter(a => LIVE.has(a.status))
    const running = liveAgents.length
    const ui = elementsOf($, e)
    const { Box, Text, Button } = ui
    // Ringe nur, wo die Oberfläche Svg zeichnet; sonst bleibt der Textbalken.
    const Svg = 'Svg' in ui ? ui.Svg : undefined
    // Mit Tooltip ist das Ring-Svg interaktiv (Sandbox-Frame, `<title>` wird schwebend angezeigt). Der Frame flackert bei jedem
    // Neuzeichnen: darum nur im Leerlauf: weder Turn noch laufender Subagent; und der Text ändert sich nur minutengenau
    // (sonst baut die Oberfläche ihn neu auf).
    const tipNow = Math.floor(now / 60_000) * 60_000
    const L = s.lang
    const ring = (percent: number, color: keyof typeof RING_HEX, tip = '') =>
      !Svg || !s.rings ? null : s.ctxTip && tip !== '' && !e.props.isWorking && running === 0 ? (
        <Svg source={ringSvg(percent, RING_HEX[color], 20, tip)} alt={tip.split(NL)[0] ?? ''} width={20} height={20} isInteractive />
      ) : (
        <Svg source={ringSvg(percent, RING_HEX[color], 20)} alt={`${Math.round(percent)} %`} width={20} height={20} />
      )
    const head = titleRow(ui, await read($, about), await read($, logo))
    const where = s.branch ? repoRow(ui, await read($, repo)) : null
    const alert = await read($, loopAlert)
    const dirWarn = await read($, dirAlert)
    const loopText = alert === null ? '' : t(L, alert.kind === 'error' ? 'loop.error' : 'loop.edit', { target: alert.target, n: alert.n })
    const fillCompact = async () => {
      const focus = compactFocus(await read($, lastPrompt), await read($, touched), await read($, steps), L)
      await $.prompt.fill({ text: focus === '' ? '/compact' : `/compact ${focus}` })
    }
    // Gruppen ohne Rahmen, getrennt durch einen gedimmten Strich (kompakter als Karten).
    const card = (...children: RenderChildren[]) => (
      <Box flexDirection="row" columnGap={2} alignItems="center">
        {children}
      </Box>
    )
    const sep = (key: string) => <Text key={key} color={THEME.border}>│</Text>
    // Waagerechte Trennlinie als Text: der Desktop zeichnet das Svg-Band nicht zuverlässig, Text schon.
    const sepLine = (key: string) => <Text key={key} color={THEME.border} wrap="truncate-end">{'─'.repeat(Math.max(10, e.props.bodyColumns - 2))}</Text>
    const mini = (key: string, label: string, onPress: () => void) => pill(ui, 'band', key, label, onPress)
    const mi = await read($, modelInfo)
    const pick = await read($, effortPick)
    const lim = u === null ? null : limitAlert(u.limits)
    const handoffMsg = await read($, handoffNote)
    const stepCols = Math.max(20, Math.floor((e.props.bodyColumns - (e.surface === 'terminal' ? 8 : 4)) / 3))
    const effortMsg = await read($, effortNote)
    const shift = await read($, effortShift)
    // Auto-Effort-Status: nach einem Urteil „Haiku: alt ─→ neu“ als kurze Animation in Stufenfarben, danach „Haiku: neu“.
    const effortView = () => {
      const lvl = (v: string, bold = false) => <Text color={effortColor(v)} bold={bold}>{v}</Text>
      if (shift === null || effortMsg !== t(L, 'effort.judged', { v: shift.to })) return <Text key="effort-note" color={THEME.textMuted}>{effortMsg}</Text>
      const by = <Text color={THEME.textMuted}>{t(L, 'effort.by')} </Text>
      if (shift.from === shift.to) return <Text key="effort-note">{by}{lvl(shift.to, true)}<Text color={THEME.textMuted}> {t(L, 'effort.same')}</Text></Text>
      if (shift.step >= EFFORT_STEPS || shift.from === null) return <Text key="effort-note">{by}{lvl(shift.to, true)}</Text>
      // Pfeil wächst in der ersten Hälfte, die neue Stufe erscheint ab der Mitte.
      const half = EFFORT_STEPS / 2
      const arrow = `${'─'.repeat(Math.min(shift.step, half))}→`
      return (
        <Text key="effort-note">
          {by}
          <Text color={effortColor(shift.from)} dimColor={shift.step >= half}>{shift.from}</Text>
          <Text color={THEME.textMuted}> {arrow} </Text>
          {shift.step >= half && lvl(shift.to, true)}
        </Text>
      )
    }
    // Sparmodus: Punkt in Statusfarbe (an grün, aus rot) mit Erklärung als Tooltip, daneben der Knopf mit dem Zustand im Label.
    // Interaktiv (Tooltip) nur im Leerlauf, wie die Ringe: interaktive Svg flackern beim Neuzeichnen.
    const saveToggle = () => {
      const tip = t(L, 'alert.saveTip', { state: t(L, s.saveMode ? 'alert.saveState.on' : 'alert.saveState.off') })
      const color = s.saveMode ? THEME.success : THEME.error
      const idle = !e.props.isWorking && running === 0
      return (
        <Box key="save-mode-box" flexDirection="row" columnGap={1} alignItems="center">
          {Svg && (idle ? (
            <Svg key="save-dot" source={ringSvg(100, color, 12, tip)} alt={tip.split(NL)[0] ?? ''} width={12} height={12} isInteractive />
          ) : (
            <Svg key="save-dot" source={ringSvg(100, color, 12)} alt={tip.split(NL)[0] ?? ''} width={12} height={12} />
          ))}
          {mini('save-mode', t(L, s.saveMode ? 'alert.saveOn' : 'alert.saveOff'), () => void setSettings($, { saveMode: !s.saveMode }).catch(() => undefined))}
        </Box>
      )
    }
    const toggleAuto = () => void update($, effortBase, () => null).then(() => setSettings($, { autoEffort: !s.autoEffort })).catch(() => undefined)

    // Im Terminal setzt ein Rahmen in orgaMAX-Blau (om-blue-500) auf dunkelblauem Grund das Band vom Transcript ab;
    // der Desktop rahmt es selbst. Voller blauer Grund wäre für die Statusfarben (grün, gelb) zu kontrastarm.
    const term = e.surface === 'terminal'
    const body = (
      <Box flexDirection="column" {...(term ? { borderStyle: 'round', borderColor: THEME.accentFill, backgroundColor: TINT.accent, paddingX: 1 } : {})}>
        {/* Checkliste ganz oben, über der Titelzeile. */}
        {showJob && job !== null && glanceBand($, ui, job, { L, now, avg: await read($, glanceAvg), asking: gateOf(await read($, gate)).ask, live: liveAgents })}
        {showJob && sepLine('glance-sep')}
        <Box flexDirection="row" justifyContent="space-between" alignItems="center">
          <Box flexDirection="row" columnGap={2} alignItems="center">
            {head}
            {where}
            {/* MR-Status in der Kopfzeile hinter dem Branch, mit etwas Abstand. */}
            {s.mrBand && <Box key="mr-head" marginLeft={2}>{await mrBandRow($, ui, L, (await read($, devMock)) ? mockData(now).mr : null)}</Box>}
          </Box>
          {/* Features rechts; Details steht in der Fußzeile unter dem Prompt, die Legende im Tooltip der Limit-Ringe. */}
          <Box flexDirection="row" columnGap={1} alignItems="center">
          {s.cardCatchUp && mini('summary', t(L, 'common.summary'), () => void $.ui.open({ ...panes(L).summary, focus: true }).catch(() => undefined))}
          {/* $.command.run aus dem Knopf kam im Desktop nicht an: Befehl ins Eingabefeld, Enter führt ihn aus. */}
          {mini('features', t(L, 'common.features'), () => void $.prompt.fill({ text: '/cockpit-features' }).catch(() => undefined))}
          </Box>
        </Box>
        {u !== null && metrics(u, await read($, lastTurnAt), await read($, cacheHit), await read($, breakdown))}
        {diagChips($, ui, s, u?.contextPercent, now)}
        {s.breath && Svg && (await (async () => {
          const tn = await read($, turn)
          const labels = { in: t(L, 'breath.in'), sip: t(L, 'breath.sip'), hold: t(L, 'breath.hold'), out: t(L, 'breath.out') }
          const name = t(L, `breath.name.${s.breathKey}`)
          // Während eines Turns zeichnet das Band ständig neu und startete SMIL dabei von vorn: dann Einzelbilder im 250-ms-Takt
          // (Atom `frame`), synchron zur Arbeitszeile. Nur im Leerlauf des Dev-Modes animiert SMIL selbst.
          const working = e.props.isWorking && tn !== null && tn.durationMs === undefined
          if (!working && !(await read($, devMock))) return null
          if (working) await read($, frame)
          const img = working
            ? <Svg key="breath" source={breathFrameSvg(breathOf(s.breathKey), now - (tn?.startedAt ?? now), labels, name, BREATH_COLOR[breathOf(s.breathKey).key], THEME.textMuted)} alt={t(L, 'settings.breath')} height={56} />
            : <Svg key="breath" source={breathSvg(breathOf(s.breathKey), labels, name, BREATH_COLOR[breathOf(s.breathKey).key], THEME.textMuted)} alt={t(L, 'settings.breath')} height={56} />
          // Durchschalten: nächste Übung der Reihe nach, gilt sofort und bleibt gespeichert.
          const idx = BREATHS.findIndex(b => b.key === s.breathKey)
          const nextKey = (BREATHS[(idx + 1) % BREATHS.length] ?? BREATHS[0])!.key
          return (
            <Box key="breath-block" flexDirection="column">
              {sepLine('breath-sep')}
              <Box key="breath-row" flexDirection="row" columnGap={1} alignItems="center" justifyContent="center">
                {img}
                {pill(ui, 'band', 'breath-next', t(L, 'breath.next'), () => void setSettings($, { breathKey: nextKey }).catch(() => undefined))}
              </Box>
            </Box>
          )
        })())}
        {dirWarn !== null && (
          <Box key="dir-warn" flexDirection="row" columnGap={2} alignItems="center" backgroundColor={TINT.error} paddingX={1}>
            <Text color={THEME.error} bold wrap="truncate-end">⚠ {t(L, 'dir.alert', { dir: dirWarn.dir })}</Text>
          </Box>
        )}
        {alert !== null && (
          <Box key="loop" flexDirection="row" columnGap={2} alignItems="center">
            <Text color={THEME.warning} wrap="truncate-end">⚠ {loopText}</Text>
            {pill(ui, 'band', 'loop-fix', t(L, 'loop.action'), () => void $.prompt.fill({ text: t(L, 'loop.prompt', { what: loopText }) }).catch(() => undefined))}
            {pill(ui, 'band', 'loop-ignore', t(L, 'loop.ignore'), () => void update($, loopIgnored, l => [...l, loopKey(alert)]).then(() => update($, loopAlert, () => null)).catch(() => undefined))}
          </Box>
        )}
        {showSteps && (
          // Drei Spalten gleicher Breite: „1: Label“ (Taste 1–3, auch im leeren Eingabefeld), die Begründung als Tooltip am ⓘ
          // dahinter; ohne Svg (Terminal) als Zeile darunter.
          <Box flexDirection="column">
            {sepLine('steps-sep')}
            <Box flexDirection="row" columnGap={2} alignItems="flex-start">
            <Text color={THEME.warning} bold>{t(L, 'band.next')}</Text>
            {next3.length === 0 && <Text key="steps-none" color={THEME.textMuted}>{t(L, 'band.noSteps')}</Text>}
            {next3.map((s, i) => (
              <Box key={`step-col-${i}`} flexDirection="column" width={stepCols}>
              <Box key={`step-row-${i}`} flexDirection="row" columnGap={1} alignItems="center">
                {s.fromClaude && chip(ui, 'band', `step-cc-${i}`, 'CC')}
                <Button
                  key={`step-${i}`}
                  label={clip(s.label, Math.max(12, stepCols - (s.fromClaude ? 8 : 4)))}
                  hotkey={String(i + 1)}
                  plain
                  // Nur ins Eingabefeld: Enter sendet ihn als Eingabe der Person, damit andere Mods (effortless) ihn wie getippt bewerten; startTurn läuft dann im eigenen prompt.submit-Hook.
                  onPress={() => void $.prompt.fill({ text: s.prompt }).catch(() => undefined)}
                />
                {Svg && s.why !== undefined && <Svg key={`step-why-${i}`} source={infoSvg(s.why, THEME.textMuted)} alt={s.why} width={14} height={14} isInteractive />}
              </Box>
                {!Svg && s.why !== undefined && <Text color={THEME.textMuted} wrap="truncate-end">{s.why}</Text>}
              </Box>
            ))}
            {pill(ui, 'band', 'steps-refresh', t(L, 'band.moreSteps'), () => void refreshSteps($).catch(() => undefined), busySteps)}
            </Box>
            {/* Zweite Zeile: Claudes Rückfragen aus der letzten Antwort als Aufträge (Tasten 4–6), nur ins Eingabefeld. */}
            {asks3.length > 0 && (
              <Box key="asks-row" flexDirection="row" columnGap={2} alignItems="flex-start">
                <Text color={THEME.warning} bold>{t(L, 'band.asks')}</Text>
                {asks3.map((a, i) => (
                  <Box key={`ask-col-${i}`} width={stepCols}>
                    <Button
                      key={`ask-${i}`}
                      label={clip(a.label, Math.max(12, stepCols - 4))}
                      hotkey={String(i + 4)}
                      plain
                      onPress={() => void $.prompt.fill({ text: a.prompt }).catch(() => undefined)}
                    />
                  </Box>
                ))}
              </Box>
            )}
          </Box>
        )}
      </Box>
    )
    // Der Rahmen selbst nimmt keinen Grund an (nur sein Inneres): eine äußere Box mit demselben Grund füllt auch die Rahmenzellen.
    const ours = term ? <Box key="band-bg" flexDirection="column" backgroundColor={TINT.accent}>{body}</Box> : body
    // Bänder anderer Mods (ponytail, effortless …) unter unserem mitzeichnen statt sie zu verdecken.
    const theirs = await next(e)
    return <Box key="band-stack" flexDirection="column">{ours}{theirs}</Box>

    function metrics(u: Usage, lastAt: number | null, hit: number | null, bd: Breakdown | null) {
      const ttl = u.limits.length > 0 ? 3600_000 : 300_000
      const cache = cacheLeftMs(lastAt, u.limits.length > 0, now)
      const isFull = (u.contextPercent ?? 0) >= COMPACT_AT
      const ctxTone = contextColor(u.contextPercent ?? 0)
      // Ohne bekannten letzten Request ist der Cache unbekannt, nicht kalt.
      const pays = lastAt !== null && handoffPays(u.contextPercent, cache)
      const level = mi?.effort ?? pick
      const levelText = level === undefined || level === null ? '' : `${String(level).charAt(0).toUpperCase()}${String(level).slice(1)} · `
      return (
        <Box flexDirection="row" flexWrap="wrap" columnGap={2} alignItems="center">
          {/* Reihenfolge: Limits (Usage), Effort, Kontext, Cache/Kosten. */}
          {s.limits && u.limits.length > 0 &&
            card(
              ...u.limits.map(l => (
                <Box key={`lim-${l.kind}`} flexDirection="row" columnGap={1} alignItems="center">
                  {ring(l.percentUsed, limitColor(l, now), s.legend ? `${limitTip(l, tipNow, L)}${NL}${NL}${legendTip(L)}` : limitTip(l, tipNow, L))}
                  <Text color={THEME[limitColor(l, now)]}>
                    {LIMIT_LABEL[l.kind] ?? l.kind} {Math.round(l.percentUsed)}%
                    {/* Restzeit bis zum Reset immer dahinter; ohne Svg (kein Ring-Tooltip) zusätzlich die Uhrzeit. */}
                    {l.resetsAt ? <Text color={THEME.textMuted}> · ↻ {resetLeft(l.resetsAt, now)}{Svg ? '' : ` (${resetLabel(l.resetsAt, now, L)})`}</Text> : null}
                  </Text>
                </Box>
              )),
              // Ab 80 % und mit Auto-Effort: Sparmodus direkt an der Limit-Anzeige statt in einer eigenen Warnzeile.
              lim !== null && s.autoEffort && saveToggle(),
            )}
          {s.limits && u.limits.length > 0 && sep('sep-lim')}
          {mi !== null && card(
            <Text key="effort" color={effortColor(level)}>◆ {levelText}{modelLabel(mi.model)}</Text>,
            mini('auto', t(L, s.autoEffort ? 'effort.autoOn' : 'effort.auto'), toggleAuto),
            s.autoEffort && effortMsg !== '' && effortView(),
          )}
          {mi !== null && sep('sep-effort')}
          {card(
            <Box key="ctx" flexDirection="row" columnGap={1} alignItems="center">
              {ring(u.contextPercent ?? 0, ctxTone, contextTip(u, bd, L))}
              <Text color={THEME[ctxTone]} bold={isFull}>{t(L, 'common.context')} {Svg && s.rings ? '' : `${bar(u.contextPercent ?? 0, 8)} `}{Math.round(u.contextPercent ?? 0)}%</Text>
            </Box>,
            // Wenn ein frischer Chat günstiger wäre, wird Handoff die Hauptaktion; Kompaktieren bleibt daneben.
            pays && <Button key="handoff" label={t(L, 'handoff.action')} variant="primary" onPress={() => void writeHandoff($).catch(() => undefined)} />,
            // Schreibt /compact mit dem, was erhalten bleiben muss, ins Eingabefeld; Enter startet es (anpassbar vor dem Senden).
            // Button nimmt (Engine 2.1.295) keine farbigen Text-Kinder: die Ampel trägt der getönte Grund, der Hover die Schriftfarbe.
            <Box key="compact-pill" backgroundColor={TINT[ctxTone === 'accent' ? 'warning' : ctxTone]} paddingX={1}>
              <Button key="compact" label={t(L, 'band.compact')} plain hover={{ color: THEME[ctxTone === 'accent' ? 'warning' : ctxTone] }} onPress={() => void fillCompact().catch(() => undefined)} />
            </Box>,
            handoffMsg !== '' && <Text key="handoff-note" color={THEME.textMuted}>{handoffMsg}</Text>,
          )}
          {s.cache && sep('sep-cache')}
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
        </Box>
      )
    }
  })

  // Profiling: Kontext-Inspektor, Agent-Profiler und Blast-Radius in einem Panel; die drei Befehle liefern Berichte als Text.
  on('ui.render', { component: 'Pane', requestId: 'tenvan-cc-profiling' }, async ($, e, next) => (drawsOn(e.surface) ? profilingPane($, e) : next(e)))

  on('command.run', { command: 'cockpit-profiling' }, async $ => {
    await $.ui.open({ ...panes(await langOf($)).profiling, focus: true })
    return {}
  })
  on('command.run', { command: 'cockpit-kontext' }, async ($, e) => {
    const L = await langOf($)
    if ((e.args ?? '').trim() === 'text') return { text: inspectReport(inspectSum(), L) }
    await $.ui.open({ ...panes(L).profiling, focus: true })
    return {}
  })
  on('command.run', { command: 'cockpit-profil' }, async ($, e) => {
    const L = await langOf($)
    const arg = (e.args ?? '').trim()
    if (arg === 'text' || arg === 'turn') return { text: profileReport(profileSum(await $.clock.now()), await $.clock.now(), L, arg === 'turn' ? 'turn' : 'session') }
    await $.ui.open({ ...panes(L).profiling, focus: true })
    return {}
  })
  on('command.run', { command: 'cockpit-blast' }, async ($, e) => {
    const text = await blastCommand($, e.args ?? '')
    return text === '' ? {} : { text }
  })

  // Zusammenfassung in eigenem Panel rechts: Knöpfe oben, darunter die Abschnitte als einfacher Text (kein Markdown-Rendering).
  on('ui.render', { component: 'Pane', requestId: 'tenvan-cc-summary' }, async ($, e, next) => {
    if (!drawsOn(e.surface)) return next(e)
    const s = await read($, settings)
    const realCu = await read($, catchUp)
    // Dev-Mode: Mock-Zusammenfassung, damit die Optik ohne Haiku-Lauf sichtbar ist.
    const cu = (await read($, devMock)) ? { ...realCu, summary: mockData(await $.clock.now()).summary } : realCu
    const L = s.lang
    const marg = s.paneMargins ? 1 : 0
    const ui = elementsOf($, e)
    const { Box, Text, Button } = ui

    return (
      <Box flexDirection="column" gap={1}>
        {await paneHead($, ui, L)}
        <Box flexDirection="row" columnGap={2} alignItems="center">
          <Button key="cu-refresh" label={t(L, 'summary.refresh')} variant="primary" onPress={() => void runCatchUp($, false).catch(() => undefined)} />
          {pill(ui, 'pane', 'cu-rebuild', t(L, 'summary.rebuild'), () => void rebuildCatchUp($).catch(() => undefined))}
          <Text dimColor>{cu.status !== '' ? cu.status : cu.updatedAt > 0 ? t(L, 'summary.updated', { time: clockTime(new Date(cu.updatedAt).toISOString()) }) : t(L, 'summary.none')}</Text>
        </Box>
        <Text dimColor>{t(L, 'summary.status', { auto: t(L, s.catchUp ? 'common.on' : 'common.off'), lang: t(L, L === 'de' ? 'common.langDe' : 'common.langEn') })}</Text>
        {cu.summary === '' && <Text dimColor>{t(L, 'summary.empty')}</Text>}
        {/* Je Überschrift eine Karte im Wechsel der Kartentöne; Aufzählungen mit hängendem Einzug, Absätze mit Luft dazwischen. */}
        {summarySections(summaryLines(cu.summary)).map((sec, i) => (
          <Box key={`cu-sec-${i}`} flexDirection="column" rowGap={1} backgroundColor={cycleTint(i)} marginX={marg} paddingX={2} paddingY={1}>
            {sec.heading !== undefined && <Text bold color={THEME.accent}>{sec.heading}</Text>}
            {sec.lines.map((l, j) =>
              l.kind === 'bullet' ? (
                <Box key={`cu-l-${i}-${j}`} flexDirection="row">
                  <Box width={2} flexShrink={0}>
                    <Text color={THEME.accent}>•</Text>
                  </Box>
                  <Text wrap="wrap">{l.text}</Text>
                </Box>
              ) : (
                <Text key={`cu-l-${i}-${j}`} wrap="wrap">{l.text}</Text>
              ),
            )}
          </Box>
        ))}
      </Box>
    )
  })

  // Merge-Request-Panel (GitLab): ausführlich, eigenes Fenster; im Dev-Mode mit Mock-MR.
  on('ui.render', { component: 'Pane', requestId: 'tenvan-cc-mr' }, async ($, e, next) => {
    if (!drawsOn(e.surface)) return next(e)
    const s = await read($, settings)
    const L = s.lang
    const marg = s.paneMargins ? 1 : 0
    const ui = elementsOf($, e)
    const now = await $.clock.now()
    await read($, tick)
    const m = (await read($, devMock)) ? mockData(now) : null
    return (
      <ui.Box flexDirection="column">
        {await paneHead($, ui, L)}
        {...(await mrPaneBody($, ui, L, m === null ? null : { mr: m.mr, mrMine: m.mrMine, mrDetails: m.mrDetails }, now, marg))}
      </ui.Box>
    )
  })

  on('command.run', { command: 'cockpit-mr' }, async $ => {
    const L = (await read($, settings)).lang
    await $.ui.open({ ...panes(L).mr, focus: true })
    void refreshMr($, true).then(() => loadMineDetails($, true)).catch(() => undefined)
    return {}
  })

  // Verlauf der Checkliste: Aufgaben eines Tages in diesem Projekt, Blättern nach Tagen, Tages- und Wochenbericht zum Kopieren.
  on('ui.render', { component: 'Pane', requestId: 'tenvan-cc-history' }, async ($, e, next) => {
    if (!drawsOn(e.surface)) return next(e)
    const s = await read($, settings)
    const L = s.lang
    const marg = s.paneMargins ? 1 : 0
    const ui = elementsOf($, e)
    const { Box, Text } = ui
    const today = dayKey(await $.clock.now())
    const picked = await read($, glanceDay)
    // Nach einem Reload ist noch kein Tag geladen: heute nachladen, das Atom zeichnet dann neu.
    if (picked === '') void loadHistory($, today).catch(() => undefined)
    const day = picked || today
    const cwd = await projectCwd($)
    const list = (await read($, glanceList)).filter(r => r.cwd === cwd && (r.steps.length > 0 || r.tools > 0))
    const go = (d: string) => () => void loadHistory($, d).catch(() => undefined)
    const at = (ms: number) => clockTime(new Date(ms).toISOString())
    const OUTCOME_COLOR = { done: THEME.success, needsYou: THEME.warning, stopped: THEME.textMuted } as const
    const spent = list.reduce((a, r) => a + Math.max(0, r.endedAt - r.startedAt), 0)
    const stepsDone = list.reduce((a, r) => a + r.steps.filter(x => x.done).length, 0)
    return (
      <Box flexDirection="column">
        {await paneHead($, ui, L)}
        <Box key="hist-nav" flexDirection="row" columnGap={2} alignItems="center">
          {pill(ui, 'pane', 'hist-prev', t(L, 'glance.prevDay'), go(shiftDay(day, -1)))}
          {day !== today && pill(ui, 'pane', 'hist-today', t(L, 'glance.today'), go(today))}
          {day < today && pill(ui, 'pane', 'hist-next', t(L, 'glance.nextDay'), go(shiftDay(day, 1)))}
          <Text bold>{day === today ? `${t(L, 'glance.today')} · ${day}` : day}</Text>
        </Box>
        {!s.glanceHistory && <Text key="hist-off" color={THEME.warning} wrap="wrap">{t(L, 'glance.historyOff')}</Text>}
        {tile(
          ui,
          'pane',
          { key: 'hist-card', tint: CARD_TINT.cardPlan ?? TINT.neutral, marg },
          list.length === 0 && <Text key="hist-none" dimColor>{t(L, 'glance.noJobs')}</Text>,
          ...list.map(r => (
            <Box key={`hist-${r.id}`} flexDirection="column" marginBottom={1}>
              <Box flexDirection="row" columnGap={1}>
                <Text dimColor>{at(r.startedAt)}</Text>
                <Box width={10} flexShrink={0}><Text color={OUTCOME_COLOR[r.outcome]}>{t(L, `glance.outcome.${r.outcome}`)}</Text></Box>
                <Text bold wrap="truncate-end">{r.title}</Text>
              </Box>
              <Text dimColor wrap="truncate-end">
                {'      '}
                {[r.steps.length > 0 ? t(L, 'glance.stepsOf', { done: r.steps.filter(x => x.done).length, total: r.steps.length }) : '', duration(Math.max(0, r.endedAt - r.startedAt)), r.files.length > 0 ? t(L, 'glance.files', { list: r.files.slice(0, 6).join(', ') + (r.files.length > 6 ? ' …' : '') }) : '']
                  .filter(x => x !== '')
                  .join(' · ')}
              </Text>
              {r.outcome === 'needsYou' && r.question !== '' && <Text color={THEME.warning} wrap="truncate-end">{'      '}? {r.question}</Text>}
            </Box>
          )),
          list.length > 0 && <Text key="hist-total" bold>{t(L, 'glance.total', { n: list.length, steps: stepsDone, d: duration(spent) })}</Text>,
        )}
        <Box key="hist-copy" flexDirection="row" columnGap={2} marginTop={1}>
          {pill(ui, 'pane', 'hist-day-report', t(L, 'glance.dayReport'), () => void copyReport($, 'day', e.surface).catch(() => undefined))}
          {pill(ui, 'pane', 'hist-week-report', t(L, 'glance.weekReport'), () => void copyReport($, 'week', e.surface).catch(() => undefined))}
        </Box>
      </Box>
    )
  })

  // Einstellungsseite: jedes Detail von Band, Panel und Chat ein- oder ausblenden; gilt sofort und für alle Sessions.
  on('ui.render', { component: 'Pane', requestId: 'tenvan-cc-settings' }, async ($, e, next) => {
    if (!drawsOn(e.surface)) return next(e)
    const s = await read($, settings)
    const L = s.lang
    const marg = s.paneMargins ? 1 : 0
    const ui = elementsOf($, e)
    const { Box, Text, Button } = ui
    const groups = [...new Set(SETTING_DEFS.map(d => d.group))]
    // Karten wie im Hauptpanel: Leerzeile davor, Grund im Wechsel der Kartentöne; ▾/▸ im Kopf klappt zu (gleicher Speicher
    // `collapsed` wie das Hauptpanel, Schlüssel mit Präfix `set:`).
    const folded = await read($, collapsed)
    const card = (i: number, id: string, head: RenderElement, ...children: RenderChildren[]) => {
      const key = `set:${id}`
      const fold = { id: `set-fold-${id}`, open: !folded.includes(key), onToggle: () => void toggleCard($, key).catch(() => undefined) }
      return tile(ui, 'pane', { key: `set-card-${id}`, tint: cycleTint(i), marg, head, fold }, ...children)
    }
    const cardKeys = ['lang', 'tiles', 'breath', ...groups].map(id => `set:${id}`)
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
        <Box flexDirection="row" columnGap={2} marginTop={1} marginBottom={marg ? 0 : 1}>
          {pill(ui, 'pane', 'set-fold-all', t(L, 'settings.foldAll'), () => void foldCards($, cardKeys, true).catch(() => undefined))}
          {pill(ui, 'pane', 'set-unfold-all', t(L, 'settings.unfoldAll'), () => void foldCards($, cardKeys, false).catch(() => undefined))}
        </Box>
        {card(
          0,
          'lang',
          <Text bold>{t(L, 'settings.langTitle')}</Text>,
          <Text dimColor wrap="wrap">{t(L, 'settings.langHint')}</Text>,
          // Je Sprache eine Zeile im selben Zwei-Spalten-Raster wie die Schalter darunter.
          ...([['de', t(L, 'common.langDe')], ['en', t(L, 'common.langEn')]] as const).map(([lang, name]) => (
            <Box key={`lang-row-${lang}`} flexDirection="row" columnGap={1} marginTop={1}>
              <Box width={SWITCH_COLS} flexShrink={0}>
                {langButton(lang, L === lang ? t(L, 'settings.on') : t(L, 'settings.off'))}
              </Box>
              <Text bold>{name}</Text>
            </Box>
          )),
        )}
        {card(
          1,
          'tiles',
          <Text bold>{t(L, 'settings.agentTilesTitle')}</Text>,
          <Text dimColor wrap="wrap">{t(L, 'settings.agentTilesHint')}</Text>,
          <Box key="tiles-row" flexDirection="row" columnGap={1} marginTop={1} alignItems="center">
            {pill(ui, 'pane', 'tiles-minus', ' − ', () => void setSettings($, { agentTiles: s.agentTiles - TILES.step }).catch(() => undefined))}
            <Box width={4} justifyContent="center"><Text bold>{s.agentTiles}</Text></Box>
            {pill(ui, 'pane', 'tiles-plus', ' + ', () => void setSettings($, { agentTiles: s.agentTiles + TILES.step }).catch(() => undefined))}
          </Box>,
        )}
        {card(
          1,
          'breath',
          <Text bold>{t(L, 'settings.breathTitle')}</Text>,
          <Text dimColor wrap="wrap">{t(L, 'settings.breathHint')}</Text>,
          ...BREATHS.map(b => (
            <Box key={`breath-row-${b.key}`} flexDirection="row" columnGap={1} marginTop={1}>
              <Box width={SWITCH_COLS} flexShrink={0}>
                {s.breathKey === b.key ? (
                  <Button key={`breath-${b.key}`} label={`● ${t(L, 'settings.on')}`} variant="primary" onPress={() => undefined} />
                ) : (
                  <Button key={`breath-${b.key}`} label={`○ ${t(L, 'settings.off')}`} dimColor onPress={() => void setSettings($, { breathKey: b.key }).catch(() => undefined)} />
                )}
              </Box>
              <Text bold color={BREATH_COLOR[b.key]}>{t(L, `breath.name.${b.key}`)}</Text>
            </Box>
          )),
        )}
        {groups.map((group, i) => {
          const keys = keysOf(group)
          return card(
            i + 2,
            group,
            <Box flexGrow={1} flexDirection="row" flexWrap="wrap" justifyContent="space-between" alignItems="center" columnGap={2}>
              <Text bold>{t(L, `settings.group.${group}`)}</Text>
              <Box flexDirection="row" flexWrap="wrap" columnGap={2} alignItems="center">
                <Text dimColor>{t(L, 'settings.count', { n: keys.filter(k => s[k]).length, m: keys.length })}</Text>
                {pill(ui, 'pane', `grp-on-${i}`, t(L, 'settings.allOn'), () => setAll(group, true))}
                {pill(ui, 'pane', `grp-off-${i}`, t(L, 'settings.allOff'), () => setAll(group, false))}
              </Box>
            </Box>,
            ...keys.map(k => {
              const flip = () => void setSettings($, { [k]: !s[k] } as Partial<ModSettings>).catch(() => undefined)
              // Der Desktop zeichnet keine Text-Kinder eines Buttons, nur das Label: der Schalterzustand steht deshalb im Label.
              return (
                // Zwei Spalten: Schalter in fester Breite links, Label und Hinweis bündig rechts daneben.
                <Box key={`row-${k}`} flexDirection="row" columnGap={1} marginTop={1}>
                  <Box width={SWITCH_COLS} flexShrink={0}>
                    {s[k] ? (
                      <Button key={`set-${k}`} label={`● ${t(L, 'settings.on')}`} variant="primary" onPress={flip} />
                    ) : (
                      <Button key={`set-${k}`} label={`○ ${t(L, 'settings.off')}`} dimColor onPress={flip} />
                    )}
                  </Box>
                  <Box flexDirection="column" flexShrink={1}>
                    <Text bold wrap="truncate-end">{settingLabel(k, L)}</Text>
                    <Text dimColor wrap="wrap">{settingHint(k, L)}</Text>
                  </Box>
                </Box>
              )
            }),
          )
        })}
        <Box flexDirection="row" paddingTop={1} columnGap={2}>
          <Button key="set-reset" label={t(L, 'settings.reset')} onPress={() => void resetSettings($).catch(() => undefined)} />
          {pill(ui, 'pane', 'set-reload', t(L, 'settings.reload'), () => void reloadSettings($).catch(() => undefined))}
        </Box>
      </Box>
    )
  })

  on('ui.render', { component: 'Pane', requestId: 'tenvan-cc' }, async ($, e, next) => {
    if (!drawsOn(e.surface)) return next(e)
    await read($, tick)
    const now = await $.clock.now()
    const s = await read($, settings)
    const realBd = await read($, breakdown)
    const bdNote = await read($, breakdownNote)
    const [mark, realPlace] = await Promise.all([read($, logo), read($, repo)])
    const [realU, realTn, realGate, realList, realEntries, realLast, realHit, info] = await Promise.all([
      read($, usage), read($, turn), read($, gate), read($, agents), read($, log), read($, lastTurnAt), read($, cacheHit), read($, about),
    ])
    // Dev-Mode: alle Panel-Werte aus mockData, die echten Atome bleiben unberührt.
    const dev = await read($, devMock)
    const m = dev ? mockData(now) : null
    const place = m?.repo ?? realPlace
    const u = m?.usage ?? realU
    const tn = m?.turn ?? realTn
    const g = gateOf(m?.gate ?? realGate)
    const list = m?.agents ?? realList
    const entries = m?.log ?? realEntries
    const last = m?.lastTurnAt ?? realLast
    const hit = m?.cacheHit ?? realHit
    const cache = u ? cacheLeftMs(last, u.limits.length > 0, now) : 0
    const bdPanel = m?.breakdown ?? realBd
    const L = s.lang
    const P = panes(L)
    const ui = elementsOf($, e)
    const { Box, Text, Button } = ui
    const Svg = 'Svg' in ui ? ui.Svg : undefined
    // Kopf jeder Karte: ▾/▸ klappt zu (nur Session, Atom `collapsed`), ✕ schaltet die Karte in den Einstellungen ab.
    const folded = await read($, collapsed)
    const isOpen = (key: SettingKey) => !folded.includes(key)
    const headButtons = (key: SettingKey) => (
      <Box flexDirection="row" columnGap={1}>
        {pill(ui, 'pane', `card-fold-${key}`, isOpen(key) ? '▾' : '▸', () => void toggleCard($, key).catch(() => undefined))}
        {pill(ui, 'pane', `card-close-${key}`, '✕', () => void setSettings($, { [key]: false } as Partial<ModSettings>).catch(() => undefined))}
      </Box>
    )
    // Eine leere Zeile zwischen den Karten: mit gedimmtem Grund braucht es eine sichtbare Lücke, Box rechnet nur in ganzen Zeilen.
    const gap = () => marg ? <Box height={1} /> : null
    // Jede Karte trägt ihren Abstand nach oben selbst: abgeschaltete Karten hinterlassen keine doppelte Lücke.
    const spaced = (inner: RenderElement) => (
      <Box flexDirection="column" flexGrow={1}>
        {gap()}
        {inner}
      </Box>
    )
    // Zeile der Übersicht: Bezeichnung in fester Spalte, Wert daneben; Prozente dreistellig rechtsbündig, damit alles fluchtet.
    const overviewRow = (key: string, label: string, color: string | undefined, value: string) => (
      <Box key={key} flexDirection="row">
        <Box width={OVERVIEW_LABEL_COLS} flexShrink={0}><Text color={color}>{label}</Text></Box>
        <Text color={color} wrap="truncate-end">{value}</Text>
      </Box>
    )
    const pct = (n: number) => `${String(Math.round(n)).padStart(3)}%`
    const marg = s.paneMargins ? 1 : 0
    const card = (key: SettingKey, name: string, ...children: RenderChildren[]) =>
      tile(
        ui,
        'pane',
        {
          key: `card-${key}`,
          tint: CARD_TINT[key] ?? TINT.neutral,
          marg,
          grow: true,
          open: isOpen(key),
          head: (
            <Box flexDirection="row" justifyContent="space-between">
              <Text bold>{name}</Text>
              {headButtons(key)}
            </Box>
          ),
        },
        ...children,
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
    const tasks = m?.bgTasks ?? (await read($, bgTasks))
    // Kontext-Diät: nie aufgerufene MCP-Server mit ihren Schema-Tokens; der Knopf schreibt /mcp ins Eingabefeld zum Abschalten.
    const dietRows = () =>
      diet.length === 0
        ? []
        : [
            <Box key="diet-head" flexDirection="row" columnGap={2} marginTop={1}>
              <Text color={THEME.warning}>{t(L, 'diet.title', { tokens: tokens(diet.reduce((n, r) => n + r.tokens, 0)) })}</Text>
              {pill(ui, 'pane', 'diet-mcp', t(L, 'diet.action'), () => void $.prompt.fill({ text: '/mcp' }).catch(() => undefined))}
            </Box>,
            ...diet.slice(0, 6).map(r => <Text key={`diet-${r.server}`} dimColor wrap="truncate-end">  {t(L, 'diet.row', { server: r.server, tokens: tokens(r.tokens), n: r.tools })}</Text>),
          ]
    const diet = m ? mcpDiet(m.mcpTools, m.usedMcp) : mcpDiet(await read($, mcpTools), await read($, usedMcp))
    const turns = m?.turnLog ?? (await read($, turnLog))
    const brakedIds = await read($, braked)
    const brake = async (a: Agent) => {
      await update($, braked, ids => (ids.includes(a.id) ? ids : [...ids, a.id]))
      await note($, 'agent', t(L, 'log.braked', { name: a.description }))
    }
    const working = s.animation && tn !== null && tn.durationMs === undefined
    const taskState = (x: BgTask) => {
      const bad = x.status === 'failed' || x.status === 'killed'
      const d = duration(x.status === 'running' || x.durationMs === undefined ? now - x.since : x.durationMs)
      return x.status === 'running' ? `● ${t(L, 'agents.running', { d })}` : `${bad ? '✗' : '✓'} ${x.status} ${d}`
    }

    return (
      <Box flexDirection="column">
        {await paneHead($, ui, L, (
          // Zusammenfassung und Einstellungen direkt unter der Modellzeile.
          <Box flexDirection="row" columnGap={2}>
            {s.cardCatchUp && pill(ui, 'pane', 'open-summary', t(L, 'common.summary'), () => void $.ui.open({ ...P.summary, focus: true }))}
            {pill(ui, 'pane', 'open-settings', t(L, 'common.settings'), () => void $.ui.open({ ...P.settings, focus: true }))}
            {dev && <Text backgroundColor={TINT.warning} color={THEME.warning} bold> {t(L, 'dev.badge')} </Text>}
          </Box>
        ))}
        {working && tn !== null && spaced(
          <Box flexDirection="column" backgroundColor={TINT.accent} marginX={marg} paddingX={1} paddingY={1}>
            <Text bold color={THEME.accent}>● {t(L, 'panel.working', { d: duration(now - tn.startedAt), tools: tn.tools, edits: tn.edits })}</Text>
            {entries.length > 0 && (
              <Text wrap="truncate-end">
                <Text dimColor>{t(L, 'panel.lastEvent', { text: '' })}</Text>
                {entryView(ui, entries[entries.length - 1] as LogEntry, true)}
              </Text>
            )}
          </Box>
        )}
        {s.cardPlan && (await (async () => {
          // Plan-Karte: ganzer Plan des aktuellen Jobs mit Start, voraussichtlichem Ende, Dauer je Schritt und Dateien.
          const j = m?.job ?? (await read($, glanceJob))
          if (j === null) return card('cardPlan', t(L, 'glance.cardTitle'), <Text dimColor>{t(L, 'glance.noJob')}</Text>)
          const avg = await read($, glanceAvg)
          // Im Dev-Mode zählt die Mock-Freigabe nicht, sonst stünde der Mock-Job immer auf „braucht dich“.
          const asking = m === null ? g.ask : 0
          const shown = glancePhase(j, now, asking)
          const eta = etaOf(j, now, avg)
          const guess = stepGuessMs(j, avg)
          const { done, total } = progressOf(j)
          const at = (ms: number) => clockTime(new Date(ms).toISOString())
          const when = [
            t(L, 'glance.started', { at: at(j.startedAt) }),
            eta !== null && !eta.longer ? t(L, 'glance.until', { at: at(now + eta.ms) }) : eta?.longer ? t(L, 'glance.longer') : '',
            j.endedAt !== undefined && shown !== 'working' && shown !== 'stuck' ? t(L, 'glance.took', { d: duration(j.endedAt - j.startedAt) }) : '',
          ].filter(x => x !== '')
          const icon = (st: JobStep) => (st.status === 'done' ? '✓' : st.status === 'active' ? '▸' : '○')
          const tone = (st: JobStep) => (st.status === 'done' ? THEME.success : st.status === 'active' ? THEME.accent : THEME.textMuted)
          const timeOf = (st: JobStep) => {
            const ms = stepMs(st, now)
            if (st.status === 'done') return ms === null ? '' : duration(ms)
            const exp = guess === null ? '' : t(L, 'glance.expected', { d: duration(guess) })
            if (st.status === 'active') return [ms === null ? '' : duration(ms), exp].filter(x => x !== '').join(' / ')
            return exp
          }
          return card(
            'cardPlan',
            t(L, 'glance.cardTitle'),
            <Box key="plan-head" flexDirection="row" columnGap={1}>
              <Text bold color={PHASE_COLOR[shown]}>{t(L, `glance.${shown}`)}</Text>
              <Text bold wrap="truncate-end">{j.title}</Text>
            </Box>,
            <Text key="plan-when" dimColor wrap="truncate-end">{[...when, ...(total > 0 ? [t(L, 'glance.progress', { done, total })] : [])].join(' · ')}</Text>,
            shown === 'needsYou' && (asking > 0 && j.phase === 'working' ? t(L, 'glance.permission') : j.question) !== '' && (
              <Text key="plan-q" color={THEME.warning} wrap="wrap">? {asking > 0 && j.phase === 'working' ? t(L, 'glance.permission') : j.question}</Text>
            ),
            j.steps.length === 0 && <Text key="plan-none" dimColor>{t(L, 'glance.noPlan')}</Text>,
            j.steps.length === 0 && j.files.length > 0 && <Text key="plan-files" color={THEME.textMuted} wrap="truncate-end">{t(L, 'glance.files', { list: j.files.join(', ') })}</Text>,
            ...j.steps.map((st, i) => (
              <Box key={`plan-s-${i}`} flexDirection="column" marginTop={i === 0 ? 1 : 0}>
                <Box flexDirection="row" justifyContent="space-between" columnGap={2}>
                  <Text color={tone(st)} bold={st.status === 'active'} wrap="truncate-end">{icon(st)} {st.name}</Text>
                  <Text dimColor>{timeOf(st)}</Text>
                </Box>
                {st.files.length > 0 && <Text color={THEME.textMuted} wrap="truncate-end">{'  '}{t(L, 'glance.files', { list: st.files.join(', ') })}</Text>}
              </Box>
            )),
            j.act !== null && j.phase === 'working' && <Text key="plan-act" color={THEME.textMuted} wrap="truncate-end">{t(L, 'glance.now', { what: actText(j, L) })}</Text>,
          )
        })())}
        {s.cardWorkdir && (place !== null || (await read($, dirs)).root !== '') &&
          card(
            'cardWorkdir',
            t(L, 'panel.workdir'),
            await (async () => {
              const d = await read($, dirs)
              const out = await read($, dirAlert)
              return (
                <Box key="wd-dirs" flexDirection="column" marginBottom={1}>
                  {labelRow(ui, 'pane', 'wd-root', t(L, 'dir.session'), <Text color={THEME.accent} bold wrap="truncate-end">{d.root || place?.path}</Text>)}
                  {d.cwd !== '' && d.cwd !== d.root && labelRow(ui, 'pane', 'wd-cwd', t(L, 'dir.process'), d.cwd)}
                  {out !== null && labelRow(ui, 'pane', 'wd-out', t(L, 'dir.outside'), <Text color={THEME.error} bold wrap="truncate-end">⚠ {out.dir}</Text>)}
                </Box>
              )
            })(),
            ...(place === null ? [] : repoPathRows(place, L)).map(([label, value]) => labelRow(ui, 'pane', `wd-row-${label}`, label, value)),
            // Verknüpfte Sessions (nur Desktop): Status, Name, Branch; neueste zuerst wie die App sie liefert.
            await (async () => {
              const ls = m?.linked ?? (await read($, linked))
              if (ls.length === 0) return null
              return (
                <Box key="wd-linked" flexDirection="column" marginTop={1}>
                  <Text dimColor>{t(L, 'panel.linked', { n: ls.length })}</Text>
                  {ls.map(x => (
                    <Box key={`wd-l-${x.id}`} flexDirection="row" columnGap={1}>
                      <Text color={x.archived ? THEME.textMuted : x.running ? THEME.success : THEME.accent}>{x.archived ? '◌' : x.running ? '●' : '○'}</Text>
                      <Box width={10} flexShrink={0}><Text dimColor>{t(L, x.archived ? 'panel.linkedArchived' : x.running ? 'panel.linkedRunning' : 'panel.linkedIdle')}</Text></Box>
                      <Text bold={!x.archived} dimColor={x.archived} wrap="truncate-end">{x.title || x.name}</Text>
                      {x.branch !== '' && <Text color={THEME.textMuted} wrap="truncate-end">⎇ {x.branch}</Text>}
                    </Box>
                  ))}
                </Box>
              )
            })(),
          )}
        {s.cardGit && place !== null &&
          card(
            'cardGit',
            t(L, 'panel.git'),
            ...(place === null ? [] : repoGitRows(place, L)).map(([label, value]) =>
              labelRow(ui, 'pane', `git-row-${label}`, label, place.dirty === null && label === t(L, 'panel.repoChanges') ? (
                <Box flexDirection="row" columnGap={1} flexShrink={1}>
                  <Text wrap="truncate-end">{value}</Text>
                  {pill(ui, 'pane', 'git-trust', ` ${t(L, 'repo.trust')} `, () => void trustPrompt($, L).then(text => (text === null ? $.ui.toast(t(L, 'repo.trustFail')) : $.prompt.fill({ text }))).catch(() => undefined))}
                </Box>
              ) : value),
            ),
            // Kurzer MR-Status unter Upstream (nur GitLab): Knopf öffnet das MR-Panel, ohne MR „n Commits ohne MR“.
            await (async () => {
              const wm = m?.mr ?? (await read($, mrCur))
              const ahead = m !== null ? 0 : await read($, mrAhead)
              if (wm === null && ahead === 0) return null
              const label = wm !== null ? ` ${mrBadge(wm).replace(/ · /g, ' ')} ` : ` ${t(L, 'mr.ahead', { n: ahead })} `
              return (
                labelRow(ui, 'pane', 'wd-mr', t(L, 'mr.short'), (
                  <Box flexDirection="row" columnGap={1} flexShrink={1}>
                    {pill(ui, 'pane', 'wd-mr-open', label, () => void $.ui.open({ ...P.mr, focus: true }).catch(() => undefined))}
                    {wm !== null && wm.title !== '' && <Text color={THEME.textMuted} wrap="truncate-end">{wm.title}</Text>}
                  </Box>
                ))
              )
            })(),
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
          // Raster: Bezeichnung in fester Spalte, Balken und rechtsbündige Prozente untereinander.
          u !== null && overviewRow('ov-ctx', t(L, 'common.context'), THEME[contextColor(u.contextPercent ?? 0)], `${bar(u.contextPercent ?? 0)} ${pct(u.contextPercent ?? 0)} · ${tokens(u.contextTokens ?? 0)} / ${tokens(u.contextWindow)}`),
          ...(u?.limits.map(l => {
            const p = pace(l, now)
            const rest = [p !== undefined ? paceText(p, L) : '', l.resetsAt ? t(L, 'panel.reset', { at: resetLabel(l.resetsAt, now, L) }) : ''].filter(x => x !== '')
            return overviewRow(`ov-${l.kind}`, LIMIT_LABEL[l.kind] ?? l.kind, THEME[limitColor(l, now)], [`${bar(l.percentUsed)} ${pct(l.percentUsed)}`, ...rest].join(' · '))
          }) ?? []),
          u?.usd !== undefined && overviewRow('ov-usd', t(L, 'panel.costLabel'), undefined, usd(u.usd)),
          u !== null && overviewRow('ov-cache', t(L, 'band.cache'), cache === 0 ? THEME.textMuted : undefined, `${cache === 0 ? t(L, 'band.cold') : duration(cache)}${hit !== null ? ` · ${t(L, 'panel.cacheHit', { n: hit })}` : ''}`),
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
            turns.length > 1 &&
              (Svg ? (
                <Svg source={turnBarsSvg(turns)} alt={t(L, 'turn.bars', { usd: usd(WASTE_USD) })} height={24} />
              ) : (
                <Text key="turn-bars">{turnBarsText(turns).map(x => <Text color={x.isWaste ? THEME.warning : THEME.accentFill}>{x.glyph}</Text>)}</Text>
              )),
            turns.some(isWasteful) && <Text color={THEME.warning}>{t(L, 'turn.waste', { n: turns.filter(isWasteful).length })}</Text>,
          )}
          {s.cardGate && spaced(
            <Box flexDirection="column" flexGrow={1} backgroundColor={TINT[g.deny > 0 ? 'error' : g.ask > 0 ? 'warning' : 'success']} marginX={marg} paddingX={1} paddingY={1}>
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
                        {chip(ui, 'pane', '', String(k))} <Text bold>{label} {n}</Text>
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
          // Ältere als kompakte Liste, nur die letzten `agentTiles` als Kachel.
          ...list.slice(0, -s.agentTiles).map((a, i) => (
            <Text key={`ago-${a.id}`} wrap="truncate-end" dimColor={!LIVE.has(a.status)}>
              {chip(ui, 'pane', '', String(i + 1))} {a.description} · {a.type}{a.model !== undefined ? ` · ${a.model}` : ''} ·{' '}
              {LIVE.has(a.status) ? t(L, 'agents.running', { d: duration(now - a.since) }) : `${a.status === 'killed' || a.status === 'failed' ? '✗' : '✓'} ${a.durationMs !== undefined ? duration(a.durationMs) : ''}`}
            </Text>
          )),
          <Box flexDirection="row" flexWrap="wrap" columnGap={1}>
            {list.slice(-s.agentTiles).map((a, k) => {
              const i = Math.max(0, list.length - s.agentTiles) + k
              const live = LIVE.has(a.status)
              const isKilled = a.status === 'killed' || a.status === 'failed'
              const isBraked = brakedIds.includes(a.id)
              const heavy = live && isAgentHeavy(a, now)
              const tone: Tone = heavy || (live && isBraked) ? 'warning' : live ? 'accent' : isKilled ? 'error' : 'success'
              return (
                <Box key={`ag-${a.id}`} flexDirection="column" width={42} backgroundColor={TINT[tone]} marginX={marg} paddingX={1} paddingY={1}>
                  <Text wrap="truncate-end">
                    {chip(ui, 'pane', '', String(i + 1))} <Text bold>{a.description}</Text>
                  </Text>
                  <Text wrap="truncate-end">{a.type}{a.model !== undefined ? ` · ${a.model}` : ''}</Text>
                  <Text dimColor wrap="truncate-end">{agentStats(a, L)}</Text>
                  {agentFlags(a, L) !== '' && <Text dimColor wrap="truncate-end">{agentFlags(a, L)}</Text>}
                  {heavy && <Text color={THEME.warning} wrap="truncate-end">{t(L, 'agents.heavy', { steps: a.steps ?? 0, d: duration(now - a.since) })}</Text>}
                  {live && isBraked && <Text color={THEME.warning}>{t(L, 'agents.braked')}</Text>}
                  {heavy && !isBraked && pill(ui, 'pane', `brake-${a.id}`, t(L, 'agents.brake'), () => void brake(a))}
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
          // Wie bei den Subagents: ältere als kompakte Liste, die letzten beiden als Kachel mit Grund nach Status.
          ...tasks.slice(0, -2).map((x, i) => (
            <Text key={`bgo-${x.id}`} wrap="truncate-end" dimColor={x.status !== 'running'}>
              {chip(ui, 'pane', '', String(i + 1))} {x.description} · {taskState(x)}
            </Text>
          )),
          <Box flexDirection="row" flexWrap="wrap" columnGap={1}>
            {tasks.slice(-2).map((x, k) => {
              const i = Math.max(0, tasks.length - 2) + k
              const live = x.status === 'running'
              const bad = x.status === 'failed' || x.status === 'killed'
              const tone: Tone = live ? 'accent' : bad ? 'error' : 'success'
              // Befehl und Ergebnis zusammen höchstens TASK_LINES Zeilen; der Rest als „+n“.
              const body = [
                ...(x.command !== undefined && x.command !== x.description ? x.command.split(/\r?\n/).map((l, j) => `${j === 0 ? '$ ' : '  '}${l}`) : []),
                ...(x.summary !== undefined && x.summary !== '' ? x.summary.split(/\r?\n/) : []),
              ]
              const shown = body.slice(0, body.length > TASK_LINES ? TASK_LINES - 1 : TASK_LINES)
              return (
                <Box key={`bg-${x.id}`} flexDirection="column" width={42} backgroundColor={TINT[tone]} marginX={marg} paddingX={1} paddingY={1}>
                  <Text wrap="truncate-end">
                    {chip(ui, 'pane', '', String(i + 1))} <Text bold>{x.description}</Text>
                  </Text>
                  {shown.map((l, j) => <Text key={`bgl-${x.id}-${j}`} color={THEME.textMuted} wrap="truncate-end">{l}</Text>)}
                  {body.length > shown.length && <Text dimColor>{t(L, 'tasks.more', { n: body.length - shown.length })}</Text>}
                  <Text color={THEME[tone]}>{taskState(x)}</Text>
                </Box>
              )
            })}
          </Box>,
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
