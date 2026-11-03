export type Limit = { kind: string; percentUsed: number; resetsAt?: string }

export type Usage = {
  contextTokens?: number
  contextWindow: number
  contextPercent?: number
  limits: Limit[]
  usd?: number
}

// MR, für den das Cockpit die Session angepinnt hat; `mine: false`, wenn sie schon vorher angepinnt war

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

// Reiter des Cockpit-Panels (ein Panel statt sechs): Jetzt, Sessions, Verbrauch, Diagnose, Git, CI/CD, Rückblick, Einstellungen
export type PaneTab = 'now' | 'sessions' | 'usage' | 'diag' | 'git' | 'ci' | 'back' | 'set'
// Unterreiter der Diagnose: Hinweise (Tipps, Wächter, Meldungen), Protokoll (Log, Fehler, Freigaben, Zeitreise), Profil, Blast-Radius
export type DiagTab = 'hints' | 'log' | 'profile' | 'blast'
// Unterreiter des Rückblicks: Zusammenfassung dieser Session oder Verlauf der Aufgaben nach Tagen
export type BackTab = 'session' | 'history'
// Filter des Protokolls
export type LogFilter = 'all' | 'errors' | 'gate' | 'snaps'
// Verlauf eines Subagents im Panel: Zeilen aus $.session.messages({ agentId }), `note` erklärt, warum nichts zu lesen ist
export type AgentView = { id: string; title: string; lines: { kind: 'user' | 'assistant' | 'tool' | 'error'; text: string }[]; note: string; at: number }

// `tool`/`target`/`isError`: strukturierter Tool-Eintrag, damit Server und Tool getrennt und farbig gezeichnet werden.
export type LogEntry = { at: number; src: LogSrc; text: string; tool?: string; target?: string; action?: string; isError?: boolean; id?: string }

// `why`: kurze Begründung; fehlt bei Vorschlägen aus älteren Versionen im State.
// `fromClaude`: Claudes eigener Vorschlag (prompt.suggest), im Band mit Chip „CC“ markiert.
export type Step = { label: string; prompt: string; why?: string; fromClaude?: true }

export type BreakdownRow = { name: string; tokens: number; percent: number; kind: 'used' | 'free' | 'buffer' }

// Kontext-Aufteilung wie /context, auf das Nötige für den Tooltip reduziert.
export type Breakdown = { total: number; window: number; percent: number; rows: BreakdownRow[] }

// Git-Stand des Arbeitsverzeichnisses für die Infozeile unter dem Titel.
// Verknüpfte Session der Desktop-App (aus dieser gestartet oder ihr Starter), aus ccd_session_mgmt.list_sessions.
export type LinkedSession = { id: string; title: string; name: string; branch: string; cwd: string; running: boolean; archived: boolean }

// Reiter Sessions: Zeilen aus der Status-Datei der Schaltzentrale (`<Benutzerordner>/.deltra/claude/sessions.json`), nur gelesen.
export type SessionStand = 'läuft' | 'wartet' | 'fertig' | 'archiviert'
export type ApprovalAction = 'commit' | 'push' | 'löschen' | 'archivieren' | 'sonstiges'
// `state` ist leer bei unbekanntem Wert; `stateRaw` hält den Text aus der Datei.
export type SessionRow = { id: string; title: string; repo: string; branch: string; state: SessionStand | ''; stateRaw: string; waitingFor: string; note: string }
export type ApprovalRow = { id: string; action: ApprovalAction; actionRaw: string; detail: string; since: string }
// `issues`: fehlende oder unbekannte Felder, die das Panel als Hinweis zeigt, ohne die Zeilen zu verwerfen.
export type SessionsView = { updated: string; sessions: SessionRow[]; approvals: ApprovalRow[]; issues: string[] }
// Ergebnis des letzten Lesens: Stand oder Fehler (Datei fehlt, leer, kaputtes JSON, falsche Form)
export type SessionsState = { view: SessionsView | null; error: { kind: 'missing' | 'empty' | 'json' | 'shape'; detail: string } | null; sig: string }

