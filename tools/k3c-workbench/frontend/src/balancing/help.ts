import type { BalanceForm } from './spec';
import { EMPTY_TUNE, type TuneForm } from './tune';

// Hilfen der Balancing-Konfiguration (WZ28.10, B-609): je Feld ein kurzer Hilfetext unter dem Feld und eine
// ausführliche Hilfe (Markdown) für das Popup; dazu Beispiel-Läufe zum Übernehmen. Die Beschreibungen der Dateien in
// data/ stehen nicht hier, sondern in docs/data/*.md (Popup der Datei-Liste).

export type HelpKey = 'seeds' | 'depth' | 'players' | 'bot' | 'grade' | 'world' | 'helps' | 'days' | 'vary' | 'label' | 'baseline' | 'mode' | 'params' | 'steps' | 'budget';

export interface FieldHelp {
  label: string;
  /** Ein Satz unter dem Feld: Bedeutung und Vorgabe. */
  hint: string;
  /** Ausführlich fürs Popup: Bedeutung, Werte, Vorgabe, Wirkung, Beispiele. */
  help: string;
}

const TARGETS = 'Die Ziele (Zielkorridore) stehen in `data/balance-targets.json` (Beschreibung: Liste „Konfigurationsdateien“).';

/** Felder mit Auswahlbox: Hilfetext nur im Popup. */
export const SELECT_HELP: readonly HelpKey[] = ['depth', 'players', 'bot', 'grade', 'world', 'helps', 'mode'];

