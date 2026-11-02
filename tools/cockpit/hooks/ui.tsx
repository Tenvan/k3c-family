// Design-System des Cockpits (Atomic Design): Tokens → Atome → Moleküle. Jede Zeichnung baut aus diesen Bausteinen,
// damit eine Style-Änderung an genau einer Stelle passiert. Jeder Baustein bekommt den Bereich (`mode`), in dem er steht:
// Pane, Skin (Chat), Sitzungsleiste oder Band; die Tokens je Bereich stehen in MODES.
// Die Bausteine bekommen die Elementtabelle `ui` der Oberfläche und Callbacks, nie `$` (der darf register.tsx nicht verlassen).
import type { EngineInterface, RenderChildren, RenderElement } from 'claude-code'
import { THEME, TINT, themed } from './core'
import type { Tone } from './core'

export type Ui = ReturnType<EngineInterface['ui']['resolve']>

// ---- Tokens ----

export type Mode = 'pane' | 'skin' | 'sidebar' | 'band'

// Je Bereich: Füllfarbe der Knopf-Pillen, ob sie abgerundet sind (runder Rahmen, nur mit Svg: im Terminal kostet ein Rahmen
// zwei Zeilen), Grund der Chips, Innenrand der Karten und Breite der Bezeichnungsspalte in Zeilen-Rastern.
// `sidebar`: Teile, die die Sitzungsleiste der Desktop-App betreffen (verknüpfte Sessions, Sitzungsleisten-Einstellungen).
// `fillHigh`: Grund der Hauptaktion eines Panels (Priorität `high`); `fill` gilt für gewöhnliche Knöpfe. Im Panel ist `fill`
// gedämpft, damit die Hauptaktion heraussticht; in Band, Chat und Sitzungsleiste sind beide gleich.
export type ModeStyle = { fill: string; fillHigh: string; round: boolean; chip: string; cardX: number; cardY: number; labelCols: number }
export const MODES: Record<Mode, ModeStyle> = themed(() => ({
  pane: { fill: THEME.paneFill, fillHigh: THEME.btnHigh, round: false, chip: THEME.chip, cardX: 1, cardY: 1, labelCols: 12 },
  band: { fill: THEME.btnHigh, fillHigh: THEME.btnHigh, round: false, chip: THEME.chip, cardX: 1, cardY: 0, labelCols: 12 },
  skin: { fill: THEME.skinFill, fillHigh: THEME.skinFill, round: true, chip: THEME.chip, cardX: 1, cardY: 0, labelCols: 12 },
  sidebar: { fill: THEME.btnHigh, fillHigh: THEME.btnHigh, round: false, chip: THEME.chip, cardX: 1, cardY: 1, labelCols: 12 },
}))

// Priorität eines Knopfs: `high` die eine Hauptaktion, `normal` gewöhnliche Aktionen, `low` Nebensachen (Schalter, Kopieren,
// Schließen, Verwerfen): ohne Fläche, gedimmte Schrift, voll erst unter dem Zeiger.
export type Prio = 'high' | 'normal' | 'low'

// Priorität je Knopf-Schlüssel, für alle Panels an einer Stelle; was nicht genannt ist, gilt als `normal`.
// high: die eine Hauptaktion eines Panels oder einer Karte. low: Klappen, Schließen, Blättern, Schalter, Kopieren,
// Rückgängig, Löschen, Nebenaufträge an Claude, Knöpfe je Datei.
const HIGH_KEYS = /^(git-m-|mr-a-merge$|mr-a-create$|mr-d-create$|cu-refresh$|hist-day-report$)/
const LOW_KEYS = new RegExp(
  [
    // Git-Panel
    'git-reload$', 'git-unstageAll$', 'git-stash$', 'git-discard$', 'git-ai-review$', 'git-c-(amend|suggest|split)$', 'git-sy-(rebase|force)$',
    'git-o-(parent|delete|ai|close)$', 'git-br-(del|ai)$', 'git-log-undo$', 'git-err-(copy|dismiss)$', 'git-conf-abort$', 'git-(st|wt|un)-b-',
    // MR-Karten
    'mr-a-refresh$', 'mr-a-squash$', 'mr-d-(discard|redraft)$', 'mine-squash-',
    // Cockpit, Zusammenfassung, Verlauf, Einstellungen, Profiling
    'card-fold-', 'card-close-', 'fold-', 'set-(fold-all|unfold-all|reload)$', 'grp-(on|off)-', 'tiles-(minus|plus)$', 'hist-(prev|today|next)$', 'hist-copy-',
    'open-(summary|settings)$', 'ag-open-', 'cu-rebuild$', 'diag-agents$',
  ]
    .map(p => `^${p}`)
    .join('|'),
)
export const prioOf = (key: string): Prio => (HIGH_KEYS.test(key) ? 'high' : LOW_KEYS.test(key) ? 'low' : 'normal')

