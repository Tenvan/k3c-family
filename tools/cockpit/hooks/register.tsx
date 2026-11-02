import { atom, read, update } from 'claude-code'
import type { EngineInterface, HookOf, PressedLink, Register, RenderChildren, RenderElement, RenderSurface, ResolveInput, SessionUsage } from 'claude-code'

import type { BgTask, LinkedSession, About, ModelInfo, Agent, Breakdown, CallMark, CatchUp, Decision, LoopAlert, McpSchema, UiLang, Repo, Gate, LogEntry, LogSrc, ModSettings, Step, Turn, TurnRecord, Usage, PaneTab, DiagTab, BackTab, LogFilter, AgentView, SessionsState } from '../types'
import { mockData } from './mock'
import { BREATHS, breathOf, breathRingSvg, breathSvg, fillAt, phaseAt } from './breath'
import type { BreathKey } from './breath'
// Jede Atemübung in eigener Farbe aus dem Design-System, damit der Wechsel auf einen Blick erkennbar ist.
const BREATH_COLOR: Record<BreathKey, string> = { hrv: THEME.accent, sigh: THEME.success, box: THEME.violet, '478': THEME.warning }
import { canMergeNow, conflictPrompt, mergeAllPrompt, openMrs, draftPrompt, inRepo, refOf, forgeRemote, mrBadge, mrRef, isPipelineRunning, mrEvents, parseDraft, pipelineTone, pollDelayMs, threadsPrompt } from './mr'
import { authMissing, canAutoMerge, ciJobAction, ciJobLog, ciNewPipeline, ciPipelineAction, ciPipelineJobs, ciPipelines, ciSupported, forgeCreate, forgeDefault, forgeFind, forgeLists, forgeLoad, forgeMerge, forgeRebase, forgeRetry, forgeThreads } from './forge'
import type { Done, Io, Remote } from './forge'
import { ciEvents, ciPollDelayMs, isActive, jobEvents, pickPipeline, tailLog } from './ci'
import { ciBody } from './ci-ui'
import type { CiJobAct, CiPipeAct } from './ci-ui'
import type { CiJobs, CiList, CiLog, CiPipeline, Mr, MrDraft, MrMine } from '../types'
import { RATE_SYMBOL, rateLabel, rt, type Rate } from './rating'
import { applyScheme, desktopModeLight, wantsLight, stopTaskPrompt, THEME, TINT, CARD_TINT, cycleTint, summarySections, splitTool, toolText, withClaude, parseTaskNotifications, notificationText, backgroundOf, NEW_STYLES, todosOf, wrapupOf, resetLeft, dirTarget, isForeignDir, nativeDir, parseLinkedSessions, infoSvg, STEP_LABEL_MAX, legendSvg, modelLabel, SPIN_MODE, spinnerText, timelineSvg, demoTable, agentStats, agentFlags, breakdownBarSvg, contextTip, limitTip, cacheTip, summaryLines, topRows, resetLabel, CATCH0, CATCH_KEEP, DEFAULT_SETTINGS, SETTING_DEFS, settingLabel, settingHint, catchUpPrompt, catchUpSystem, nextChunk, settingsOf, startIndex, fingerprint } from './core'
import type { SettingGroup, SettingKey, Tone } from './core'
import { TILES, splitLive, EFFORT_LEVELS, effortColor, effectiveEffort, effortJudgeText, handoffMarkdown, handoffPays, isTrivialPrompt, limitAlert } from './core'
import type { EffortLevel } from './core'
import { agentTranscript, CACHE_WARN_MS, WASTE_USD, compactFocus, detectLoop, isAgentHeavy, isWasteful, mcpDiet, mcpKey, turnBarsSvg, turnBarsText } from './core'
import { RISKY_GIT_CONFIG, COMPACT_AT, DECISION_COLOR, RING_HEX, contextColor, LEGEND, LEGEND_NOTE, paceText, LIMIT_LABEL, LOG_MAX, SRC_COLOR, bar, cacheHitPct, cacheLeftMs, clockTime, duration, limitColor, pace, parseSteps, questionsOf, pushCapped, markdownStats, shortHash, statsLines, toolInfoLines, oneLine, parseRepo, trustText, trustEntry, isTrusted, isSafeTrustEntry, repoGitRows, repoPathRows, repoShort, ringSvg, svgSource, titleText, toBreakdown, breakdownTitle, categoryColor, spinner, title, toolTarget, toolAction, toolKind, tokens, usd } from './core'
import { t } from './i18n'
import { parseSessions, sessionsFilePath } from './sessions'
import { sessionsBody } from './sessions-ui'
import type { Key } from './i18n'
import { ALERT_COLOR, CODE, ICON_COLOR, TONE_COLOR, detailInput, detailOutput, diffCardSvg, diffOf, errorText, firstErrorLine, iconKind, inlineTokens, inputPath, isReadOnlyShell, isShell, isStyled, lineCountOf, mdLiteral, QUIET_TOOLS, quietLine, shellCommand, splitMessage, toolIconSvg } from './skins'
import { askContext, askInfos, askSummary, mathSvg } from './skins'
import type { RichPart, Segment, SplitOptions } from './skins'
import { diagramType, mermaidSvg } from './mermaid'
import { chip, labelRow, link, pill, tabs, tile } from './ui'
import type { LinkTone, Ui } from './ui'
import { addRecord, avgStepMs, completeAll, continuePrompt, dayKey, dayReport, etaOf, finishJob, isWorthShowing, newJob, noteAct, noteFile, planUpdate, progressOf, recordsOf, shiftDay, shownPhase, staleKeys, stepGuessMs, storeKey, toRecord, touch, visibleSteps, weekReport } from './glance'
import type { ShownPhase } from './glance'
import type { Job, JobRecord, JobStep } from '../types'
import { bandRate as inspectRate, bandText as inspectBand, ingestAppend, ingestCompaction, ingestToolCall, ingestToolResult, inputOf, newInspect, paneView as inspectView, reportText as inspectReport, sampleInput, summarize as summarizeInspect } from './inspect'
import type { InspectSummary } from './inspect'
import { bandRate as profileRate, bandText as profileBand, newProfiler, paneView as profileView, reportText as profileReport, stepBegin, stepChunk, stepEnd, stopFailure, subagentStart, subagentStop, summarize as summarizeProfile, toolApproval, toolBegin, toolEnd, turnComplete, turnStart } from './profiler'
import type { ProfileSummary } from './profiler'
import { advice as blastAdvice, bandRate as blastRate, bandText as blastBand, blastScan, normalizeTarget, paneView as blastView, parseCommand, reportText as blastReport } from './blast'
import type { BlastIo, BlastResult } from './blast'
import { bandRate as errRate, bandText as errBand, errorText as inboxErrorText, newErrors, noteCall as errCall, noteError, paneView as errView, reportText as errReport, tx as errTx } from './errors'
import { LIMITS as GUARD_LIMITS, alertAdvice, alertText, bandRate as guardRate, bandText as guardBand, newGuard, noteCall as guardCall, notePrompt as guardPrompt, noteSpawn as guardSpawn, paneView as guardView, resetCalls as guardReset, shouldHold, tx as guardTx } from './guard'
import type { GuardAlert } from './guard'
import { bandRate as tipRate, bandText as tipBand, paneView as tipView, tipsOf, tx as tipTx } from './tips'
import type { Tip } from './tips'
import { addStep, noFilterArgs, parseNameStatus, newTimeline, reportText as tlReport, restorePlan, shownSteps, snapOf, stepClock, stepLine, tx as tlTx, MAX_RESTORE } from './timeline'
import { protocolCounts, protocolRows } from './protocol'
import type { Step as TlStep } from './timeline'
import { GIT_SAFE_ARGS, PHASES, RISKY_ACTION_CONFIG, ageText, cleanupPlan, isProtected, commitAsk, errorPrompt, fileLine, incomingPrompt, inferParents, isBranchName, isMerged, mrPrompt, onMain, opOf, parseLog, parseRefs, parseStatus, parseSuggestion, parseTownParents, parseWorktrees, phaseOf, primaryAction, resolvePrompt, reviewPrompt, splitPrompt, commitPrompt, cleanupPrompt, tidyPrompt, treeRows } from './git'
import type { GitAction } from './git'
import type { GitBranch, GitError, GitState } from '../types'

