// Theme-System des Cockpits, nach Atomic Design in drei Stufen:
//   1. Tokens (Atome): die Paletten DARK und LIGHT, je Farbrolle ein Wert – hier und nur hier stehen Hex-Werte der Oberfläche.
//   2. Moleküle: Tabellen, die Tokens zu Rollen bündeln (DECISION_COLOR, MODES …), gebaut mit `themed(() => …)`.
//   3. Organismen (Panels, Band, Skins) lesen nur THEME/TINT/CARD_TINT oder Moleküle, nie ein Hex-Literal.
// Die Paletten sind veränderlich: `applyScheme` überschreibt sie an Ort und Stelle, alle Importe bleiben gültig, und jede
// `themed`-Tabelle rechnet sich neu. Reine Funktionen, ohne `$` – direkt testbar.

export type Tone = 'accent' | 'success' | 'warning' | 'error'
export type Appearance = 'auto' | 'hell' | 'dunkel'

export type ThemeColors = Record<
  | 'accent' | 'accentFill' | 'info' | 'success' | 'ratingGood' | 'warning' | 'error' | 'textSecondary' | 'textMuted' | 'border'
  | 'chip' | 'max' | 'orga' | 'surface' | 'violet' | 'codeBg' | 'paneFill' | 'btnHigh' | 'skinFill' | 'onFill',
  string
>
type Tints = Record<Tone | 'neutral', string>

const CARD_KEYS = ['cardWorkdir', 'cardGit', 'cardContext', 'cardOverview', 'cardTurn', 'cardAgents', 'cardTasks', 'cardPlan', 'cardLog'] as const
type CardKey = (typeof CARD_KEYS)[number]
// Reihenfolge für Seiten mit gleichartigen Karten (Einstellungen, Zusammenfassung): Karte i nimmt CYCLE_KEYS[i % n].
const CYCLE_KEYS: readonly CardKey[] = ['cardWorkdir', 'cardContext', 'cardOverview', 'cardTurn', 'cardAgents', 'cardTasks', 'cardLog']

type Palette = { colors: ThemeColors; tints: Tints; cards: Record<CardKey, string> }

// Farben nach ErpApi/design-system (`om-*`, Rollen --text-*, --status-*-fg, --border-default); Kartengründe sind gedimmt.
const DARK: Palette = {
  colors: {
    accent: '#8eadfd', // om-blue-400 · Akzent für Text und Rahmen
    accentFill: '#4277fc', // om-blue-500 · Akzent für Flächen (Ringbögen, Balken)
    info: '#8eadfd',
    success: '#89c958', // om-green-500
    ratingGood: '#c9d65a', // Ampel „gut“: zwischen success und warning
    warning: '#e8b641', // om-amber-500
    error: '#fa6e6e', // om-red-400
    textSecondary: '#aeb2ba', // om-ink-300
    textMuted: '#8c929c', // om-ink-400
    border: '#4a5565', // om-ink-600
    chip: '#2f3a59', // om-navy-700: Grund der Nummern- und Typ-Chips
    max: '#82b1ff', // Wortmarke „MAX“ (brand/README.md)
    orga: '#ffffff', // Wortmarke „orga“
    surface: '#222222', // Grund schwebender Hinweise
    violet: '#aa8ed6', // Agent-Symbol, Hinweis „Wichtig“
    codeBg: '#2b2e35', // Inline-Code
    paneFill: '#2c4a93', // gedämpfte Knopf-Fläche im Panel
    btnHigh: '#4277fc', // Fläche der Hauptaktion und der Band-Knöpfe
    skinFill: '#9a5b2e', // Knopf-Fläche im Chat
    onFill: '#222222', // Schrift auf Knopf-Flächen beim Überfahren
  },
  tints: { neutral: '#262b33', accent: '#1f2a44', success: '#1f2c1c', warning: '#33290f', error: '#3a1f22' },
  cards: {
    cardWorkdir: '#232a3a', // Navy
    cardGit: '#1f2a40', // Nachtblau: gehört zum Arbeitsverzeichnis darüber
    cardContext: '#1e2e33', // Petrol
    cardOverview: '#2a2538', // Indigo-Violett
    cardTurn: '#1f2e2a', // Tannengrün
    cardAgents: '#2d2733', // Pflaume
    cardTasks: '#1f2b36', // Stahlblau
    cardPlan: '#33261e', // Kupfer
    cardLog: '#2e2a24', // Sand
  },
}

