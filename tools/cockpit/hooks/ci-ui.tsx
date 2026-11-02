// Reiter CI/CD: zeichnet die Pipelines des Projekts, die Jobs der gewählten Pipeline nach Stages und das Job-Log. Ohne `$`: Lesen
// und Aktionen kommen als Callbacks aus register.tsx (Atome, forge.ts), die Logik aus ci.ts.
import type { PressedLink, RenderElement } from 'claude-code'

import type { CiJobs, CiList, CiLog, CiPipeline, UiLang } from '../types'
import { ago, byStage, ciTone, clock, jobCan, pipelineCan } from './ci'
import { CARD_TINT, THEME, TINT } from './core'
import { t } from './i18n'
import { badge, link, tile } from './ui'
import type { Ui } from './ui'

export type CiJobAct = 'retry' | 'cancel' | 'play'
export type CiPipeAct = 'retry' | 'cancel' | 'new'

export type CiProps = {
  ui: Ui
  L: UiLang
  marg: number
  now: number
  // false: das Remote ist kein GitLab (Hinweiskarte statt Listen)
  supported: boolean
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

const W = { id: 9, status: 11, age: 8, dur: 8 } as const

export function ciBody(p: CiProps): RenderElement[] {
  const { ui, L, marg, now, list, jobs, shown, log, busy, confirm, down } = p
  const { Box, Text, Markdown } = ui
  const out: RenderElement[] = []
  // Aktion als Link; läuft eine andere Aktion (busy) oder gerade dieser Klick, ist er gesperrt.
  const act = (key: string, label: string, run: () => void, tone: 'link' | 'muted' | 'warn' | 'bad' | 'ok' = 'link') =>
    link(ui, key, label, run, { busy: down === key ? true : busy !== '' ? 'locked' : false, tone })
  const cell = (key: string, width: number, child: RenderElement | string, dim = false) => (
    <Box key={key} width={width} flexShrink={0}>{typeof child === 'string' ? <Text dimColor={dim} wrap="truncate-end">{child}</Text> : child}</Box>
  )

  if (!p.supported) {
    out.push(
      tile(ui, 'pane', { key: 'card-ci-none', tint: TINT.neutral, marg, grow: true, head: <Text bold>{t(L, 'ci.head')}</Text> }, <Text key="ci-nogl" dimColor>{t(L, 'ci.noGitlab')}</Text>),
    )
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
        <Text bold>{t(L, 'ci.head')}</Text>
        <Text dimColor>{pipes.length}</Text>
        {act('ci-reload', t(L, 'ci.refresh'), p.onReload, 'muted')}
        {p.branch !== '' && act('ci-new', newAsk ? t(L, 'ci.newAsk', { ref: p.branch }) : t(L, 'ci.new'), () => p.onPipe('new'), newAsk ? 'warn' : 'link')}
      </Box>,
      busy !== '' && <Text key="ci-busy" color={THEME.accent}>◌ {busy}</Text>,
      list === null && <Text key="ci-load" dimColor>{t(L, 'ci.loading')}</Text>,
      list?.error ? (
        <Box key="ci-err" flexDirection="column" marginTop={1}>
          <Text color={THEME.error} bold>{t(L, 'ci.error', { why: list.error })}</Text>
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
        <Text key="ci-none" dimColor>{list.error ? '–' : t(L, 'ci.none')}</Text>
      ) : (
        <Box key="ci-grid" flexDirection="column">
          {pipes.map(x => {
            const sel = shown?.id === x.id
            return (
              <Box key={`ci-row-${x.id}`} flexDirection="row" columnGap={1} alignItems="center">
                {cell(`ci-row-${x.id}-id`, W.id, link(ui, `ci-pick-${x.id}`, `${sel ? '▸ ' : ''}#${x.id}`, () => p.onPick(x.id), { tone: sel ? 'strong' : 'link' }))}
                {cell(`ci-row-${x.id}-st`, W.status, <Box>{badge(ui, `ci-row-${x.id}-b`, x.status, ciTone(x.status))}</Box>)}
                <Box key={`ci-row-${x.id}-ref`} flexGrow={1} flexShrink={1}><Text bold={x.ref === p.branch} wrap="truncate-end">{x.ref} <Text dimColor>{x.sha}</Text></Text></Box>
                {cell(`ci-row-${x.id}-age`, W.age, ago(x.createdAt, now), true)}
                {cell(`ci-row-${x.id}-dur`, W.dur, clock(x.durationS), true)}
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
  const dur = cur.durationS > 0 ? clock(cur.durationS) : '–'
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
            <Text bold>{t(L, 'ci.sel', { id: cur.id })}</Text>
            {badge(ui, 'ci-sel-b', cur.status, ciTone(cur.status))}
          </Box>
        ),
      },
      cur.title !== '' && <Text key="ci-title" wrap="truncate-end">{cur.title}</Text>,
      <Text key="ci-meta" dimColor wrap="truncate-end">{t(L, 'ci.meta', { ref: cur.ref, sha: cur.sha, when: ago(cur.createdAt, now), dur, user: cur.user === '' ? '' : ` · @${cur.user}` })}</Text>,
      <Box key="ci-pact" flexDirection="row" columnGap={2} flexWrap="wrap" marginTop={1}>
        {can.retry && act('ci-p-retry', t(L, 'ci.retry'), () => p.onPipe('retry'))}
        {can.cancel && act('ci-p-cancel', cancelAsk ? t(L, 'ci.cancelAsk') : t(L, 'ci.cancel'), () => p.onPipe('cancel'), cancelAsk ? 'bad' : 'link')}
        {cur.url !== '' && <Markdown key="ci-p-url" onLinkPress={p.ext} text={`[${t(L, 'ci.open')}](${cur.url})`} />}
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
                return (
                  <Box key={`ci-job-${j.id}`} flexDirection="row" columnGap={1} alignItems="center" flexWrap="wrap">
                    {cell(`ci-job-${j.id}-st`, W.status, <Box>{badge(ui, `ci-job-${j.id}-b`, j.status, ciTone(j.status))}</Box>)}
                    <Box key={`ci-job-${j.id}-name`} flexGrow={1} flexShrink={1}>
                      <Text wrap="truncate-end">{j.name}{j.allowFailure ? <Text dimColor> ({t(L, 'ci.allowFailure')})</Text> : null}</Text>
                    </Box>
                    {cell(`ci-job-${j.id}-dur`, W.dur, clock(j.durationS), true)}
                    {c.log && act(`ci-j-log-${j.id}`, t(L, 'ci.job.log'), () => p.onLog(j.id), 'muted')}
                    {c.play && act(`ci-j-play-${j.id}`, t(L, 'ci.job.play'), () => p.onJob(j.id, 'play'), 'ok')}
                    {c.retry && act(`ci-j-retry-${j.id}`, t(L, 'ci.job.retry'), () => p.onJob(j.id, 'retry'))}
                    {c.cancel && act(cancelKey, t(L, 'ci.job.cancel'), () => p.onJob(j.id, 'cancel'), 'muted')}
                  </Box>
                )
              })}
            </Box>
          ))}
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
