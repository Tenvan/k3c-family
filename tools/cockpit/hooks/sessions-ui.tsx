// Reiter Sessions: zeichnet die gelesene Status-Datei als zwei Karten, Session-Übersicht und Freigabe-Inbox. Ohne `$`: das
// Lesen der Datei und der Neu-Lesen-Knopf kommen aus register.tsx (`refreshSessions`, Callback `onReload`).
import type { RenderElement } from 'claude-code'

import type { ApprovalAction, SessionStand, SessionsState } from '../types'
import { CARD_TINT, THEME, TINT } from './core'
import { t } from './i18n'
import type { Key } from './i18n'
import { isWaiting } from './sessions'
import { badge, link, tile } from './ui'
import type { BadgeTone, Ui } from './ui'
import type { UiLang } from '../types'

const STAND_TONE: Record<SessionStand | '', BadgeTone> = { läuft: 'success', wartet: 'warning', fertig: 'accent', archiviert: 'neutral', '': 'neutral' }
const ACTION_TONE: Record<ApprovalAction, BadgeTone> = { commit: 'accent', push: 'warning', löschen: 'error', archivieren: 'neutral', sonstiges: 'neutral' }

// Spaltenbreiten in Zeichen; Branch und Notiz teilen sich den Rest und werden gekürzt.
const W = { id: 7, repo: 18, state: 12, waiting: 11, since: 12 } as const

export type SessionsProps = { ui: Ui; L: UiLang; marg: number; st: SessionsState | null; path: string; onReload: () => void }