// Hell: Text- und Statusfarben abgedunkelt (Kontrast ≥ 4,5 auf Weiß), Chips und Karten als helle Pastelltöne.
const LIGHT: Palette = {
  colors: {
    accent: '#1f4fc4',
    accentFill: '#4277fc',
    info: '#1f4fc4',
    success: '#2d7a1c',
    ratingGood: '#6a8a00',
    warning: '#8f5f00',
    error: '#c4262e',
    textSecondary: '#4a5565',
    textMuted: '#667085',
    border: '#c3c9d4',
    chip: '#dbe4fb',
    max: '#1f4fc4',
    orga: '#1a1d23',
    surface: '#f3f4f6',
    violet: '#6b46c1',
    codeBg: '#e9ebf0',
    // Knopf-Schrift folgt dem Host (im hellen Theme dunkel, nicht einstellbar, auch nicht als Text-Kind: SPEC.md, 2026-10-27): deshalb helle Flächen statt satter Blautöne.
    paneFill: '#cfdcfa',
    btnHigh: '#a9c2f9',
    skinFill: '#f3d9c0',
    onFill: '#1a1d23',
  },
  tints: { neutral: '#f6f7f8', accent: '#f0f4fe', success: '#f0f7ec', warning: '#fdf6e5', error: '#fdeff0' },
  cards: {
    cardWorkdir: '#f1f4fa',
    cardGit: '#edf2fa',
    cardContext: '#edf7f8',
    cardOverview: '#f4f1fa',
    cardTurn: '#eef7f3',
    cardAgents: '#f7f1f7',
    cardTasks: '#eef4f9',
    cardPlan: '#faf3ed',
    cardLog: '#f7f4ee',
  },
}

// Standard ist dunkel (bisheriges Verhalten).
export const THEME: ThemeColors = { ...DARK.colors }
export const TINT: Tints = { ...DARK.tints }
export const CARD_TINT: Record<string, string> = { ...DARK.cards }
export const CARD_CYCLE: string[] = CYCLE_KEYS.map(k => DARK.cards[k])
export const cycleTint = (i: number): string => CARD_CYCLE[i % CARD_CYCLE.length] as string

let light = false
export const isLightScheme = (): boolean => light

const refreshers: Array<() => void> = []

// Molekül: eine aus Tokens gebaute Tabelle, die sich bei jedem Schemawechsel neu rechnet (am Ort, die Referenz bleibt).
export function themed<T extends object>(build: () => T): T {
  const table = build()
  refreshers.push(() => void Object.assign(table, build()))
  return table
}

// Schaltet die Paletten um; true, wenn sich das Schema geändert hat (dann neu zeichnen lassen).
export function applyScheme(toLight: boolean): boolean {
  const changed = toLight !== light
  light = toLight
  const p = toLight ? LIGHT : DARK
  Object.assign(THEME, p.colors)
  Object.assign(TINT, p.tints)
  Object.assign(CARD_TINT, p.cards)
  CYCLE_KEYS.forEach((k, i) => void (CARD_CYCLE[i] = p.cards[k]))
  refreshers.forEach(fn => fn())
  return changed
}

// Host-Theme (`/config` → theme: dark, light, light-daltonized …) → hell?
export const isLightTheme = (value: unknown): boolean => typeof value === 'string' && value.startsWith('light')

// Einstellung „Darstellung“ gegen das Host-Theme: `hell`/`dunkel` gelten immer, `auto` folgt dem Host-Theme.
// „auto“ kann das Desktop-Theme nicht sehen (die Engine meldet nur das CLI-Theme, ungeprüft); dann hilft `hell`.
export const wantsLight = (appearance: unknown, hostTheme: unknown): boolean =>
  appearance === 'hell' ? true : appearance === 'dunkel' ? false : isLightTheme(hostTheme)

// Desktop-App: `userThemeMode` aus %APPDATA%/Claude/config.json (light | dark | system). `system` folgt Windows:
// `reg query …\Themes\Personalize /v AppsUseLightTheme` liefert `0x1` (hell) oder `0x0` (dunkel). undefined = unbekannt.
export function desktopModeLight(configJson: string, regOut: string | undefined): boolean | undefined {
  let mode: unknown
  try {
    mode = (JSON.parse(configJson) as { userThemeMode?: unknown }).userThemeMode
  } catch {
    return undefined
  }
  if (mode === 'light') return true
  if (mode === 'dark') return false
  const m = /AppsUseLightTheme\s+REG_DWORD\s+0x([0-9a-f]+)/i.exec(regOut ?? '')
  return m === null ? undefined : m[1] !== '0'
}