// Reiter CI/CD (GitLab, über `glab api`): Pipelines des Projekts und die Jobs der gewählten Pipeline. Status im GitLab-Vokabular
// (created, pending, running, success, failed, canceled, skipped, manual, scheduled …).
// `user`: Login des Auslösers, `userName`: Anzeigename (nur GitLab); `source`: Auslöser im Vokabular der Plattform (push, merge_request_event,
// pull_request, web, workflow_dispatch, schedule …); `tag`: Pipeline eines Tags (nur GitLab); `startedAt`/`finishedAt`: ISO-Zeitpunkte, leer = unbekannt bzw. läuft noch.
export type CiPipeline = { id: number; status: string; ref: string; sha: string; url: string; source: string; createdAt: string; updatedAt: string; startedAt: string; finishedAt: string; durationS: number; user: string; userName: string; tag: boolean; title: string }
// `steps`: nur GitHub Actions (Schritte des Jobs); GitLab-Jobs haben keine.
export type CiStep = { name: string; status: string; durationS: number }
export type CiJob = { id: number; name: string; stage: string; status: string; url: string; durationS: number; allowFailure: boolean; manual: boolean; startedAt: string; finishedAt: string; steps?: CiStep[] }
// Artefakt eines Jobs (GitLab, `job` = Job-Id) oder eines Runs (GitHub, `job` = 0). `url`: Seite im Browser, auf der es sich laden lässt; das Cockpit lädt selbst nichts.
export type CiArtifact = { id: number; job: number; name: string; size: number; expired: boolean; url: string }
// `error`: Grund, warum nichts geladen werden konnte (leer = in Ordnung); die letzten Pipelines bleiben dann stehen.
// `branch`: ausgecheckter Branch beim Abruf (für „neueste Pipeline des Branches“ und „Neue Pipeline“)
export type CiList = { pipelines: CiPipeline[]; error: string; branch: string }
export type CiJobs = { pipeline: CiPipeline; jobs: CiJob[]; artifacts: CiArtifact[] }
// Bitbucket Server (nur lesen): Commits des Branches mit dem Build-Status, den Bamboo/Jenkins an sie melden. `state` im Vokabular von
// Bitbucket (SUCCESSFUL, FAILED, INPROGRESS); `error`: leer in Ordnung, `auth` = kein Zugang im Git-Credential-Store, sonst Klartext.
export type BbBuild = { state: string; key: string; name: string; url: string; description: string; at: string; durationS: number }
export type BbCommit = { sha: string; short: string; title: string; author: string; at: string; builds: BbBuild[] }
export type BbList = { commits: BbCommit[]; error: string; branch: string }
export type CiLog = { job: number; name: string; lines: string[]; truncated: boolean; error: string; loading: boolean }

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

// Merge Request des aktuellen Branches auf GitLab (über glab); `threads`: offene, lösbare Review-Diskussionen.
// Plattform des Remotes; Zustände und Pipeline-Status von GitHub und Bitbucket sind auf GitLabs Vokabular abgebildet.
export type Forge = 'gitlab' | 'github' | 'bitbucket'
export type Mr = {
  forge: Forge
  iid: number
  title: string
  state: string
  draft: boolean
  url: string
  source: string
  target: string
  mergeStatus: string
  conflicts: boolean
  notes: number
  threads: number
  pipeline: { id: number; status: string; url: string } | null
  approvals: { required: number; left: number; by: string[] } | null
  // Für das MR-Pane: Beschreibung, Autor, letzte Änderung, Umfang und die offenen Threads (höchstens 20)
  description: string
  author: string
  updatedAt: string
  changes: string
  threadList: { path: string; line: number | null; author: string; body: string }[]
}
export type MrMine = { ref: string; title: string; url: string; draft: boolean }

// Git-Panel: eine Datei mit Kurzcode (M, A, D, R, C, T), ein Commit, ein lokaler Branch im Abhängigkeitsbaum
export type GitFile = { path: string; code: string }
export type GitCommit = { sha: string; subject: string; author: string; at: number }
// `ahead`/`behind` gegenüber dem Eltern-Branch, nicht gegenüber dem Upstream
export type GitBranch = { name: string; parent: string | null; upstream: string; gone: boolean; ahead: number; behind: number; merged: boolean }
export type GitOp = '' | 'merge' | 'rebase' | 'cherry-pick' | 'revert'
export type GitState = {
  // Wurzel des Repos, auf das sich der Stand bezieht (git rev-parse --show-toplevel)
  top: string
  branch: string
  detached: boolean
  // Upstream des Branches und Abstand dazu
  upstream: string
  ahead: number
  behind: number
  // Commits auf HEAD, die auf keinem Remote liegen (0 = HEAD ist veröffentlicht)
  localOnly: number
  // Upstream-Commits ohne patch-gleiches Gegenstück lokal (0 nach eigenem Rebase/Amend = Force-Push genügt)
  upstreamOnly: number
  // Repo-Hooks (pre-commit, commit-msg, pre-push), die Befehle aus dem Panel nicht ausführen
  hooks: string[]
  staged: GitFile[]
  changed: GitFile[]
  untracked: string[]
  conflicts: string[]
  op: GitOp
  main: string
  remote: string
  branches: GitBranch[]
  // Commits auf dem Upstream, die lokal fehlen, und die letzten eigenen
  incoming: GitCommit[]
  history: GitCommit[]
  stashes: number
  // letzter Fetch (Änderungszeit von FETCH_HEAD), 0 = unbekannt
  fetchedAt: number
  // Git Town konfiguriert (Eltern aus git-town-branch.*.parent)
  town: boolean
  // Branches, die in einem anderen Worktree ausgecheckt sind
  elsewhere: string[]
  readAt: number
}
export type GitError = { cmd: string; out: string; at: number }
export type MrDraft = { title: string; description: string; source: string; target: string }