export function sessionsBody({ ui, L, marg, st, path, onReload }: SessionsProps): RenderElement[] {
  const { Box, Text } = ui
  const cell = (key: string, width: number, child: RenderElement | string, dim = false) => (
    <Box key={key} width={width} flexShrink={0}>{typeof child === 'string' ? <Text dimColor={dim} wrap="truncate-end">{child}</Text> : child}</Box>
  )
  const head = (key: string, cols: [string, number | null][]) => (
    <Box key={key} flexDirection="row" columnGap={1}>
      {cols.map(([name, w], i) => (w === null ? <Box key={`${key}-${i}`} flexGrow={1} flexShrink={1}><Text dimColor bold>{name}</Text></Box> : cell(`${key}-${i}`, w, <Text dimColor bold>{name}</Text>)))}
    </Box>
  )
  const out: RenderElement[] = []

  // Kopfkarte: Stand der Datei, Pfad, Neu lesen und Lesefehler als lesbare Meldung (der letzte gute Stand bleibt darunter).
  const err = st?.error ?? null
  const view = st?.view ?? null
  out.push(
    tile(
      ui,
      'pane',
      { key: 'card-sessions-file', tint: err !== null ? TINT.error : CARD_TINT.cardWorkdir ?? TINT.neutral, marg, grow: true },
      <Box key="sf-top" flexDirection="row" columnGap={2} flexWrap="wrap" alignItems="center">
        <Text bold>{t(L, 'sessions.head')}</Text>
        <Text dimColor>{t(L, 'sessions.updated', { updated: view === null ? '–' : view.updated === '' ? t(L, 'sessions.noDate') : view.updated })}</Text>
        {link(ui, 'sessions-reload', t(L, 'sessions.reload'), onReload, { tone: 'muted' })}
      </Box>,
      <Box key="sf-path" flexDirection="row" columnGap={1}>
        <Text dimColor>{t(L, 'sessions.file')}</Text>
        <Text color={THEME.textMuted} wrap="truncate-end">{path}</Text>
      </Box>,
      st === null && <Text key="sf-load" dimColor>{t(L, 'sessions.loading')}</Text>,
      err !== null && (
        <Box key="sf-err" flexDirection="column" marginTop={1}>
          <Text color={THEME.error} bold>{t(L, `sessions.err.${err.kind}` as Key, { path, detail: err.detail })}</Text>
          {view !== null && <Text dimColor>{t(L, 'sessions.err.keep')}</Text>}
        </Box>
      ),
      view !== null && view.issues.length > 0 && (
        <Box key="sf-issues" flexDirection="column" marginTop={1}>
          <Text color={THEME.warning}>{t(L, 'sessions.issues', { n: view.issues.length })}</Text>
          {view.issues.map((x, i) => <Text key={`sf-i-${i}`} dimColor wrap="truncate-end">· {x}</Text>)}
        </Box>
      ),
    ),
  )
  if (view === null) return out

  // Session-Übersicht: wer auf Ralf wartet, steht oben (Sortierung in parseSessions) und liegt auf getöntem Grund.
  const waiting = view.sessions.filter(s => isWaiting(s.waitingFor, s.state)).length
  out.push(
    tile(
      ui,
      'pane',
      {
        key: 'card-sessions-list',
        tint: CARD_TINT.cardTasks ?? TINT.neutral,
        marg,
        grow: true,
        head: (
          <Box flexDirection="row" columnGap={2}>
            <Text bold>{t(L, 'sessions.head')}</Text>
            <Text dimColor>{view.sessions.length}</Text>
            {waiting > 0 && badge(ui, 'sl-waiting', t(L, 'sessions.waitingYou', { n: waiting }), 'warning')}
          </Box>
        ),
      },
      view.sessions.length === 0 ? (
        <Text key="sl-none" dimColor>{t(L, 'sessions.none')}</Text>
      ) : (
        <Box key="sl-grid" flexDirection="column">
          {head('sl-head', [[t(L, 'sessions.col.id'), W.id], [t(L, 'sessions.col.repo'), W.repo], [t(L, 'sessions.col.branch'), null], [t(L, 'sessions.col.state'), W.state], [t(L, 'sessions.col.waiting'), W.waiting]])}
          {view.sessions.map((s, i) => {
            const wait = isWaiting(s.waitingFor, s.state)
            const label = s.state === '' ? (s.stateRaw === '' ? t(L, 'sessions.stand.unknown') : s.stateRaw) : t(L, `sessions.stand.${s.state}` as Key)
            // Zwei Zeilen je Session: Kennzahlen oben, darunter Titel und Notiz in voller Breite (umbrechend, eingerückt unter Repo).
            return (
              <Box key={`sl-${i}`} flexDirection="column" marginTop={i === 0 ? 0 : 1} backgroundColor={wait ? TINT.warning : undefined}>
                <Box flexDirection="row" columnGap={1}>
                  {cell(`sl-${i}-id`, W.id, <Text bold color={wait ? THEME.warning : undefined} wrap="truncate-end">{s.id || '?'}</Text>)}
                  {cell(`sl-${i}-repo`, W.repo, s.repo)}
                  <Box key={`sl-${i}-branch`} flexGrow={1} flexShrink={1}><Text dimColor wrap="truncate-end">{s.branch}</Text></Box>
                  {cell(`sl-${i}-state`, W.state, <Box>{badge(ui, `sl-${i}-b`, label, STAND_TONE[s.state])}</Box>)}
                  {cell(`sl-${i}-wait`, W.waiting, s.waitingFor, !wait)}
                </Box>
                {(s.title !== '' || s.note !== '') && (
                  <Box key={`sl-${i}-note`} flexDirection="row" columnGap={1}>
                    <Box width={W.id} flexShrink={0} />
                    <Box flexGrow={1} flexShrink={1}>
                      <Text>
                        {s.title !== '' && <Text bold>{s.title}</Text>}
                        {s.title !== '' && s.note !== '' ? <Text dimColor> – </Text> : null}
                        {s.note !== '' && <Text dimColor>{s.note}</Text>}
                      </Text>
                    </Box>
                  </Box>
                )}
              </Box>
            )
          })}
        </Box>
      ),
    ),
  )

  // Freigabe-Inbox: offene Freigaben in Dateireihenfolge; leerer Zustand mit klarer Meldung.
  out.push(
    tile(
      ui,
      'pane',
      {
        key: 'card-sessions-inbox',
        tint: CARD_TINT.cardPlan ?? TINT.neutral,
        marg,
        grow: true,
        head: (
          <Box flexDirection="row" columnGap={2}>
            <Text bold>{t(L, 'sessions.inbox')}</Text>
            <Text dimColor>{view.approvals.length}</Text>
          </Box>
        ),
      },
      view.approvals.length === 0 ? (
        <Text key="ib-none" color={THEME.success}>{t(L, 'sessions.inboxEmpty')}</Text>
      ) : (
        <Box key="ib-grid" flexDirection="column">
          {head('ib-head', [[t(L, 'sessions.col.id'), W.id], [t(L, 'sessions.col.action'), null], [t(L, 'sessions.col.since'), W.since]])}
          {view.approvals.map((a, i) => (
            // Zwei Zeilen je Freigabe: ID, Aktion und seit oben, das Detail darunter in voller Breite (umbrechend).
            <Box key={`ib-${i}`} flexDirection="column" marginTop={i === 0 ? 0 : 1}>
              <Box flexDirection="row" columnGap={1}>
                {cell(`ib-${i}-id`, W.id, <Text bold wrap="truncate-end">{a.id || '?'}</Text>)}
                <Box key={`ib-${i}-act`} flexGrow={1} flexShrink={1}>{badge(ui, `ib-${i}-b`, a.action === 'sonstiges' && a.actionRaw !== '' && a.actionRaw.toLowerCase() !== 'sonstiges' ? `${t(L, 'sessions.action.sonstiges')}: ${a.actionRaw}` : t(L, `sessions.action.${a.action}` as Key), ACTION_TONE[a.action])}</Box>
                {cell(`ib-${i}-since`, W.since, a.since, true)}
              </Box>
              {a.detail !== '' && (
                <Box key={`ib-${i}-detail`} flexDirection="row" columnGap={1}>
                  <Box width={W.id} flexShrink={0} />
                  <Box flexGrow={1} flexShrink={1}><Text>{a.detail}</Text></Box>
                </Box>
              )}
            </Box>
          ))}
        </Box>
      ),
    ),
  )
  return out
}