export const FIELD_HELP: Record<HelpKey, FieldHelp> = {
  seeds: {
    label: 'Seeds',
    hint: 'Wie viele Level je Ziel gespielt werden; leer = alle Seeds der Ziele.',
    help: `Ein **Seed** erzeugt ein Level samt Zufall des Laufs; gleicher Seed = gleiches Ergebnis.\n\n- Werte: ganze Zahl ab 1; genommen werden die ersten N Seeds aus \`data/balance-targets.json\` › \`seeds\`.\n- Leer: alle Seeds der Ziele (heute 100).\n- Wirkung: mehr Seeds = genauere Anteile und Mediane, aber längerer Lauf. Zum Ausprobieren 10–20, für eine Entscheidung alle.\n\n${TARGETS}`,
  },
  depth: {
    label: 'Tiefe',
    hint: '0 = Wald, 1 = Höhle, 2 = Mine, 3 = Eisenstollen, 4 = Kristallhöhle; „Ziel“ = Tiefe je Ziel.',
    help: 'Die **Tiefe** wählt die Stufe (und damit das Biom), auf der alle Zielläufe spielen.\n\n| Tiefe | Stufe | Biom |\n|---|---|---|\n| 0 | Oberwelt | `forest` |\n| 1 | Höhle | `cave` |\n| 2 | Mine | `mine` |\n| 3 | Eisenstollen | `ironhold` |\n| 4 | Kristallhöhle | `crystal` |\n\n- Leer: jedes Ziel spielt in seiner eigenen Tiefe (Feld `depth` des Ziels, heute 0).\n- Wirkung: tiefere Stufen haben stärkere Gegner (Skalierung je Tiefe in `data/worlds/*.json` › `depthScaling`) und andere Rohstoffe.',
  },
  players: {
    label: 'Spieler',
    hint: 'Anzahl Monarchen (Bots) je Lauf; „Ziel“ = Spielerzahl je Ziel.',
    help: '**Spieler** ist die Zahl der Monarchen, die der Bot gleichzeitig steuert.\n\n- Werte: ganze Zahl ab 1 (Koop-Bots brauchen mehr: `coop2` ab 2, `coop4` ab 4).\n- Leer: Spielerzahl des Ziels (heute 2).\n- Wirkung: mehr Spieler bringen mehr Gold und Bauten, Wellen wachsen aber mit (`data/waves.json` › `perExtraPlayer`).',
  },
  bot: {
    label: 'Bot',
    hint: 'Spielweise der Monarchen: ein Profil für alle oder „je Spieler“; „Ziel“ = Bot je Ziel.',
    help: 'Der **Bot** spielt die Monarchen ohne Zufall nach einem festen Profil.\n\n| Bot | Spielweise |\n|---|---|\n| `passive` | steht am Hub, zahlt nichts (Untergrenze) |\n| `saver` | sparsam: erst Mauern, dann Landstreicher anwerben |\n| `walls` | Mauern samt Ausbau, dann Türme und Hub-Ausbau, anwerben zuletzt |\n| `economy` | Wirtschaft zuerst, Mauern ab Tag 2 |\n| `wirtschaft` | Standard: Mauern, Turm, Farm, Werkstatt, Hub-Ausbau, Bögen, anwerben |\n| `coop2`, `coop4` | Rollen nach Spieler: gerade Mauern, ungerade Wirtschaft (ab 2 bzw. 4 Spielern) |\n\n- Leer: Bot des Ziels (heute `wirtschaft`).\n- Wirkung: Die Zielkorridore sind für `wirtschaft` gedacht; andere Bots zeigen, wie robust die Werte gegen andere Spielweisen sind.',
  },
  world: {
    label: 'Welt',
    hint: 'Ganze Insel 1 statt einer Stufe.',
    help: 'Mit einer **Welt** spielt jeder Lauf die ganze erste Insel (alle Tiefen) statt einer einzelnen Stufe (BS11, B-642).\n\n- Werte: Welt-Dateien aus `data/worlds/` (heute `standard`: Wald, Höhle, Mine, Eisenstollen, Kristallhöhle).\n- Leer: eine Stufe in der Tiefe des Ziels (wie bisher).\n- Bots steigen ab, sobald der Miniboss ihrer Stufe besiegt ist, bauen auf jeder Stufe auf und kämpfen am Ende gegen den Endboss (Begleitschutz der Truppen); mit mehreren Spielern steigen nur Spieler 2, 4 … ab.\n- Der Bericht zeigt je Tiefe Ankunft, Burgfall, gefallene Monarchen, Hub-Stufe und Kämpfer, je Boss Erscheinen und Sieg, den Endboss und „Schiff bezahlbar ab Tag“.\n- Ein Lauf dauert Minuten (40 Tage ≈ 1–4 min je Seed). Mit der Welt gelten **alle** Ziele für die Insel; für den Meilenstein Bot `wirtschaft`, 1 Spieler, 40 Tage wählen.',
  },
  helps: {
    label: 'Hilfen',
    hint: 'Dev-Hilfen wie im Stufen-Test (nur mit Welt); leer = keine.',
    help: 'Die **Hilfen** schalten dieselben Dev-Hilfen ein wie der Stufen-Test `task balance:fullgame` (BS3.4). Ohne Hilfen läuft der offene Meilenstein der Balance: Grad normal, nichts geschenkt.\n\n- `hilfen`: Grad `dev` (falls keiner gewählt), Dev-Baukosten, voller Insel-Vorrat.\n- `voll`: zusätzlich alle Stufen voll ausgebaut (Hub 5, Mauern, Türme). Standard des Stufen-Tests.\n- `boss`: zusätzlich Endboss mit 300 HP und jeden Tick geheilte Monarchen (Nachweis der Kette bis zum Sieg).\n- Nur mit **Welt**; ohne Welt lehnt der Lauf ab.\n- Leer: keine Hilfen. Erst dann sagt der Lauf etwas über die echte Balance.',
  },
  grade: {
    label: 'Grad',
    hint: 'Schwierigkeitsgrad: dev, easy, normal, hard, ultra; leer = normal.',
    help: 'Der **Grad** skaliert Wellengröße, Gegner-HP und Gegner-Schaden (`data/difficulty.json` › `grades`).\n\n- Werte: `dev`, `easy`, `normal`, `hard`, `ultra` (Kleinbuchstaben).\n- Leer: `normal`.\n- Wirkung: Die Zielkorridore gelten für `normal`; andere Grade zeigen den Abstand der Stufen.',
  },
  days: {
    label: 'Tage',
    hint: 'Gespielte Tage je Lauf (ein Tag = helle Phase + Nacht mit Welle); leer = Tage je Ziel.',
    help: '**Tage** begrenzt die Länge jedes Laufs; nach jedem Tag kommt eine Nacht mit Welle.\n\n- Werte: ganze Zahl ab 1.\n- Leer: Tage des Ziels (heute 5).\n- Wirkung: mehr Tage prüfen spätere Wellen, der Lauf dauert länger. Die Ziele „Nacht 1–5“ werten nur die ersten fünf Nächte.',
  },
  vary: {
    label: 'Varianten',
    hint: 'Eine je Zeile: datei.json:pfad=wert; ändert den Wert nur für diesen Lauf.',
    help: 'Eine **Variante** setzt einen Zahlenwert aus `data/` nur im Speicher für diesen Lauf; die Dateien bleiben unverändert.\n\n- Form: `datei.json:pfad.zum.wert=wert`, eine je Zeile; Listen über den Index (`levels.0.cost`).\n- Nur Dateien, die in der Liste „Konfigurationsdateien“ den Knopf „Werte“ haben (economy, buildings, hub, troops, enemies, waves, monarch); andere Dateien lädt der Lauf nicht neu.\n- Den Pfad am einfachsten in der Liste „Konfigurationsdateien“ über „Werte“ wählen und „Als Variante“ klicken.\n\nBeispiele:\n\n```\neconomy.json:purse.startGold=120\nbuildings.json:wall.cost.gold=3\nwaves.json:perExtraPlayer=0.5\n```\n\nWirkung: Der Bericht nennt die Varianten; mit „Vergleichsbasis“ (Liste „Berichte“) lässt er sich gegen einen Lauf ohne Variante vergleichen.',
  },
  label: {
    label: 'Label',
    hint: 'Freier Name des Laufs für Warteschlange und Bericht; leer = Lauf-ID.',
    help: 'Das **Label** ist nur ein Name für die Warteschlange (z. B. „Startgold 120“). Es ändert den Lauf nicht.',
  },
  baseline: {
    label: 'Baseline',
    hint: 'Name eines gesicherten Basis-Laufs (z. B. M1) als Vergleich; leer = Baseline des Repos.',
    help: 'Eine **Baseline** ist ein gesicherter, durchgelaufener Lauf (`reports/baseline/<name>/`), meist der eines Meilensteins. Der Bericht vergleicht den neuen Lauf gegen sie, je Werte-Gruppe mit Ampel und Trend.\n\n- Werte: Name einer gesicherten Baseline; in „Beispiele › Basis-Läufe“ stehen alle.\n- Leer: die Baseline des Repos (`testdata/balance/baseline.json`).\n- Eingefrorene Gruppen melden sich nur, wenn sie kippen; Tuning-Hinweise nennen nur Hebel freier Gruppen.\n- Gesichert wird mit `sim_test {action: baseline, id, saveBaseline: M1}`.',
  },
  mode: {
    label: 'Abstimm-Modus',
    hint: 'Aus = ein Lauf; Sweep = volles Raster; Suche = beste Werte mit Budget.',
    help: '**Abstimmen** sucht Werte, die alle Zielkorridore treffen, und schlägt sie mit Diff vor (`data/` bleibt unverändert).\n\n- **Aus:** ein normaler Lauf.\n- **Sweep:** jeder Rasterpunkt ein Lauf (Schritte je Parameter, bei zwei Parametern Schritte²).\n- **Suche:** ein Parameter per Ternärsuche, zwei per Raster mit zwei Verfeinerungen, bis das Budget verbraucht ist.\n\nErgebnis: Bericht „Tuning“ mit Score (0 = alle Korridore getroffen), bestem Punkt, Begründung und Diff zum Kopieren.',
  },
  params: {
    label: 'Parameter',
    hint: '1–2 Zeilen datei.json:pfad=min..max (min < max).',
    help: 'Die **Parameter** sind die Werte, die das Abstimmen verändert.\n\n- Form: `datei.json:pfad=min..max`, eine je Zeile, höchstens zwei; min < max; Dezimalzahlen erlaubt.\n- Ganze Zahlen in der Datei bleiben ganz.\n- In der Liste „Konfigurationsdateien“ über „Werte“ wählen und „Als Parameter“ klicken (Bereich ±25 %).\n\nBeispiele:\n\n```\neconomy.json:purse.startGold=60..140\nbuildings.json:wall.cost.gold=2..6\n```',
  },
  steps: {
    label: 'Schritte',
    hint: 'Rasterpunkte je Parameter, 2–21; leer = 5.',
    help: '**Schritte** ist die Zahl der Rasterpunkte je Parameter zwischen min und max.\n\n- Werte: 2 bis 21; leer = 5.\n- Wirkung: Sweep mit 2 Parametern und 5 Schritten = 25 Läufe; mehr Schritte = feiner, aber länger.',
  },
  budget: {
    label: 'Budget',
    hint: 'Höchstzahl Auswertungen der Suche, 1–1000; leer = 40.',
    help: 'Das **Budget** begrenzt, wie viele Punkte die Suche auswertet (jeder Punkt ist ein Lauf über alle Seeds).\n\n- Werte: 1 bis 1000; leer = 40. Nur für „Suche“.\n- Faustregel: Dauer ≈ Budget × Dauer eines Laufs; über Nacht ruhig 100–200.',
  },
};

