// Welt-Filter für Listen von Assets (B-1169, WZ62.2): Welt und Modus (gesamt oder exklusiv), dazu Gruppe, Art und
// „nur geänderte“. Reine Funktionen ohne React; der Zustand steht in der Adresse (`#welt=sengoku&modus=exklusiv`), damit
// andere Seiten (Welten-Editor, WZ62.4) den Katalog gefiltert aufrufen können. Katalog und Sound nutzen dieselbe Logik.

export type WeltModus = 'gesamt' | 'exklusiv';

export const WELT_MODI: readonly WeltModus[] = ['gesamt', 'exklusiv'];

export interface WeltFilterState {
  /** Welt-ID; '' = keine Welt gewählt, alle Assets. */
  welt: string;
  modus: WeltModus;
  /** Gruppe (`gegner`, `figuren` …); '' = alle. */
  gruppe: string;
  /** Art = Rig (`humanoid`, `quadruped` …); '' = alle. */
  art: string;
  /** Nur Assets mit Änderung gegen HEAD. */
  geaendert: boolean;
}

export const WELT_VORGABE: WeltFilterState = { welt: '', modus: 'gesamt', gruppe: '', art: '', geaendert: false };

/** Welche Assets eine Welt sieht (Antwort des Backends, `WeltSicht`): Schlüssel sind `<gruppe>/<asset>`. */
export interface WeltSichtLike {
  welt: string;
  /** Herkunft der Assets ohne Welt-Zuordnung (die Basis-Welt). */
  basis: string;
  /** Assets mit Welt-Zuordnung innerhalb der Kette der Welt → Herkunftswelt. */
  eigene: Readonly<Record<string, string>>;
  /** Assets, die die Welt nicht sieht (fremd oder per Skin ersetzt). */
  aus: readonly string[];
}

export interface WeltZuordnung {
  /** Zeigt die Liste das Asset im gewählten Modus? */
  sichtbar: boolean;
  /** Herkunftswelt; bei Basis-Assets die Basis-Welt. */
  herkunft: string;
  /** Exklusiv für die gewählte Welt? */
  exklusiv: boolean;
  /** Von einer anderen Welt der Kette geerbt (auch Basis), nicht exklusiv. */
  geerbt: boolean;
}

/** Zuordnung eines Assets zur Welt der Sicht; ohne Sicht (keine Welt gewählt) ist alles sichtbar. */
export function zuordnung(sicht: WeltSichtLike | null, modus: WeltModus, key: string): WeltZuordnung {
  if (!sicht) return { sichtbar: true, herkunft: '', exklusiv: false, geerbt: false };
  const eigene = sicht.eigene[key];
  if (eigene === undefined && sicht.aus.includes(key)) return { sichtbar: false, herkunft: '', exklusiv: false, geerbt: false };
  const exklusiv = eigene === sicht.welt;
  return {
    sichtbar: modus === 'gesamt' || exklusiv,
    herkunft: eigene ?? sicht.basis,
    exklusiv,
    geerbt: !exklusiv,
  };
}

const KEYS = ['welt', 'modus', 'gruppe', 'art', 'geaendert'] as const;

/** Filter aus einer Adresse (`location.hash`, mit oder ohne `#`); Unbekanntes und Ungültiges fällt auf die Vorgabe. */
export function parseWeltHash(hash: string): WeltFilterState {
  const p = new URLSearchParams(hash.replace(/^#/, ''));
  const modus = p.get('modus');
  return {
    welt: p.get('welt') ?? '',
    modus: WELT_MODI.find((m) => m === modus) ?? 'gesamt',
    gruppe: p.get('gruppe') ?? '',
    art: p.get('art') ?? '',
    geaendert: p.get('geaendert') === '1',
  };
}

/** Adresse mit dem Filter; fremde Parameter der alten Adresse bleiben, Vorgaben werden weggelassen. */
export function formatWeltHash(state: WeltFilterState, old = ''): string {
  const p = new URLSearchParams(old.replace(/^#/, ''));
  for (const k of KEYS) p.delete(k);
  if (state.welt) p.set('welt', state.welt);
  if (state.welt && state.modus !== 'gesamt') p.set('modus', state.modus);
  if (state.gruppe) p.set('gruppe', state.gruppe);
  if (state.art) p.set('art', state.art);
  if (state.geaendert) p.set('geaendert', '1');
  const s = p.toString();
  return s ? `#${s}` : '';
}