// ---- Atome ----

// Knopf als Pille: Fläche in der Füllfarbe des Bereichs, beim Überfahren dunkler Text. Ein Button nimmt selbst keine Farbe an,
// deshalb die Fläche drumherum. `busy`: grau mit „◌ … “, solange die Aktion läuft; `'locked'`: grau ohne Spinner und ohne Klick (eine andere Aktion läuft). `flat`: ohne Rundung, schmaler.
export function pill(ui: Ui, mode: Mode, key: string, label: string, onPress: () => void, busy: boolean | 'locked' = false, flat = false, prio: Prio = prioOf(key)): RenderElement {
  const { Box, Button } = ui
  const m = MODES[mode]
  const low = prio === 'low' && !busy
  const fill = busy ? THEME.chip : low ? undefined : prio === 'high' ? m.fillHigh : m.fill
  const round = m.round && !flat && !low && 'Svg' in ui
  const press = busy === 'locked' ? () => undefined : onPress
  return (
    <Box key={`${key}-pill`} backgroundColor={fill} paddingX={1} {...(round && fill !== undefined ? { borderStyle: 'round' as const, borderColor: fill } : {})}>
      <Button key={key} label={busy === true ? `◌ ${label} …` : label} plain dimColor={low} hover={{ color: low ? THEME.accent : THEME.onFill }} onPress={press} />
    </Box>
  )
}

// Link: Aktion als Text ohne Fläche; das Band trägt nur solche (keine Knöpfe). Beim Überfahren Akzentfarbe und unterstrichen.
// Der Desktop zeichnet einen Button nativ in der Host-Schrift und ignoriert Farbe und farbige Text-Kinder (Entscheidungslog
// 2026-10-27, Variante B). Der Ton kommt darum nicht aus der Schrift: `muted` dimmt, `warn`/`bad`/`ok`/`strong` legen einen
// getönten Grund aus TINT darunter (folgt hell/dunkel). Ampelfarben eines Werts trägt der Ring oder ein farbiges Zeichen davor.
// `busy`: „◌ … “ und gedimmt, `'locked'` ohne Klick.
export type LinkTone = 'link' | 'muted' | 'ok' | 'warn' | 'bad' | 'strong'
const LINK_TINT: Partial<Record<LinkTone, Tone>> = { ok: 'success', warn: 'warning', bad: 'error', strong: 'accent' }
export type LinkOpts = { tone?: LinkTone; busy?: boolean | 'locked'; hotkey?: string }
export function link(ui: Ui, key: string, label: string, onPress: () => void, o: LinkOpts = {}): RenderElement {
  const { Box, Button } = ui
  const busy = o.busy ?? false
  const tone = o.tone ?? 'link'
  const text = busy === true ? `◌ ${label} …` : label
  // Eigene Hover-Gruppe je Link: ein Hover braucht eine Box mit Schlüssel oder einen `scope`.
  const btn = (
    <Button
      key={key}
      label={text}
      plain
      dimColor={busy !== false || tone === 'muted'}
      {...(o.hotkey !== undefined ? { hotkey: o.hotkey } : {})}
      hover={{ scope: `lk:${key}`.slice(0, 64), color: THEME.accent, underline: true }}
      onPress={busy === 'locked' ? () => undefined : onPress}
    />
  )
  const tint = busy !== false ? undefined : LINK_TINT[tone]
  return tint === undefined ? btn : <Box key={`${key}-tone`} backgroundColor={TINT[tint]} paddingX={1}>{btn}</Box>
}

