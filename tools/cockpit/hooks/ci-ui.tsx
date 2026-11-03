// Reiter CI/CD: zeichnet die Pipelines des Projekts, die Jobs der gewählten Pipeline nach Stages und das Job-Log. Ohne `$`: Lesen
// und Aktionen kommen als Callbacks aus register.tsx (Atome, forge.ts), die Logik aus ci.ts.
import type { PressedLink, RenderElement } from 'claude-code'

import type { BbList, CiJobs, CiList, CiLog, CiPipeline, Forge, UiLang } from '../types'
import { bbRollup, bbStatus } from './ci-bitbucket'
import { ago, byStage, ciCaps, ciTone, clock, jobCan, liveSeconds, pipelineCan, sizeText, sourceKey, span, stamp } from './ci'
import { CARD_TINT, THEME, TINT } from './core'
import { t } from './i18n'
import type { Key } from './i18n'
import { badge, link, tile } from './ui'
import type { Ui } from './ui'

export type CiJobAct = 'retry' | 'cancel' | 'play'
export type CiPipeAct = 'retry' | 'cancel' | 'new'

// Text je Plattform: GitHub sagt „Workflow-Runs“ statt „Pipelines“ (Schlüssel mit Endung `.gh`), alles andere gleich.
export const ciT = (L: UiLang, key: string, forge: Forge, vars?: Record<string, string | number>): string => t(L, (forge === 'github' ? `${key}.gh` : key) as Key, vars)

export type CiProps = {
  ui: Ui
  L: UiLang
  marg: number
  now: number
  // false: das Remote ist weder GitLab noch GitHub (Hinweiskarte statt Listen)
  supported: boolean
  forge: Forge
  // Backend, gegen das der Reiter läuft (Kopfzeile); null, solange das Remote unbekannt ist
  backend: { kind: Forge; host: string; project: string } | null
  // Bitbucket: Commits des Branches mit Build-Status (nur lesen); null vor dem ersten Abruf
  bb: BbList | null
  bbSel: string
  onBbPick: (sha: string) => void
  branch: string
  list: CiList | null
  jobs: CiJobs | null
  shown: CiPipeline | null
  log: CiLog | null
  busy: string
  confirm: string
  down: string
  ext: (link: PressedLink) => void
  onPick: (id: number) => void
  onReload: () => void
  onPipe: (act: CiPipeAct) => void
  onJob: (id: number, act: CiJobAct) => void
  onLog: (id: number) => void
  onLogClose: () => void
}

// Zeichen vor einem Schritt: ✓ grün, ✗ rot, ◌ läuft oder wartet, – übersprungen.
const stepMark = (status: string): string => (status === 'success' ? '✓' : status === 'failed' ? '✗' : status === 'skipped' || status === 'canceled' ? '–' : '◌')

const W = { id: 9, status: 11, age: 8, dur: 8, who: 14, span: 24 } as const

const FORGE_NAME: Record<Forge, string> = { gitlab: 'GitLab', github: 'GitHub', bitbucket: 'Bitbucket' }