/** Bots und Grade wie in Go (balance.BotNames, data/difficulty.json › grades); TestAuswahlWieGo hält sie gleich. */
export const BOTS = ['passive', 'saver', 'walls', 'economy', 'wirtschaft', 'coop2', 'coop4'] as const;
export const GRADES = ['dev', 'easy', 'normal', 'hard', 'ultra'] as const;
/** Welten wie in Go (balance.Worlds, data/worlds/*.json). */
export const WORLDS = ['standard'] as const;
/** Hilfen wie in Go (balance.HelpsNames). */
export const HELPS = ['hilfen', 'voll', 'boss'] as const;

/** Bot je Spieler (WZ28.12, B-611): Profile mit `+` verbunden, Spieler 1 zuerst; ohne coop2/coop4. */
export const PER_PLAYER = 'je-spieler';
export const MIX_SEP = '+';
export const SOLO_BOTS = BOTS.filter((b) => !b.startsWith('coop'));
export const isMix = (bot: string) => bot.includes(MIX_SEP);
/** Spielerzahl für die Mischung: gewählte Zahl, sonst die der Ziele (2). */
export const mixCount = (players: string) => Number(players) || 2;

/** Profile der Mischung für n Spieler: fehlende Plätze `saver`, überzählige fallen weg; ein Spieler ergibt ein Profil. */
export function resizeMix(bot: string, n: number): string {
  const parts = isMix(bot) ? bot.split(MIX_SEP) : [bot];
  return Array.from({ length: n }, (_, i) => parts[i] || 'saver').join(MIX_SEP);
}