// Einstellungen: jedes Detail von Band, Panel und Chat lässt sich ein- und ausblenden.
export type ModSettings = {
  rings: boolean
  legend: boolean
  branch: boolean
  limits: boolean
  cache: boolean
  ctxTip: boolean
  steps: boolean
  // Band: Effort/Modell, Kontext mit Kompaktieren, Kosten, Agent-Zahl, Meldungszeile (Titel und Ordner/Branch bleiben)
  bandEffort: boolean
  bandContext: boolean
  bandCost: boolean
  bandAgents: boolean
  bandAlerts: boolean
  animation: boolean
  cardWorkdir: boolean
  cardCatchUp: boolean
  cardOverview: boolean
  cardContext: boolean
  cardTurn: boolean
  cardGate: boolean
  cardAgents: boolean
  cardTasks: boolean
  mrPane: boolean
  // Git-Panel (/cockpit-git) und Haiku-Vorschlag für die Commit-Nachricht
  gitPane: boolean
  gitSuggest: boolean
  paneMargins: boolean
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
  skinMermaid: boolean
  skinMath: boolean
  skinAsk: boolean
  skinMode: boolean
  skinHint: boolean
  dirGuard: boolean
  skinTodos: boolean
  skinMarkIcon: boolean
  skinMarkLabel: boolean
  // Auto-Effort und Sparmodus (Idee: effortless)
  autoEffort: boolean
  saveMode: boolean
  // Atemübung im Band, solange Claude arbeitet (Idee: Mindful-Claude)
  breath: boolean
  mrBand: boolean
  mrNotify: boolean
  // CI/CD-Reiter: Hinweise bei Pipeline-/Job-Wechseln des Branches und Reiter bei Fehlschlag von selbst öffnen
  ciNotify: boolean
  ciAutoOpen: boolean
  breathKey: 'hrv' | 'sigh' | 'box' | '478'
  // Anzahl der Subagent-Kacheln im Hauptpanel (gerade, 2 bis 12)
  agentTiles: number
  // Checkliste (Idee: glanceflow): Band, Plan-Karte, Verlauf
  glance: boolean
  cardPlan: boolean
  glanceHistory: boolean
  // Diagnose (Idee: my-claude-code-mods): Kurzinfos im Band, je ein eigenes Panel
  inspectBand: boolean
  profileBand: boolean
  blastBand: boolean
  // Fehler-Eingang, Nutzungswächter (warnt; hält mit guardHold Subagents an), Tipps und Zeitreise (Snapshots als git-Objekte)
  errorsBand: boolean
  guardBand: boolean
  guardHold: boolean
  tipsBand: boolean
  timelineSnap: boolean
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

// Checkliste im Band (Idee: glanceflow): ein Job je Prompt, Schritte aus Claudes eigener Aufgabenliste.
export type JobStepStatus = 'upcoming' | 'active' | 'done'
// `ref`: Nummer aus TaskCreate, damit TaskUpdate den Schritt findet; `files`: geänderte Dateien (nur Namen)
export type JobStep = { name: string; status: JobStepStatus; ref?: string; startedAt?: number; doneAt?: number; files: string[] }
// `stuck` ist nicht gespeichert, sondern aus `signAt` abgeleitet (glance.ts, isStuck)
export type JobPhase = 'working' | 'needsYou' | 'paused' | 'stopped' | 'done'
export type ActKind = 'read' | 'search' | 'edit' | 'shell' | 'web' | 'agent' | 'plan' | 'ask' | 'other'
// `ask`: Frage aus der Desktop-Abschlussbox (wrap_up), hat Vorrang vor der Frage aus der Antwort; `signAt`: letztes Lebenszeichen
export type Job = {
  id: string
  title: string
  cwd: string
  startedAt: number
  endedAt?: number
  phase: JobPhase
  question: string
  ask: string
  steps: JobStep[]
  act: { kind: ActKind; n: number } | null
  signAt: number
  tools: number
  files: string[]
  // Slash-Befehl (`/name args`): sichtbar auch ohne Tools; geht ein Turn daraus hervor, gehört er zu diesem Job
  cmd?: boolean
  // getippter Prompt (gekappt, glance.ts PROMPT_MAX) für den Kopier-Knopf im Verlauf; fehlt bei Nachrichten von Agents/Sessions
  prompt?: string
}
// Verlaufseintrag im $.store unter `glance:<JJJJ-MM-TT>` (30 Tage, über alle Projekte)
export type JobRecord = {
  id: string
  title: string
  prompt?: string
  cwd: string
  startedAt: number
  endedAt: number
  outcome: 'done' | 'needsYou' | 'stopped'
  question: string
  tools: number
  steps: { name: string; done: boolean; ms?: number }[]
  files: string[]
}

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
      // Fragen aus Claudes letzter Antwort, als Aufträge an Claude umformuliert (zweite Zeile unter den Folge-Schritten)
      asks: Step[]
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
      linked: LinkedSession[]
      lastAnswer: string
      pressed: string
      stepsBusy: boolean
      // tool_use_ids mit aufgeklappten Details (Chat-Skin)
      expanded: string[]
      // in diesem Turn geänderte Dateien (Hervorhebungen)
      touched: string[]
      // tool_use_ids lesender Shell-Befehle (ruhige Ergebnisse)
      quietIds: string[]
      // Einstellungsschlüssel der im Hauptpanel zugeklappten Karten (Kopie des Stands im $.store, gilt je User)
      collapsed: string[]
      marks: CallMark[]
      loopAlert: LoopAlert | null
      dirs: { cwd: string; root: string; home: string }
      dirAlert: { dir: string; tool: string } | null
      mcpTools: McpSchema[]
      // Serverteile (mcpKey) der in dieser Session aufgerufenen MCP-Tools
      usedMcp: string[]
      turnLog: TurnRecord[]
      // agentIds gebremster Subagents
      braked: string[]
      // Auto-Effort: Haikus Stufe für den laufenden Prompt; Effort der Session, wie zuletzt gesehen (Wechsel = manuell)
      effortPick: 'low' | 'medium' | 'high' | null
      effortBase: string | number | null
      // Ergebnis des letzten Handoffs (Pfad oder Fehler), leer ohne
      handoffNote: string
      // Auto-Effort-Status im Band: prüft / Haiku: Stufe / behalten
      effortNote: string
      // Merge Requests (GitLab): Host, MR des Branches, eigene offene MRs, Commits ohne MR, Entwurf, laufende Aktion, Rückfrage
      mrHost: string
      mrCur: Mr | null
      mrMine: MrMine[]
      // Status der eigenen MRs im selben Repo (Schlüssel = Referenz aus mrMine), nur bei offenem MR-Panel nachgeladen
      mrDetails: Record<string, Mr>
      // offene MRs, in denen du als Reviewer eingetragen bist
      mrReview: MrMine[]
      mrAhead: number
      mrDraft: MrDraft | null
      mrBusy: string
      mrConfirm: string
      // MRs mit angehaktem „Squash“ (`merge-<iid>`), Standard aus
      mrSquash: string[]
      // ignorierte Schleifen-Warnungen (`kind:target`), gelten bis zum Ende der Session
      loopIgnored: string[]
      // Wechsel der Auto-Effort-Stufe für die kurze Animation im Band; `step` zählt die Bilder hoch
      effortShift: { from: string | null; to: string; step: number } | null
      // Dev-Mode: Panel zeigt Mockdaten statt der Session-Werte (nur Session)
      devMock: boolean
      // Checkliste: aktueller Job dieser Session, angezeigter Verlaufstag (JJJJ-MM-TT) und dessen Einträge, mittlere Schrittdauer
      job: Job | null
      historyDay: string
      historyList: JobRecord[]
      stepAvg: number | null
      // Git-Panel: gelesener Stand, laufende Aktion (Beschriftung), Rückfrage (Schlüssel), letzter Fehler,
      // Commit-Nachricht (Vorschlag oder getippt) mit Fingerabdruck des gestagten Stands, Eingabe für neue Branches, Schalter
      gitState: GitState | null
      // warum kein Stand da ist: '' (lädt oder da), 'noRepo', 'risky'
      gitNote: string
      gitBusy: string
      gitConfirm: string
      gitError: GitError | null
      gitMsg: string
      gitMsgFor: string
      gitNewBranch: string
      gitRebase: boolean
      gitAmend: boolean
      // im Branch-Baum ausgewählter Branch (Optionen aufgeklappt), leer ohne
      gitSel: string
      // Ein Panel mit Reitern: gewählter Reiter, Unterreiter von Diagnose und Rückblick, Filter des Protokolls
      paneTab: PaneTab
      // CI/CD: Pipeline-Liste, Jobs der gewählten Pipeline, gewählte Pipeline (0 = automatisch), geöffnetes Job-Log, laufende Aktion, Rückfrage
      ciList: CiList | null
      ciJobs: CiJobs | null
      ciSel: number
      ciLog: CiLog | null
      bbList: BbList | null
      bbSel: string
      ciBusy: string
      ciConfirm: string
      sessionsFile: SessionsState | null
      diagTab: DiagTab
      backTab: BackTab
      logFilter: LogFilter
      agentView: AgentView | null
    }
  }
}
