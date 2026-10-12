export type Limit = { kind: string; percentUsed: number; resetsAt?: string }

export type Usage = {
  contextTokens?: number
  contextWindow: number
  contextPercent?: number
  limits: Limit[]
  usd?: number
}

export type Turn = {
  startedAt: number
  durationMs?: number
  tools: number
  edits: number
  errors: number
  usdStart?: number
  usdDelta?: number
}

// Subagent mit den Angaben aus agent.spawn und dem Ende seines Turns; was die Engine nicht liefert (z. B. Effort), fehlt.
// Hintergrundaufgabe (Shell mit run_in_background oder per Strg+B verschoben): Start aus tool.call, Ende aus der Task-Benachrichtigung.
export type BgTask = { id: string; description: string; command?: string; status: string; since: number; durationMs?: number; summary?: string }

export type Agent = {
  id: string
  type: string
  description: string
  status: string
  since: number
  model?: string
  requested?: string
  background?: boolean
  fork?: boolean
  teammate?: boolean
  permission?: string
  tokensIn?: number
  tokensOut?: number
  cacheHit?: number | null
  durationMs?: number
  steps?: number
}

// `you`: eine Rückfrage, die du (oder der Modus) freigegeben hast; `ask`: noch offen.
export type Decision = 'allow' | 'you' | 'ask' | 'deny'

// Zähler je Entscheidung und Werkzeugart; `trail` hält die letzten Entscheidungen für die Kachelzeile.
export type Gate = { allow: number; you: number; ask: number; deny: number; file: number; shell: number; other: number; trail: Decision[]; asked: string[] }

export type LogSrc = 'you' | 'main' | 'gate' | 'agent'

// `tool`/`target`/`isError`: strukturierter Tool-Eintrag, damit Server und Tool getrennt und farbig gezeichnet werden.
export type LogEntry = { at: number; src: LogSrc; text: string; tool?: string; target?: string; isError?: boolean }

// `why`: kurze Begründung; fehlt bei Vorschlägen aus älteren Versionen im State.
// `fromClaude`: Claudes eigener Vorschlag (prompt.suggest), im Band mit Chip „CC“ markiert.
export type Step = { label: string; prompt: string; why?: string; fromClaude?: true }

export type BreakdownRow = { name: string; tokens: number; percent: number; kind: 'used' | 'free' | 'buffer' }

// Kontext-Aufteilung wie /context, auf das Nötige für den Tooltip reduziert.
export type Breakdown = { total: number; window: number; percent: number; rows: BreakdownRow[] }

// Git-Stand des Arbeitsverzeichnisses für die Infozeile unter dem Titel.
export type Repo = {
  name: string
  isLinked: boolean
  branch: string
  sha: string
  // null: nicht geprüft, weil die Repo-Konfiguration Befehle ausführen könnte (siehe loadRepo)
  dirty: number | null
  ahead: number | null
  behind: number | null
  path: string
  subdir: string
}

export type UiLang = 'de' | 'en'

// Einstellungen: jedes Detail von Band, Panel und Chat lässt sich ein- und ausblenden.
export type ModSettings = {
  rings: boolean
  legend: boolean
  branch: boolean
  limits: boolean
  cache: boolean
  ctxTip: boolean
  steps: boolean
  animation: boolean
  cardWorkdir: boolean
  cardCatchUp: boolean
  cardOverview: boolean
  cardContext: boolean
  cardTurn: boolean
  cardGate: boolean
  cardAgents: boolean
  cardTasks: boolean
  cardLog: boolean
  chatInfo: boolean
  chatLines: boolean
  catchUp: boolean
  // Chat-Skins: Tool-Symbole, ruhige Ergebnisse, Diff-Karte, Details aufklappen, Prompt-Rahmen, Code-/Tabellenkarten, Hervorhebungen
  skinIcons: boolean
  skinQuiet: boolean
  skinDiff: boolean
  skinExpand: boolean
  skinPrompt: boolean
  skinCards: boolean
  skinInline: boolean
  skinLinks: boolean
  skinMermaid: boolean
  skinMath: boolean
  skinAsk: boolean
  skinMode: boolean
  skinHint: boolean
  skinMarkIcon: boolean
  skinMarkLabel: boolean
  lang: UiLang
}

// Fortlaufende Zusammenfassung der Session (Idee: catch-me-up).
export type CatchUp = { sessionId: string; summary: string; covered: number; marker: string; updatedAt: number; status: string }

export type About = { version: string; builtAt: number }

// Aktuelles Modell der Haupt-Session: aus turn.step (Modell, Effort), prompt.compose (Ausgabestil) und session.model.
// `window`: Kontextfenster aus session.measure, hier statt aus `usage`, damit die Panel-Köpfe nicht bei jeder Messung neu zeichnen.
export type ModelInfo = { model: string; effort?: string | number; style?: string | null; cc?: string; window?: number }

// Ein Aufruf des Haupt-Agents im laufenden Turn (Schleifen-Erkennung) und die erkannte Schleife.
export type CallMark = { key: string; isError: boolean; isEdit: boolean }
export type LoopAlert = { kind: 'error' | 'edit'; target: string; n: number }
// Ein geladenes MCP-Schema aus dem Kontext-Breakdown (Kontext-Diät).
export type McpSchema = { name: string; serverName: string; tokens: number; isLoaded: boolean }
// Ein abgeschlossener Turn für die Turn-Bilanz.
export type TurnRecord = { usd?: number; tools: number; edits: number; durationMs: number }

declare module 'claude-code' {
  interface PluginState {
    'tenvan-cc': {
      usage: Usage | null
      lastTurnAt: number | null
      turn: Turn | null
      agents: Agent[]
      gate: Gate
      log: LogEntry[]
      tick: number
      steps: Step[]
      bgTasks: BgTask[]
      claudeHint: Step | null
      cacheHit: number | null
      about: About | null
      lastPrompt: string
      frame: number
      breakdown: Breakdown | null
      breakdownNote: string
      logo: string | null
      modelInfo: ModelInfo | null
      settings: ModSettings
      catchUp: CatchUp
      repo: Repo | null
      // tool_use_ids mit aufgeklappten Details (Chat-Skin)
      expanded: string[]
      // in diesem Turn geänderte Dateien (Hervorhebungen)
      touched: string[]
      // tool_use_ids lesender Shell-Befehle (ruhige Ergebnisse)
      quietIds: string[]
      // Einstellungsschlüssel der im Hauptpanel zugeklappten Karten (nur Session, nicht im Store)
      collapsed: string[]
      marks: CallMark[]
      loopAlert: LoopAlert | null
      mcpTools: McpSchema[]
      // Serverteile (mcpKey) der in dieser Session aufgerufenen MCP-Tools
      usedMcp: string[]
      turnLog: TurnRecord[]
      // agentIds gebremster Subagents
      braked: string[]
    }
  }
}
