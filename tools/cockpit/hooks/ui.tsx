// Design-System des Cockpits (Atomic Design): Tokens → Atome → Moleküle. Jede Zeichnung baut aus diesen Bausteinen,
// damit eine Style-Änderung an genau einer Stelle passiert. Jeder Baustein bekommt den Bereich (`mode`), in dem er steht:
// Pane, Skin (Chat), Sitzungsleiste oder Band; die Tokens je Bereich stehen in MODES.
// Die Bausteine bekommen die Elementtabelle `ui` der Oberfläche und Callbacks, nie `$` (der darf register.tsx nicht verlassen).
import type { EngineInterface, RenderChildren, RenderElement } from 'claude-code'
import { THEME } from './core'

export type Ui = ReturnType<EngineInterface['ui']['resolve']>

// ---- Tokens ----

export type Mode = 'pane' | 'skin' | 'sidebar' | 'band'

// Je Bereich: Füllfarbe der Knopf-Pillen, ob sie abgerundet sind (runder Rahmen, nur mit Svg: im Terminal kostet ein Rahmen
// zwei Zeilen), Grund der Chips, Innenrand der Karten und Breite der Bezeichnungsspalte in Zeilen-Rastern.
// `sidebar`: Teile, die die Sitzungsleiste der Desktop-App betreffen (verknüpfte Sessions, Sitzungsleisten-Einstellungen).
export const MODES: Record<Mode, { fill: string; round: boolean; chip: string; cardX: number; cardY: number; labelCols: number }> = {
  pane: { fill: THEME.accentFill, round: false, chip: THEME.chip, cardX: 1, cardY: 1, labelCols: 12 },
  band: { fill: THEME.accentFill, round: false, chip: THEME.chip, cardX: 1, cardY: 0, labelCols: 12 },
  skin: { fill: '#9a5b2e', round: true, chip: THEME.chip, cardX: 1, cardY: 0, labelCols: 12 },
  sidebar: { fill: THEME.accentFill, round: false, chip: THEME.chip, cardX: 1, cardY: 1, labelCols: 12 },
}

// Grund eines Knopfs, solange seine Aktion läuft (in allen Bereichen gleich).
export const BUSY_FILL = THEME.chip

// ---- Atome ----

// Knopf als Pille: Fläche in der Füllfarbe des Bereichs, beim Überfahren dunkler Text. Ein Button nimmt selbst keine Farbe an,
// deshalb die Fläche drumherum. `busy`: grau mit „◌ … “, solange die Aktion läuft; `'locked'`: grau ohne Spinner und ohne Klick (eine andere Aktion läuft). `flat`: ohne Rundung, schmaler.
export function pill(ui: Ui, mode: Mode, key: string, label: string, onPress: () => void, busy: boolean | 'locked' = false, flat = false): RenderElement {
  const { Box, Button } = ui
  const m = MODES[mode]
  const fill = busy ? BUSY_FILL : m.fill
  const round = m.round && !flat && 'Svg' in ui
  const press = busy === 'locked' ? () => undefined : onPress
  return (
    <Box key={`${key}-pill`} backgroundColor={fill} paddingX={1} {...(round ? { borderStyle: 'round' as const, borderColor: fill } : {})}>
      <Button key={key} label={busy === true ? `◌ ${label} …` : label} plain hover={{ color: THEME.surface }} onPress={press} />
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
        {pill(ui, mode, o.fold.id, open ? '▾' : '▸', o.fold.onToggle)}
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