// Bitbucket Server: Commits des Branches mit dem Build-Status, den Bamboo/Jenkins gemeldet haben (nur lesen). Kein Wiederholen, kein Log,
// keine Artefakte: das gibt die Build-Status-API nicht her; wer ein Build ansehen will, folgt dem Link ins Build-System.
function bbCards(p: CiProps): RenderElement[] {
  const { ui, L, marg, now, bb } = p
  const { Box, Text, Markdown } = ui
  const out: RenderElement[] = []
  const commits = bb?.commits ?? []
  const cell = (key: string, width: number, child: RenderElement | string, dim = false) => (
    <Box key={key} width={width} flexShrink={0}>{typeof child === 'string' ? <Text dimColor={dim} wrap="truncate-end">{child}</Text> : child}</Box>
  )
  const err = bb?.error ?? ''
  out.push(
    tile(
      ui,
      'pane',
      { key: 'card-bb-list', tint: err !== '' ? TINT.error : CARD_TINT.cardTasks ?? TINT.neutral, marg, grow: true },
      <Box key="bb-top" flexDirection="row" columnGap={2} flexWrap="wrap" alignItems="center">
        <Text bold>{t(L, 'ci.bb.commits', { branch: bb?.branch === undefined || bb.branch === '' ? '–' : bb.branch })}</Text>
        <Text dimColor>{commits.length}</Text>
        {link(ui, 'ci-reload', t(L, 'ci.refresh'), p.onReload, { tone: 'muted' })}
      </Box>,
      bb === null && <Text key="bb-load" dimColor>{t(L, 'ci.bb.loading')}</Text>,
      err !== '' && (
        <Box key="bb-err" flexDirection="column" marginTop={1}>
          <Text color={THEME.error} bold>{err === 'auth' ? t(L, 'ci.bb.auth', { host: p.backend?.host ?? '' }) : t(L, 'ci.bb.error', { why: err })}</Text>
          {commits.length > 0 && <Text dimColor>{t(L, 'ci.errorKeep')}</Text>}
        </Box>
      ),
      bb !== null && err === '' && commits.length === 0 && <Text key="bb-none" dimColor>{t(L, 'ci.bb.none')}</Text>,
      commits.length > 0 && (
        <Box key="bb-grid" flexDirection="column" marginTop={1}>
          {commits.map(c => {
            const sel = (p.bbSel === '' ? commits[0]?.sha : p.bbSel) === c.sha
            const roll = bbRollup(c.builds)
            return (
              <Box key={`bb-row-${c.sha}`} flexDirection="row" columnGap={1} alignItems="center">
                {cell(`bb-row-${c.sha}-sha`, W.id, link(ui, `bb-pick-${c.sha}`, `${sel ? '▸ ' : ''}${c.short}`, () => p.onBbPick(c.sha), { tone: sel ? 'strong' : 'link' }))}
                {cell(`bb-row-${c.sha}-st`, W.status, <Box>{roll === '' ? <Text dimColor>{t(L, 'ci.bb.noBuild')}</Text> : badge(ui, `bb-row-${c.sha}-b`, roll, ciTone(roll))}</Box>)}
                <Box key={`bb-row-${c.sha}-title`} flexGrow={1} flexShrink={1}><Text wrap="truncate-end">{c.title}</Text></Box>
                {cell(`bb-row-${c.sha}-who`, W.who, c.author, true)}
                {cell(`bb-row-${c.sha}-age`, W.age, ago(c.at, now), true)}
              </Box>
            )
          })}
        </Box>
      ),
    ),
  )
  const cur = commits.find(c => c.sha === p.bbSel) ?? commits[0]
  if (cur === undefined) return out
  out.push(
    tile(
      ui,
      'pane',
      {
        key: 'card-bb-commit',
        tint: CARD_TINT.cardPlan ?? TINT.neutral,
        marg,
        grow: true,
        head: (
          <Box flexDirection="row" columnGap={2} alignItems="center" flexWrap="wrap">
            <Text bold>{t(L, 'ci.bb.commit', { sha: cur.short })}</Text>
            {cur.builds.length > 0 && badge(ui, 'bb-sel-b', bbRollup(cur.builds), ciTone(bbRollup(cur.builds)))}
          </Box>
        ),
      },
      cur.title !== '' && <Text key="bb-title" wrap="truncate-end">{cur.title}</Text>,
      <Text key="bb-meta" dimColor wrap="truncate-end">{t(L, 'ci.bb.meta', { when: stamp(cur.at, L), ago: ago(cur.at, now), author: cur.author === '' ? '–' : cur.author })}</Text>,
      cur.builds.length === 0 ? (
        <Text key="bb-nob" dimColor>{t(L, 'ci.bb.noBuilds')}</Text>
      ) : (
        <Box key="bb-builds" flexDirection="column" marginTop={1}>
          {cur.builds.map((b, i) => (
            <Box key={`bb-b-${i}`} flexDirection="column">
              <Box flexDirection="row" columnGap={1} alignItems="center" flexWrap="wrap">
                {cell(`bb-b-${i}-st`, W.status, <Box>{badge(ui, `bb-b-${i}-badge`, bbStatus(b.state), ciTone(bbStatus(b.state)))}</Box>)}
                <Box key={`bb-b-${i}-name`} flexGrow={1} flexShrink={1}><Text wrap="truncate-end">{b.name}{b.key !== '' && b.key !== b.name ? <Text dimColor> ({b.key})</Text> : null}</Text></Box>
                {cell(`bb-b-${i}-at`, W.span, b.at === '' ? '' : stamp(b.at, L), true)}
                {cell(`bb-b-${i}-dur`, W.dur, b.durationS > 0 ? clock(b.durationS) : '', true)}
                {b.url !== '' && <Markdown key={`bb-b-${i}-url`} onLinkPress={p.ext} text={`[${t(L, 'ci.bb.open')}](${b.url})`} />}
              </Box>
              {b.description !== '' && <Box key={`bb-b-${i}-desc`} paddingLeft={2}><Text dimColor wrap="truncate-end">{b.description}</Text></Box>}
            </Box>
          ))}
        </Box>
      ),
      <Text key="bb-note" dimColor>{t(L, 'ci.bb.note')}</Text>,
    ),
  )
  return out
}

