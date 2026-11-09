import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';
import { clientLog, installClientLog } from '../core/clientLog';
import { installPadNav } from '../core/padNav';
import { goHome, installPageChrome } from '../core/shell';
import { CLIENT, formatBuild } from '../core/version';
import { antialiasOff, rendererRequest } from '../core/renderInfo';
import { readGameRenderInfo } from '../core/gameRenderInfo';
import { installRenderLog } from '../core/renderLogInstall';
import { applyTexts, t } from '../tools/texts';
import stressJson from '../../data/stress.json';
import { StressScene } from './StressScene';
import { KNOWN } from './known';
import { deviceTag, sendStressReport } from './stressReport';
import { hashText, parseScenarios, stageCount, withScenery, type Scenario } from './stressPlan';
import { StressRun, type RunResult } from './stressRun';
import { runSearch } from '../core/runConfig';

installPageChrome({ padNav: false });
installPadNav(() => !el('menu').hidden, goHome); // das Menü bedient der Controller, während des Laufs nicht; B = zurück ins Spielmenü
installClientLog();
applyTexts();

const el = <T extends HTMLElement>(id: string): T => document.getElementById(id) as T;

let game: Phaser.Game | null = null;
/** Beendet den laufenden Lauf mit Endzeile (Zurück zum Menü) */
let abort: (() => void) | null = null;
let ramp = new URLSearchParams(runSearch()).get('ramp') !== '0';

function stop(): void {
  abort?.();
  abort = null;
  game?.destroy(true);
  game = null;
  el('back').hidden = true;
  el('menu').hidden = false;
}

function launch(id: string, base: Scenario, players: 1 | 2): void {
  const scenario = withScenery(base, runSearch());
  const params = new URLSearchParams(runSearch());
  const seed = Number(params.get('seed') ?? '1') || 1;
  const startedAt = new Date().toISOString();
  const runId = `${deviceTag()}-${id}-p${players}-s${seed}-${Date.now().toString(36)}`;
  const want = rendererRequest(runSearch());
  el('menu').hidden = true;
  el('back').hidden = false;
  const g = new Phaser.Game({
    type: want === 'webgl' ? Phaser.WEBGL : want === 'canvas' ? Phaser.CANVAS : Phaser.AUTO,
    render: { powerPreference: 'high-performance', antialiasGL: !antialiasOff(runSearch()) },
    parent: 'game',
    width: GAME_WIDTH,
    height: GAME_HEIGHT,
    backgroundColor: '#000000',
    scale: { mode: Phaser.Scale.FIT, autoCenter: Phaser.Scale.CENTER_BOTH },
    input: { gamepad: true },
    scene: [],
  });
  game = g;
  if (import.meta.env.DEV) (window as unknown as { game: Phaser.Game }).game = g; // Zugriff für Debugging über die Browser-Konsole
  const version = formatBuild(CLIENT);
  const renderLog = installRenderLog(g, version);
  const run = new StressRun(
    { runId, scenario: id, players, seed, fileHash: hashText(JSON.stringify(stressJson)), params: runSearch(), version, userAgent: navigator.userAgent, stages: stageCount(scenario, ramp), everyMs: scenario.ramp.everyMs },
    { log: clientLog, info: () => readGameRenderInfo(g), heapMiB: () => ((performance as unknown as { memory?: { usedJSHeapSize: number } }).memory?.usedJSHeapSize ?? NaN) / (1024 * 1024) || null },
  );
  const onEnd = (r: RunResult): void => {
    abort = null;
    sendStressReport({ runId, scenario: id, players, seed, startedAt, mode: 'canvas' }, r);
  };
  abort = () => onEnd(run.finish('aborted'));
  g.scene.add('stress', StressScene, true, { scenario, players, ramp, seed, known: KNOWN, run, renderLog, version, onEnd });
}

function menu(scenarios: Record<string, Scenario>): void {
  const box = el('buttons');
  const rampBtn = document.createElement('button');
  rampBtn.type = 'button';
  const label = (): void => void (rampBtn.textContent = t(ramp ? 'stress.rampOn' : 'stress.rampOff'));
  label();
  rampBtn.addEventListener('click', () => {
    ramp = !ramp;
    label();
  });
  box.append(rampBtn);
  for (const [id, s] of Object.entries(scenarios)) {
    for (const players of [1, 2] as const) {
      const b = document.createElement('button');
      b.type = 'button';
      b.textContent = t('stress.btn', { name: s.name, players });
      b.addEventListener('click', () => launch(id, s, players));
      box.append(b);
    }
  }
  box.querySelector('button')?.focus();
}

function boot(): void {
  el('back').addEventListener('click', stop);
  let scenarios: Record<string, Scenario>;
  try {
    scenarios = parseScenarios(stressJson as Record<string, unknown>, KNOWN);
  } catch (e) {
    const error = e instanceof Error ? e.message : String(e);
    el('error').textContent = t('stress.error', { error });
    clientLog('error', `🚫 Szenario-Datei ungültig: ${error}`);
    return;
  }
  menu(scenarios);
  // Mit ?scenario=<id> (und optional ?players=1|2) startet der Lauf sofort, auch ohne Bedienung am TV
  const params = new URLSearchParams(runSearch());
  const id = params.get('scenario');
  const s = id ? scenarios[id] : undefined;
  if (id && s) {
    const p = Number(params.get('players') ?? s.players);
    launch(id, s, p === 2 ? 2 : 1);
  } else if (id) {
    el('error').textContent = t('stress.error', { error: id });
  }
}

boot();