/** Auswahl der Felder mit festen Werten; '' = Wert des Ziels. */
export const CHOICES = {
  depth: [
    { value: '', label: 'Ziel (je Ziel)' }, { value: '0', label: '0 · Wald' }, { value: '1', label: '1 · Höhle' },
    { value: '2', label: '2 · Mine' }, { value: '3', label: '3 · Eisenstollen' }, { value: '4', label: '4 · Kristallhöhle' },
  ],
  players: [{ value: '', label: 'Ziel (je Ziel)' }, ...['1', '2', '3', '4'].map((n) => ({ value: n, label: n }))],
  bot: [{ value: '', label: 'Ziel (je Ziel)' }, ...BOTS.map((b) => ({ value: b, label: b })), { value: PER_PLAYER, label: 'je Spieler …' }],
  grade: [{ value: '', label: 'normal (Vorgabe)' }, ...GRADES.map((g) => ({ value: g, label: g }))],
  helps: [{ value: '', label: 'keine (Vorgabe)' }, ...HELPS.map((x) => ({ value: x, label: x }))],
  world: [{ value: '', label: 'Einzelstufe (je Ziel)' }, ...WORLDS.map((w) => ({ value: w, label: `${w} · ganze Insel 1` }))],
};

/** Beispiel-Lauf zum Übernehmen in die Konfiguration. */
export interface Example {
  name: string;
  text: string;
  form: Partial<Omit<BalanceForm, 'tune'>> & { tune?: Partial<TuneForm> };
}