const PLUGIN = 'tenvan-cc'
// Slash-Command des Hauptpanels: kurz statt der Plugin-ID.
const COMMAND = 'cockpit'
// Panels mit Titel in der eingestellten Sprache. Geöffnet wird mit `focus`: der erste Klick in ein Panel ohne Fokus holt erst
// den Fokus, der Host zeichnet dabei neu und tauscht den Button unter dem Zeiger aus; der Klick verpufft. Der Host gewährt den
// Fokus nur bei leerem Eingabefeld mit Tastatur, nicht aus einem Knopf in Band oder Panel heraus.
// Ein Panel mit Reitern (Atom `paneTab`) statt früher sechs Panels; die Befehle öffnen es am passenden Reiter (`openTab`).
const panes = (lang: UiLang) => ({
  main: { id: PLUGIN, title: t(lang, 'common.paneMain') },
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
// Reiter Sessions: zuletzt gelesene Status-Datei der Schaltzentrale (nur lesen); `sig` = Pfad, Änderungszeit und Größe.
const sessionsFile = atom({ plugin: 'tenvan-cc', key: 'sessionsFile' } as const, null as SessionsState | null)
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
// Git-Panel
const gitState = atom({ plugin: 'tenvan-cc', key: 'gitState' } as const, null as GitState | null)
const gitNote = atom({ plugin: 'tenvan-cc', key: 'gitNote' } as const, '')
const gitBusy = atom({ plugin: 'tenvan-cc', key: 'gitBusy' } as const, '')
const gitConfirm = atom({ plugin: 'tenvan-cc', key: 'gitConfirm' } as const, '')
const gitError = atom({ plugin: 'tenvan-cc', key: 'gitError' } as const, null as GitError | null)
const gitMsg = atom({ plugin: 'tenvan-cc', key: 'gitMsg' } as const, '')
const gitMsgFor = atom({ plugin: 'tenvan-cc', key: 'gitMsgFor' } as const, '')
const gitNewBranch = atom({ plugin: 'tenvan-cc', key: 'gitNewBranch' } as const, '')
const gitRebase = atom({ plugin: 'tenvan-cc', key: 'gitRebase' } as const, true)
const gitAmend = atom({ plugin: 'tenvan-cc', key: 'gitAmend' } as const, false)
const gitSel = atom({ plugin: 'tenvan-cc', key: 'gitSel' } as const, '')
// Ein Panel mit Reitern: gewählter Reiter, Unterreiter von Diagnose und Rückblick, Filter des Protokolls
const paneTab = atom({ plugin: 'tenvan-cc', key: 'paneTab' } as const, 'now' as PaneTab)
// CI/CD (GitLab): Pipeline-Liste, Jobs der gewählten Pipeline, gewählte Pipeline (0 = neueste des Branches), Job-Log, laufende Aktion, Rückfrage
const ciList = atom({ plugin: 'tenvan-cc', key: 'ciList' } as const, null as CiList | null)
const ciJobs = atom({ plugin: 'tenvan-cc', key: 'ciJobs' } as const, null as CiJobs | null)
const ciSel = atom({ plugin: 'tenvan-cc', key: 'ciSel' } as const, 0)
const ciLog = atom({ plugin: 'tenvan-cc', key: 'ciLog' } as const, null as CiLog | null)
const ciBusy = atom({ plugin: 'tenvan-cc', key: 'ciBusy' } as const, '')
const ciConfirm = atom({ plugin: 'tenvan-cc', key: 'ciConfirm' } as const, '')
const diagTab = atom({ plugin: 'tenvan-cc', key: 'diagTab' } as const, 'hints' as DiagTab)
const backTab = atom({ plugin: 'tenvan-cc', key: 'backTab' } as const, 'session' as BackTab)
const logFilter = atom({ plugin: 'tenvan-cc', key: 'logFilter' } as const, 'all' as LogFilter)
// Verlauf eines Subagents im Reiter Jetzt (↗, wenn die Engine die Hauptansicht nicht umschalten kann)
const agentView = atom({ plugin: 'tenvan-cc', key: 'agentView' } as const, null as AgentView | null)

const EFFORT_STEPS = 10
const EFFORT_FRAME_MS = 140
let effortTimer: { cancel(): void } | undefined

const STEPS_SYSTEM =
  'Du schlägst die nächsten sinnvollen Schritte in einer Coding-Session vor. ' +
  'Antworte nur mit einem JSON-Array aus genau 3 Objekten {"label": string, "why": string, "prompt": string}, das wichtigste zuerst. ' +
  '"prompt": der nächste Prompt, den der Nutzer an Claude schickt, also immer ein Auftrag AN CLAUDE: ' +
  'Imperativ in der Du-Form an Claude gerichtet (z. B. "Führe die Tests aus", "Committe die Änderungen"), konkret und ausführlich genug, ' +
  'dass Claude ohne Rückfrage loslegen kann (Ziel, Umfang, Prüfkriterium; ein bis drei Sätze), ohne Höflichkeitsfloskeln. Nie eine Anweisung an den Nutzer selbst ("Prüfe im Browser", "Starte die App neu"); ' +
  'braucht es seine Mitwirkung, formuliere den Auftrag an Claude, der sie vorbereitet oder abfragt. ' +
  `"label": sehr kurze Beschriftung des Auftrags im selben Imperativ für einen schmalen Knopf, höchstens ${STEP_LABEL_MAX} Zeichen (2 bis 4 Wörter, z. B. "Tests ausführen"), nie ein ganzer Satz. ` +
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
async function note($: EngineInterface, src: LogSrc, text: string, tool?: { tool: string; target: string; action?: string; isError: boolean; id?: string }) {
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
                <Text color={CODE.fg} backgroundColor={CODE.bg}> {p.text} </Text>
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

// Titelzeile: Logo links, Text daneben; ohne Logo oder Svg steht nur der Text.
// `versioned` false (Band): ohne Versionsangabe, die steht im Panel rechts.
function titleRow(ui: Ui, info: About | null, mark: string | null, versioned = true) {
  const { Box, Text } = ui
  const Svg = 'Svg' in ui ? ui.Svg : undefined
  const shownInfo = versioned ? info : null
  if (mark === null || !Svg) return <Text bold color={THEME.accent}>{title(shownInfo)}</Text>
  return (
    <Box flexDirection="row" columnGap={1} alignItems="center">
      <Svg source={mark} alt="Tenvan" width={16} height={20} />
      {/* Wortmarke als Text in den Markenfarben für dunklen Grund: „orga“ weiß, „MAX“ #82b1ff (brand/README.md) */}
      <Text italic>
        <Text color={THEME.orga}>orga</Text>
        <Text bold color={THEME.max}>MAX</Text>
      </Text>
      <Text bold color={THEME.accent}>{titleText(shownInfo)}</Text>
    </Box>
  )
}

function repoRow({ Text }: Ui, r: Repo | null) {
  return r === null ? null : <Text dimColor wrap="truncate-end">{repoShort(r)}</Text>
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
  await loadHistory($, dayKey(await $.clock.now())).catch(() => undefined)
  await openTab($, 'back', 'history')
}

// Reiter (und Unterreiter) wählen; `open` (Standard) öffnet dazu das Cockpit-Panel. Git liest beim Wechsel sofort neu (früher
// beim Öffnen des Git-Panels); `refresh` false spart das, wenn der Aufrufer selbst gerade gelesen hat.
async function openTab($: EngineInterface, tab: PaneTab, sub?: DiagTab | BackTab, o: { open?: boolean; refresh?: boolean } = {}) {
  if ((await read($, paneTab)) !== tab) await update($, paneTab, () => tab)
  if (tab === 'sessions') void refreshSessions($, true).catch(() => undefined)
  if (tab === 'diag' && sub !== undefined) await update($, diagTab, () => sub as DiagTab)
  if (tab === 'back' && sub !== undefined) await update($, backTab, () => sub as BackTab)
  if (tab === 'git' && o.refresh !== false) {
    void refreshGit($, true).catch(() => undefined)
    void refreshMr($, true).then(() => loadMineDetails($, true)).catch(() => undefined)
  }
  if (tab === 'ci') void refreshMr($, true).catch(() => undefined).then(() => refreshCi($, true)).catch(() => undefined)
  if (o.open !== false) await $.ui.open({ ...panes(await langOf($)).main, focus: true })
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

// Checkliste im Band: Kopfzeile (Status, Titel, Fortschritt, Restzeit, Links), Rückfrage samt Antworten, Schritte.
// `breath` (0..1, nur während der Arbeit) lässt den Statusring atmen (ersetzt die frühere 56-px-Zeile der Atemübung);
// `answers` sind Claudes Rückfragen als Aufträge (Tasten 4–6), direkt unter der Frage statt in einer eigenen Zeile.
function glanceBand($: EngineInterface, ui: Ui, j: Job, o: { L: UiLang; now: number; avg: number | null; asking: number; live: readonly Agent[]; breath: number | null; answers: readonly Step[] }): RenderElement {
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
  // Laufender Schritt nur, wenn er sichtbar ist: beim Falten (ab 5 Schritten) kann er in „… n weitere“ stecken, dann gehören
  // Tätigkeit und Helfer unter die Kopfzeile.
  const rowsShown = visibleSteps(j.steps)
  const hasActive = rowsShown.some(r => r.kind !== 'fold' && r.step.status === 'active')
  const pct = total > 0 ? (100 * done) / total : 0
  const ring = !Svg ? null
    : o.breath !== null ? <Svg key="glance-ring" source={breathRingSvg(pct, o.breath, PHASE_COLOR[shown], 16)} alt={t(L, 'glance.progress', { done, total })} width={16} height={16} />
    : total > 0 ? <Svg key="glance-ring" source={ringSvg(pct, PHASE_COLOR[shown], 14)} alt={t(L, 'glance.progress', { done, total })} width={14} height={14} />
    : null
  const answers = waiting || o.answers.length === 0 ? null : (
    <Box key="glance-answers" flexDirection="row" flexWrap="wrap" columnGap={2}>
      <Text color={muted}>{'  '}</Text>
      {o.answers.map((a, i) =>
        link(ui, `ask-${i}`, clip(a.label, 40), () => void $.prompt.fill({ text: a.prompt }).catch(() => undefined), { hotkey: String(i + 4) }),
      )}
    </Box>
  )
  return (
    <Box key="glance" flexDirection="column">
      <Box key="glance-head" flexDirection="row" justifyContent="space-between" alignItems="center" columnGap={2}>
        <Box flexDirection="row" columnGap={1} alignItems="center" flexShrink={1}>
          {ring}
          <Text key="glance-phase" bold color={PHASE_COLOR[shown]}>{t(L, `glance.${shown}`)}</Text>
          <Box flexShrink={1}><Text key="glance-title" bold wrap="truncate-end">{j.title}</Text></Box>
          <Box flexShrink={0}><Text key="glance-progress" color={muted}>{total > 0 ? `· ${t(L, 'glance.progress', { done, total })}` : `· ${t(L, 'glance.noSteps')}`}{when !== '' ? ` · ${when}` : ''}</Text></Box>
        </Box>
        <Box flexDirection="row" columnGap={2} alignItems="center" flexShrink={0}>
          {shown === 'working' && link(ui, 'glance-pause', t(L, 'glance.pause'), () => void pauseJob($).catch(() => undefined), { tone: 'muted' })}
          {canContinue && link(ui, 'glance-continue', t(L, 'glance.continue'), () => void continueJob($).catch(() => undefined), { tone: 'strong' })}
          {link(ui, 'glance-plan', t(L, 'glance.plan'), () => void openTab($, 'now').catch(() => undefined), { tone: 'muted' })}
        </Box>
      </Box>
      {shown === 'stuck' && <Text key="glance-stuck" color={THEME.warning} wrap="truncate-end">{t(L, 'glance.stuckNote', { d: duration(now - j.signAt) })}</Text>}
      {waiting && <Text key="glance-perm" color={THEME.warning} wrap="truncate-end">? {t(L, 'glance.permission')}</Text>}
      {!waiting && shown === 'needsYou' && j.question !== '' && <Text key="glance-question" color={THEME.warning} wrap="truncate-end">? {j.question}</Text>}
      {answers}
      {rowsShown.map(r => {
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
  if (!s.mrBand && !s.mrPane && !s.mrNotify) return
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

// Status-Datei der Schaltzentrale lesen (nie schreiben). Gelesen wird nur bei geänderter Zeit/Größe oder mit `force`; ein Fehler
// (Datei fehlt, leer, kaputtes JSON) behält den letzten lesbaren Stand und meldet den Grund. Pfad: Einstellung `sessionsFile`.
let sessionsOverride: unknown = ''
let sessionsPath = sessionsFilePath('', '')
async function refreshSessions($: EngineInterface, force = false) {
  const home = (await $.env.get('USERPROFILE').then(v => v ?? $.env.get('HOME')).catch(() => undefined)) ?? ''
  const path = (sessionsPath = sessionsFilePath(sessionsOverride, home))
  const st = await $.fs.stat(path).catch(() => null)
  const sig = `${path}|${st === null ? 'missing' : `${st.mtimeMs}:${st.size}`}`
  const prev = await read($, sessionsFile)
  if (!force && prev !== null && prev.sig === sig) return
  let raw: string | null = null
  let why = 'ENOENT'
  if (st !== null) {
    try {
      raw = await $.fs.read(path)
    } catch (err) {
      why = err instanceof Error ? err.message : String(err)
    }
  }
  const r = parseSessions(raw, why)
  const next: SessionsState = r.ok ? { view: r.view, error: null, sig } : { view: prev?.view ?? null, error: { kind: r.kind, detail: r.detail }, sig }
  if (!same(prev, next)) await update($, sessionsFile, () => next)
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

// ── CI/CD (GitLab über `glab api`; Ein-/Ausgabe in forge.ts, Übersetzung in ci.ts, Anzeige in ci-ui.tsx) ──
// Mechanik: Polling statt Webhook (siehe ci.ts). Lesend im Takt (15 s bei laufender Pipeline, sonst 1 min, solange Hinweise oder
// der Reiter es brauchen); schreibend nur per Klick.
let ciPolledAt = 0
let ciInflight: Promise<void> | null = null
let ciRunning = false

async function ciRemote($: EngineInterface): Promise<Remote | null> {
  if (mrRemote !== null) return mrRemote
  return forgeRemote(await gitText($, ['remote', 'get-url', await gitRemote($)]))
}

async function refreshCi($: EngineInterface, force = false) {
  if (force && ciInflight !== null) await ciInflight.catch(() => undefined)
  const s = await read($, settings)
  const onTab = (await read($, paneTab)) === 'ci'
  if (!force && !onTab && !s.ciNotify && !s.ciAutoOpen) return
  const r = await ciRemote($)
  if (!ciSupported(r)) return
  const prev = await read($, ciList)
  const now = await $.clock.now()
  if (ciInflight !== null || (!force && now - ciPolledAt < ciPollDelayMs(prev?.pipelines ?? []))) return
  ciPolledAt = now
  const run = pollCi($, s, r, prev, onTab).finally(() => {
    ciInflight = null
  })
  ciInflight = run
  await run
}

async function pollCi($: EngineInterface, s: ModSettings, r: Remote, prev: CiList | null, onTab: boolean) {
  const io = forgeIo($)
  const branch = await gitText($, ['branch', '--show-current'])
  let pipelines: CiPipeline[]
  try {
    pipelines = await ciPipelines(io, r)
  } catch (err) {
    const next: CiList = { pipelines: prev?.pipelines ?? [], error: (err instanceof Error ? err.message : String(err)).slice(0, 160), branch }
    if (!same(prev, next)) await update($, ciList, () => next)
    return
  }
  const watch = s.ciNotify || s.ciAutoOpen
  const mrPid = (await read($, mrCur))?.pipeline?.id ?? 0
  const mine = (p: CiPipeline) => p.ref === branch || p.id === mrPid
  const tracked = pickPipeline(pipelines, await read($, ciSel), branch)
  const prevJobs = await read($, ciJobs)
  // Jobs der verfolgten Pipeline: im Reiter immer, sonst solange sie läuft oder gerade ihren Status gewechselt hat (für „Job fehlgeschlagen“).
  const sameJobs = tracked !== null && prevJobs?.pipeline.id === tracked.id
  const loadJobs = tracked !== null && (onTab || isActive(tracked.status) || (sameJobs && prevJobs.pipeline.status !== tracked.status))
  const jobs = loadJobs ? await ciPipelineJobs(io, r, tracked).catch(() => null) : null
  const events = watch ? ciEvents(prev === null ? null : prev.pipelines, pipelines, mine) : []
  const failedJobs = watch && jobs !== null && tracked !== null && mine(tracked) ? jobEvents(sameJobs ? prevJobs.jobs : null, jobs.jobs) : []
  {
    const v: CiList = { pipelines, error: '', branch }
    if (!same(prev, v)) await update($, ciList, () => v)
  }
  if (jobs !== null && !same(prevJobs, jobs)) await update($, ciJobs, () => jobs)
  // Hinweis (abschaltbar) und bei Fehlschlag der Reiter von selbst, ohne den Eingabefokus zu nehmen (einmal je Abfrage).
  let opened = false
  const say = async (text: string, bad: boolean) => {
    if (s.ciNotify && text !== '') {
      $.ui.toast(text)
      await note($, 'main', text)
    }
    if (bad && s.ciAutoOpen && !opened) {
      opened = true
      await update($, paneTab, () => 'ci')
      await $.ui.open({ ...panes(s.lang).main }).catch(() => undefined)
    }
  }
  for (const { event, pipeline } of events) {
    // Die MR-Hinweise melden grün/rot der MR-Pipeline schon; doppelt wäre Lärm.
    const dup = s.mrNotify && pipeline.id === mrPid && (event === 'pipelineSuccess' || event === 'pipelineFailed')
    await say(dup ? '' : t(s.lang, `ci.ev.${event}`, { id: pipeline.id, ref: pipeline.ref }), event === 'pipelineFailed')
  }
  for (const j of failedJobs) await say(t(s.lang, 'ci.ev.jobFailed', { name: j.name, id: tracked?.id ?? 0 }), true)
}

// Globale Sperre wie bei den MR-Aktionen: solange eine schreibende CI-Aktion läuft, nimmt keine zweite einen Klick an.
async function ciStep($: EngineInterface, what: string, run: (r: Remote) => Promise<Done>) {
  const r = await ciRemote($)
  if (!ciSupported(r) || ciRunning) return
  ciRunning = true
  const L = (await read($, settings)).lang
  await update($, ciBusy, () => `${what} …`)
  try {
    const x = await run(r)
    const text = x.ok ? t(L, 'mr.done', { what }) : t(L, 'mr.failed', { what, why: x.err })
    $.ui.toast(text)
    await note($, 'main', text)
  } finally {
    await update($, ciBusy, () => '')
    ciRunning = false
    await refreshCi($, true).catch(() => undefined)
  }
}

// Zwei Klicks für Folgenreiches (abbrechen, neue Pipeline): der erste fragt, der zweite führt aus; die Frage verfällt nach 6 s.
async function ciAsk($: EngineInterface, key: string): Promise<boolean> {
  if ((await read($, ciConfirm)) === key) {
    await update($, ciConfirm, () => '')
    return true
  }
  await update($, ciConfirm, () => key)
  $.clock.after(6_000, () => void update($, ciConfirm, c => (c === key ? '' : c)).catch(() => undefined))
  return false
}

// Pipeline wählen: Jobs dazu laden, ein offenes Log gehört zur alten Pipeline und schließt.
async function ciPick($: EngineInterface, id: number) {
  await update($, ciSel, () => id)
  await update($, ciLog, () => null)
  const p = (await read($, ciList))?.pipelines.find(x => x.id === id)
  const r = await ciRemote($)
  if (p === undefined || !ciSupported(r)) return
  const jobs = await ciPipelineJobs(forgeIo($), r, p).catch(() => null)
  if (jobs !== null) await update($, ciJobs, () => jobs)
}

async function ciPipe($: EngineInterface, act: CiPipeAct, branch: string) {
  const L = (await read($, settings)).lang
  if (act === 'new') {
    if (branch === '' || !(await ciAsk($, 'ci-new'))) return
    return ciStep($, t(L, 'ci.act.pipeNew', { ref: branch }), r => ciNewPipeline(forgeIo($), r, branch))
  }
  const list = await read($, ciList)
  const p = list === null ? null : pickPipeline(list.pipelines, await read($, ciSel), branch)
  if (p === null) return
  if (act === 'cancel' && !(await ciAsk($, `ci-cancel-${p.id}`))) return
  return ciStep($, t(L, act === 'retry' ? 'ci.act.pipeRetry' : 'ci.act.pipeCancel', { id: p.id }), r => ciPipelineAction(forgeIo($), r, p.id, act))
}

async function ciJob($: EngineInterface, id: number, act: CiJobAct) {
  const j = (await read($, ciJobs))?.jobs.find(x => x.id === id)
  if (j === undefined) return
  const L = (await read($, settings)).lang
  const key = act === 'retry' ? 'ci.act.jobRetry' : act === 'cancel' ? 'ci.act.jobCancel' : 'ci.act.jobPlay'
  return ciStep($, t(L, key, { name: j.name }), r => ciJobAction(forgeIo($), r, id, act))
}

// Job-Log lesen (letzte Zeilen); ein zweiter Klick auf denselben Job schließt es.
async function ciOpenLog($: EngineInterface, id: number) {
  const j = (await read($, ciJobs))?.jobs.find(x => x.id === id)
  const r = await ciRemote($)
  if (j === undefined || !ciSupported(r)) return
  if ((await read($, ciLog))?.job === id) {
    await update($, ciLog, () => null)
    return
  }
  await update($, ciLog, () => ({ job: id, name: j.name, lines: [], truncated: false, error: '', loading: true }))
  const x = await ciJobLog(forgeIo($), r, id).catch(() => null)
  if ((await read($, ciLog))?.job !== id) return
  const next: CiLog = x === null || !x.ok ? { job: id, name: j.name, lines: [], truncated: false, error: x?.err || 'glab', loading: false } : { job: id, name: j.name, ...tailLog(x.text), error: '', loading: false }
  await update($, ciLog, () => next)
}

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
  await openTab($, 'git', undefined, { refresh: false }).catch(() => undefined)
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

// Sammelauftrag für alle offenen MRs (Branch-MR und eigene) ins Eingabefeld; gesendet wird erst mit Enter.
async function mergeAllToClaude($: EngineInterface, mock: { mr: Mr; mrMine: MrMine[]; mrDetails: Record<string, Mr> } | null = null) {
  const L = (await read($, settings)).lang
  const project = mrRemote?.project ?? ''
  const items = mock !== null
    ? openMrs(mock.mr, mock.mrMine, mock.mrDetails, project)
    : openMrs(await read($, mrCur), await read($, mrMine), await read($, mrDetails), project)
  if (items.length < 2) return
  await $.prompt.fill({ text: mergeAllPrompt(items, L) })
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
  const btn = (key: string, label: string, run: () => Promise<unknown>, tone: LinkTone = 'link') => link(ui, key, label, () => void act($, key, run).catch(() => undefined), { busy: down === key, tone })
  // Im Dev-Mode der Mock-MR, damit die Zeile auch ohne GitLab-Repo sichtbar ist.
  const m = devMr ?? cur
  const details = btn('mr-details', t(L, 'mr.details'), () => openTab($, 'git'), 'muted')
  if (busy !== '') return <Box key="mr" flexDirection="row" columnGap={1}><Text color={THEME.accent}>⇄ {busy}</Text></Box>
  if (m !== null) {
    return (
      <Box key="mr" flexDirection="row" columnGap={1} alignItems="center" flexWrap="wrap">
        <Text color={THEME.textMuted}>⇄</Text>
        {withDots(ui, mrParts(ui, m, L), 'mr-b')}
        {m.state === 'opened' && m.threads > 0 && btn('mr-claude', t(L, 'mr.toClaude'), () => threadsToClaude($))}
        {m.state === 'opened' && m.conflicts && btn('mr-fix', t(L, 'mr.fixConflict'), () => conflictToClaude($), 'warn')}
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
    <Box key="mr" flexDirection="row" columnGap={1} alignItems="center" flexWrap="wrap">
      <Box flexShrink={0}><Text color={THEME.textMuted}>⇄ {t(L, 'mr.ahead', { n: ahead })}</Text></Box>
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
// Zeilen der Unterhaltung für Kontext-Inspektor und Hintergrundaufgaben (registriert je Tür, siehe `session.append` in register).
const onAppend: HookOf<'session.append'> = async ($, e, next) => {
  observeInspect(() => ingestAppend(inspectSt, e as never))
  const text = notificationText(e.door, e.message.content)
  if (text.includes('<task-notification>')) await finishBackground($, text).catch(() => undefined)
  return next(e)
}

// Ausgabestil für die Modellzeile aus den Einstellungen (früher aus `prompt.compose`, das sec-default an uns vorbeileitet).
// Ungeprüft: ob ein in der Desktop-App gewählter Stil dort steht; sonst bleibt die Anzeige leer.
async function refreshStyle($: EngineInterface) {
  const raw: unknown = await $.settings.read({ key: 'outputStyle' }).catch(() => undefined)
  const style = typeof raw === 'string' && raw.trim() !== '' && raw !== 'default' ? raw : null
  const mi = await read($, modelInfo)
  if (mi !== null && mi.style !== style) await update($, modelInfo, () => ({ ...mi, style }))
}

// ---- Git-Panel ----
// Ablauf Commit → MR → Merge → Aufräumen: Stand lesen (`loadGit`), Befehle der Reihe nach ausführen (`gitStep`), zeichnen
// (`gitPaneBody`); was ansteht, entscheidet git.ts. Netzwerk nur auf Knopfdruck (fetch, pull, push), git fragt nie interaktiv.

// Pfade wörtlich (Next.js-Routen wie `app/[id]/page.tsx` sind sonst Globs).
const GIT_ENV = { GIT_TERMINAL_PROMPT: '0', GIT_EDITOR: 'true', GCM_INTERACTIVE: 'never', GIT_LITERAL_PATHSPECS: '1' }
const GIT_NET_MS = 120_000
// Branches mit eigener Eltern-Schätzung (je drei git-Aufrufe); weitere hängen an main.
// ponytail: feste Obergrenze; bei sehr vielen Branches wäre ein einziges `git log --simplify-by-decoration` billiger.
const GIT_BRANCH_MAX = 15
const GIT_FILES_MAX = 8
const GIT_LOG = '--format=%h%x00%an%x00%ct%x00%s'
let gitInflight: Promise<void> | null = null
let gitPaneAt = 0
let gitRunning = false
// Wurzel des Repos: Status-Pfade sind wurzelrelativ, deshalb laufen Aktionen dort, nicht im (Unter-)Ordner der Session.
let gitTop = ''

type GitOut = { code: number; out: string; err: string }
type GitCmd = { argv: string[]; optional?: boolean; timeoutMs?: number }

// git an der Repo-Wurzel (vor dem ersten Lesen im Session-Ordner), ohne Rückfragen (Zugangsdaten, Editor) und, wie jeder
// git-Aufruf über `$.process.run`, ohne Repo-Hooks; ein abgebrochener Lauf zählt als Fehler mit Meldung.
async function gitRun($: EngineInterface, args: readonly string[], timeoutMs = 30_000, root = false, extraEnv: Record<string, string> = {}): Promise<GitOut> {
  const cwd = gitTop !== '' && !root ? gitTop : await $.session.root().catch(() => undefined)
  const r = await $.process
    .run([...GIT_BASE, ...GIT_SAFE_ARGS, ...args], { ...(cwd === undefined ? {} : { cwd }), env: { ...GIT_ENV, ...extraEnv }, timeoutMs })
    .catch((err: unknown) => ({ exitCode: -1, stdout: '', stderr: err instanceof Error ? err.message : String(err) }))
  return { code: r.exitCode, out: r.stdout, err: r.stderr }
}

const gitLines = (r: GitOut | undefined): string[] => (r !== undefined && r.code === 0 ? r.out.split(/\r?\n/).map(l => l.trim()).filter(Boolean) : [])

// Änderungszeit einer Datei im Git-Ordner, null, wenn es sie nicht gibt.
async function gitPathAt($: EngineInterface, name: string): Promise<{ path: string; at: number } | null> {
  const path = (await gitRun($, ['rev-parse', '--path-format=absolute', '--git-path', name])).out.trim()
  if (path === '') return null
  const st = await $.fs.stat(path).catch(() => null)
  return st === null ? null : { path, at: st.mtimeMs }
}

// Ganzer Stand fürs Panel. 'noRepo' außerhalb eines Repos, 'risky' bei Repo-Konfiguration mit Befehlen (dann läuft nichts
// weiter), undefined bei vorübergehenden Fehlern (der letzte Stand bleibt stehen).
async function loadGit($: EngineInterface): Promise<GitState | 'noRepo' | 'risky' | undefined> {
  const top = await gitRun($, ['rev-parse', '--show-toplevel'], 30_000, true)
  if (top.code !== 0) {
    const none = /not a git repository/i.test(top.err)
    if (none) gitTop = ''
    return none ? 'noRepo' : undefined
  }
  gitTop = top.out.trim()
  if (await isRiskyRepo($, gitTop).catch(() => true)) return 'risky'
  const [st, refs, townCfg, townMain, stash, wt, remote, fetchHead, rebaseMerge, rebaseApply, merge, pick, revert, recent, localOnly, hooksPath] = await Promise.all([
    gitRun($, ['status', '--porcelain=v2', '--branch', '-z', '--ignore-submodules=all']),
    gitRun($, ['for-each-ref', 'refs/heads', '--format=%(refname:short)%00%(upstream:short)%00%(upstream:track,nobracket)']),
    gitRun($, ['config', '--get-regexp', '^git-town-branch\\..*\\.parent$']),
    gitRun($, ['config', '--get', 'git-town.main-branch']),
    gitRun($, ['stash', 'list']),
    gitRun($, ['worktree', 'list', '--porcelain']),
    gitRemote($),
    gitPathAt($, 'FETCH_HEAD'),
    gitPathAt($, 'rebase-merge'),
    gitPathAt($, 'rebase-apply'),
    gitRun($, ['rev-parse', '-q', '--verify', 'MERGE_HEAD']),
    gitRun($, ['rev-parse', '-q', '--verify', 'CHERRY_PICK_HEAD']),
    gitRun($, ['rev-parse', '-q', '--verify', 'REVERT_HEAD']),
    gitRun($, ['log', '-n', '5', GIT_LOG]),
    gitRun($, ['rev-list', '--count', 'HEAD', '--not', '--remotes']),
    gitRun($, ['config', '--get', 'core.hooksPath']),
  ])
  if (st.code !== 0) return undefined
  // Hooks, die ein Commit oder Push im Terminal ausführen würde (core.hooksPath relativ zur Wurzel, sonst .git/hooks).
  const hp = hooksPath.out.trim()
  const hookDir = hp === '' ? (await gitRun($, ['rev-parse', '--path-format=absolute', '--git-path', 'hooks'])).out.trim() : /^([a-z]:)?[\\/]/i.test(hp) ? hp : `${gitTop}/${hp}`
  const hooks = (
    await Promise.all(['pre-commit', 'commit-msg', 'pre-push'].map(async h => (hookDir !== '' && (await $.fs.stat(`${hookDir}/${h}`).catch(() => null)) !== null ? h : '')))
  ).filter(Boolean)
  const s = parseStatus(st.out)
  const op = opOf({ rebase: rebaseMerge !== null || rebaseApply !== null, merge: merge.code === 0, pick: pick.code === 0, revert: revert.code === 0 })
  // Im Rebase steht HEAD lose; welcher Branch gerade umgebaut wird, steht in head-name.
  let branch = s.branch
  const rb = rebaseMerge ?? rebaseApply
  if (op === 'rebase' && s.detached && rb !== null) {
    const head = await $.fs.read(`${rb.path}/head-name`).catch(() => '')
    branch = String(head).trim().replace(/^refs\/heads\//, '')
  }
  const detached = s.detached && branch === ''
  const refList = parseRefs(refs.out)
  const names = refList.map(r => r.name)
  const town = parseTownParents(townCfg.out)
  const remoteHead = await gitRun($, ['symbolic-ref', '--short', `refs/remotes/${remote}/HEAD`])
  const main =
    townMain.out.trim() ||
    (remoteHead.code === 0 ? remoteHead.out.trim().slice(remote.length + 1) : '') ||
    (names.includes('master') && !names.includes('main') ? 'master' : 'main')
  const base = names.includes(main) ? main : `${remote}/${main}`
  const own = names.filter(n => n !== main).slice(0, GIT_BRANCH_MAX)
  const [depthOut, mergedOut] = await Promise.all([
    Promise.all(own.map(n => gitRun($, ['rev-list', '--count', `${base}..${n}`]))),
    Promise.all(own.map(n => gitRun($, ['for-each-ref', 'refs/heads', '--merged', n, '--format=%(refname:short)']))),
  ])
  const depth: Record<string, number> = {}
  const mergedInto: Record<string, string[]> = {}
  own.forEach((n, i) => {
    // Ohne Basis (kein main, kein origin/HEAD) keine Tiefe: sonst gälte jeder gepushte Branch als gemergt.
    const d = depthOut[i]
    if (d !== undefined && d.code === 0) depth[n] = Number(d.out.trim()) || 0
    mergedInto[n] = gitLines(mergedOut[i])
  })
  const parents = inferParents(names, main, town, mergedInto, depth)
  const ab = await Promise.all(names.map(n => (parents[n] ? gitRun($, ['rev-list', '--left-right', '--count', `${parents[n]}...${n}`]) : Promise.resolve(null))))
  const branches: GitBranch[] = refList.map((r, i) => {
    const m = /^(\d+)\s+(\d+)$/.exec(ab[i]?.out.trim() ?? '')
    return { ...r, parent: parents[r.name] ?? null, behind: Number(m?.[1] ?? 0), ahead: Number(m?.[2] ?? 0), merged: isMerged(r, main, depth[r.name] ?? 1) }
  })
  const [incoming, upstreamOnly] =
    s.upstream !== '' && s.behind > 0
      ? await Promise.all([
          gitRun($, ['log', '-n', '10', GIT_LOG, 'HEAD..@{upstream}']).then(r => parseLog(r.out)),
          gitRun($, ['rev-list', '--cherry-pick', '--right-only', '--count', 'HEAD...@{upstream}']).then(r => (r.code === 0 ? Number(r.out.trim()) || 0 : s.behind)),
        ])
      : [[], 0]
  return {
    ...s,
    top: top.out.trim(),
    branch,
    detached,
    op,
    localOnly: localOnly.code === 0 ? Number(localOnly.out.trim()) || 0 : s.ahead,
    upstreamOnly,
    hooks,
    main,
    remote,
    branches,
    incoming,
    history: parseLog(recent.out),
    stashes: gitLines(stash).length,
    fetchedAt: fetchHead?.at ?? 0,
    town: Object.keys(town).length > 0 || townMain.out.trim() !== '',
    elsewhere: parseWorktrees(wt.out, top.out.trim()),
    readAt: await $.clock.now(),
  }
}

// Stand neu lesen; eine laufende Abfrage wird mitgenutzt, `force` wartet sie ab und liest danach selbst.
async function refreshGit($: EngineInterface, force = false) {
  if (gitInflight !== null) {
    if (!force) return gitInflight
    await gitInflight.catch(() => undefined)
  }
  const run = (async () => {
    const r = await loadGit($)
    if (r === undefined) return
    if (r === 'noRepo' || r === 'risky') {
      await update($, gitState, () => null)
      await update($, gitNote, () => r)
      return
    }
    await update($, gitNote, () => '')
    await update($, gitState, () => r)
    void suggestCommit($, r).catch(() => undefined)
  })().finally(() => {
    gitInflight = null
  })
  gitInflight = run
  return run
}

// Gestagter Stand als Schlüssel: ein neuer Vorschlag nur, wenn sich Dateien oder der letzte Commit ändern.
// ponytail: geänderter Inhalt einer schon gestagten Datei zählt nicht; dafür gibt es „Neu vorschlagen“.
const stagedKey = (s: GitState): string => `${s.history[0]?.sha ?? ''}|${s.staged.map(fileLine).join('|')}`

// Commit-Zeile von Haiku aus dem gestagten Diff. Automatisch nur mit Einstellung `gitSuggest` und nie über Getipptes
// (`gitMsgFor` = 'typed'); `force` (Knopf) überschreibt beides.
async function suggestCommit($: EngineInterface, s: GitState, force = false) {
  const set = await read($, settings)
  if (s.staged.length === 0 || (!force && !set.gitSuggest)) return
  const key = stagedKey(s)
  const was = await read($, gitMsgFor)
  if (!force && (was === key || was === 'busy' || was === 'typed')) return
  await update($, gitMsgFor, () => 'busy')
  let line = ''
  try {
    const [stat, diff, recent] = await Promise.all([
      gitRun($, ['diff', '--cached', '--stat', '--no-color']),
      gitRun($, ['diff', '--cached', '--no-color', '--no-ext-diff', '--no-textconv']),
      gitRun($, ['log', '-n', '10', '--format=%s']),
    ])
    const r = await $.model.complete({ model: 'haiku', prompt: commitAsk(stat.out, diff.out, recent.out, set.lang), maxTokens: 200, effort: 'low', timeoutMs: 30_000 }).catch(() => null)
    line = parseSuggestion(r !== null && r.isAnswered ? r.text : '')
    // Wurde währenddessen getippt, bleibt der getippte Text stehen.
    if (line !== '' && (await read($, gitMsgFor)) === 'busy') await update($, gitMsg, () => line)
  } finally {
    await update($, gitMsgFor, v => (v === 'busy' ? key : v))
  }
}

// Vor jeder Aktion neu geprüft (nicht nur beim Lesen): riskante Repo-Konfiguration fürs Lesen und für schreibende Befehle.
// Liefert die gefundenen Schlüssel, leer = frei. Ein Fehler beim Prüfen sperrt.
async function gitActionBlock($: EngineInterface): Promise<string[]> {
  if (await isRiskyRepo($, gitTop || undefined).catch(() => true)) return ['(Repo-Konfiguration gesperrt)']
  const found: string[] = []
  for (const scope of ['--local', '--worktree']) {
    const r = await gitRun($, ['config', scope, '--includes', '--name-only', '--get-regexp', RISKY_ACTION_CONFIG])
    if (r.code === 0) found.push(...gitLines(r))
    // Exit 1 = kein Treffer; --worktree ohne extensions.worktreeConfig meldet einen Fehler, der dann nichts sperrt.
    else if (r.code !== 1 && scope === '--local') return ['(Repo-Konfiguration nicht lesbar)']
  }
  return found
}

// Befehle nacheinander mit Sperre und Statuszeile; Fehler landen in der Fehlerkarte (`gitError`), Ergebnis als Toast.
// `keepGoing`: nach einem Fehler weitermachen (Löschen mehrerer Branches). Danach immer neu lesen.
async function gitStep($: EngineInterface, what: string, cmds: readonly GitCmd[], o: { keepGoing?: boolean; after?: () => Promise<unknown> } = {}): Promise<boolean> {
  if (gitRunning) return false
  gitRunning = true
  const L = (await read($, settings)).lang
  await update($, gitBusy, () => what)
  const fails: { cmd: string; out: string }[] = []
  try {
    const blocked = await gitActionBlock($)
    if (blocked.length > 0) fails.push({ cmd: what, out: t(L, 'git.blocked', { keys: blocked.join(', ') }) })
    for (const c of blocked.length > 0 ? [] : cmds) {
      const r = await gitRun($, c.argv, c.timeoutMs)
      if (r.code === 0 || c.optional) continue
      fails.push({ cmd: `git ${c.argv.join(' ')}`, out: (r.err.trim() || r.out.trim() || `exit ${r.code}`).slice(-4000) })
      if (!o.keepGoing) break
    }
    const at = await $.clock.now()
    const first = fails[0]
    await update($, gitError, () => (first === undefined ? null : { cmd: first.cmd, out: fails.map(f => (fails.length > 1 ? `$ ${f.cmd}\n${f.out}` : f.out)).join('\n\n'), at }))
    const text = t(L, first === undefined ? 'git.done' : 'git.failed', { what })
    $.ui.toast(text)
    await note($, 'main', text)
    if (first === undefined && o.after) await o.after()
  } finally {
    await update($, gitBusy, () => '')
    gitRunning = false
    await refreshGit($, true).catch(() => undefined)
    void refreshRepo($).catch(() => undefined)
  }
  return fails.length === 0
}

// Rückfrage für Knöpfe, die etwas verwerfen oder überschreiben: erster Klick fragt (6 s gültig), zweiter führt aus.
async function gitAsk($: EngineInterface, key: string): Promise<boolean> {
  if ((await read($, gitConfirm)) === key) {
    await update($, gitConfirm, () => '')
    return true
  }
  await update($, gitConfirm, () => key)
  $.clock.after(6_000, () => void update($, gitConfirm, c => (c === key ? '' : c)).catch(() => undefined))
  return false
}

// Auftrag an Claude ins Eingabefeld; gesendet wird erst mit Enter.
async function gitFill($: EngineInterface, text: string) {
  await $.prompt.fill({ text })
  $.ui.toast(t((await read($, settings)).lang, 'mr.filled'))
}

async function gitCommit($: EngineInterface) {
  const s = await read($, gitState)
  if (s === null) return
  const L = (await read($, settings)).lang
  const msg = (await read($, gitMsg)).trim()
  const amend = await read($, gitAmend)
  if (!amend && s.staged.length === 0) return $.ui.toast(t(L, 'git.nothingStaged'))
  if (!amend && msg === '') return $.ui.toast(t(L, 'git.msgMissing'))
  const argv = ['commit', ...(amend ? ['--amend'] : []), ...(msg === '' ? ['--no-edit'] : ['-m', msg])]
  // Läuft ohne Repo-Hooks (`$.process.run`); mit Hooks bietet das Panel den Commit über Claude an (Bash mit Hooks).
  if (await gitStep($, t(L, amend ? 'git.a.amend' : 'git.a.commit'), [{ argv, timeoutMs: 300_000 }])) {
    await update($, gitMsg, () => '')
    await update($, gitMsgFor, () => '')
    await update($, gitAmend, () => false)
  }
}

// Neuer Branch vom aktuellen Stand; offene Änderungen wandern mit (auf main: „Branch anlegen und mitnehmen“).
// Mit Git Town wird der aktuelle Branch als Eltern eingetragen.
async function gitCreate($: EngineInterface) {
  const s = await read($, gitState)
  if (s === null) return
  const L = (await read($, settings)).lang
  const name = (await read($, gitNewBranch)).trim()
  if (!isBranchName(name)) return $.ui.toast(t(L, 'git.badName', { name: name || '–' }))
  const from = s.detached ? s.main : s.branch
  const cmds: GitCmd[] = [{ argv: ['switch', '-c', name] }]
  if (s.town) cmds.push({ argv: ['config', `git-town-branch.${name}.parent`, from], optional: true })
  if (await gitStep($, `${t(L, 'git.newBranch')} ${name}`, cmds)) await update($, gitNewBranch, () => '')
}

// Alle übrigen Aktionen; die Rückfrage-Schlüssel heißen `git-<Aktion>`, damit der Knopf seine Frage zeigen kann.
type GitAct = GitAction | 'fetch' | 'stage' | 'unstage' | 'stageAll' | 'unstageAll' | 'stash' | 'stashPop' | 'discard' | 'abort' | 'switch' | 'deleteMerged' | 'undo' | 'rebaseOnto' | 'mergeIn' | 'pushBranch' | 'setParent' | 'deleteBranch'

async function gitAction($: EngineInterface, a: GitAct, arg = ''): Promise<unknown> {
  const s = await read($, gitState)
  if (s === null || gitRunning) return
  const L = (await read($, settings)).lang
  const net = (argv: string[]): GitCmd => ({ argv, timeoutMs: GIT_NET_MS })
  const me = s.branches.find(b => b.name === s.branch)
  const parent = me?.parent ?? s.main
  const asked = () => gitAsk($, `git-${a}`)
  // Branch-Namen aus Refs nie ungeprüft in argv: ein von Hand angelegter Ref „-x“ wäre sonst eine Option.
  if (['switch', 'rebaseOnto', 'mergeIn', 'pushBranch', 'setParent', 'deleteBranch'].includes(a) && !isBranchName(arg)) return $.ui.toast(t(L, 'git.badName', { name: arg }))
  if (['cleanup', 'syncParent', 'push'].includes(a) && !(isBranchName(s.branch) && isBranchName(parent))) return $.ui.toast(t(L, 'git.badName', { name: `${s.branch} / ${parent}` }))
  switch (a) {
    case 'fetch':
      return gitStep($, t(L, 'git.fetch'), [net(['fetch', '--prune', s.remote])])
    case 'pull':
      return gitStep($, t(L, 'git.a.pull'), [net(['pull', ...((await read($, gitRebase)) ? ['--rebase', '--autostash'] : ['--ff-only'])])])
    case 'pullRebase':
      return gitStep($, t(L, 'git.a.pullRebase'), [net(['pull', '--rebase', '--autostash'])])
    case 'push': {
      // Zeigt der Upstream auf einen anderen Branch (z. B. origin/main nach `switch -c x origin/main`), gezielt auf den eigenen.
      const own = s.upstream === `${s.remote}/${s.branch}`
      return gitStep($, t(L, 'git.a.push'), [net(own ? ['push'] : ['push', '-u', s.remote, `HEAD:refs/heads/${s.branch}`])], { after: () => refreshMr($, true) })
    }
    case 'force':
      // `--force-if-includes`: fremde Commits, die nur geholt, aber nicht eingearbeitet sind, verhindern den Push.
      return (await asked()) ? gitStep($, t(L, 'git.a.force'), [net(['push', '--force-with-lease', '--force-if-includes'])], { after: () => refreshMr($, true) }) : undefined
    case 'commitClaude':
      return gitFill($, commitPrompt(L))
    case 'stage':
      return gitStep($, `git add ${arg}`, [{ argv: ['add', '--', arg] }])
    case 'unstage':
      return gitStep($, `git restore --staged ${arg}`, [{ argv: ['restore', '--staged', '--', arg] }])
    case 'stageAll':
      return gitStep($, t(L, 'git.stageAll'), [{ argv: ['add', '-A'] }])
    case 'unstageAll':
      return gitStep($, t(L, 'git.unstageAll'), [{ argv: ['restore', '--staged', '--', '.'] }])
    case 'stash':
      return gitStep($, t(L, 'git.stash'), [{ argv: ['stash', 'push', '-u'] }])
    case 'stashPop':
      return gitStep($, t(L, 'git.stashPop', { n: s.stashes }), [{ argv: ['stash', 'pop'] }])
    case 'discard':
      // Nur Änderungen an versionierten Dateien im Arbeitsbaum; Gestagtes und neue Dateien bleiben.
      return (await asked()) ? gitStep($, t(L, 'git.discard'), [{ argv: ['restore', '--worktree', '--', '.'] }]) : undefined
    case 'commit':
      return gitCommit($)
    case 'branchOff':
      return gitCreate($)
    case 'continue': {
      const argv = s.op === 'merge' ? ['commit', '--no-edit'] : [s.op === '' ? 'rebase' : s.op, '--continue']
      return gitStep($, `${t(L, 'git.a.continue')} (${s.op})`, [{ argv, timeoutMs: 300_000 }])
    }
    case 'abort':
      return s.op !== '' && (await asked()) ? gitStep($, `${t(L, 'git.a.abort')} (${s.op})`, [{ argv: [s.op, '--abort'] }]) : undefined
    case 'syncParent': {
      // Auf den Remote-Stand des Eltern-Branches rebasen (lokal, wenn er keinen Upstream hat).
      const p = s.branches.find(b => b.name === parent)
      return gitStep($, t(L, 'git.a.syncParent'), [net(['fetch', s.remote]), { argv: ['rebase', '--autostash', p !== undefined && !p.gone && p.upstream !== '' ? p.upstream : parent], timeoutMs: 300_000 }])
    }
    case 'switch':
      return gitStep($, `git switch ${arg}`, [{ argv: ['switch', arg] }], { after: () => update($, gitSel, () => '') })
    case 'deleteMerged': {
      const gone = s.branches.filter(b => b.merged && b.name !== s.branch && !isProtected(b.name, s.main) && !s.elsewhere.includes(b.name))
      // `-d` löscht nur, was git selbst als gemergt kennt; Squash-Merges bleiben mit Fehler stehen (dafür „Branches aufräumen“).
      return gone.length > 0 && (await asked()) ? gitStep($, t(L, 'git.deleteMerged', { n: gone.length }), gone.map(b => ({ argv: ['branch', '-d', b.name] })), { keepGoing: true }) : undefined
    }
    case 'undo':
      return (await asked()) ? gitStep($, t(L, 'git.undo'), [{ argv: ['reset', '--soft', 'HEAD~1'] }]) : undefined
    case 'cleanup': {
      if (!(await asked())) return
      const mr = await read($, mrCur)
      const plan = cleanupPlan(s, { parent, mrMerged: mr !== null && mr.state === 'merged' && mr.source === s.branch, parentElsewhere: s.elsewhere.includes(parent) })
      return gitStep($, t(L, 'git.a.cleanup'), plan.map(p => ({ ...p, ...(p.argv[0] === 'fetch' ? { timeoutMs: GIT_NET_MS } : {}) })), { after: () => refreshMr($, true) })
    }
    case 'mrDraft':
      return draftMr($)
    case 'merge':
      return mergeMr($)
    case 'rebaseOnto':
      return gitStep($, `git rebase ${arg}`, [{ argv: ['rebase', '--autostash', arg], timeoutMs: 300_000 }], { after: () => update($, gitSel, () => '') })
    case 'mergeIn':
      return gitStep($, `git merge ${arg}`, [{ argv: ['merge', '--no-edit', arg], timeoutMs: 300_000 }], { after: () => update($, gitSel, () => '') })
    case 'pushBranch':
      return gitStep($, `git push -u ${s.remote} ${arg}`, [net(['push', '-u', s.remote, arg])], { after: () => update($, gitSel, () => '') })
    case 'setParent':
      return gitStep($, t(L, 'git.o.parent', { cur: s.branch }), [{ argv: ['config', `git-town-branch.${s.branch}.parent`, arg] }], { after: () => update($, gitSel, () => '') })
    case 'deleteBranch':
      // Nur `-d`: ein nicht gemergter Branch bleibt mit Fehlermeldung stehen. main, master, develop nie.
      if (isProtected(arg, s.main) || arg === s.branch) return
      return (await asked()) ? gitStep($, `git branch -d ${arg}`, [{ argv: ['branch', '-d', arg] }], { after: () => update($, gitSel, () => '') }) : undefined
    case 'resolve':
      return gitFill($, resolvePrompt(s.op, s.conflicts, L))
  }
}

// Inhalt des Git-Panels. `mock` (Dev-Mode): Stand und MR aus mock.ts, Knöpfe führen nichts aus.
async function gitPaneBody($: EngineInterface, ui: Ui, L: UiLang, mock: { git: GitState; mr: Mr; mrMine: MrMine[]; mrDetails: Record<string, Mr> } | null, now: number, marg: number): Promise<RenderElement[]> {
  const { Box, Text, Button } = ui
  // Eingabefelder hat nicht jede Oberfläche; ohne sie bleiben Vorschlag und Claude-Aufträge.
  const Input = 'Input' in ui ? ui.Input : undefined
  const section = (key: string, tint: string, ...children: RenderChildren[]) => tile(ui, 'pane', { key, tint, marg }, ...children)
  const note0 = await read($, gitNote)
  const s = mock !== null ? mock.git : await read($, gitState)
  if (mock === null) {
    gitPaneAt = now
    if (gitInflight === null && !gitRunning && (s === null ? note0 === '' : now - s.readAt > 20_000)) void refreshGit($).catch(() => undefined)
  }
  if (s === null) {
    if (note0 === 'risky')
      return [
        section(
          'git-s-risky',
          TINT.warning,
          <Text key="git-risky" color={THEME.warning}>{t(L, 'git.risky')}</Text>,
          <Box key="git-trust-row" marginTop={1}>
            {pill(ui, 'pane', 'git-trust', ` ${t(L, 'repo.trust')} `, () => void trustPrompt($, L).then(async text => {
              if (text === null) $.ui.toast(t(L, 'repo.trustFail'))
              else await $.prompt.fill({ text })
            }).catch(() => undefined))}
          </Box>,
        ),
      ]
    return [section('git-s-none', TINT.neutral, <Text key="git-none" color={THEME.textMuted}>{t(L, note0 === 'noRepo' ? 'git.noRepo' : 'git.loading')}</Text>)]
  }
  const mr = mock !== null ? mock.mr : await read($, mrCur)
  const [busy, confirm, down, err, msg, msgFor, newName, rebase, amend, sel, mrAsk] = await Promise.all([
    read($, gitBusy), read($, gitConfirm), read($, pressed), read($, gitError), read($, gitMsg), read($, gitMsgFor), read($, gitNewBranch), read($, gitRebase), read($, gitAmend), read($, gitSel), read($, mrConfirm),
  ])
  const lock = busy !== ''
  const run = (key: string, label: string, f: () => Promise<unknown>) =>
    pill(ui, 'pane', key, label, () => void (mock !== null ? Promise.resolve($.ui.toast(t(L, 'dev.badge'))) : act($, key, f)).catch(() => undefined), down === key ? true : lock ? 'locked' : false)
  type Act = Parameters<typeof gitAction>[1]
  const git = (key: string, label: string, a: Act, arg = '') => run(key, label, () => gitAction($, a, arg))
  // Knopf mit Rückfrage: zeigt seine Frage, solange `git-<Aktion>` offen ist.
  const ask = (key: string, label: string, q: Key, a: Act, arg = '', vars?: Record<string, string | number>) => git(key, confirm === `git-${a}` ? t(L, q, vars) : label, a, arg)
  const ai = (key: string, label: string, text: string) => run(key, `✦ ${label}`, () => gitFill($, text))
  const row = (key: string, ...items: RenderChildren[]) => <Box key={key} flexDirection="row" columnGap={1} rowGap={0} flexWrap="wrap" marginTop={1}>{items}</Box>
  const phase = phaseOf(s, mr)
  const primary = primaryAction(s, mr)
  const me = s.branches.find(b => b.name === s.branch)
  const parent = me?.parent ?? s.main
  const dirty = s.staged.length + s.changed.length + s.untracked.length
  const own = mr !== null && mr.source === s.branch ? mr : null
  const unpushed = s.upstream === '' ? s.localOnly : s.ahead
  const noHooks = (h: string) => (s.hooks.includes(h) ? t(L, 'git.noHooks') : '')
  // Enter im Eingabefeld: im Dev-Mode wie die Knöpfe nur ein Hinweis.
  const submit = (key: string, f: () => Promise<unknown>) => () => void (mock !== null ? Promise.resolve($.ui.toast(t(L, 'dev.badge'))) : act($, key, f)).catch(() => undefined)
  const out: RenderElement[] = []

  // Kopf: Branch, Abstand zum Upstream, letzter Fetch, Ablauf-Leiste, Fetch und Neu lesen.
  const idx = phase === 'done' ? PHASES.length : PHASES.indexOf(phase)
  const bad = s.op !== '' || err !== null
  out.push(
    <Box key="git-head" flexDirection="column" marginX={marg}>
      {/* Repo, auf das sich der Stand bezieht: Name und Wurzelordner (die Karte Arbeitsverzeichnis steht im Reiter Jetzt). */}
      {s.top !== '' && (
        <Box key="git-repo" flexDirection="row" columnGap={1}>
          <Text dimColor>{t(L, 'git.repo')}</Text>
          <Text bold>{s.top.replace(/[\\/]+$/, '').split(/[\\/]/).pop()}</Text>
          <Text color={THEME.textMuted} wrap="truncate-end">{s.top}</Text>
        </Box>
      )}
      <Box flexDirection="row" columnGap={1} flexWrap="wrap">
        <Text bold color={THEME.accent} wrap="truncate-end">⎇ {s.branch || 'HEAD'}</Text>
        {s.upstream !== '' ? chip(ui, 'pane', 'git-ab', `↑${s.ahead} ↓${s.behind}`) : chip(ui, 'pane', 'git-noup', t(L, 'git.noUpstream'), THEME.warning)}
        <Text dimColor>{s.fetchedAt > 0 ? t(L, 'git.fetched', { age: ageText(now - s.fetchedAt) }) : t(L, 'git.fetchedNever')}</Text>
      </Box>
      <Box flexDirection="row" columnGap={1} flexWrap="wrap" marginTop={1}>
        {PHASES.flatMap((p, i) => {
          const done = i < idx
          const cur = i === idx
          const color = cur && bad ? THEME.error : done ? THEME.success : cur ? THEME.accent : THEME.textMuted
          const mark = cur && bad ? '✗' : done ? '✓' : cur ? '●' : '○'
          const step = <Text key={`git-ph-${p}`} bold={cur} color={color}>{mark} {t(L, `git.phase.${p}`)}</Text>
          return i === 0 ? [step] : [<Text key={`git-ph-a-${p}`} dimColor>→</Text>, step]
        })}
      </Box>
      {row('git-head-btns', git('git-fetch', t(L, 'git.fetch'), 'fetch'), run('git-reload', t(L, 'git.reload'), () => refreshGit($, true)))}
      {lock && <Text key="git-busy" color={THEME.accent}>◌ {busy} …</Text>}
    </Box>,
  )

  // Status: Lage in einer Zeile, Upstream, MR, Hauptaktion.
  const st: [string, Tone] =
    s.op !== ''
      ? [s.conflicts.length > 0 ? t(L, 'git.st.op', { op: s.op, n: s.conflicts.length }) : t(L, 'git.st.opDone', { op: s.op }), s.conflicts.length > 0 ? 'error' : 'warning']
      : s.detached
        ? [t(L, 'git.st.detached'), 'warning']
        : onMain(s) && dirty > 0
          ? [t(L, 'git.st.onMain', { main: s.main }), 'warning']
          : phase === 'cleanup'
            ? [t(L, 'git.st.merged', { mr: own === null ? '' : mrRef(own) }), 'success']
            : s.upstream !== '' && s.ahead > 0 && s.behind > 0
              ? [t(L, 'git.st.diverged', { a: s.ahead, b: s.behind }), 'warning']
              : s.upstream !== '' && s.behind > 0
                ? [t(L, 'git.st.behind', { n: s.behind, up: s.upstream }), 'warning']
                : phase === 'mr' && (me?.behind ?? 0) > 0
                  ? [t(L, 'git.st.parentAhead', { parent, n: me?.behind ?? 0 }), 'warning']
                  : dirty > 0
                    ? [t(L, 'git.st.dirty', { n: dirty }), 'accent']
                    : phase === 'mr'
                      ? [t(L, 'git.st.ready', { n: me?.ahead || s.ahead }), 'success']
                      : phase === 'merge'
                        ? [t(L, 'git.st.mrOpen'), 'accent']
                        : [t(L, 'git.st.clean'), 'success']
  const newBranchRow = (key: string, label: string) =>
    Input === undefined ? null : row(
      key,
      <Box key={`${key}-in`} flexGrow={1} flexShrink={1} minWidth={16}>
        <Input key="git-newname" value={newName} placeholder={t(L, 'git.newPlaceholder')} onInput={(v: string) => void update($, gitNewBranch, () => v).catch(() => undefined)} onSubmit={submit(`${key}-go`, () => gitCreate($))} />
      </Box>,
      run(`${key}-go`, label, () => gitCreate($)),
    )
  const mainBtn: Record<GitAction, () => RenderChildren> = {
    resolve: () => ai('git-m-resolve', t(L, 'git.ai.resolve'), resolvePrompt(s.op, s.conflicts, L)),
    continue: () => git('git-m-continue', `${t(L, 'git.a.continue')} (${s.op})`, 'continue'),
    branchOff: () => newBranchRow('git-m-branch', t(L, 'git.a.branchOff')),
    cleanup: () => ask('git-m-cleanup', t(L, 'git.a.cleanup'), 'git.a.cleanupAsk', 'cleanup'),
    pullRebase: () => git('git-m-pullRebase', t(L, 'git.a.pullRebase'), 'pullRebase'),
    force: () => ask('git-m-force', `${t(L, 'git.a.force')}${noHooks('pre-push')}`, 'git.a.forceAsk', 'force'),
    commitClaude: () => ai('git-m-commitClaude', t(L, 'git.ai.commit'), commitPrompt(L)),
    pull: () => git('git-m-pull', t(L, 'git.a.pull'), 'pull'),
    commit: () => git('git-m-commit', t(L, 'git.a.commit'), 'commit'),
    syncParent: () => git('git-m-sync', t(L, 'git.a.syncParent'), 'syncParent'),
    mrDraft: () => git('git-m-mr', t(L, 'git.a.mrDraft'), 'mrDraft'),
    push: () => git('git-m-push', `${t(L, 'git.a.push')}${unpushed > 0 ? ` (${unpushed})` : ''}${noHooks('pre-push')}`, 'push'),
    merge: () => {
      // Gemergt wird über die Plattform wie in der MR-Karte: erster Klick fragt, läuft die Pipeline, „Mergen, wenn grün“.
      if (own === null || !(canMergeNow(own) || (isPipelineRunning(own) && canAutoMerge(own)))) return null
      const label = mrAsk === `merge-${own.iid}` ? t(L, 'mr.confirm') : t(L, canMergeNow(own) || !canAutoMerge(own) ? 'mr.merge' : 'mr.mergeAuto')
      return git('git-m-merge', label, 'merge')
    },
  }
  const mrValue: RenderChildren =
    own !== null ? (
      <Box flexDirection="row" columnGap={1} flexShrink={1}>
        {chip(ui, 'pane', 'git-mr-chip', mrBadge(own).replace(/ · /g, ' '))}
        {own.title !== '' && <Text color={THEME.textMuted} wrap="truncate-end">{own.title}</Text>}
      </Box>
    ) : onMain(s) ? (
      t(L, 'git.mrMain', { main: s.main })
    ) : (
      t(L, 'git.mrNone', { n: me?.ahead ?? unpushed })
    )
  out.push(
    section(
      'git-s-status',
      TINT[st[1]],
      <Text key="git-st-h" bold>{t(L, 'git.c.status')}</Text>,
      <Text key="git-st" color={THEME[st[1]]} wrap="truncate-end">{st[0]}</Text>,
      labelRow(ui, 'pane', 'git-up', t(L, 'git.upstream'), s.upstream || t(L, 'git.noUpstream')),
      labelRow(ui, 'pane', 'git-mr', t(L, 'git.mr'), mrValue),
      primary === null ? null : primary === 'branchOff' ? mainBtn.branchOff() : row('git-main', mainBtn[primary]()),
    ),
  )

  // Fehler des letzten Befehls mit Ausgabe (letzte 12 Zeilen) und Auftrag an Claude.
  if (err !== null)
    out.push(
      section(
        'git-s-err',
        TINT.error,
        <Text key="git-err-h" bold>{t(L, 'git.c.error')}</Text>,
        <Text key="git-err-cmd" dimColor wrap="truncate-end">$ {err.cmd}</Text>,
        <Text key="git-err-out" color={THEME.error}>{err.out.split(/\r?\n/).slice(-12).join('\n')}</Text>,
        row(
          'git-err-btns',
          ai('git-err-ai', t(L, 'git.ai.error'), errorPrompt(err.cmd, err.out, L)),
          run('git-err-copy', t(L, 'git.copy'), () => $.ui.copy({ text: err.out }).then(() => $.ui.toast(t(L, 'git.copied')))),
          run('git-err-dismiss', t(L, 'git.dismiss'), () => update($, gitError, () => null)),
        ),
      ),
    )

  // Laufende Operation mit Konflikten.
  if (s.op !== '')
    out.push(
      section(
        'git-s-conf',
        TINT.error,
        <Text key="git-conf-h" bold>{t(L, 'git.c.conflicts')}</Text>,
        ...s.conflicts.map(f => (
          <Box key={`git-conf-${f}`} flexDirection="row" columnGap={1}>
            <Text color={THEME.error}>UU</Text>
            <Text wrap="truncate-end">{f}</Text>
          </Box>
        )),
        row(
          'git-conf-btns',
          s.conflicts.length > 0 && ai('git-conf-ai', t(L, 'git.ai.resolve'), resolvePrompt(s.op, s.conflicts, L)),
          git('git-conf-continue', t(L, 'git.a.continue'), 'continue'),
          ask('git-conf-abort', t(L, 'git.a.abort'), 'git.a.abortAsk', 'abort'),
        ),
      ),
    )

  // Merge Request des Branches (Beschreibung, Threads, Aktionen, Entwurf), eigene offene MRs und Reviews.
  if ((await read($, settings)).mrPane && (mock !== null || (await read($, mrHost)) !== ''))
    out.push(...(await mrPaneBody($, ui, L, mock === null ? null : { mr: mock.mr, mrMine: mock.mrMine, mrDetails: mock.mrDetails }, now, marg)))

  // Aufräumen nach dem Merge: die Schritte, die der Hauptknopf ausführt.
  if (phase === 'cleanup') {
    const plan = cleanupPlan(s, { parent, mrMerged: own?.state === 'merged', parentElsewhere: s.elsewhere.includes(parent) })
    out.push(
      section(
        'git-s-clean',
        TINT.success,
        <Text key="git-clean-h" bold>{t(L, 'git.c.cleanup')}</Text>,
        ...plan.map((p, i) => (
          <Box key={`git-clean-${i}`} flexDirection="row" columnGap={1}>
            <Text color={THEME.success}>○</Text>
            <Text wrap="truncate-end" dimColor={p.optional}>git {p.argv.join(' ')}</Text>
          </Box>
        )),
        <Text key="git-clean-note" dimColor>{t(L, 'git.cleanupNote')}</Text>,
        row('git-clean-btns', ai('git-clean-ai', t(L, 'git.ai.cleanup'), cleanupPrompt(s.branch, parent, own === null ? s.branch : mrRef(own), L))),
      ),
    )
  }

  // Änderungen: je Datei stagen/entstagen, dazu Sammelknöpfe.
  if (dirty > 0 || s.stashes > 0) {
    const fileRow = (prefix: string, key: string, code: string, path: string, label: string, a: Act) => (
      <Box key={`${prefix}-${key}`} flexDirection="row" columnGap={1}>
        <Box width={2} flexShrink={0}><Text color={code === 'D' ? THEME.error : code === 'A' || code === '?' ? THEME.success : THEME.warning}>{code}</Text></Box>
        <Box flexGrow={1} flexShrink={1}><Text wrap="truncate-start">{path}</Text></Box>
        {git(`${prefix}-b-${key}`, label, a, path)}
      </Box>
    )
    const group = (head: string, items: RenderElement[], total: number, key: string) =>
      total === 0 ? [] : [<Text key={`${key}-h`} dimColor>{head}</Text>, ...items, ...(total > items.length ? [<Text key={`${key}-more`} dimColor>{t(L, 'git.more', { n: total - items.length })}</Text>] : [])]
    out.push(
      section(
        'git-s-changes',
        cycleTint(0),
        <Text key="git-ch-h" bold>{t(L, 'git.c.changes')}</Text>,
        ...group(t(L, 'git.staged', { n: s.staged.length }), s.staged.slice(0, GIT_FILES_MAX).map((f, i) => fileRow('git-st', String(i), f.code, f.path, '−', 'unstage')), s.staged.length, 'git-st'),
        ...group(t(L, 'git.changed', { n: s.changed.length }), s.changed.slice(0, GIT_FILES_MAX).map((f, i) => fileRow('git-wt', String(i), f.code, f.path, '+', 'stage')), s.changed.length, 'git-wt'),
        ...group(t(L, 'git.untracked', { n: s.untracked.length }), s.untracked.slice(0, GIT_FILES_MAX).map((p, i) => fileRow('git-un', String(i), '?', p, '+', 'stage')), s.untracked.length, 'git-un'),
        row(
          'git-ch-btns',
          s.changed.length + s.untracked.length > 0 && git('git-stageAll', t(L, 'git.stageAll'), 'stageAll'),
          s.staged.length > 0 && git('git-unstageAll', t(L, 'git.unstageAll'), 'unstageAll'),
          dirty > 0 && git('git-stash', t(L, 'git.stash'), 'stash'),
          s.stashes > 0 && git('git-stashPop', t(L, 'git.stashPop', { n: s.stashes }), 'stashPop'),
          s.changed.length > 0 && ask('git-discard', t(L, 'git.discard'), 'git.discardAsk', 'discard'),
          s.staged.length > 0 && ai('git-ai-review', t(L, 'git.ai.review'), reviewPrompt(L)),
        ),
      ),
    )
  }

  // Commit: Nachricht (Vorschlag von Haiku oder getippt), Amend, Aufträge an Claude. Auf main führt der Weg über einen Branch.
  if (dirty > 0 && !onMain(s) && s.op === '')
    out.push(
      section(
        'git-s-commit',
        cycleTint(1),
        <Text key="git-c-h" bold>{t(L, 'git.c.commit')}</Text>,
        Input === undefined ? (
          <Text key="git-msg-text" color={THEME.violet} wrap="truncate-end">{msg || '–'}</Text>
        ) : (
        <Input
          key="git-msg"
          value={msg}
          placeholder={t(L, 'git.msgPlaceholder')}
          onInput={(v: string) => void update($, gitMsg, () => v).then(() => update($, gitMsgFor, () => 'typed')).catch(() => undefined)}
          onSubmit={submit('git-c-commit', () => gitCommit($))}
        />
        ),
        <Text key="git-c-note" dimColor>{msgFor === 'busy' ? t(L, 'git.suggesting') : t(L, 'git.suggestNote')}</Text>,
        row(
          'git-c-btns',
          git('git-c-commit', `${t(L, amend ? 'git.a.amend' : 'git.a.commit')}${noHooks('pre-commit') || noHooks('commit-msg')}`, 'commit'),
          run('git-c-amend', `${amend ? '☑' : '☐'} ${t(L, 'git.a.amend')}`, () => update($, gitAmend, v => !v)),
          s.staged.length > 0 && run('git-c-suggest', t(L, 'git.suggest'), () => suggestCommit($, s, true)),
          ai('git-c-ai', t(L, 'git.ai.commit'), commitPrompt(L)),
          ai('git-c-split', t(L, 'git.ai.split'), splitPrompt(L)),
        ),
      ),
    )

  // Eingehende Commits vom Upstream.
  if (s.incoming.length > 0)
    out.push(
      section(
        'git-s-in',
        cycleTint(2),
        <Text key="git-in-h" bold>{t(L, 'git.c.incoming', { n: s.behind })}</Text>,
        ...s.incoming.map(c => (
          <Box key={`git-in-${c.sha}`} flexDirection="row" columnGap={1}>
            <Text color={THEME.accent}>{c.sha}</Text>
            <Box flexGrow={1} flexShrink={1}><Text wrap="truncate-end">{c.subject}</Text></Box>
            <Text dimColor>{c.author} · {ageText(now - c.at)}</Text>
          </Box>
        )),
        row('git-in-btns', ai('git-in-ai', t(L, 'git.ai.incoming'), incomingPrompt(s.behind, s.upstream, L))),
      ),
    )

  // Sync: Pull (wahlweise mit Rebase), Push, Force nur bei Divergenz; vor dem MR der Hinweis auf den Eltern-Branch.
  if (phase !== 'cleanup' && !s.detached && s.op === '')
    out.push(
      section(
        'git-s-sync',
        cycleTint(3),
        <Text key="git-sy-h" bold>{t(L, 'git.c.sync')}</Text>,
        phase === 'mr' && (me?.behind ?? 0) > 0 ? <Text key="git-sy-hint" color={THEME.warning}>{t(L, 'git.syncHint', { parent, n: me?.behind ?? 0 })}</Text> : null,
        row(
          'git-sy-btns',
          s.upstream !== '' && git('git-sy-pull', t(L, 'git.a.pull'), 'pull'),
          s.upstream !== '' && run('git-sy-rebase', `${rebase ? '☑' : '☐'} ${t(L, 'git.rebase')}`, () => update($, gitRebase, v => !v)),
          (unpushed > 0 || s.upstream === '') && git('git-sy-push', `${t(L, 'git.a.push')}${unpushed > 0 ? ` (${unpushed})` : ''}${noHooks('pre-push')}`, 'push'),
          s.ahead > 0 && s.behind > 0 && primary !== 'force' && ask('git-sy-force', `${t(L, 'git.a.force')}${noHooks('pre-push')}`, 'git.a.forceAsk', 'force'),
          phase === 'mr' && own === null && ai('git-sy-ai', t(L, 'git.ai.mr'), mrPrompt(s.branch, parent, L)),
        ),
      ),
    )

  // Branches als Baum: nur Abhängigkeiten, keine Commits. Klick auf einen Namen klappt darunter seine Optionen auf.
  const rows = treeRows(s.branches, s.main)
  const free = s.op === '' && !s.detached
  const options = (b: GitBranch, indent: number): RenderElement => {
    const cur = b.name === s.branch
    const away = s.elsewhere.includes(b.name)
    return (
      <Box key={`git-bo-${b.name}`} marginLeft={indent}>
        {row(
          `git-bo-row-${b.name}`,
          !cur && !away && git('git-o-switch', t(L, 'git.o.switch'), 'switch', b.name),
          !cur && free && git('git-o-rebase', t(L, 'git.o.rebase', { b: b.name }), 'rebaseOnto', b.name),
          !cur && free && git('git-o-merge', t(L, 'git.o.merge', { b: b.name, cur: s.branch }), 'mergeIn', b.name),
          b.upstream === '' && !b.merged && git('git-o-push', t(L, 'git.o.push'), 'pushBranch', b.name),
          s.town && free && !cur && b.name !== me?.parent && git('git-o-parent', t(L, 'git.o.parent', { cur: s.branch }), 'setParent', b.name),
          !cur && !away && !isProtected(b.name, s.main) && ask('git-o-delete', t(L, 'git.o.delete'), 'git.o.deleteAsk', 'deleteBranch', b.name, { b: b.name }),
          ai('git-o-ai', t(L, 'git.ai.compare'), t(L, 'git.p.compare', { b: b.name, base: b.parent ?? s.main, cur: s.branch })),
          run('git-o-close', '✕', () => update($, gitSel, () => '')),
        )}
      </Box>
    )
  }
  const mergedN = s.branches.filter(b => b.merged && b.name !== s.branch && !isProtected(b.name, s.main) && !s.elsewhere.includes(b.name)).length
  out.push(
    section(
      'git-s-br',
      cycleTint(4),
      <Text key="git-br-h" bold>{t(L, 'git.c.branches', { n: s.branches.length })}</Text>,
      ...rows.flatMap(({ b, prefix }) => {
        const cur = b.name === s.branch
        const away = s.elsewhere.includes(b.name)
        const open = sel === b.name
        const line = (
          <Box key={`git-br-${b.name}`} flexDirection="row" columnGap={1} {...(open ? { backgroundColor: TINT.accent } : {})}>
            {prefix !== '' && <Text color={THEME.border}>{prefix}</Text>}
            <Text color={cur ? THEME.accent : b.merged ? THEME.textMuted : THEME.textSecondary}>{open ? '▾' : cur ? '●' : b.merged ? '○' : '•'}</Text>
            <Button key={`git-sel-${b.name}`} plain label={b.name} hover={{ color: THEME.accent }} onPress={() => void update($, gitSel, v => (v === b.name ? '' : b.name)).catch(() => undefined)} />
            {away && <Text dimColor>(worktree)</Text>}
            {b.parent !== null && !b.merged && (b.ahead > 0 || b.behind > 0) && chip(ui, 'pane', '', `↑${b.ahead} ↓${b.behind}`, b.behind > 2 ? THEME.warning : undefined)}
            {b.merged && chip(ui, 'pane', '', t(L, 'git.tag.merged'), THEME.success)}
            {!b.merged && b.upstream === '' && b.name !== s.main && chip(ui, 'pane', '', t(L, 'git.tag.noUp'), THEME.warning)}
            {cur && chip(ui, 'pane', '', 'HEAD', THEME.accent)}
          </Box>
        )
        return open ? [line, options(b, prefix.length + 2)] : [line]
      }),
      s.branches.length > rows.length ? <Text key="git-br-more" dimColor>{t(L, 'git.more', { n: s.branches.length - rows.length })}</Text> : null,
      <Text key="git-br-note" dimColor>{t(L, 'git.treeNote')}</Text>,
      primary === 'branchOff' ? null : newBranchRow('git-br-new', t(L, 'git.create')),
      row(
        'git-br-btns',
        mergedN > 0 && ask('git-br-del', t(L, 'git.deleteMerged', { n: mergedN }), 'git.deleteAsk', 'deleteMerged', '', { n: mergedN }),
        ai('git-br-ai', t(L, 'git.ai.tidy'), tidyPrompt(L)),
      ),
    ),
  )

  // Verlauf: letzte Commits, noch nicht gepushte mit ↑; „Letzten Commit zurück“ nur für einen ungepushten.
  if (s.history.length > 0)
    out.push(
      section(
        'git-s-log',
        cycleTint(5),
        <Text key="git-log-h" bold>{t(L, 'git.c.history')}</Text>,
        ...s.history.map((c, i) => (
          <Box key={`git-log-${c.sha}`} flexDirection="row" columnGap={1}>
            <Text color={i < unpushed ? THEME.success : THEME.accent}>{i < unpushed ? '↑' : ' '} {c.sha}</Text>
            <Box flexGrow={1} flexShrink={1}><Text wrap="truncate-end">{c.subject}</Text></Box>
            <Text dimColor>{ageText(now - c.at)}</Text>
          </Box>
        )),
        s.localOnly > 0 && s.op === '' ? row('git-log-btns', ask('git-log-undo', t(L, 'git.undo'), 'git.undoAsk', 'undo')) : null,
      ),
    )
  return out
}

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
  const ciBtn = mock !== null || ciSupported(mrRemote) ? btn('mr-a-ci', t(L, 'ci.git'), () => openTab($, 'ci')) : null
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
            pill(ui, 'pane', 'mr-a-squash', t(L, squashed.includes(`merge-${m.iid}`) ? 'mr.squashOn' : 'mr.squashOff'), () => { if (!lock) void toggleSquash($, m.iid).catch(() => undefined) }, lock ? 'locked' : false, false, 'low')}
          {m.pipeline !== null && m.pipeline.status === 'failed' && m.pipeline.id > 0 && btn('mr-a-retry', t(L, 'mr.retry'), () => retryPipeline($))}
          {ciBtn}
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
  if (!(m !== null && m.state === 'opened')) branch.push(<Box key="mr-refresh-row" flexDirection="row" columnGap={2} marginTop={1}>{ciBtn}{refreshBtn}</Box>)
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
              pill(ui, 'pane', `mine-squash-${d.iid}`, t(L, squashed.includes(`merge-${d.iid}`) ? 'mr.squashOn' : 'mr.squashOff'), () => { if (!lock) void toggleSquash($, d.iid).catch(() => undefined) }, lock ? 'locked' : false, false, 'low')}
          </Box>
        )}
      </Box>
    )
  })
  // Ab zwei offenen MRs (Branch-MR und eigene ohne Doppelte) der AI-Knopf für den Sammelauftrag.
  const openCount = openMrs(m, mine, details, mrRemote?.project ?? '').length
  if (mine.length > 0) {
    out.push(
      section(
        4,
        'mr-s-mine',
        <Text key="mr-mine-h" bold>{t(L, 'mr.mine')}</Text>,
        ...mineRows,
        openCount >= 2 && (
          <Box key="mr-mine-all" flexDirection="row" marginTop={1}>
            {btn('mr-a-mergeall', t(L, 'mr.mergeAll'), () => mergeAllToClaude($, mock))}
          </Box>
        ),
      ),
    )
  }
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
// Fehler-Eingang, Nutzungswächter und Zeitreise: Zustand im Speicher, beginnt mit jeder Session von vorn.
let errSt = newErrors()
let guardSt = newGuard()
let tlSt = newTimeline()
// Tokens, die die letzte Hauptantwort in den Cache geschrieben hat (für die Cache-Tipps).
let lastCacheWrite = 0
// Snapshots laufen nacheinander (ein gemeinsamer Index); `tlResult` ist die Antwort auf einen Knopf (Diff, Zurück).
let tlChain: Promise<unknown> = Promise.resolve()
let tlResult: { n: number; lines: string[]; ok: boolean } | null = null
let tlUsable: { at: number; ok: boolean } | null = null
let tlIndexReady = false

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
    await openTab($, 'diag', 'blast')
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

type DiagSection = { title: string; rows: readonly { label: string; value: string; pct?: number; tone?: 'ok' | 'warn' | 'bad' | 'dim'; rate?: Rate }[]; note?: string }

// Ampelfarben der vier Stufen; die Stufe steht zusätzlich als Wort in der Legende und in den Berichten (nicht nur als Farbe).
const RATE_COLOR: Record<Rate, string> = { great: THEME.success, good: THEME.ratingGood, poor: THEME.warning, bad: THEME.error }

// Zeilen eines Moduls im Profiling-Panel: Abschnittsüberschrift gedimmt, Zeilen als Bezeichnung und Wert (kurze Bezeichnungen
// im Raster, lange – Dateipfade – gedimmt davor), Anteile als Textbalken, zwischen den Abschnitten eine Leerzeile.
function diagRows(ui: Ui, key: string, sections: readonly DiagSection[]): RenderElement[] {
  const { Box, Text } = ui
  const TONE = { ok: THEME.success, warn: THEME.warning, bad: THEME.error, dim: THEME.textMuted } as const
  return sections.flatMap((sec, i) => [
    <Text key={`${key}-${i}-h`} bold color={THEME.textMuted}>{sec.title}</Text>,
    ...sec.rows.map((r, j) => {
      // Hinweise umbrechen statt abschneiden (Wunsch Ralf, 2026-10-31); der Wert schrumpft, die Beschriftung bleibt stehen.
      const value = (
        <Box key={`${key}-${i}-${j}-v`} flexShrink={1}>
          <Text wrap="wrap" color={r.tone === undefined ? undefined : TONE[r.tone]}>
            {r.rate === undefined ? null : <Text color={RATE_COLOR[r.rate]}>{RATE_SYMBOL} </Text>}
            {r.pct === undefined ? '' : `${bar(r.pct, 12)} `}
            {r.value}
          </Text>
        </Box>
      )
      // Ohne Beschriftung (Sätze der Bewertung) volle Breite; lange Beschriftung: eigene Zeile, der Wert darunter.
      if (r.label === '') return <Box key={`${key}-${i}-${j}`} flexDirection="row">{value}</Box>
      return r.label.length <= 12 ? (
        labelRow(ui, 'pane', `${key}-${i}-${j}`, r.label, value)
      ) : (
        <Box key={`${key}-${i}-${j}`} flexDirection="column">
          <Text dimColor wrap="wrap">{r.label}</Text>
          <Box flexDirection="row" paddingLeft={2}>{value}</Box>
        </Box>
      )
    }),
    ...(sec.note === undefined ? [] : [<Text key={`${key}-${i}-note`} dimColor wrap="wrap">{sec.note}</Text>]),
    ...(i < sections.length - 1 ? [<Box key={`${key}-${i}-gap`} height={1} />] : []),
  ])
}

// Profiling-Panel im Schema des Cockpit-Panels: Kopf mit Modellzeile, darunter je Modul eine Karte mit ▾/▸ (Atom `collapsed`).
// ── Zeitreise: Snapshots als git-Tree-Objekte ──

// Darf das Cockpit hier Snapshots anlegen? Nur in einem Git-Repo ohne riskante Konfiguration; die Antwort gilt eine Minute und
// ist nur die schnelle Vorprüfung. Maßgeblich ist `tlSafeArgs` direkt vor jedem `add` und `restore`.
async function tlGitOk($: EngineInterface): Promise<boolean> {
  const now = await $.clock.now()
  if (tlUsable !== null && now - tlUsable.at < 60_000) return tlUsable.ok
  const cwd = await $.session.root().catch(() => undefined)
  const inside = (await gitRun($, ['rev-parse', '--is-inside-work-tree'], 15_000, true)).out.trim() === 'true'
  const ok = inside && !(await isRiskyRepo($, cwd).catch(() => true))
  tlUsable = { at: now, ok }
  return ok
}

// Direkt vor `git add` bzw. `git restore` der Zeitreise, ohne Cache und im selben Ordner wie `gitRun`: Risiko-Prüfung der
// Repo-Konfiguration und git-Optionen, die jeden Filter abschalten (`noFilterArgs`). Sonst könnte eine eben geänderte
// `.git/config` samt `.gitattributes` oder `.git/info/attributes` über einen Clean- oder Smudge-Filter Befehle ohne Freigabe
// starten (Security-Review 2026-10-28). null = nicht ausführen.
async function tlSafeArgs($: EngineInterface): Promise<string[] | null> {
  const cwd = gitTop !== '' ? gitTop : await $.session.root().catch(() => undefined)
  if (await isRiskyRepo($, cwd).catch(() => true)) return null
  const r = await gitRun($, ['config', '--name-only', '--get-regexp', '^filter\\.'])
  if (r.code !== 0 && r.code !== 1) return null
  return noFilterArgs(r.code === 0 ? r.out.split(/\r?\n/) : [])
}

// Stand des Arbeitsbaums (samt neuer Dateien, ohne Ignoriertes) als Tree-Hash; '' ohne Repo oder bei einem Fehler.
// Eigener Index in `<git-dir>/cockpit-tm.index`: Index, Branch und Arbeitsbaum des Nutzers bleiben unberührt.
async function snapshotTree($: EngineInterface): Promise<string> {
  if (!(await tlGitOk($))) return ''
  const dir = (await gitRun($, ['rev-parse', '--path-format=absolute', '--git-dir'], 15_000, true)).out.trim()
  if (dir === '') return ''
  const env = { GIT_INDEX_FILE: `${dir}/cockpit-tm.index` }
  if (!tlIndexReady) {
    const head = await gitRun($, ['read-tree', 'HEAD'], 30_000, false, env)
    if (head.code !== 0 && (await gitRun($, ['read-tree', '--empty'], 30_000, false, env)).code !== 0) return ''
    tlIndexReady = true
  }
  const safe = await tlSafeArgs($)
  if (safe === null) return ''
  if ((await gitRun($, [...safe, 'add', '-A'], 60_000, false, env)).code !== 0) return ''
  const tree = await gitRun($, ['write-tree'], 30_000, false, env)
  const sha = tree.out.trim()
  return tree.code === 0 && /^[0-9a-f]{40,64}$/.test(sha) ? sha : ''
}

// In die Warteschlange der Snapshots einreihen (ein Index, Reihenfolge = Reihenfolge der Aufrufe).
function inTimeline<T>(fn: () => Promise<T>): Promise<T> {
  const run = tlChain.then(fn, fn)
  tlChain = run.catch(() => undefined)
  return run
}

// Kann dieser Aufruf Dateien ändern? Datei-Werkzeuge ja, Shell nur, wenn sie nicht erkennbar nur liest.
const mayChange = (tool: string, e: unknown): boolean => EDIT_TOOLS.has(tool) || (isShell(tool) && !isReadOnlyShell(shellCommand(e)))

// Vor dem ersten ändernden Aufruf: der Stand davor (Schritt 0).
async function tlBase($: EngineInterface): Promise<void> {
  if (tlSt.base !== '' || !(await read($, settings)).timelineSnap) return
  await inTimeline(async () => {
    if (tlSt.base === '') tlSt.base = await snapshotTree($)
  }).catch(() => undefined)
}

function tlSnap($: EngineInterface, step: TlStep): void {
  void read($, settings)
    .then(s => (s.timelineSnap ? inTimeline(() => snapshotTree($)) : ''))
    .then(sha => {
      if (sha !== '') {
        step.snap = sha
        $.ui.invalidate('ui.render')
      }
    })
    .catch(() => undefined)
}

function tlShow($: EngineInterface, n: number, ok: boolean, lines: string[]): void {
  tlResult = { n, ok, lines }
  $.ui.invalidate('ui.render')
}

// „Diff“: was hat sich seit diesem Schritt geändert? (Snapshot von jetzt gegen den Snapshot des Schritts.)
async function tlDiff($: EngineInterface, n: number): Promise<void> {
  const L = (await read($, settings)).lang
  const from = snapOf(tlSt, n)
  const now = from === '' ? '' : await inTimeline(() => snapshotTree($))
  if (from === '' || now === '') return tlShow($, n, false, [tlTx(L, 'tl.noRepo')])
  const d = await gitRun($, ['diff', '--name-status', '-z', '--no-renames', from, now])
  if (d.code !== 0) return tlShow($, n, false, [tlTx(L, 'tl.diffFail', { why: oneLine(d.err, 120) })])
  const changes = parseNameStatus(d.out)
  const lines = changes.slice(0, 40).map(c => `${c.status}  ${c.path}`)
  if (changes.length > 40) lines.push(`… +${changes.length - 40}`)
  tlShow($, n, true, changes.length === 0 ? [tlTx(L, 'tl.diffNone')] : lines)
}

// „Zurück“: Sicherung des jetzigen Standes, dann geänderte und gelöschte Dateien auf den Stand des Schritts setzen.
// Seit damals neue Dateien bleiben liegen. Zweistufig (Rückfrage), Claude bekommt den Hinweis als Auftrag ins Eingabefeld.
async function tlBack($: EngineInterface, n: number): Promise<void> {
  if (!(await gitAsk($, `tl-back-${n}`))) return
  const L = (await read($, settings)).lang
  const target = snapOf(tlSt, n)
  const safety = target === '' ? '' : await inTimeline(() => snapshotTree($))
  if (target === '' || safety === '') return tlShow($, n, false, [tlTx(L, 'tl.noRepo')])
  const d = await gitRun($, ['diff', '--name-status', '-z', '--no-renames', target, safety])
  if (d.code !== 0) return tlShow($, n, false, [tlTx(L, 'tl.restoreFail', { why: oneLine(d.err, 120) })])
  const plan = restorePlan(parseNameStatus(d.out))
  if (plan.tooMany) return tlShow($, n, false, [tlTx(L, 'tl.tooMany', { max: MAX_RESTORE })])
  const keep = plan.keep.length > 0 ? [tlTx(L, 'tl.restoredKeep', { k: plan.keep.length })] : []
  if (plan.restore.length === 0) return tlShow($, n, true, [tlTx(L, 'tl.diffNone'), ...keep])
  const guard = addStep(tlSt, { at: await $.clock.now(), tool: 'cockpit', target: tlTx(L, 'tl.safety'), ok: true })
  guard.snap = safety
  const safe = await tlSafeArgs($)
  if (safe === null) return tlShow($, n, false, [tlTx(L, 'tl.noRepo')])
  const r = await gitRun($, [...safe, 'restore', `--source=${target}`, '--worktree', '--', ...plan.restore])
  if (r.code !== 0) return tlShow($, n, false, [tlTx(L, 'tl.restoreFail', { why: oneLine(r.err, 160) })])
  tlSnap($, addStep(tlSt, { at: await $.clock.now(), tool: 'cockpit', target: `← #${n}`, ok: true }))
  tlShow($, n, true, [tlTx(L, 'tl.restored', { n, k: plan.restore.length, s: guard.n }), ...keep])
  await $.prompt.fill({ text: tlTx(L, 'tl.fill', { n, files: plan.restore.slice(0, 5).join(', ') + (plan.restore.length > 5 ? ', …' : '') }) })
}

// ── Nutzungswächter ──

// Neue Meldung: Hinweis und Eintrag im Log (nur wenn der Wächter eingeschaltet ist).
async function guardNotify($: EngineInterface, alerts: readonly GuardAlert[]): Promise<void> {
  const s = await read($, settings)
  if (alerts.length === 0 || !s.guardBand) return
  for (const a of alerts) {
    $.ui.toast(guardTx(s.lang, 'guard.toast', { text: alertText(a, s.lang) }))
    await note($, 'main', `${alertText(a, s.lang)} – ${alertAdvice(a, s.lang)}`)
  }
}

// ── Cache- und Kontext-Tipps ──

function tipsNow(u: Usage | null, lastAt: number | null, now: number): Tip[] {
  if (u === null) return []
  return tipsOf({
    now,
    isSubscription: u.limits.length > 0,
    lastAt,
    ...(u.contextTokens === undefined ? {} : { contextTokens: u.contextTokens }),
    ...(u.contextPercent === undefined ? {} : { contextPercent: u.contextPercent }),
    lastWrite: lastCacheWrite,
    limits: u.limits,
  })
}

// Karten der Diagnose (früher alle untereinander im Panel „Profiling“): jetzt je Reiter nur die passenden. Der
// Kontext-Inspektor steht im Reiter Verbrauch, Tipps und Wächter unter Diagnose → Hinweise, Profil und Blast-Radius je
// als eigener Unterreiter; die Zeitreise ist Teil des Protokolls.
type DiagId = 'inspect' | 'profile' | 'blast' | 'tips' | 'guard'
async function diagCards($: EngineInterface, ui: Ui, ids: readonly DiagId[]): Promise<RenderElement[]> {
  const s = await read($, settings)
  const L = s.lang
  const marg = s.paneMargins ? 1 : 0
  const { Box, Text } = ui
  const now = await $.clock.now()
  await read($, tick)
  const folded = await read($, collapsed)
  const tips = tipsNow(await read($, usage), await read($, lastTurnAt), now)
  const all: { id: DiagId; title: string; sections: () => readonly DiagSection[]; action: RenderElement | null }[] = [
    {
      id: 'inspect',
      title: t(L, 'diag.cardInspect'),
      sections: () => inspectView(inspectSum(), L),
      action: link(ui, 'diag-agents', t(L, inspectAll ? 'diag.mainOnly' : 'diag.withAgents'), () => {
        inspectAll = !inspectAll
        $.ui.invalidate('ui.render')
      }, { tone: 'muted' }),
    },
    { id: 'profile', title: t(L, 'diag.cardProfile'), sections: () => profileView(profileSum(now), now, L), action: null },
    {
      id: 'blast',
      title: t(L, 'diag.cardBlast'),
      sections: () => blastView(blastRes, blastWatched, L),
      action: link(ui, 'diag-refresh', t(L, 'diag.refresh'), () => void refreshBlast($, true).catch(() => undefined), { busy: blastRun !== null }),
    },
    { id: 'tips', title: tipTx(L, 'tip.cardTitle'), sections: () => tipView(tips, L), action: null },
    { id: 'guard', title: guardTx(L, 'guard.cardTitle'), sections: () => guardView(guardSt, now, L), action: null },
  ]
  return all
    .filter(c => ids.includes(c.id))
    .map((c, i) => {
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
              <Box flexDirection="row" columnGap={2}>
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
}

// Legende der Ampelpunkte (Diagnose).
function diagLegend(ui: Ui, L: UiLang, marg: number): RenderElement {
  const { Box, Text } = ui
  return (
    <Box key="diag-legend" flexDirection="row" columnGap={1} flexWrap="wrap" paddingX={marg}>
      <Text dimColor>{rt(L, 'rate.legend')}:</Text>
      {(['great', 'good', 'poor', 'bad'] as const).map(r => (
        <Text key={`diag-legend-${r}`} color={RATE_COLOR[r]}>{RATE_SYMBOL} {rateLabel(L, r)}</Text>
      ))}
    </Box>
  )
}

// Meldungen: Verzeichniswechsel, Schleifen-Warnung und die Kurzinfos der Diagnose mit ihrer Ampel. Das Band zeigt davon nur
// gelbe und rote (die wichtigste, dahinter „+n Hinweise“), der Reiter Diagnose → Hinweise alle.
// `detail`: Erklärung in Sätzen, nur unter Diagnose → Hinweise (das Band bleibt einzeilig).
type Signal = { key: string; rate: Rate | undefined; text: string; actions: RenderElement[]; detail?: string[] }
// `forBand`: die Schalter „… im Band“ gelten nur fürs Band; Diagnose → Hinweise zeigt jede Meldung.
function signalsOf($: EngineInterface, ui: Ui, s: ModSettings, o: { percent: number | undefined; now: number; tips: readonly Tip[]; loop: LoopAlert | null; dir: { dir: string } | null; forBand: boolean }): Signal[] {
  const L = s.lang
  const { now, percent, tips } = o
  const safe = <T,>(fn: () => T, fallback: T): T => {
    try {
      return fn()
    } catch {
      return fallback
    }
  }
  const out: Signal[] = []
  if (o.dir !== null) out.push({ key: 'dir', rate: 'bad', text: t(L, 'dir.alert', { dir: o.dir.dir }), actions: [] })
  const loop = o.loop
  if (loop !== null) {
    const loopText = t(L, loop.kind === 'error' ? 'loop.error' : 'loop.edit', { target: loop.target, n: loop.n })
    out.push({
      key: 'loop',
      rate: 'poor',
      text: loopText,
      actions: [
        link(ui, 'loop-fix', t(L, 'loop.action'), () => void $.prompt.fill({ text: t(L, 'loop.prompt', { what: loopText }) }).catch(() => undefined)),
        link(ui, 'loop-ignore', t(L, 'loop.ignore'), () => void update($, loopIgnored, l => [...l, loopKey(loop)]).then(() => update($, loopAlert, () => null)).catch(() => undefined), { tone: 'muted' }),
      ],
    })
  }
  const diag: { key: string; on: boolean; text: () => string; rate: () => Rate | undefined; detail?: () => string[] }[] = [
    { key: 'diag-errors', on: s.errorsBand, text: () => errBand(errSt, now, L), rate: () => errRate(errSt, now) },
    { key: 'diag-guard', on: s.guardBand, text: () => guardBand(guardSt, now, L), rate: () => guardRate(guardSt, now) },
    { key: 'diag-tips', on: s.tipsBand, text: () => tipBand(tips, L), rate: () => tipRate(tips) },
    { key: 'diag-inspect', on: s.inspectBand, text: () => inspectBand(inspectSum(now, true), percent, L), rate: () => inspectRate(inspectSum(now, true), percent) },
    { key: 'diag-profile', on: s.profileBand, text: () => profileBand(profileSum(now, true), now, L), rate: () => profileRate(profileSum(now, true)) },
    // Blast: die Sätze der Bewertung, außer den grünen, damit klar ist, was Gelb und Rot meinen.
    { key: 'diag-blast', on: s.blastBand, text: () => blastBand(blastRes, L), rate: () => blastRate(blastRes), detail: () => blastAdvice(blastRes, L).filter(w => isAlarm(w.rate)).map(w => w.text) },
  ]
  for (const d of diag) {
    const text = d.on || !o.forBand ? safe(d.text, '') : ''
    const detail = o.forBand || d.detail === undefined ? [] : safe(d.detail, [])
    if (text !== '') out.push({ key: d.key, rate: safe(d.rate, undefined), text, actions: [], ...(detail.length > 0 ? { detail } : {}) })
  }
  return out
}

const isAlarm = (r: Rate | undefined): r is 'poor' | 'bad' => r === 'poor' || r === 'bad'

// Meldungszeile im Band: nur gelbe und rote Meldungen, rote zuerst; die wichtigste mit ihren Links, dahinter der Weg zu allen.
function bandAlerts($: EngineInterface, ui: Ui, s: ModSettings, sig: readonly Signal[]): RenderElement | null {
  const { Box, Text } = ui
  const L = s.lang
  const hot = sig.filter(x => isAlarm(x.rate)).sort((a, b) => (a.rate === b.rate ? 0 : a.rate === 'bad' ? -1 : 1))
  const top = hot[0]
  if (top === undefined) return null
  const bad = top.rate === 'bad'
  return (
    <Box key="alerts" flexDirection="row" columnGap={2} alignItems="center" flexWrap="wrap">
      <Box flexShrink={1}><Text key={`alert-${top.key}`} color={bad ? THEME.error : THEME.warning} bold={bad} wrap="truncate-end">{bad ? '✗' : '▲'} {top.text}</Text></Box>
      {...top.actions}
      {link(ui, 'alerts-more', hot.length > 1 ? t(L, 'band.moreHints', { n: hot.length - 1 }) : t(L, 'band.hints'), () => void openTab($, 'diag', 'hints').catch(() => undefined), { tone: 'muted' })}
    </Box>
  )
}

// Hinweise im Reiter Diagnose: jede Meldung mit Ampelpunkt, auch die grünen (die früheren Kurzinfos im Band).
function signalsCard(ui: Ui, L: UiLang, marg: number, sig: readonly Signal[]): RenderElement {
  const { Box, Text } = ui
  return tile(
    ui,
    'pane',
    { key: 'card-hints', tint: cycleTint(0), marg, grow: true, head: <Text bold>{t(L, 'hints.title')}</Text> },
    sig.length === 0 && <Text key="hints-none" dimColor>{t(L, 'hints.none')}</Text>,
    ...sig.map(x => (
      <Box key={`hint-${x.key}`} flexDirection="column">
        <Box flexDirection="row" columnGap={1} alignItems="center" flexWrap="wrap">
          <Text color={x.rate === undefined ? THEME.textMuted : RATE_COLOR[x.rate]}>{RATE_SYMBOL}</Text>
          <Box flexShrink={1}><Text wrap="wrap">{x.text}</Text></Box>
          {...x.actions}
        </Box>
        {(x.detail ?? []).map((d, i) => (
          <Box key={`hint-${x.key}-d${i}`} paddingLeft={2}><Text dimColor wrap="wrap">{d}</Text></Box>
        ))}
      </Box>
    )),
  )
}

// Tool-Freigaben (früher eigene Karte im Panel): Zähler, Kachelzeile der letzten Entscheidungen, Datei/Shell/Sonstige.
function gateBlock(ui: Ui, L: UiLang, g: Gate, marg: number): RenderElement {
  const { Box, Text } = ui
  const tone = g.deny > 0 ? 'error' : g.ask > 0 ? 'warning' : 'success'
  return (
    <Box key="gate-block" flexDirection="column" backgroundColor={TINT[tone]} marginX={marg} paddingX={1} paddingY={1} marginBottom={1}>
      <Box flexDirection="row" justifyContent="space-between">
        <Text bold color={THEME[tone]}>{t(L, 'gate.title')}</Text>
        <Text dimColor>{t(L, 'gate.checks', { n: g.allow + g.you + g.ask + g.deny })}</Text>
      </Box>
      <Text>{g.trail.length === 0 ? <Text dimColor>{t(L, 'gate.none')}</Text> : g.trail.map((d, i) => <Text key={`gt-${i}`} color={DECISION_COLOR[d]}>■</Text>)}</Text>
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
  )
}

// Protokoll (Diagnose → Protokoll): Log, Fehler-Eingang, Tool-Freigaben und Zeitreise in einer Liste, neueste zuerst.
// Snapshot-Einträge tragen Diff und Zurück (zweistufig), Fehler ihre erste Fehlerzeile.
async function protocolBody($: EngineInterface, ui: Ui, s: ModSettings, entries: readonly LogEntry[], g: Gate): Promise<RenderElement[]> {
  const { Box, Text } = ui
  const L = s.lang
  const marg = s.paneMargins ? 1 : 0
  const filter = await read($, logFilter)
  const confirm = await read($, gitConfirm)
  const base = tlSt.base !== ''
  const counts = protocolCounts(entries, errSt.list, tlSt.steps, base)
  const rows = protocolRows(entries, errSt.list, tlSt.steps, base, filter)
  const items = (['all', 'errors', 'gate', 'snaps'] as const).map(id => ({ id, label: `${t(L, `log.filter.${id}`)} ${counts[id]}` }))
  const out: RenderElement[] = [
    <Box key="log-filter-row" paddingX={marg} marginBottom={1}>{tabs(ui, 'log-filter', items, filter, f => void update($, logFilter, () => f).catch(() => undefined))}</Box>,
  ]
  if (s.cardGate && (filter === 'all' || filter === 'gate')) out.push(gateBlock(ui, L, g, marg))
  // Filter Fehler: die Bilanz des Fehler-Eingangs (gesamt, letzte 10 min, Aufrufe, häufigstes Tool) über der Liste.
  if (filter === 'errors') out.push(<Box key="log-err-sum" flexDirection="column" paddingX={marg} marginBottom={1}>{...diagRows(ui, 'log-err', errView(errSt, await $.clock.now(), L).slice(0, 1))}</Box>)
  if (tlResult !== null) {
    const res = tlResult
    out.push(
      <Box key="tl-res" flexDirection="column" paddingX={marg} marginBottom={1}>
        <Text bold color={THEME.textMuted}>{res.n === 0 ? tlTx(L, 'tl.title.result') : tlTx(L, 'tl.title.diff', { n: res.n })}</Text>
        {res.lines.map((line, i) => <Text key={`tl-res-${i}`} wrap="truncate-end" color={res.ok ? undefined : THEME.warning}>{line}</Text>)}
      </Box>,
    )
  }
  if (rows.length === 0) out.push(<Text key="log-none" dimColor>{filter === 'snaps' ? tlTx(L, 'tl.none') : t(L, 'log.empty')}</Text>)
  out.push(
    ...rows.map((r, i) => (
      <Box key={`pr-${i}`} flexDirection="column" paddingX={marg}>
        <Box flexDirection="row" columnGap={1} alignItems="center">
          <Box flexShrink={0}><Text dimColor>{r.at > 0 ? clockTime(new Date(r.at).toISOString()) : '  #0 '}</Text></Box>
          <Box width={6} flexShrink={0}><Text bold color={SRC_COLOR[r.src]}>{r.src}</Text></Box>
          <Box flexShrink={1}>
            {r.tool !== undefined ? toolLabel(ui, r.tool, r.target ?? '', r.isError, r.action ?? '') : r.snap === 0 ? <Text>{tlTx(L, 'tl.start')}</Text> : <Text bold={r.src === 'you'} dimColor={r.src !== 'you'} wrap="truncate-end">{r.text}</Text>}
          </Box>
          {r.times !== undefined && r.times > 1 && <Text color={THEME.error}>×{r.times}</Text>}
          {r.snap !== undefined && (
            <Box flexShrink={0} flexDirection="row" columnGap={2}>
              {link(ui, `tl-diff-${r.snap}`, tlTx(L, 'tl.diff'), () => void tlDiff($, r.snap ?? 0).catch(() => undefined), { tone: 'muted' })}
              {link(ui, `tl-back-${r.snap}`, confirm === `tl-back-${r.snap}` ? tlTx(L, 'tl.backAsk') : tlTx(L, 'tl.back'), () => void tlBack($, r.snap ?? 0).catch(() => undefined), { tone: confirm === `tl-back-${r.snap}` ? 'warn' : 'muted' })}
            </Box>
          )}
        </Box>
        {r.errText !== undefined && r.errText !== '' && <Text color={r.denied ? THEME.warning : THEME.error} wrap="truncate-end">{'       '}{r.agent ? '↳ ' : ''}{r.errText}</Text>}
      </Box>
    )),
  )
  if (filter === 'all' || filter === 'snaps') out.push(<Text key="tl-note" dimColor wrap="wrap">{tlTx(L, 'tl.hint')}</Text>)
  return out
}

// Host-Theme (`/config` → theme) lesen und die Cockpit-Palette danach hell oder dunkel setzen; true, wenn sie wechselte.
async function syncScheme($: EngineInterface, appearance: unknown): Promise<boolean> {
  const row = (await $.config.list()).find(r => r.key === 'theme')
  return applyScheme(wantsLight(appearance, row?.value))
}

// auto in der Desktop-App: deren eigenes Theme (das CLI-Theme gilt dort nicht). Läuft nach dem Start im Hintergrund,
// damit der Registry-Aufruf den Session-Start nicht aufhält; zeichnet neu, wenn das Schema wechselt.
// `surfaces()` ist beim Session-Start oft noch leer (die Desktop-App meldet sich erst danach, ungeprüft): der Start-Sync
// verwirft sich dann still, und die Palette bliebe bis `/reload-plugins` beim CLI-Theme. Darum prüft auch das erste
// Zeichnen auf dem Desktop (`desktopKnown`) und danach höchstens alle 15 s, so folgt das Cockpit auch einem Wechsel
// von Windows oder Claude zur Laufzeit.
let schemeCheckedAt = 0
const SCHEME_EVERY_MS = 15_000

async function syncDesktopScheme($: EngineInterface, appearance: unknown, desktopKnown = false): Promise<void> {
  if (appearance === 'hell' || appearance === 'dunkel') return
  if (!desktopKnown && !(await $.session.surfaces()).includes('desktop')) return
  const home = (await $.env.get('USERPROFILE').catch(() => undefined)) ?? ''
  const cfg = await $.fs.read(`${home}/AppData/Roaming/Claude/config.json`).catch(() => '')
  const reg = await $.process
    .run(['reg', 'query', String.raw`HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize`, '/v', 'AppsUseLightTheme'])
    .then(r => r.stdout)
    .catch(() => undefined)
  const light = desktopModeLight(cfg, reg)
  if (light !== undefined && applyScheme(light)) await update($, about, a => (a === null ? null : { ...a }))
}

// Reiter Rückblick → Session: Zusammenfassung (Knöpfe oben, darunter die Abschnitte als einfacher Text, kein Markdown).
async function summaryBody($: EngineInterface, ui: Ui): Promise<RenderElement> {
  const s = await read($, settings)
  const realCu = await read($, catchUp)
  // Dev-Mode: Mock-Zusammenfassung, damit die Optik ohne Haiku-Lauf sichtbar ist.
  const cu = (await read($, devMock)) ? { ...realCu, summary: mockData(await $.clock.now()).summary } : realCu
  const L = s.lang
  const marg = s.paneMargins ? 1 : 0
  const { Box, Text, Button } = ui

  return (
    <Box flexDirection="column" gap={1}>
      <Box flexDirection="row" columnGap={2} alignItems="center">
        {pill(ui, 'pane', 'cu-refresh', t(L, 'summary.refresh'), () => void runCatchUp($, false).catch(() => undefined))}
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
}

// Reiter Rückblick → Verlauf: Aufgaben eines Tages in diesem Projekt, Blättern nach Tagen, Tages- und Wochenbericht.
async function historyBody($: EngineInterface, ui: Ui, surface: RenderSurface): Promise<RenderElement> {
  const s = await read($, settings)
  const L = s.lang
  const marg = s.paneMargins ? 1 : 0
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
              {/* Kopiert den getippten Prompt (ältere Einträge ohne: den Titel) in die Zwischenablage. */}
              {pill(ui, 'pane', `hist-copy-${r.id}`, '⧉', () => void $.ui.copy({ text: r.prompt ?? r.title, surface }).then(() => $.ui.toast(t(L, 'glance.promptCopied'))).catch(() => undefined))}
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
        {pill(ui, 'pane', 'hist-day-report', t(L, 'glance.dayReport'), () => void copyReport($, 'day', surface).catch(() => undefined))}
        {pill(ui, 'pane', 'hist-week-report', t(L, 'glance.weekReport'), () => void copyReport($, 'week', surface).catch(() => undefined))}
      </Box>
    </Box>
  )
}

// Reiter Einstellungen: jedes Detail von Band, Panel und Chat ein- oder ausblenden; gilt sofort und für alle Sessions.
async function settingsBody($: EngineInterface, ui: Ui): Promise<RenderElement> {
  const s = await read($, settings)
  const L = s.lang
  const marg = s.paneMargins ? 1 : 0
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
}

export const register: Register = (on, options) => {
  sessionsOverride = options.sessionsFile
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

  // Themenwechsel zur Laufzeit: Palette neu setzen und die Anzeigen neu zeichnen lassen.
  on('config.set', { key: 'theme' }, async ($, e, next) => {
    const out = await next(e)
    if (!('deny' in out && out.deny) && applyScheme(wantsLight(options.appearance, e.value))) await update($, about, a => (a === null ? null : { ...a }))
    return out
  }).catch(($, e, next) => next(e))

  on('session.start', async ($, e, next) => {
    await syncScheme($, options.appearance).catch(() => false)
    schemeCheckedAt = 0
    void syncDesktopScheme($, options.appearance).catch(() => undefined)
    const info = await loadAbout($)
    await update($, about, () => info)
    // Diagnose: neue Session, neue Messung (der Zustand liegt in Modul-Variablen und überlebt keinen Reload).
    inspectSt = newInspect()
    profileSt = newProfiler()
    inspectVer++
    profileVer++
    blastRes = null
    errSt = newErrors()
    guardSt = newGuard()
    tlSt = newTimeline()
    lastCacheWrite = 0
    tlResult = null
    tlUsable = null
    tlIndexReady = false
    tlChain = Promise.resolve()
    await loadSettings($, { steps: options.nextSteps !== false, chatInfo: options.chatInfo !== false })
    void $.store.get('collapsed').then(v => update($, collapsed, () => foldedOf(v))).catch(() => undefined)
    void loadCatchUp($).catch(() => undefined)
    await update($, logo, () => null)
    void Promise.all([$.session.model(), $.session.version()])
      .then(([model, v]) => update($, modelInfo, mi => ({ ...mi, model, cc: v.version })))
      .then(() => refreshStyle($))
      .catch(() => undefined)
    void Promise.all([loadLogo($).then(l => update($, logo, () => l)), refreshRepo($), refreshLinked($), refreshDirs($)]).catch(() => undefined)
    // ponytail: Beschreibungen in der Sprache beim Session-Start; nach einem Sprachwechsel erst mit dem nächsten Start/Reload
    const lang = await langOf($)
    await $.command.register({ name: COMMAND, description: t(lang, 'common.cmdPanel') })
    await $.command.register({ name: 'cockpit-zusammenfassung', description: t(lang, 'common.cmdSummary') })
    await $.command.register({ name: 'cockpit-einstellungen', description: t(lang, 'common.cmdSettings') })
    await $.command.register({ name: 'cockpit-demo', description: t(lang, 'common.cmdDemo') })
    await $.command.register({ name: 'cockpit-dev', description: t(lang, 'common.cmdDev') })
    await $.command.register({ name: 'cockpit-git', description: t(lang, 'common.cmdGit') })
    await $.command.register({ name: 'cockpit-sessions', description: t(lang, 'common.cmdSessions') })
    await $.command.register({ name: 'cockpit-ci', description: t(lang, 'common.cmdCi') })
    await $.command.register({ name: 'cockpit-features', description: t(lang, 'common.cmdFeatures') })
    await $.command.register({ name: 'cockpit-verlauf', description: t(lang, 'common.cmdHistory') })
    await $.command.register({ name: 'cockpit-profiling', description: t(lang, 'common.cmdProfiling') })
    await $.command.register({ name: 'cockpit-kontext', description: t(lang, 'common.cmdInspect') })
    await $.command.register({ name: 'cockpit-profil', description: t(lang, 'common.cmdProfile') })
    await $.command.register({ name: 'cockpit-blast', description: t(lang, 'common.cmdBlast') })
    await $.command.register({ name: 'cockpit-fehler', description: t(lang, 'common.cmdErrors') })
    await $.command.register({ name: 'cockpit-zeitreise', description: t(lang, 'common.cmdTimeline') })
    void refreshDirs($).then(() => loadBlast($)).catch(() => undefined)
    // Checkliste: mittlere Schrittdauer aus dem Verlauf, sobald der Session-Ordner bekannt ist.
    void refreshDirs($).then(() => projectCwd($)).then(async cwd => loadAvg($, cwd, await $.clock.now())).catch(() => undefined)
    // ponytail: Git-Stand alle 30 s neu (auch nach Worktree-Wechsel oder Neustart ohne neuen Turn), classic.CwdChanged wäre sofort
    // Im selben Takt prüft die Checkliste „hängt?“ (das Band zeichnet über `tick` ohnehin neu).
    void $.clock.every(30_000, () => void Promise.all([update($, tick, n => n + 1), syncAgents($), watchCache($), refreshRepo($), refreshLinked($), refreshDirs($)]).catch(() => undefined))
    void $.clock.every(15_000, () => void refreshMr($).then(() => loadMineDetails($)).catch(() => undefined))
    void $.clock.every(15_000, () => void refreshCi($).catch(() => undefined))
    void $.clock.every(10_000, () => void syncSettings($).catch(() => undefined))
    // Sessions-Reiter: Status-Datei alle 5 s auf Änderung prüfen, solange der Reiter gewählt ist (stat statt lesen).
    void $.clock.every(5_000, () => void (async () => { if ((await read($, paneTab)) === 'sessions') await refreshSessions($) })().catch(() => undefined))
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
          // Rückblick und Frage umbrechen (früher bei „truncate-end“ am rechten Rand abgeschnitten).
          <Box flexGrow={1} flexShrink={1}>
            <Text color={THEME.accent}>
              <Text bold>{t(s.lang, 'skin.wrapupHead')}</Text> · {wrap?.recap}
              {wrap?.ask ? ` · ${t(s.lang, 'skin.wrapupAsk', { what: wrap.ask })}` : ''}
            </Text>
          </Box>
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

  // Rückfrage: ein Kopfbalken über dem Dialog der Engine; Fragen, Optionen und Eingabe bleiben unberührt (`theirs`).
  // Kopf: Zahl und Themen der Fragen, darunter Aufgabe und laufender Schritt aus der Checkliste; auf dem Desktop je Frage
  // Auswahlart, Zahl der Optionen und die empfohlene Option. Das Terminal bekommt nur Kopf und Kontext (Zeilenbudget 12).
  on('ui.render', { component: 'AskUserQuestion' }, async ($, e, next) => {
    const theirs = await next(e)
    if (!drawsOn(e.surface)) return theirs
    const s = await read($, settings)
    if (!s.skinAsk) return theirs
    const ui = elementsOf($, e)
    const { Box, Text } = ui
    const L = s.lang
    const key = `om-ask-${shortHash(JSON.stringify(e.props.questions))}`
    const ctx = askContext(s.glance ? await read($, glanceJob) : null, L)
    const infos = 'Svg' in ui ? askInfos(e.props.questions).slice(0, 4) : []
    return (
      <Box key={key} flexDirection="column">
        <Box flexDirection="column" borderStyle="round" borderColor={THEME.accent} paddingX={1}>
          <Box flexDirection="row" columnGap={1} alignItems="center">
            {skinMark(ui, await markerOf($), `${key}-m`, 'style.ask')}
            <Text color={THEME.accent} bold wrap="truncate-end">{askSummary(e.props.questions, L)}</Text>
          </Box>
          {ctx !== '' && <Text key={`${key}-ctx`} dimColor wrap="truncate-end">{ctx}</Text>}
          {infos.map((q, i) => (
            <Box key={`${key}-q${i}`} flexDirection="row" columnGap={1}>
              <Text color={THEME.accent}>{i + 1}.</Text>
              {q.head !== '' && <Text bold wrap="truncate-end">{q.head}</Text>}
              <Text dimColor>{t(L, q.multi ? 'skin.askMulti' : 'skin.askSingle')}{q.options > 0 ? ` · ${t(L, 'skin.askOptions', { n: q.options })}` : ''}</Text>
              {q.recommended !== '' && <Text color={THEME.success} wrap="truncate-end">{t(L, 'skin.askRec', { label: q.recommended })}</Text>}
            </Box>
          ))}
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
    // Einziger Einstieg ins Cockpit-Panel unter dem Prompt (Muster effortless: kurzes Label, Leerzeichen als Innenrand,
    // Hover-Hintergrund wie die Pillen der App). Der MR-Status steht nur noch im Band (Link auf den Reiter Git).
    const pill = (scope: string) => ({ scope, backgroundColor: THEME.chip })
    return (
      <Box key="om-mode" flexDirection="row" gap={1} alignItems="center">
        {modes.length > 0 && s.skinMode && skinMark(ui, await markerOf($), 'om-mode-m', 'style.mode')}
        {modes.length > 0 && (s.skinMode ? <Text color={THEME.accent}>{modes.join(' & ')}</Text> : <Text dimColor>{modes.join(' & ')}</Text>)}
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
    await openTab($, 'back', 'session')
    return { text: t(lang, 'common.openedSummary') }
  })

  on('command.run', { command: 'cockpit-einstellungen' }, async $ => {
    const lang = await langOf($)
    await openTab($, 'set')
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

  // Zeilen der Unterhaltung für den Kontext-Inspektor und Benachrichtigungen von Hintergrundaufgaben. Nur die Türen, die
  // `cc-plugin-sec-default` an Plugins der Nutzer-Ebene durchlässt: Anhänge, Hook-Kontext, Kompaktierung und Skill-Texte
  // gehen an ihnen vorbei, ein Hook darauf liefe nie und hinterließe je Zeile einen Hinweis im Chat.
  // ponytail: Endet eine Hintergrundaufgabe mitten im Turn (Anhang), merkt das Cockpit es erst beim nächsten Prompt.
  // Matcher als Literale und ein Hook auf Modulebene, sonst kann `claude plugin validate` sie nicht auflösen.
  on('session.append', { door: 'prompt' }, onAppend)
  on('session.append', { door: 'command' }, onAppend)
  on('session.append', { door: 'response' }, onAppend)
  on('session.append', { door: 'tool-result' }, onAppend)
  on('session.append', { door: 'delivery' }, onAppend)
  on('session.append', { door: 'note' }, onAppend)
  on('session.append', { door: 'notice' }, onAppend)
  on('session.append', { door: 'queue' }, onAppend)

  on('prompt.submit', async ($, e, next) => {
    // Benachrichtigungen kommen auch gebündelt oder mit einem getippten Prompt zusammen: jeden Prompt danach absuchen.
    await finishBackground($, e.text)
    await update($, dirAlert, () => null)
    if (e.origin?.kind !== 'task-notification') {
      const heavy = guardPrompt(guardSt, await stamp($), (await read($, usage))?.contextPercent)
      if (heavy !== null) await guardNotify($, [heavy])
    }
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
    if (isMain && mayChange(e.tool, e)) await tlBase($)
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
    // Fehler-Eingang, Nutzungswächter (gleicher Aufruf in Folge) und Zeitreise: nur beobachten, nie die Session stören.
    try {
      const target = toolTarget(args)
      const denied = ran.deny !== undefined
      errCall(errSt)
      if (denied || isError) noteError(errSt, { at: startedAt, tool: e.tool, target, kind: denied ? 'denied' : 'error', text: inboxErrorText(ran), agent: !isMain, id: e.tool_use_id })
      if (isMain) {
        const repeat = guardCall(guardSt, startedAt, e.tool, args, target)
        if (repeat !== null) await guardNotify($, [repeat])
        const step = addStep(tlSt, { at: startedAt, tool: e.tool, target, ok: ranOk, id: e.tool_use_id })
        if (mayChange(e.tool, e)) tlSnap($, step)
      }
    } catch {
      // verworfen
    }
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
    // Aufrufe eines Subagents mit Quelle `agent`; die tool_use_id verbindet die Zeile im Protokoll mit Fehler und Snapshot.
    await note($, isMain ? 'main' : 'agent', `${toolText(e.tool)}${target === '' ? '' : ` → ${target}`}${isError ? ' ✗' : ''}`, { tool: e.tool, target, action: toolAction(e), isError, id: e.tool_use_id })
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
      lastCacheWrite = result.usage.cache_creation_input_tokens
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
    {
      const s = await read($, settings)
      const live = (await read($, agents)).filter(a => LIVE.has(a.status)).length
      const at = await stamp($)
      if (shouldHold(guardSt, live, s.guardHold)) {
        const why = guardTx(s.lang, 'guard.deny', { n: live, max: GUARD_LIMITS.hold })
        noteError(errSt, { at, tool: 'Agent', target: e.subagentType, kind: 'denied', text: why, agent: false })
        await guardNotify($, [{ rule: 'agents', at, n: live, detail: '' }])
        return { deny: why }
      }
      await guardNotify($, guardSpawn(guardSt, at, live))
    }
    const spawned = await next(e)
    if (spawned.agentId) {
      const id = spawned.agentId
      const at = await stamp($)
      observeProfile(() => subagentStart(profileSt, id, e.subagentType, at, e.tool_use_id))
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
      guardReset(guardSt)
    }
    else await touchJob($)
    return started
  })

  on('turn.complete', async ($, e, next) => {
    const done = await next(e)
    const doneAt = await stamp($)
    observeProfile(() => turnComplete(profileSt, e.turnId, doneAt, e.reason))
    // Ende eines Subagents und Abbruch durch einen API-Fehler (früher `classic.SubagentStop`/`classic.StopFailure`).
    const sub = e.agentId
    if (sub !== undefined) observeProfile(() => subagentStop(profileSt, sub, doneAt))
    if (e.reason === 'error') observeProfile(() => stopFailure(profileSt, sub, doneAt, 'error'))
    if (!e.agentId) void refreshStyle($).catch(() => undefined)
    if (!e.agentId) void refreshMr($, true).catch(() => undefined)
    // Git-Panel nur nachziehen, wenn es kürzlich gezeichnet wurde (offen); sonst kostet jeder Turn ein Dutzend git-Aufrufe.
    if (!e.agentId && (await $.clock.now()) - gitPaneAt < 90_000) void refreshGit($, true).catch(() => undefined)
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

  // Kein Hook auf `classic.*`, `prompt.compose` und die Engine-Zeilen von `session.append`: das eingebaute Mod
  // `cc-plugin-sec-default` lässt Plugins der Nutzer-Ebene dort nicht mitreden, jeder Versuch wäre eine Hinweiszeile im Chat.
  // Subagents kommen aus `agent.spawn` und `turn.complete`, der Ausgabestil aus `$.settings.read`.

  on('ui.render', { component: 'AbovePrompt' }, async ($, e, next) => {
    if (e.surface === 'desktop') {
      const at = await stamp($)
      if (at - schemeCheckedAt >= SCHEME_EVERY_MS) {
        schemeCheckedAt = at
        void syncDesktopScheme($, options.appearance, true).catch(() => undefined)
      }
    }
    const s = await read($, settings)
    const u = await read($, usage)
    // Kappung auch hier: $.state überlebt Reloads und kann noch Listen älterer Versionen (fünf Schritte) halten.
    const next3 = e.props.isWorking || !s.steps ? [] : (await read($, steps)).slice(0, 3)
    const asks3 = e.props.isWorking || !s.steps ? [] : (await read($, asks)).slice(0, 3)
    // Die Zeile „Weiter“ steht immer, wenn Folge-Schritte an sind und Claude nicht arbeitet, auch ohne Vorschläge.
    const showSteps = s.steps && !e.props.isWorking
    const busySteps = await read($, stepsBusy)
    // Checkliste: zeigt sich auch ohne Messung, sobald der Job Plan, Tool-Arbeit oder eine Rückfrage hat.
    const job = s.glance ? await read($, glanceJob) : null
    const showJob = job !== null && isWorthShowing(job)
    const alert = await read($, loopAlert)
    const dirWarn = await read($, dirAlert)
    if (!drawsOn(e.surface) || e.props.hasSurvey || (u === null && !showJob && next3.length === 0 && alert === null && dirWarn === null)) return next(e)
    await read($, tick)
    const now = await $.clock.now()
    const lastAt = await read($, lastTurnAt)
    const bandTips = s.tipsBand ? tipsNow(u, lastAt, now) : []
    const liveAgents = (await read($, agents)).filter(a => LIVE.has(a.status))
    const running = liveAgents.length
    const ui = elementsOf($, e)
    const { Box, Text } = ui
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
    const head = titleRow(ui, await read($, about), await read($, logo), false)
    const where = s.branch ? repoRow(ui, await read($, repo)) : null
    const mrRow = s.mrBand ? await mrBandRow($, ui, L, (await read($, devMock)) ? mockData(now).mr : null) : null
    const fillCompact = async () => {
      const focus = compactFocus(await read($, lastPrompt), await read($, touched), await read($, steps), L)
      await $.prompt.fill({ text: focus === '' ? '/compact' : `/compact ${focus}` })
    }
    // Gruppen ohne Rahmen, getrennt durch einen gedimmten Strich; jede Gruppe bricht als Ganzes um.
    // Trennstriche merken, damit ausgeblendete Gruppen keine doppelten oder hängenden Striche hinterlassen.
    const seps = new Set<RenderElement>()
    const sep = (key: string) => {
      const el = <Text key={key} color={THEME.border}>│</Text>
      seps.add(el)
      return el
    }
    const group = (key: string, ...children: RenderChildren[]) => (
      <Box key={key} flexDirection="row" columnGap={1} alignItems="center" flexShrink={0}>
        {children}
      </Box>
    )
    // Werte sind Links auf den passenden Reiter des Panels; Aktionen sind Links ohne Fläche (keine Knöpfe im Band).
    const toUsage = () => void openTab($, 'usage').catch(() => undefined)
    const mi = await read($, modelInfo)
    const pick = await read($, effortPick)
    const lim = u === null ? null : limitAlert(u.limits)
    const handoffMsg = await read($, handoffNote)
    const stepCols = Math.max(20, Math.floor((e.props.bodyColumns - (e.surface === 'terminal' ? 8 : 4)) / 3))
    // Weiter-Zeile ohne Umbruch: Breite abzüglich „Weiter:“, Refresh-Icon und Abständen teilen sich die Vorschläge; wird es eng,
    // fallen die hinteren Vorschläge weg statt in eine zweite Zeile zu rutschen.
    const stepsAvail = e.props.bodyColumns - (e.surface === 'terminal' ? 8 : 4) - 23
    const stepsShown = Math.max(1, Math.min(3, Math.floor(stepsAvail / 24)))
    const stepsCol = Math.max(16, Math.floor(stepsAvail / stepsShown))
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
    // Sparmodus: Punkt in Statusfarbe (an grün, aus rot) mit Erklärung als Tooltip, daneben der Link mit dem Zustand.
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
          {link(ui, 'save-mode', t(L, s.saveMode ? 'alert.saveOn' : 'alert.saveOff'), () => void setSettings($, { saveMode: !s.saveMode }).catch(() => undefined), { tone: s.saveMode ? 'ok' : 'muted' })}
        </Box>
      )
    }
    const toggleAuto = () => void update($, effortBase, () => null).then(() => setSettings($, { autoEffort: !s.autoEffort })).catch(() => undefined)
    // Atemübung: solange Claude arbeitet, atmet der Statusring der Checkliste (Einzelbilder im 250-ms-Takt, Atom `frame`);
    // die ganze Übung mit Phasen steht im Reiter Jetzt.
    const tn = await read($, turn)
    const working = e.props.isWorking && tn !== null && tn.durationMs === undefined
    const breathes = working && s.breath && s.animation && Svg !== undefined && showJob
    if (breathes) await read($, frame)
    const breath = breathes ? fillAt(breathOf(s.breathKey), now - (tn?.startedAt ?? now)) : null
    const asking = gateOf(await read($, gate)).ask
    const phase = showJob && job !== null ? glancePhase(job, now, asking) : null
    // Claudes Rückfragen stehen direkt unter der Frage in der Checkliste; ohne sichtbare Frage in einer eigenen Zeile.
    const answersInGlance = job !== null && phase === 'needsYou' && !(asking > 0 && job.phase === 'working') && job.question !== ''
    const sig = signalsOf($, ui, s, { percent: u?.contextPercent, now, tips: bandTips, loop: alert, dir: dirWarn, forBand: true })
    // Folge-Schritt oder Antwort als Link mit Taste 1–3 bzw. 4–6 (auch im leeren Eingabefeld; das Terminal zeigt „1: …“).
    // Nur ins Eingabefeld: Enter sendet als Eingabe der Person, damit andere Mods (effortless) ihn wie getippt bewerten.
    const stepLink = (key: string, n: number, label: string, prompt: string, max: number) =>
      link(ui, key, clip(label, max), () => void $.prompt.fill({ text: prompt }).catch(() => undefined), { hotkey: String(n) })
    // Ampel eines Werts als Linkton (Gelb/Rot als getönter Grund); ohne Ring (Terminal, Ringe aus) zusätzlich ein farbiger Punkt.
    const toneOf = (c: string): LinkTone => (c === 'error' ? 'bad' : c === 'warning' ? 'warn' : 'link')
    const dot = (key: string, c: keyof typeof RING_HEX) => (Svg && s.rings ? null : <Text key={key} color={THEME[c]}>●</Text>)

    // Im Terminal setzt ein Rahmen in Markenblau (om-blue-500) auf dunkelblauem Grund das Band vom Transcript ab;
    // der Desktop rahmt es selbst. Voller blauer Grund wäre für die Statusfarben (grün, gelb) zu kontrastarm.
    const term = e.surface === 'terminal'
    const body = (
      <Box flexDirection="column" {...(term ? { borderStyle: 'round', borderColor: THEME.accentFill, backgroundColor: TINT.accent, paddingX: 1 } : {})}>
        {/* Checkliste ganz oben. */}
        {showJob && job !== null && glanceBand($, ui, job, { L, now, avg: await read($, glanceAvg), asking, live: liveAgents, breath, answers: answersInGlance ? asks3 : [] })}
        {/* Kopf und Messwerte in einer Zeile; sie bricht in Gruppen um, nur der Branch kürzt sich. */}
        <Box key="band-main" flexDirection="row" flexWrap="wrap" columnGap={2} alignItems="center">
          <Box flexShrink={0}>{head}</Box>
          {where !== null && <Box flexShrink={1}>{where}</Box>}
          {mrRow !== null && <Box key="mr-head" flexShrink={0}>{mrRow}</Box>}
          {...(u !== null ? metrics(u, lastAt, await read($, cacheHit), await read($, breakdown)) : [])}
        </Box>
        {/* Eine Meldungszeile statt Warnzeilen und Kurzinfos: nur Gelb und Rot, der Rest im Reiter Diagnose. */}
        {s.bandAlerts && bandAlerts($, ui, s, sig)}
        {showSteps && (
          // Drei Spalten gleicher Breite: Ziffer und Vorschlag als Link, die Begründung als Tooltip am ⓘ dahinter;
          // ohne Svg (Terminal) als Zeile darunter. Hellgelb hinterlegt (TINT.warning, folgt hell/dunkel), damit sich der
          // Vorschlagsblock von den Messwerten absetzt.
          <Box key="steps" flexDirection="row" columnGap={2} alignItems="flex-start" flexWrap="nowrap" backgroundColor={TINT.warning} paddingX={1}>
            <Box flexShrink={0}><Text color={THEME.textMuted} wrap="truncate-end">{t(L, 'band.next')}</Text></Box>
            {next3.length === 0 && <Text key="steps-none" color={THEME.textMuted}>{t(L, 'band.noSteps')}</Text>}
            {next3.slice(0, stepsShown).map((st, i) => {
              const max = Math.min(STEP_LABEL_MAX, Math.max(10, stepsCol - (st.fromClaude ? 10 : 6)))
              // Tooltip: bei gekürztem Knopftext zuerst der volle Text, dann die Begründung.
              const tip = [st.label.length > max ? st.label : '', st.why ?? ''].filter(x => x !== '').join('\n')
              return (
                <Box key={`step-col-${i}`} flexDirection="column" width={stepsCol} flexShrink={1}>
                  <Box key={`step-row-${i}`} flexDirection="row" columnGap={1} alignItems="center" flexWrap="nowrap">
                    {st.fromClaude && chip(ui, 'band', `step-cc-${i}`, 'CC')}
                    {stepLink(`step-${i}`, i + 1, st.label, st.prompt, max)}
                    {Svg && tip !== '' && <Svg key={`step-why-${i}`} source={infoSvg(tip, THEME.textMuted)} alt={tip} width={14} height={14} isInteractive />}
                  </Box>
                  {!Svg && st.why !== undefined && <Text color={THEME.textMuted} wrap="truncate-end">{st.why}</Text>}
                </Box>
              )
            })}
            <Box flexShrink={0}>
              {link(ui, 'steps-refresh', busySteps ? '◌' : '↻', () => void refreshSteps($).catch(() => undefined), { tone: 'muted', busy: busySteps ? 'locked' : false })}
            </Box>
          </Box>
        )}
        {!answersInGlance && asks3.length > 0 && (
          <Box key="asks-row" flexDirection="row" columnGap={2} alignItems="flex-start" flexWrap="wrap" backgroundColor={TINT.warning} paddingX={1}>
            <Text color={THEME.warning}>{t(L, 'band.asks')}</Text>
            {asks3.map((a, i) => (
              <Box key={`ask-col-${i}`} width={stepCols}>
                {stepLink(`ask-${i}`, i + 4, a.label, a.prompt, Math.max(12, stepCols - 6))}
              </Box>
            ))}
          </Box>
        )}
      </Box>
    )
    // Der Rahmen selbst nimmt keinen Grund an (nur sein Inneres): eine äußere Box mit demselben Grund füllt auch die Rahmenzellen.
    const ours = term ? <Box key="band-bg" flexDirection="column" backgroundColor={TINT.accent}>{body}</Box> : body
    // Bänder anderer Mods (ponytail, effortless …) unter unserem mitzeichnen statt sie zu verdecken.
    const theirs = await next(e)
    return <Box key="band-stack" flexDirection="column">{ours}{theirs}</Box>

    // Messwerte als Gruppen: Limits, Effort, Kontext, Cache/Kosten/Agents. Werte sind Links auf den Reiter Verbrauch.
    function metrics(u: Usage, lastAt: number | null, hit: number | null, bd: Breakdown | null): RenderElement[] {
      const ttl = u.limits.length > 0 ? 3600_000 : 300_000
      const cache = cacheLeftMs(lastAt, u.limits.length > 0, now)
      const isFull = (u.contextPercent ?? 0) >= COMPACT_AT
      const ctxTone = contextColor(u.contextPercent ?? 0)
      // Ohne bekannten letzten Request ist der Cache unbekannt, nicht kalt.
      const pays = lastAt !== null && handoffPays(u.contextPercent, cache)
      const level = mi?.effort ?? pick
      const levelText = level === undefined || level === null ? '' : `${String(level).charAt(0).toUpperCase()}${String(level).slice(1)} · `
      const out: RenderElement[] = [sep('sep-head')]
      if (s.limits && u.limits.length > 0) {
        out.push(
          group(
            'g-lim',
            ...u.limits.map(l => {
              const label = `${LIMIT_LABEL[l.kind] ?? l.kind} ${Math.round(l.percentUsed)}%`
              // Restzeit bis zum Reset immer dahinter; ohne Svg (kein Ring-Tooltip) zusätzlich die Uhrzeit.
              const rest = l.resetsAt ? ` · ↻ ${resetLeft(l.resetsAt, now)}${Svg ? '' : ` (${resetLabel(l.resetsAt, now, L)})`}` : ''
              const c = limitColor(l, now)
              return (
                <Box key={`lim-${l.kind}`} flexDirection="row" columnGap={1} alignItems="center">
                  {ring(l.percentUsed, c, s.legend ? `${limitTip(l, tipNow, L)}${NL}${NL}${legendTip(L)}` : limitTip(l, tipNow, L))}
                  {dot(`lim-dot-${l.kind}`, c)}
                  {link(ui, `lim-link-${l.kind}`, `${label}${rest}`, toUsage, { tone: toneOf(c) })}
                </Box>
              )
            }),
            // Ab 80 % und mit Auto-Effort: Sparmodus direkt an der Limit-Anzeige.
            lim !== null && s.autoEffort && saveToggle(),
          ),
          sep('sep-lim'),
        )
      }
      if (mi !== null && s.bandEffort) {
        out.push(
          group(
            'g-effort',
            <Text key="effort" color={effortColor(level)}>◆ {levelText}{modelLabel(mi.model)}</Text>,
            link(ui, 'auto', t(L, s.autoEffort ? 'effort.autoOn' : 'effort.auto'), toggleAuto, { tone: s.autoEffort ? 'link' : 'muted' }),
            s.autoEffort && effortMsg !== '' && effortView(),
          ),
          sep('sep-effort'),
        )
      }
      const ctxLabel = `${t(L, 'common.context')} ${Svg && s.rings ? '' : `${bar(u.contextPercent ?? 0, 8)} `}${Math.round(u.contextPercent ?? 0)}%`
      if (s.bandContext) out.push(
        group(
          'g-ctx',
          <Box key="ctx" flexDirection="row" columnGap={1} alignItems="center">
            {ring(u.contextPercent ?? 0, ctxTone, contextTip(u, bd, L))}
            {dot('ctx-dot', ctxTone)}
            {link(ui, 'ctx-link', ctxLabel, toUsage, { tone: toneOf(ctxTone) })}
          </Box>,
          // Wenn ein frischer Chat günstiger wäre, wird Handoff die Hauptaktion; Kompaktieren bleibt daneben.
          pays && link(ui, 'handoff', t(L, 'handoff.action'), () => void writeHandoff($).catch(() => undefined), { tone: 'strong' }),
          // Schreibt /compact mit dem, was erhalten bleiben muss, ins Eingabefeld; ab 85 % auf gelbem Grund.
          link(ui, 'compact', t(L, 'band.compact'), () => void fillCompact().catch(() => undefined), { tone: isFull ? 'warn' : 'muted' }),
          handoffMsg !== '' && <Text key="handoff-note" color={THEME.textMuted}>{handoffMsg}</Text>,
        ),
      )
      // Cache, Kosten und Agent-Zahl teilen sich eine Gruppe; jeder Teil hat seinen eigenen Schalter.
      const showUsd = s.bandCost && u.usd !== undefined
      const showAgents = s.bandAgents && running > 0 && !showJob
      if (s.cache || showUsd || showAgents) {
        const cacheLabel = `${t(L, 'band.cache')} ${cache === 0 ? (u.contextTokens !== undefined ? t(L, 'band.coldCost', { n: tokens(u.contextTokens) }) : t(L, 'band.cold')) : duration(cache)}${cache > 0 && hit !== null ? ` · ${hit}%` : ''}`
        const cacheTone: LinkTone = (cache === 0 && u.contextTokens !== undefined) || (cache > 0 && cache <= CACHE_WARN_MS) ? 'warn' : cache === 0 ? 'muted' : 'link'
        out.push(
          sep('sep-cache'),
          group(
            'g-cache',
            s.cache && (
              <Box key="cache" flexDirection="row" columnGap={1} alignItems="center">
                {ring((100 * cache) / ttl, 'accent', cacheTip(cacheLeftMs(lastAt, u.limits.length > 0, tipNow), ttl, hit, lastAt, tipNow, L))}
                {link(ui, 'cache-link', cacheLabel, toUsage, { tone: cacheTone })}
              </Box>
            ),
            showUsd && u.usd !== undefined && <Text key="usd">{usd(u.usd)}</Text>,
            // Laufende Helfer nennt die Checkliste („↳ 2× executor“); ohne sichtbare Checkliste steht die Zahl hier.
            showAgents && link(ui, 'agents', t(L, 'band.agents', { n: running }), () => void openTab($, 'now').catch(() => undefined)),
          ),
        )
      }
      // Ausgeblendete Gruppen hinterlassen sonst doppelte oder hängende Trennstriche.
      const isSep = (x: RenderElement | undefined) => x !== undefined && seps.has(x)
      return out.filter((x, i) => !isSep(x) || (i < out.length - 1 && !isSep(out[i + 1])))
    }
  })

  on('command.run', { command: 'cockpit-profiling' }, async $ => {
    await openTab($, 'diag')
    return {}
  })
  on('command.run', { command: 'cockpit-fehler' }, async ($, e) => {
    const L = await langOf($)
    if ((e.args ?? '').trim() === 'text') return { text: errReport(errSt, await $.clock.now(), L) }
    await update($, logFilter, () => 'errors' as LogFilter)
    await openTab($, 'diag', 'log')
    return {}
  })
  on('command.run', { command: 'cockpit-zeitreise' }, async ($, e) => {
    const L = await langOf($)
    if ((e.args ?? '').trim() === 'text') return { text: tlReport(tlSt, L) }
    await update($, logFilter, () => 'snaps' as LogFilter)
    await openTab($, 'diag', 'log')
    return {}
  })
  on('command.run', { command: 'cockpit-kontext' }, async ($, e) => {
    const L = await langOf($)
    if ((e.args ?? '').trim() === 'text') return { text: inspectReport(inspectSum(), L) }
    await openTab($, 'usage')
    return {}
  })
  on('command.run', { command: 'cockpit-profil' }, async ($, e) => {
    const L = await langOf($)
    const arg = (e.args ?? '').trim()
    if (arg === 'text' || arg === 'turn') return { text: profileReport(profileSum(await $.clock.now()), await $.clock.now(), L, arg === 'turn' ? 'turn' : 'session') }
    await openTab($, 'diag', 'profile')
    return {}
  })
  on('command.run', { command: 'cockpit-blast' }, async ($, e) => {
    const text = await blastCommand($, e.args ?? '')
    return text === '' ? {} : { text }
  })

  on('command.run', { command: 'cockpit-git' }, async $ => {
    await openTab($, 'git')
    return {}
  })

  on('command.run', { command: 'cockpit-sessions' }, async $ => {
    await openTab($, 'sessions')
    return {}
  })

  on('command.run', { command: 'cockpit-ci' }, async $ => {
    await openTab($, 'ci')
    return {}
  })

  // Das eine Cockpit-Panel: Kopf mit Titel, Modellzeile und Reitern, darunter nur der gewählte Reiter (Atom `paneTab`).
  // Jetzt: Arbeit, Plan, Helfer, letzter Turn · Verbrauch: Limits, Kontext, Kosten · Diagnose: Hinweise, Protokoll, Profil,
  // Blast-Radius · Git: Stand und Ablauf · Rückblick: Zusammenfassung und Verlauf · Einstellungen.
  on('ui.render', { component: 'Pane', requestId: 'tenvan-cc' }, async ($, e, next) => {
    if (!drawsOn(e.surface)) return next(e)
    await read($, tick)
    const now = await $.clock.now()
    const s = await read($, settings)
    // Nur lesen, was der gewählte Reiter zeigt: jedes gelesene Atom zeichnet das Panel bei Änderung neu (Einstellungen, Rückblick
    // und Git zeichnen so nicht bei jedem Tool-Aufruf; Git liest dort sonst alle 20 s neu).
    const tab = await read($, paneTab)
    const needs = (...tabs: PaneTab[]) => tabs.includes(tab)
    const realBd = needs('usage') ? await read($, breakdown) : null
    const bdNote = needs('usage') ? await read($, breakdownNote) : ''
    const realSessions = needs('sessions') ? await read($, sessionsFile) : null
    const realPlace = needs('now', 'git') ? await read($, repo) : null
    const realU = needs('usage', 'diag') ? await read($, usage) : null
    const realTn = needs('now') ? await read($, turn) : null
    const realGate = needs('now', 'diag') ? await read($, gate) : GATE0
    const realList = needs('now') ? await read($, agents) : []
    const realEntries = needs('now', 'diag') ? await read($, log) : []
    const realLast = needs('usage', 'diag') ? await read($, lastTurnAt) : null
    const realHit = needs('usage') ? await read($, cacheHit) : null
    // Dev-Mode: alle Panel-Werte aus mockData, die echten Atome bleiben unberührt.
    const dev = await read($, devMock)
    const m = dev ? mockData(now) : null
    const place = m?.repo ?? realPlace
    const sess = m?.sessions ?? realSessions
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
    const ui = elementsOf($, e)
    const { Box, Text } = ui
    const Svg = 'Svg' in ui ? ui.Svg : undefined
    const marg = s.paneMargins ? 1 : 0
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
    const gap = () => (marg ? <Box height={1} /> : null)
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
    // Unterreiter (Diagnose, Rückblick) und Filter: eine Link-Zeile mit Abstand nach unten.
    const subRow = (key: string, row: RenderElement) => (
      <Box key={key} paddingX={marg} marginTop={marg} marginBottom={marg ? 0 : 1}>
        {row}
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
    const brakedIds = await read($, braked)
    const brake = async (a: Agent) => {
      await update($, braked, ids => (ids.includes(a.id) ? ids : [...ids, a.id]))
      await note($, 'agent', t(L, 'log.braked', { name: a.description }))
    }
    const taskState = (x: BgTask) => {
      const bad = x.status === 'failed' || x.status === 'killed'
      const d = duration(x.status === 'running' || x.durationMs === undefined ? now - x.since : x.durationMs)
      return x.status === 'running' ? `● ${t(L, 'agents.running', { d })}` : `${bad ? '✗' : '✓'} ${x.status} ${d}`
    }
    // Öffnen: das Transcript des Subagents in der Hauptansicht (nur auf Klick der Person). Das kann nur das Terminal; der Desktop
    // antwortet „no main view here“, ein beendeter Subagent „that agent has ended“. Dann zeigt eine Karte im Reiter Jetzt seinen
    // Verlauf aus `$.session.messages({ agentId })`; ein zweiter Klick auf denselben schließt sie.
    const openView = async (id: string) => {
      const shown = await read($, agentView)
      if (shown?.id === id) return update($, agentView, () => null)
      const r = await $.ui.view({ agentId: id })
      if ('deny' in r && typeof r.deny === 'string') await loadView(id)
    }
    const loadView = async (id: string) => {
      const got = await $.session.messages({ agentId: id })
      const at = await $.clock.now()
      const title = list.find(a => a.id === id)?.description ?? id
      const v: AgentView = Array.isArray(got) ? { id, title, lines: agentTranscript(got), note: '', at } : { id, title, lines: [], note: got.deny, at }
      await update($, agentView, () => v)
    }
    // Stopp einer Hintergrundaufgabe: TaskStop ist Sache des Modells, darum steht der Auftrag im Eingabefeld.
    const stopTask = async (x: BgTask) => {
      await $.prompt.fill({ text: stopTaskPrompt(x.id, x.description, L) })
      $.ui.toast(t(L, 'tasks.stopFilled'))
    }
    // Symbolknöpfe vor jedem Eintrag, nur was geht: Stopp (■) solange er läuft (Subagent: bremsen, Aufgabe: Auftrag an Claude),
    // Öffnen (↗) nur bei Subagents. Geht nichts, fehlt die Zeile.
    const rowButtons = (key: string, id: string, stop: (() => Promise<unknown>) | null) => {
      if (stop === null && key !== 'ag') return null
      return (
        <Box key={`${key}-btns`} flexDirection="row" columnGap={1}>
          {stop !== null && pill(ui, 'pane', key === 'ag' ? `brake-${id}` : `${key}-stop-${id}`, t(L, 'agents.stop'), () => void stop().catch(() => undefined))}
          {key === 'ag' && pill(ui, 'pane', `ag-open-${id}`, t(L, 'agents.open'), () => void openView(id).catch(() => undefined))}
        </Box>
      )
    }

    // ── Karten ──────────────────────────────────────────────────────────────────────────────────────────────────────
    // Lade-Karte, solange ein Turn läuft; mit Atemübung (früher als 56-px-Zeile im Band) und dem letzten Log-Eintrag. Die Übung
    // animiert sich selbst (SMIL): das Panel liest `frame` nicht, sonst zeichnete es 4×/s neu und der erste Klick verpuffte
    // (Entscheidungslog 2026-10-12).
    const workingCard = async () => {
      if (!s.animation || tn === null || tn.durationMs !== undefined) return null
      const b = breathOf(s.breathKey)
      let breathRow: RenderElement | null = null
      if (s.breath && Svg) {
        const labels = { in: t(L, 'breath.in'), sip: t(L, 'breath.sip'), hold: t(L, 'breath.hold'), out: t(L, 'breath.out') }
        const idx = BREATHS.findIndex(x => x.key === b.key)
        const nextKey = (BREATHS[(idx + 1) % BREATHS.length] ?? BREATHS[0])!.key
        breathRow = (
          <Box key="breath-row" flexDirection="row" columnGap={2} alignItems="center" marginTop={1}>
            <Svg key="breath" source={breathSvg(b, labels, t(L, `breath.name.${b.key}`), BREATH_COLOR[b.key], THEME.textMuted)} alt={t(L, 'settings.breath')} height={56} />
            {link(ui, 'breath-next', t(L, 'breath.next'), () => void setSettings($, { breathKey: nextKey }).catch(() => undefined), { tone: 'muted' })}
          </Box>
        )
      }
      return spaced(
        <Box flexDirection="column" backgroundColor={TINT.accent} marginX={marg} paddingX={1} paddingY={1}>
          <Text bold color={THEME.accent}>● {t(L, 'panel.working', { d: duration(now - tn.startedAt), tools: tn.tools, edits: tn.edits })}</Text>
          {entries.length > 0 && (
            <Text wrap="truncate-end">
              <Text dimColor>{t(L, 'panel.lastEvent', { text: '' })}</Text>
              {entryView(ui, entries[entries.length - 1] as LogEntry, true)}
            </Text>
          )}
          {breathRow}
        </Box>,
      )
    }

    // Plan-Karte: ganzer Plan des aktuellen Jobs mit Start, voraussichtlichem Ende, Dauer je Schritt und Dateien.
    const planCard = async () => {
      if (!s.cardPlan) return null
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
        // Verlauf früherer Aufgaben steht im Reiter Rückblick.
        s.glanceHistory && <Box key="plan-hist" marginTop={1}>{link(ui, 'glance-history', t(L, 'glance.history'), () => void openHistory($).catch(() => undefined), { tone: 'muted' })}</Box>,
      )
    }

    // Arbeitsverzeichnis: Session- und Prozess-Ordner, Worktree, verknüpfte Sessions. Ohne Git-Teil (Schalter „Git-Panel“ aus)
    // stehen hier auch Branch, Commit, Änderungen und Upstream (früher eigene Karte „Git“).
    const workdirCard = async () => {
      const d = await read($, dirs)
      if (!s.cardWorkdir || (place === null && d.root === '')) return null
      const out = await read($, dirAlert)
      const ls = m?.linked ?? (await read($, linked))
      return card(
        'cardWorkdir',
        t(L, 'panel.workdir'),
        <Box key="wd-dirs" flexDirection="column" marginBottom={1}>
          {labelRow(ui, 'pane', 'wd-root', t(L, 'dir.session'), <Text color={THEME.accent} bold wrap="truncate-end">{d.root || place?.path}</Text>)}
          {d.cwd !== '' && d.cwd !== d.root && labelRow(ui, 'pane', 'wd-cwd', t(L, 'dir.process'), d.cwd)}
          {out !== null && labelRow(ui, 'pane', 'wd-out', t(L, 'dir.outside'), <Text color={THEME.error} bold wrap="truncate-end">⚠ {out.dir}</Text>)}
        </Box>,
        ...(place === null ? [] : repoPathRows(place, L)).map(([label, value]) => labelRow(ui, 'pane', `wd-row-${label}`, label, value)),
        ...(place === null || s.gitPane ? [] : repoGitRows(place, L)).map(([label, value]) =>
          labelRow(ui, 'pane', `git-row-${label}`, label, place?.dirty === null && label === t(L, 'panel.repoChanges') ? (
            <Box flexDirection="row" columnGap={2} flexShrink={1}>
              <Text wrap="truncate-end">{value}</Text>
              {link(ui, 'git-trust', t(L, 'repo.trust'), () => void trustPrompt($, L).then(async text => { if (text === null) $.ui.toast(t(L, 'repo.trustFail')); else await $.prompt.fill({ text }) }).catch(() => undefined))}
            </Box>
          ) : value),
        ),
        // Verknüpfte Sessions (nur Desktop): Status, Name, Branch; neueste zuerst wie die App sie liefert.
        ls.length > 0 && (
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
        ),
      )
    }

    // Kontextfenster: Kategorien mit Balken, dazu die Kontext-Diät (nie aufgerufene MCP-Server mit ihren Schema-Tokens).
    const contextCard = async () => {
      if (!s.cardContext) return null
      if (bdPanel === null) return card('cardContext', t(L, 'panel.context'), <Text dimColor>{bdNote !== '' ? bdNote : t(L, 'panel.contextPending')}</Text>)
      const diet = m ? mcpDiet(m.mcpTools, m.usedMcp) : mcpDiet(await read($, mcpTools), await read($, usedMcp))
      return card(
        'cardContext',
        t(L, 'panel.context'),
        <Text dimColor>{breakdownTitle(bdPanel, L)}</Text>,
        Svg && <Svg source={breakdownBarSvg(bdPanel)} alt={breakdownTitle(bdPanel, L)} height={14} />,
        ...bdPanel.rows.map((r, i) => (
          <Box key={`bd-${i}`} flexDirection="row" columnGap={1}>
            <Text color={categoryColor(r, i)}>■</Text>
            <Box width={24}><Text>{r.name}</Text></Box>
            <Box width={7} justifyContent="flex-end"><Text dimColor>{tokens(r.tokens)}</Text></Box>
            <Box width={7} justifyContent="flex-end"><Text bold>{r.percent.toFixed(1)}%</Text></Box>
            <Text color={categoryColor(r, i)}>{bar(r.percent, 12)}</Text>
          </Box>
        )),
        ...(diet.length === 0
          ? []
          : [
              <Box key="diet-head" flexDirection="row" columnGap={2} marginTop={1}>
                <Text color={THEME.warning}>{t(L, 'diet.title', { tokens: tokens(diet.reduce((n, r) => n + r.tokens, 0)) })}</Text>
                {link(ui, 'diet-mcp', t(L, 'diet.action'), () => void $.prompt.fill({ text: '/mcp' }).catch(() => undefined))}
              </Box>,
              ...diet.slice(0, 6).map(r => <Text key={`diet-${r.server}`} dimColor wrap="truncate-end">  {t(L, 'diet.row', { server: r.server, tokens: tokens(r.tokens), n: r.tools })}</Text>),
            ]),
      )
    }

    // Übersicht: Kontext in Tokens, Limits mit Takt und Reset, Kosten, Prompt-Cache.
    const overviewCard = () =>
      s.cardOverview &&
      card(
        'cardOverview',
        t(L, 'panel.overview'),
        u === null && <Text dimColor>{t(L, 'panel.noMeasure')}</Text>,
        u !== null && overviewRow('ov-ctx', t(L, 'common.context'), THEME[contextColor(u.contextPercent ?? 0)], `${bar(u.contextPercent ?? 0)} ${pct(u.contextPercent ?? 0)} · ${tokens(u.contextTokens ?? 0)} / ${tokens(u.contextWindow)}`),
        ...(u?.limits.map(l => {
          const p = pace(l, now)
          const rest = [p !== undefined ? paceText(p, L) : '', l.resetsAt ? t(L, 'panel.reset', { at: resetLabel(l.resetsAt, now, L) }) : ''].filter(x => x !== '')
          return overviewRow(`ov-${l.kind}`, LIMIT_LABEL[l.kind] ?? l.kind, THEME[limitColor(l, now)], [`${bar(l.percentUsed)} ${pct(l.percentUsed)}`, ...rest].join(' · '))
        }) ?? []),
        u?.usd !== undefined && overviewRow('ov-usd', t(L, 'panel.costLabel'), undefined, usd(u.usd)),
        u !== null && overviewRow('ov-cache', t(L, 'band.cache'), cache === 0 ? THEME.textMuted : undefined, `${cache === 0 ? t(L, 'band.cold') : duration(cache)}${hit !== null ? ` · ${t(L, 'panel.cacheHit', { n: hit })}` : ''}`),
      )

    // Letzter Turn mit Turn-Bilanz (Kosten je Turn, teure Turns ohne Edit gelb).
    const turnCard = async () => {
      if (!s.cardTurn) return null
      const turns = m?.turnLog ?? (await read($, turnLog))
      return card(
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
            <Text key="turn-bars">{turnBarsText(turns).map((x, i) => <Text key={`tb-${i}`} color={x.isWaste ? THEME.warning : THEME.accentFill}>{x.glyph}</Text>)}</Text>
          )),
        turns.some(isWasteful) && <Text color={THEME.warning}>{t(L, 'turn.waste', { n: turns.filter(isWasteful).length })}</Text>,
      )
    }

    // Subagents: nur laufende als Kachel (die neuesten `agentTiles`); beendete und weitere als Zeile im Verlauf, darunter die
    // gemeinsame Zeitachse aller Subagents.
    const agentsCard = () => {
      if (!s.cardAgents) return null
      const { tiles, past } = splitLive(list, a => LIVE.has(a.status), s.agentTiles)
      return card(
        'cardAgents',
        t(L, 'agents.title', { live: list.filter(a => LIVE.has(a.status)).length, all: list.length }),
        list.length === 0 && <Text dimColor>{t(L, 'agents.none')}</Text>,
        tiles.length > 0 && <Box flexDirection="row" flexWrap="wrap" columnGap={1}>
          {tiles.map(({ item: a, n }) => {
            const isBraked = brakedIds.includes(a.id)
            const heavy = isAgentHeavy(a, now)
            const tone: Tone = heavy || isBraked ? 'warning' : 'accent'
            return (
              <Box key={`ag-${a.id}`} flexDirection="column" width={42} backgroundColor={TINT[tone]} marginX={marg} paddingX={1} paddingY={1}>
                <Box flexDirection="row" columnGap={1}>
                  {rowButtons('ag', a.id, isBraked ? null : () => brake(a))}
                  <Text wrap="truncate-end">
                    {chip(ui, 'pane', '', String(n))} <Text bold>{a.description}</Text>
                  </Text>
                </Box>
                <Text wrap="truncate-end">{a.type}{a.model !== undefined ? ` · ${a.model}` : ''}</Text>
                <Text dimColor wrap="truncate-end">{agentStats(a, L)}</Text>
                {agentFlags(a, L) !== '' && <Text dimColor wrap="truncate-end">{agentFlags(a, L)}</Text>}
                {heavy && <Text color={THEME.warning} wrap="truncate-end">{t(L, 'agents.heavy', { steps: a.steps ?? 0, d: duration(now - a.since) })}</Text>}
                {isBraked && <Text color={THEME.warning}>{t(L, 'agents.braked')}</Text>}
                <Text color={THEME.accent}>{t(L, 'agents.running', { d: duration(now - a.since) })}</Text>
              </Box>
            )
          })}
        </Box>,
        list.length > 0 && <Text dimColor>{t(L, 'agents.timeline')}</Text>,
        // Verlauf: beendete Subagents (und laufende über der Kachelzahl) als Zeile mit Öffnen (↗), laufende zusätzlich mit Stopp.
        ...past.map(({ item: a, n }) => {
          const live = LIVE.has(a.status)
          const isKilled = a.status === 'killed' || a.status === 'failed'
          const d = duration(live || a.durationMs === undefined ? now - a.since : a.durationMs)
          return (
            <Box key={`ago-${a.id}`} flexDirection="row" columnGap={1}>
              {rowButtons('ag', a.id, live && !brakedIds.includes(a.id) ? () => brake(a) : null)}
              <Text wrap="truncate-end">
                <Text dimColor={!live}>{chip(ui, 'pane', '', String(n))} {a.description} · {a.type}{a.model !== undefined ? ` · ${a.model}` : ''} · </Text>
                <Text color={live ? THEME.accent : isKilled ? THEME.error : THEME.success}>{t(L, live ? 'agents.running' : isKilled ? 'agents.killed' : 'agents.done', { d })}</Text>
              </Text>
            </Box>
          )
        }),
        ...timeline(list),
      )
    }

    // Verlauf eines Subagents (↗ in der Subagent-Karte), solange die Karte Subagents an ist.
    const agentViewCard = async () => {
      const v = s.cardAgents ? await read($, agentView) : null
      if (v === null) return null
      const tone = (k: AgentView['lines'][number]['kind']) => (k === 'error' ? THEME.error : k === 'tool' ? THEME.textMuted : k === 'user' ? THEME.accent : undefined)
      return card(
        'cardAgents',
        t(L, 'agents.viewTitle', { name: v.title }),
        <Box key="agv-links" flexDirection="row" columnGap={2}>
          {link(ui, 'agv-reload', t(L, 'agents.viewReload'), () => void loadView(v.id).catch(() => undefined), { tone: 'muted' })}
          {link(ui, 'agv-close', t(L, 'agents.viewClose'), () => void update($, agentView, () => null).catch(() => undefined), { tone: 'muted' })}
        </Box>,
        v.note !== '' && <Text key="agv-note" color={THEME.warning} wrap="wrap">{t(L, 'agents.viewDeny', { why: v.note })}</Text>,
        v.note === '' && v.lines.length === 0 && <Text key="agv-none" dimColor>{t(L, 'agents.viewEmpty')}</Text>,
        ...v.lines.map((l, i) => (
          <Text key={`agv-${i}`} color={tone(l.kind)} dimColor={l.kind === 'tool'} wrap="wrap">
            {l.kind === 'user' ? '› ' : l.kind === 'assistant' ? '◆ ' : '  '}{l.text}
          </Text>
        )),
      )
    }

    // Hintergrundaufgaben (Shell-Jobs): wie die Subagents, nur laufende als Kachel, beendete als Zeile im Verlauf.
    const tasksCard = async () => {
      if (!s.cardTasks) return null
      const tasks = m?.bgTasks ?? (await read($, bgTasks))
      const { tiles, past } = splitLive(tasks, x => x.status === 'running')
      return card(
        'cardTasks',
        t(L, 'tasks.title', { live: tiles.length, all: tasks.length }),
        tasks.length === 0 && <Text dimColor>{t(L, 'tasks.none')}</Text>,
        tiles.length > 0 && <Box flexDirection="row" flexWrap="wrap" columnGap={1}>
          {tiles.map(({ item: x, n }) => {
            // Befehl und Ergebnis zusammen höchstens TASK_LINES Zeilen; der Rest als „+n“.
            const body = [
              ...(x.command !== undefined && x.command !== x.description ? x.command.split(/\r?\n/).map((l, j) => `${j === 0 ? '$ ' : '  '}${l}`) : []),
              ...(x.summary !== undefined && x.summary !== '' ? x.summary.split(/\r?\n/) : []),
            ]
            const shown = body.slice(0, body.length > TASK_LINES ? TASK_LINES - 1 : TASK_LINES)
            return (
              <Box key={`bg-${x.id}`} flexDirection="column" width={42} backgroundColor={TINT.accent} marginX={marg} paddingX={1} paddingY={1}>
                <Box flexDirection="row" columnGap={1}>
                  {rowButtons('bg', x.id, () => stopTask(x))}
                  <Text wrap="truncate-end">
                    {chip(ui, 'pane', '', String(n))} <Text bold>{x.description}</Text>
                  </Text>
                </Box>
                {shown.map((l, j) => <Text key={`bgl-${x.id}-${j}`} color={THEME.textMuted} wrap="truncate-end">{l}</Text>)}
                {body.length > shown.length && <Text dimColor>{t(L, 'tasks.more', { n: body.length - shown.length })}</Text>}
                <Text color={THEME.accent}>{taskState(x)}</Text>
              </Box>
            )
          })}
        </Box>,
        tasks.length > 0 && <Text dimColor>{t(L, 'agents.timeline')}</Text>,
        // Verlauf: beendete Aufgaben als Zeile mit Status und erster Ergebniszeile; ohne Knöpfe, denn Stopp gibt es nur, solange sie läuft.
        ...past.map(({ item: x, n }) => {
          const bad = x.status === 'failed' || x.status === 'killed'
          const result = x.summary?.split(/\r?\n/).find(l => l.trim() !== '')
          return (
            <Box key={`bgo-${x.id}`} flexDirection="row">
              <Text wrap="truncate-end">
                <Text dimColor>{chip(ui, 'pane', '', String(n))} {x.description} · </Text>
                <Text color={bad ? THEME.error : THEME.success}>{taskState(x)}</Text>
                {result !== undefined && <Text dimColor> · {result}</Text>}
              </Text>
            </Box>
          )
        }),
        ...timeline(tasks),
      )
    }

    // Über das Cockpit: Version und Build, Funktionsübersicht (früher Knopf „Features“ im Band) und Skin-Demo.
    const aboutCard = async () =>
      tile(
        ui,
        'pane',
        { key: 'card-about', tint: cycleTint(1), marg, grow: true, head: <Text bold>{t(L, 'about.title')}</Text> },
        <Text key="about-ver" dimColor wrap="truncate-end">{title(await read($, about))}</Text>,
        <Box key="about-links" flexDirection="row" columnGap={2} flexWrap="wrap">
          {/* $.command.run aus einem Klick kam im Desktop nicht an: Befehl ins Eingabefeld, Enter führt ihn aus. */}
          {link(ui, 'features', t(L, 'about.features'), () => void $.prompt.fill({ text: '/cockpit-features' }).catch(() => undefined))}
          {link(ui, 'about-demo', t(L, 'about.demo'), () => void $.prompt.fill({ text: '/cockpit-demo' }).catch(() => undefined))}
        </Box>,
      )

    // ── Reiter ──────────────────────────────────────────────────────────────────────────────────────────────────────
    const TABS = (['now', 'sessions', 'usage', 'diag', 'git', 'ci', 'back', 'set'] as const).map(id => ({ id, label: t(L, `tab.${id}`) }))
    const body: RenderChildren[] = []
    if (tab === 'now') {
      body.push(await workdirCard(), await workingCard(), await planCard(), agentsCard(), await agentViewCard(), await tasksCard(), await turnCard())
    } else if (tab === 'sessions') {
      body.push(...sessionsBody({ ui, L, marg, st: sess, path: sessionsPath, onReload: () => void refreshSessions($, true).catch(() => undefined) }))
    } else if (tab === 'usage') {
      body.push(overviewCard(), await contextCard(), ...(await diagCards($, ui, ['inspect'])))
    } else if (tab === 'diag') {
      const dTab = await read($, diagTab)
      const subs = (['hints', 'log', 'profile', 'blast'] as const).map(id => ({ id, label: t(L, `diag.sub.${id}`) }))
      body.push(subRow('diag-sub', tabs(ui, 'diag-sub', subs, dTab, id => void openTab($, 'diag', id, { open: false }).catch(() => undefined))))
      if (dTab === 'hints') {
        const sig = signalsOf($, ui, s, { percent: u?.contextPercent, now, tips: tipsNow(u, last, now), loop: await read($, loopAlert), dir: await read($, dirAlert), forBand: false })
        body.push(diagLegend(ui, L, marg), signalsCard(ui, L, marg, sig), ...(await diagCards($, ui, ['tips', 'guard'])))
      } else if (dTab === 'log') {
        body.push(...(await protocolBody($, ui, s, entries, g)))
      } else {
        body.push(diagLegend(ui, L, marg), ...(await diagCards($, ui, [dTab])))
      }
    } else if (tab === 'git') {
      // Ohne Git-Teil zeigt die Karte Arbeitsverzeichnis Branch und Änderungen auch hier.
      if (!s.gitPane) body.push(await workdirCard())
      else body.push(...(await gitPaneBody($, ui, L, m === null ? null : { git: m.git, mr: m.mr, mrMine: m.mrMine, mrDetails: m.mrDetails }, now, marg)))
    } else if (tab === 'ci') {
      // Dev-Mode: Mockdaten statt GitLab.
      const list = m?.ci.list ?? (await read($, ciList))
      const jobs = m?.ci.jobs ?? (await read($, ciJobs))
      const branch = list?.branch ?? ''
      const host = await read($, mrHost)
      body.push(
        ...ciBody({
          ui, L, marg, now, branch, list, jobs, log: await read($, ciLog), busy: await read($, ciBusy), confirm: await read($, ciConfirm), down: await read($, pressed),
          // Vor dem ersten Abruf ist das Remote unbekannt: dann lädt der Reiter, statt „kein GitLab“ zu melden.
          supported: mrRemote === null ? host === '' || list !== null : ciSupported(mrRemote),
          shown: list === null ? null : pickPipeline(list.pipelines, await read($, ciSel), branch),
          ext: link => void openExternal($, link.href).catch(() => undefined),
          onPick: id => void ciPick($, id).catch(() => undefined),
          onReload: () => void refreshCi($, true).catch(() => undefined),
          onPipe: act => void ciPipe($, act, branch).catch(() => undefined),
          onJob: (id, act) => void ciJob($, id, act).catch(() => undefined),
          onLog: id => void ciOpenLog($, id).catch(() => undefined),
          onLogClose: () => void update($, ciLog, () => null).catch(() => undefined),
        }),
      )
    } else if (tab === 'back') {
      // Ohne Zusammenfassung (Schalter aus) bleibt nur der Verlauf.
      const bTab = s.cardCatchUp ? await read($, backTab) : 'history'
      if (s.cardCatchUp) {
        const subs = (['session', 'history'] as const).map(id => ({ id, label: t(L, `back.${id}`) }))
        body.push(subRow('back-sub', tabs(ui, 'back-sub', subs, bTab, id => void openTab($, 'back', id, { open: false }).catch(() => undefined))))
      }
      body.push(bTab === 'session' ? await summaryBody($, ui) : await historyBody($, ui, e.surface))
    } else {
      body.push(await settingsBody($, ui), await aboutCard())
    }

    return (
      <Box flexDirection="column">
        {await paneHead($, ui, L, (
          <Box flexDirection="row" columnGap={2} flexWrap="wrap" alignItems="center" marginTop={1}>
            {tabs(ui, 'pane-tabs', TABS, tab, id => void openTab($, id, undefined, { open: false }).catch(() => undefined))}
            {dev && <Text backgroundColor={TINT.warning} color={THEME.warning} bold> {t(L, 'dev.badge')} </Text>}
          </Box>
        ))}
        {...body}
      </Box>
    )
  })
}