export function ciBody(p: CiProps): RenderElement[] {
  const { ui, L, marg, now, list, jobs, shown, log, busy, confirm, down, forge } = p
  const caps = ciCaps(forge)
  const { Box, Text, Markdown } = ui
  const out: RenderElement[] = []
  // Aktion als Link; läuft eine andere Aktion (busy) oder gerade dieser Klick, ist er gesperrt.
  const act = (key: string, label: string, run: () => void, tone: 'link' | 'muted' | 'warn' | 'bad' | 'ok' = 'link') =>
    link(ui, key, label, run, { busy: down === key ? true : busy !== '' ? 'locked' : false, tone })
  const cell = (key: string, width: number, child: RenderElement | string, dim = false) => (
    <Box key={key} width={width} flexShrink={0}>{typeof child === 'string' ? <Text dimColor={dim} wrap="truncate-end">{child}</Text> : child}</Box>
  )

  // Backend-Zeile: Plattform als Badge, dahinter Host und Repo/Projekt.
  const backendRow = p.backend === null ? null : (
    <Box key="ci-backend" flexDirection="row" columnGap={1} alignItems="center" flexWrap="wrap">
      {badge(ui, 'ci-backend-b', FORGE_NAME[p.backend.kind], 'neutral')}
      <Text dimColor wrap="truncate-end">{`${p.backend.host}/${p.backend.project}`}</Text>
    </Box>
  )

  if (!p.supported) {
    // Bitbucket Server hat kein eigenes CI (Builds melden Bamboo/Jenkins nur als Status an Commits): Hinweis statt Listen.
    const isBb = p.backend?.kind === 'bitbucket'
    out.push(
      tile(ui, 'pane', { key: 'card-ci-none', tint: TINT.neutral, marg, grow: true, head: <Text bold>{t(L, isBb ? 'ci.bb.head' : 'ci.head')}</Text> }, backendRow, <Text key="ci-nogl" dimColor>{t(L, isBb ? 'ci.noBitbucket' : 'ci.noGitlab')}</Text>),
    )
    if (isBb) out.push(...bbCards(p))
    return out
  }

  // Kopfkarte: Titel, Aktualisieren, neue Pipeline für den Branch (Rückfrage), Fehler.
  const newAsk = confirm === 'ci-new'
  const pipes = list?.pipelines ?? []
  out.push(
    tile(
      ui,
      'pane',
      { key: 'card-ci-head', tint: list?.error ? TINT.error : CARD_TINT.cardWorkdir ?? TINT.neutral, marg, grow: true },
      <Box key="ci-top" flexDirection="row" columnGap={2} flexWrap="wrap" alignItems="center">
        <Text bold>{ciT(L, 'ci.head', forge)}</Text>
        <Text dimColor>{pipes.length}</Text>
        {act('ci-reload', t(L, 'ci.refresh'), p.onReload, 'muted')}
        {caps.newRun && p.branch !== '' && act('ci-new', newAsk ? t(L, 'ci.newAsk', { ref: p.branch }) : t(L, 'ci.new'), () => p.onPipe('new'), newAsk ? 'warn' : 'link')}
      </Box>,
      backendRow,
      busy !== '' && <Text key="ci-busy" color={THEME.accent}>◌ {busy}</Text>,
      list === null && <Text key="ci-load" dimColor>{ciT(L, 'ci.loading', forge)}</Text>,
      list?.error ? (
        <Box key="ci-err" flexDirection="column" marginTop={1}>
          <Text color={THEME.error} bold>{ciT(L, 'ci.error', forge, { why: list.error })}</Text>
          {pipes.length > 0 && <Text dimColor>{t(L, 'ci.errorKeep')}</Text>}
        </Box>
      ) : null,
    ),
  )
  if (list === null) return out

  // Pipeline-Liste: neueste zuerst, die gewählte mit ▸, Pipelines des aktuellen Branches fett.
  out.push(
    tile(
      ui,
      'pane',
      { key: 'card-ci-list', tint: CARD_TINT.cardTasks ?? TINT.neutral, marg, grow: true },
      pipes.length === 0 ? (
        <Text key="ci-none" dimColor>{list.error ? '–' : ciT(L, 'ci.none', forge)}</Text>
      ) : (
        <Box key="ci-grid" flexDirection="column">
          {pipes.map(x => {
            const sel = shown?.id === x.id
            return (
              <Box key={`ci-row-${x.id}`} flexDirection="row" columnGap={1} alignItems="center">
                {cell(`ci-row-${x.id}-id`, W.id, link(ui, `ci-pick-${x.id}`, `${sel ? '▸ ' : ''}#${x.id}`, () => p.onPick(x.id), { tone: sel ? 'strong' : 'link' }))}
                {cell(`ci-row-${x.id}-st`, W.status, <Box>{badge(ui, `ci-row-${x.id}-b`, x.status, ciTone(x.status))}</Box>)}
                <Box key={`ci-row-${x.id}-ref`} flexGrow={1} flexShrink={1}><Text bold={x.ref === p.branch} wrap="truncate-end">{x.ref} <Text dimColor>{x.sha}</Text></Text></Box>
                {cell(`ci-row-${x.id}-who`, W.who, x.user === '' ? '' : `@${x.user}`, true)}
                {cell(`ci-row-${x.id}-age`, W.age, ago(x.createdAt, now), true)}
                {cell(`ci-row-${x.id}-dur`, W.dur, clock(liveSeconds(x.status, x.startedAt || x.createdAt, x.durationS, now)), true)}
              </Box>
            )
          })}
        </Box>
      ),
    ),
  )
  if (shown === null) return out

  // Gewählte Pipeline: Kopf mit Meta und Aktionen, darunter die Jobs je Stage.
  const cur = jobs !== null && jobs.pipeline.id === shown.id ? jobs.pipeline : shown
  const can = pipelineCan(cur)
  const cancelAsk = confirm === `ci-cancel-${cur.id}`
  const dur = clock(liveSeconds(cur.status, cur.startedAt || cur.createdAt, cur.durationS, now))
  // Auslöser und Ersteller wie im Browser: „Push · Ralf Stich (@ralf)“; Tag-Pipelines heißen „Tag“.
  const srcKey = sourceKey(cur.source, cur.tag)
  const trigger = srcKey === '' ? cur.source : t(L, `ci.src.${srcKey}` as Key)
  const creator = cur.user === '' ? '' : cur.userName !== '' ? `${cur.userName} (@${cur.user})` : `@${cur.user}`
  const artifacts = jobs !== null && jobs.pipeline.id === shown.id ? jobs.artifacts : []
  const jobName = (id: number): string => jobs?.jobs.find(j => j.id === id)?.name ?? ''
  out.push(
    tile(
      ui,
      'pane',
      {
        key: 'card-ci-pipe',
        tint: CARD_TINT.cardPlan ?? TINT.neutral,
        marg,
        grow: true,
        head: (
          <Box flexDirection="row" columnGap={2} alignItems="center" flexWrap="wrap">
            <Text bold>{ciT(L, 'ci.sel', forge, { id: cur.id })}</Text>
            {badge(ui, 'ci-sel-b', cur.status, ciTone(cur.status))}
          </Box>
        ),
      },
      cur.title !== '' && <Text key="ci-title" wrap="truncate-end">{cur.title}</Text>,
      <Text key="ci-meta" dimColor wrap="truncate-end">{t(L, 'ci.meta', { ref: cur.ref, sha: cur.sha, when: ago(cur.createdAt, now), dur, user: cur.user === '' ? '' : ` · @${cur.user}` })}</Text>,
      <Text key="ci-meta2" dimColor wrap="truncate-end">{t(L, 'ci.meta2', { span: span(cur.startedAt || cur.createdAt, cur.finishedAt, L), trigger: trigger === '' ? '–' : trigger, creator: creator === '' ? '–' : creator })}</Text>,
      <Box key="ci-pact" flexDirection="row" columnGap={2} flexWrap="wrap" marginTop={1}>
        {can.retry && act('ci-p-retry', ciT(L, 'ci.retry', forge), () => p.onPipe('retry'))}
        {can.cancel && act('ci-p-cancel', cancelAsk ? t(L, 'ci.cancelAsk') : ciT(L, 'ci.cancel', forge), () => p.onPipe('cancel'), cancelAsk ? 'bad' : 'link')}
        {cur.url !== '' && <Markdown key="ci-p-url" onLinkPress={p.ext} text={`[${ciT(L, 'ci.open', forge)}](${cur.url})`} />}
      </Box>,
      jobs === null || jobs.pipeline.id !== shown.id ? (
        <Text key="ci-jl" dimColor>{jobs === null ? t(L, 'ci.loading') : t(L, 'ci.jobsNone')}</Text>
      ) : jobs.jobs.length === 0 ? (
        <Text key="ci-jn" dimColor>{t(L, 'ci.jobsNone')}</Text>
      ) : (
        <Box key="ci-jobs" flexDirection="column" marginTop={1}>
          {byStage(jobs.jobs).map(g => (
            <Box key={`ci-st-${g.stage}`} flexDirection="column" marginTop={1}>
              <Text bold dimColor>{g.stage}</Text>
              {g.jobs.map(j => {
                const c = jobCan(j)
                const cancelKey = `ci-j-cancel-${j.id}`
                // GitHub: die Schritte der Jobs, die laufen oder fehlgeschlagen sind (die grünen bleiben zugeklappt).
                const steps = j.status === 'failed' || j.status === 'running' ? (j.steps ?? []) : []
                return (
                  <Box key={`ci-job-${j.id}`} flexDirection="column">
                  <Box key={`ci-job-${j.id}-row`} flexDirection="row" columnGap={1} alignItems="center" flexWrap="wrap">
                    {cell(`ci-job-${j.id}-st`, W.status, <Box>{badge(ui, `ci-job-${j.id}-b`, j.status, ciTone(j.status))}</Box>)}
                    <Box key={`ci-job-${j.id}-name`} flexGrow={1} flexShrink={1}>
                      <Text wrap="truncate-end">{j.name}{j.allowFailure ? <Text dimColor> ({t(L, 'ci.allowFailure')})</Text> : null}</Text>
                    </Box>
                    {cell(`ci-job-${j.id}-span`, W.span, j.startedAt === '' ? '' : span(j.startedAt, j.finishedAt, L), true)}
                    {cell(`ci-job-${j.id}-dur`, W.dur, clock(liveSeconds(j.status, j.startedAt, j.durationS, now)), true)}
                    {c.log && act(`ci-j-log-${j.id}`, t(L, 'ci.job.log'), () => p.onLog(j.id), 'muted')}
                    {caps.play && c.play && act(`ci-j-play-${j.id}`, t(L, 'ci.job.play'), () => p.onJob(j.id, 'play'), 'ok')}
                    {c.retry && act(`ci-j-retry-${j.id}`, t(L, 'ci.job.retry'), () => p.onJob(j.id, 'retry'))}
                    {caps.jobCancel && c.cancel && act(cancelKey, t(L, 'ci.job.cancel'), () => p.onJob(j.id, 'cancel'), 'muted')}
                  </Box>
                  {steps.map((st, i) => (
                    <Box key={`ci-job-${j.id}-step-${i}`} flexDirection="row" columnGap={1} paddingLeft={2}>
                      <Text color={st.status === 'failed' ? THEME.error : undefined} dimColor={st.status !== 'failed'} wrap="truncate-end">{`${stepMark(st.status)} ${st.name}`}</Text>
                      <Text dimColor>{clock(st.durationS)}</Text>
                    </Box>
                  ))}
                  </Box>
                )
              })}
            </Box>
          ))}
        </Box>
      ),
      // Artefakte: Name, Größe, Job; geladen wird im Browser (der Link öffnet die Seite des Jobs bzw. Runs), das Cockpit lädt nichts.
      artifacts.length > 0 && (
        <Box key="ci-arts" flexDirection="column" marginTop={1}>
          <Text bold dimColor>{t(L, 'ci.art.head', { n: artifacts.length })}</Text>
          {artifacts.map(a => (
            <Box key={`ci-art-${a.job}-${a.id}`} flexDirection="row" columnGap={2} flexWrap="wrap">
              <Text wrap="truncate-end">{a.name} <Text dimColor>{sizeText(a.size, L)}{a.job > 0 && jobName(a.job) !== '' ? ` · ${jobName(a.job)}` : ''}</Text></Text>
              {a.expired ? <Text key={`ci-art-${a.job}-${a.id}-exp`} dimColor>{t(L, 'ci.art.expired')}</Text> : a.url !== '' && <Markdown key={`ci-art-${a.job}-${a.id}-url`} onLinkPress={p.ext} text={`[${t(L, 'ci.art.browser')}](${a.url})`} />}
            </Box>
          ))}
          <Text key="ci-art-note" dimColor>{t(L, 'ci.art.note')}</Text>
        </Box>
      ),
    ),
  )

  // Job-Log: die letzten Zeilen, aufbereitet; ✕ schließt.
  if (log !== null) {
    out.push(
      tile(
        ui,
        'pane',
        {
          key: 'card-ci-log',
          tint: TINT.neutral,
          marg,
          grow: true,
          head: (
            <Box flexDirection="row" columnGap={2} alignItems="center" flexWrap="wrap">
              <Text bold>{t(L, 'ci.log.head', { name: log.name })}</Text>
              {log.truncated && <Text dimColor>{t(L, 'ci.log.tail', { n: log.lines.length })}</Text>}
              {link(ui, 'ci-log-close', t(L, 'ci.log.close'), p.onLogClose, { tone: 'muted' })}
            </Box>
          ),
        },
        log.loading ? (
          <Text key="ci-log-load" dimColor>{t(L, 'ci.log.loading')}</Text>
        ) : log.error !== '' ? (
          <Text key="ci-log-err" color={THEME.error}>{t(L, 'ci.log.failed', { why: log.error })}</Text>
        ) : log.lines.length === 0 ? (
          <Text key="ci-log-empty" dimColor>{t(L, 'ci.log.empty')}</Text>
        ) : (
          <Box key="ci-log-lines" flexDirection="column">
            {log.lines.map((l, i) => <Text key={`ci-log-${i}`} dimColor wrap="truncate-end">{l === '' ? ' ' : l}</Text>)}
          </Box>
        ),
      ),
    )
  }
  return out
}