export const EXAMPLES: Example[] = [
  { name: 'Insel 1 solo (Stufen-Test 1–4, voll)', text: 'Gleiche Konfiguration wie `task balance:fullgame` (fullgame_test.go): Seed k3c-dev, 1 Spieler, Bot wirtschaft, 40 Tage, Hilfen voll (Standard des Tests). Der Test bricht je Stufe beim Ziel ab (1 Goblin-Anführer, 2 tiefste Stufe, 3 alle Minibosse, 4 Endboss); der Lauf hier spielt bis Tag 40 und zeigt Endboss-Sieg an Tag 19.', form: { seeds: '1', players: '1', bot: 'wirtschaft', world: 'standard', helps: 'voll', days: '40', label: 'Insel 1 solo' } },
  { name: 'Insel 1 solo, Hilfen (Stufen-Test hilfen)', text: 'Gleiche Konfiguration wie `task balance:fullgame` (fullgame_test.go): Seed k3c-dev, 1 Spieler, Bot wirtschaft, 40 Tage, Hilfen hilfen (Grad dev, Dev-Baukosten, Vorrat; K3C_FULLGAME=hilfen).', form: { seeds: '1', players: '1', bot: 'wirtschaft', world: 'standard', helps: 'hilfen', days: '40', label: 'Insel 1 hilfen' } },
  { name: 'Insel 1 solo, Endboss schwach (Stufen-Test boss)', text: 'Gleiche Konfiguration wie `task balance:fullgame` (fullgame_test.go): Seed k3c-dev, 1 Spieler, Bot wirtschaft, 40 Tage, Hilfen boss (voll plus Endboss 300 HP, Monarch geheilt; K3C_FULLGAME=boss).', form: { seeds: '1', players: '1', bot: 'wirtschaft', world: 'standard', helps: 'boss', days: '40', label: 'Insel 1 boss' } },
  { name: 'Insel 1 solo ohne Hilfen (Stufen-Test ohne)', text: 'Gleiche Konfiguration wie `task balance:fullgame` (fullgame_test.go): Seed k3c-dev, 1 Spieler, Bot wirtschaft, 40 Tage, keine Hilfen (K3C_FULLGAME=ohne): der offene Meilenstein B-645, schlägt heute fehl.', form: { seeds: '1', players: '1', bot: 'wirtschaft', world: 'standard', days: '40', label: 'Insel 1 ohne Hilfen' } },
  { name: 'Insel 1 komplett, coop2', text: 'Wie „Insel 1 solo“ (Hilfen voll, Seed k3c-dev, 40 Tage), aber coop2 mit 2 Spielern. Ohne Hilfen: Hilfen leeren.', form: { seeds: '1', players: '2', bot: 'coop2', world: 'standard', helps: 'voll', days: '40', label: 'Insel 1 coop2' } },
  { name: 'Insel 1, Startgold hoch', text: 'Wie „Insel 1 solo“ (Hilfen voll), dazu Startgold und Beutel 500: zeigt, wie viel die Münzskala bremst. Zum Vergleich mit Hilfen leer starten.', form: { seeds: '1', players: '1', bot: 'wirtschaft', world: 'standard', helps: 'voll', days: '40', vary: 'economy.json:purse.startGold=500\neconomy.json:purse.maxGold=500', label: 'Insel 1 reich' } },
  { name: 'Schneller Überblick', text: '20 Seeds im Wald mit den Zielen, Bot saver.', form: { seeds: '20', depth: '0', label: 'Überblick Wald' } },
  { name: 'Startgold variieren', text: 'Gleicher Lauf mit 120 statt 100 Startgold; danach mit dem Überblick als Vergleichsbasis vergleichen.', form: { seeds: '20', depth: '0', vary: 'economy.json:purse.startGold=120', label: 'Startgold 120' } },
  { name: 'Höhle, hart', text: '20 Seeds in Tiefe 1 mit Grad hard.', form: { seeds: '20', depth: '1', grade: 'hard', label: 'Höhle hart' } },
  { name: 'Mauerkosten abstimmen', text: 'Suche nach Mauerkosten 2–6 Gold, die alle Korridore treffen (10 Seeds, Budget 20).', form: { seeds: '10', label: 'Mauerkosten', tune: { mode: 'search', params: 'buildings.json:wall.cost.gold=2..6', budget: '20' } } },
  { name: 'Koop: einer baut, einer wartet', text: '2 Spieler, Spieler 1 saver, Spieler 2 passive: trägt ein Spieler das Spiel allein?', form: { seeds: '20', depth: '0', players: '2', bot: 'saver+passive', label: 'saver+passive' } },
  { name: 'Mauern gegen Wirtschaft', text: '2 Spieler, Spieler 1 walls, Spieler 2 economy (wie coop2, aber frei wählbar).', form: { seeds: '20', players: '2', bot: 'walls+economy', label: 'walls+economy' } },
  { name: 'Allein spielen', text: '1 Spieler mit saver: hält die Burg ohne Mitspieler?', form: { seeds: '20', depth: '0', players: '1', bot: 'saver', label: 'Solo' } },
  { name: 'Vier Spieler Koop', text: '4 Spieler mit coop4 (Rollen Mauern/Wirtschaft im Wechsel).', form: { seeds: '10', players: '4', bot: 'coop4', label: 'Koop 4' } },
  { name: 'Untergrenze: niemand zahlt', text: 'Bot passive: wie lange hält die Burg ganz ohne Bauten?', form: { seeds: '10', bot: 'passive', label: 'Passiv' } },
  { name: 'Lange Partie', text: '10 Tage statt 5: wie entwickeln sich spätere Wellen?', form: { seeds: '10', days: '10', label: '10 Tage' } },
  { name: 'Mine, ultra', text: 'Tiefe 2 mit Grad ultra als Stresstest.', form: { seeds: '10', depth: '2', grade: 'ultra', label: 'Mine ultra' } },
  { name: 'Wellen je Mitspieler', text: 'Wellen wachsen je zusätzlichem Spieler langsamer (perExtraPlayer 0.5).', form: { seeds: '20', vary: 'waves.json:perExtraPlayer=0.5', label: 'perExtraPlayer 0.5' } },
  { name: 'Startgold abstimmen', text: 'Suche nach dem Startgold (60–140), das alle Korridore trifft.', form: { seeds: '10', label: 'Startgold', tune: { mode: 'search', params: 'economy.json:purse.startGold=60..140', budget: '20' } } },
  { name: 'Morgengold × Mauer (Sweep)', text: 'Raster 3 × 3 über Gold je Spieler am Morgen und Mauerkosten.', form: { seeds: '10', label: 'Sweep Morgengold×Mauer', tune: { mode: 'sweep', params: 'economy.json:dawnGoldPerPlayer=1..6\nbuildings.json:wall.cost.gold=2..6', steps: '3' } } },
  { name: 'Startgold × Mauer (Sweep)', text: 'Raster 3 × 3 über Startgold und Mauerkosten.', form: { seeds: '10', label: 'Sweep Gold×Mauer', tune: { mode: 'sweep', params: 'economy.json:purse.startGold=60..140\nbuildings.json:wall.cost.gold=2..6', steps: '3' } } },
];

/** Beispiel → Formular: leeres Formular mit den Werten des Beispiels (alte Eingaben verschwinden). */
export function formFromExample(e: Example, empty: BalanceForm): BalanceForm {
  return { ...empty, ...e.form, tune: { ...EMPTY_TUNE, ...e.form.tune } };
}

/** Zeile anhängen (ohne doppelte Zeile), z. B. Variante oder Parameter aus der Datei-Liste. */
export function appendLine(text: string, line: string): string {
  const lines = text.split('\n').map((l) => l.trim()).filter(Boolean);
  return lines.includes(line) ? lines.join('\n') : [...lines, line].join('\n');
}

/** Bereich ±25 % um einen Wert für einen Abstimm-Parameter; ganze Zahlen bleiben ganz, 0 ergibt 0..1. */
export function rangeAround(v: number): string {
  if (v === 0) return '0..1';
  const round = (x: number) => (Number.isInteger(v) ? Math.round(x) : Number(x.toFixed(3)));
  let [lo, hi] = [round(v * 0.75), round(v * 1.25)].sort((a, b) => a - b);
  if (lo === hi) hi = lo + 1;
  return `${lo}..${hi}`;
}