// Reiter: der gewählte fett und unterstrichen in Akzentfarbe, die anderen als gedimmte Links. Ein Panel mit Reitern statt
// vieler Panels oder langer Kartenstapel; der Zustand liegt beim Aufrufer (Atom).
export function tabs<T extends string>(ui: Ui, key: string, items: readonly { id: T; label: string }[], active: T, onPick: (id: T) => void): RenderElement {
  const { Box, Text } = ui
  return (
    <Box key={key} flexDirection="row" flexWrap="wrap" columnGap={2} alignItems="center">
      {items.map(it =>
        it.id === active ? (
          <Text key={`${key}-${it.id}`} bold underline color={THEME.accent}>{it.label}</Text>
        ) : (
          link(ui, `${key}-${it.id}`, it.label, () => onPick(it.id), { tone: 'muted' })
        ),
      )}
    </Box>
  )
}

// Chip: kurzer Text auf getöntem Grund (Nummern, „CC“, „new style“). Leerer `key` = keiner (Chip mitten in einer Textzeile).
export function chip(ui: Ui, mode: Mode, key: string, text: string, color?: string): RenderElement {
  return <ui.Text key={key === '' ? undefined : key} color={color} backgroundColor={MODES[mode].chip}> {text} </ui.Text>
}

// ---- Moleküle ----

// Zeile im Raster: Bezeichnung in fester Spalte, Wert daneben (gekürzt).
export function labelRow(ui: Ui, mode: Mode, key: string, label: string, value: RenderChildren): RenderElement {
  const { Box, Text } = ui
  return (
    <Box key={key} flexDirection="row" columnGap={1}>
      <Box width={MODES[mode].labelCols} flexShrink={0}><Text dimColor>{label}</Text></Box>
      {typeof value === 'string' ? <Text wrap="truncate-end">{value}</Text> : value}
    </Box>
  )
}

// Karte: optionaler Leerraum davor (Panel-Ränder), getönter Grund, optionaler Kopf; `fold` setzt einen Klapp-Pfeil vor den
// Kopf (offen ▾, zu ▸), `open` blendet den Inhalt ohne Pfeil aus. Zugeklappt zeigt die Karte nur den Kopf.
export function tile(
  ui: Ui,
  mode: Mode,
  o: { key: string; tint: string; marg: number; head?: RenderElement; fold?: { id: string; open: boolean; onToggle: () => void }; open?: boolean; padX?: number; grow?: boolean },
  ...children: RenderChildren[]
): RenderElement {
  const { Box } = ui
  const m = MODES[mode]
  const open = o.fold?.open ?? o.open ?? true
  const head =
    o.head === undefined ? null : o.fold === undefined ? (
      o.head
    ) : (
      <Box flexDirection="row" columnGap={1} alignItems="center">
        {pill(ui, mode, o.fold.id, open ? '▾' : '▸', o.fold.onToggle, false, false, 'low')}
        <Box flexGrow={1} flexShrink={1}>{o.head}</Box>
      </Box>
    )
  const grow = o.grow ? { flexGrow: 1 } : {}
  return (
    <Box key={o.key} flexDirection="column" {...grow}>
      {o.marg ? <Box height={1} /> : null}
      <Box flexDirection="column" {...grow} backgroundColor={o.tint} marginX={o.marg} paddingX={o.padX ?? m.cardX} paddingY={m.cardY}>
        {head}
        {open && children}
      </Box>
    </Box>
  )
}

// Badge: kurzer Statustext fett in der Farbe des Tons auf getöntem Grund (Stand einer Session, Aktion einer Freigabe).
// Der Ton kommt aus THEME/TINT zur Zeichenzeit, damit hell und dunkel stimmen.
export type BadgeTone = Tone | 'neutral'
export function badge(ui: Ui, key: string, text: string, tone: BadgeTone): RenderElement {
  const color = tone === 'neutral' ? THEME.textMuted : THEME[tone]
  return <ui.Text key={key} bold color={color} backgroundColor={TINT[tone]}> {text} </ui.Text>
}
