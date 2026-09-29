import Phaser from 'phaser';
import { toggleFullscreen as requestFullscreenToggle } from '../core/fullscreen';
import { installPageChrome } from '../core/shell';

/**
 * Gamepad-Testseite für Edge auf der Xbox (Schritt 0 der Roadmap).
 * Sammelt alles in `report` und schickt es per POST /api/report an den Heimnetz-Server (-> reports/*.json).
 */

const BUTTON_NAMES = ['A', 'B', 'X', 'Y', 'LB', 'RB', 'LT', 'RT', 'View', 'Menu', 'LS', 'RS', '↑', '↓', '←', '→', 'Xbox'];
const AXIS_NAMES = ['LS X', 'LS Y', 'RS X', 'RS Y'];
const BTN = { Y: 3, X: 2, VIEW: 8, MENU: 9 } as const;
const PERF_STEPS = [100, 300, 600, 1000, 2000, 4000];
const PERF_STEP_SECONDS = 4;

interface PadReport {
  id: string;
  mapping: string;
  buttonCount: number;
  axisCount: number;
  buttonsSeen: number[];
  maxAbsAxes: number[];
  vibration: boolean;
}

interface PerfResult {
  sprites: number;
  avgFps: number;
  minFps: number;
}

const nav = navigator as Navigator & { gamepadInputEmulation?: string };

const report = {
  version: 1,
  createdAt: new Date().toISOString(),
  userAgent: navigator.userAgent,
  isSecureContext: window.isSecureContext,
  location: location.href,
  gamepadApi: typeof navigator.getGamepads === 'function',
  gamepadInputEmulation: undefined as string | undefined,
  screen: `${screen.width}x${screen.height}`,
  viewport: `${innerWidth}x${innerHeight}`,
  devicePixelRatio: devicePixelRatio,
  webgl: (() => {
    try {
      return !!document.createElement('canvas').getContext('webgl2') ? 'webgl2' : !!document.createElement('canvas').getContext('webgl') ? 'webgl' : 'none';
    } catch {
      return 'error';
    }
  })(),
  fullscreenApi: typeof document.documentElement.requestFullscreen === 'function',
  fullscreenAttempts: [] as { via: string; ok: boolean; error?: string }[],
  pads: {} as Record<number, PadReport>,
  maxSimultaneousPads: 0,
  backNavigations: 0,
  keyEvents: [] as string[],
  visibilityChanges: 0,
  rumble: [] as { pad: number; ok: boolean; error?: string }[],
  perf: [] as PerfResult[],
  perfRenderer: '',
  log: [] as string[],
};

// Altes Edge (UWP) hatte diesen Schalter, um die Maus-Emulation des Controllers abzuschalten. Schaden kann es nicht.
if ('gamepadInputEmulation' in nav) {
  try {
    nav.gamepadInputEmulation = 'gamepad';
  } catch {
    /* ignorieren */
  }
  report.gamepadInputEmulation = nav.gamepadInputEmulation;
}

const $ = <T extends HTMLElement>(id: string) => document.getElementById(id) as T;
const logEl = $('log');
const padsEl = $('pads');
const statusEl = $('report-status');

function log(message: string): void {
  const line = `${new Date().toLocaleTimeString()}  ${message}`;
  report.log.push(line);
  if (report.log.length > 300) report.log.shift();
  const div = document.createElement('div');
  div.textContent = line;
  logEl.prepend(div);
}

function renderEnv(): void {
  const rows: [string, string, boolean?][] = [
    ['Secure Context', report.isSecureContext ? 'ja (HTTPS/localhost)' : 'nein (HTTP)', report.isSecureContext],
    ['Gamepad API', report.gamepadApi ? 'vorhanden' : 'fehlt', report.gamepadApi],
    ['Fullscreen API', report.fullscreenApi ? 'vorhanden' : 'fehlt', report.fullscreenApi],
    ['WebGL', report.webgl, report.webgl.startsWith('webgl')],
    ['Bildschirm', `${report.screen} · Fenster ${innerWidth}x${innerHeight} · DPR ${devicePixelRatio}`],
    ['Zurück-Navigationen', String(report.backNavigations), report.backNavigations === 0],
    ['Browser', report.userAgent],
  ];
  if (report.gamepadInputEmulation !== undefined) rows.splice(3, 0, ['gamepadInputEmulation', report.gamepadInputEmulation]);
  $('env').innerHTML = rows
    .map(([k, v, ok]) => `<dt>${k}</dt><dd class="${ok === undefined ? '' : ok ? 'ok' : 'bad'}">${escapeHtml(v)}</dd>`)
    .join('');
}

function escapeHtml(s: string): string {
  return s.replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c]!);
}

addEventListener('keydown', (e) => {
  const entry = `key="${e.key}" code="${e.code}" keyCode=${e.keyCode}`;
  if (!report.keyEvents.includes(entry)) report.keyEvents.push(entry);
  log(`Taste: ${entry}`);
});
document.addEventListener('visibilitychange', () => {
  report.visibilityChanges++;
  log(`Sichtbarkeit: ${document.visibilityState}`);
});
addEventListener('gamepadconnected', (e) => log(`🎮 verbunden: #${e.gamepad.index} ${e.gamepad.id}`));
addEventListener('gamepaddisconnected', (e) => log(`🎮 getrennt: #${e.gamepad.index} ${e.gamepad.id}`));
document.addEventListener('fullscreenchange', () => log(`Vollbild: ${document.fullscreenElement ? 'an' : 'aus'}`));

// ---------- Aktionen ----------
async function toggleFullscreen(via: string): Promise<void> {
  // Läuft über die Shell (Landingpage), damit Vollbild auch beim Seitenwechsel erhalten bleibt.
  const error = await requestFullscreenToggle();
  report.fullscreenAttempts.push(error ? { via, ok: false, error } : { via, ok: true });
  log(error ? `Vollbild über ${via} fehlgeschlagen: ${error}` : `Vollbild über ${via}: ok`);
}

async function rumble(): Promise<void> {
  const pads = navigator.getGamepads().filter((p): p is Gamepad => !!p);
  if (pads.length === 0) return log('Vibration: kein Controller');
  for (const pad of pads) {
    const actuator = (pad as Gamepad & { vibrationActuator?: GamepadHapticActuator }).vibrationActuator;
    try {
      if (!actuator) throw new Error('kein vibrationActuator');
      await actuator.playEffect('dual-rumble', { duration: 400, strongMagnitude: 1, weakMagnitude: 0.6 });
      report.rumble.push({ pad: pad.index, ok: true });
      log(`Vibration #${pad.index}: ok`);
    } catch (err) {
      report.rumble.push({ pad: pad.index, ok: false, error: String(err) });
      log(`Vibration #${pad.index}: ${String(err)}`);
    }
  }
}

async function sendReport(): Promise<void> {
  report.viewport = `${innerWidth}x${innerHeight}`;
  statusEl.textContent = 'Sende …';
  try {
    const res = await fetch('/api/report', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(report) });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    statusEl.textContent = `✔ Bericht gespeichert (${new Date().toLocaleTimeString()})`;
    statusEl.className = 'sub ok';
    log('Bericht gesendet');
  } catch (err) {
    statusEl.textContent = `✘ Senden fehlgeschlagen: ${String(err)}`;
    statusEl.className = 'sub bad';
  }
}

$('btn-fullscreen').addEventListener('click', () => toggleFullscreen('klick'));
$('btn-rumble').addEventListener('click', rumble);
$('btn-report').addEventListener('click', sendReport);
$('btn-perf').addEventListener('click', () => togglePerf());

// ---------- Controller-Anzeige ----------
const previousPressed = new Map<number, Set<number>>();
const comboUsed = new Set<number>();

function pollPads(): void {
  const pads = navigator.getGamepads().filter((p): p is Gamepad => !!p);
  report.maxSimultaneousPads = Math.max(report.maxSimultaneousPads, pads.length);

  for (const pad of pads) {
    const entry = (report.pads[pad.index] ??= {
      id: pad.id,
      mapping: pad.mapping || '(leer)',
      buttonCount: pad.buttons.length,
      axisCount: pad.axes.length,
      buttonsSeen: [],
      maxAbsAxes: pad.axes.map(() => 0),
      vibration: 'vibrationActuator' in pad && !!(pad as Gamepad & { vibrationActuator?: unknown }).vibrationActuator,
    });
    const pressed = new Set<number>();
    pad.buttons.forEach((b, i) => {
      if (!b.pressed) return;
      pressed.add(i);
      if (!entry.buttonsSeen.includes(i)) {
        entry.buttonsSeen.push(i);
        entry.buttonsSeen.sort((a, b) => a - b);
      }
    });
    pad.axes.forEach((v, i) => (entry.maxAbsAxes[i] = Math.max(entry.maxAbsAxes[i] ?? 0, Math.round(Math.abs(v) * 100) / 100)));

    const before = previousPressed.get(pad.index) ?? new Set<number>();
    const justPressed = (i: number) => pressed.has(i) && !before.has(i);
    // View/Menu lösen erst beim Loslassen aus, und nur wenn sie nicht Teil der Kombi View+Menu (= zur Startseite) waren.
    if (pressed.has(BTN.VIEW) && pressed.has(BTN.MENU)) comboUsed.add(pad.index);
    const released = (i: number) => !pressed.has(i) && before.has(i) && !comboUsed.has(pad.index);
    for (const i of pressed) if (!before.has(i)) log(`#${pad.index} ${BUTTON_NAMES[i] ?? `Taste ${i}`} gedrückt`);

    if (perfGame) {
      if (released(BTN.VIEW)) togglePerf();
    } else {
      if (justPressed(BTN.Y)) void sendReport();
      if (justPressed(BTN.X)) void rumble();
      if (released(BTN.VIEW)) togglePerf();
      if (released(BTN.MENU)) void toggleFullscreen('controller-menu');
    }
    if (!pressed.has(BTN.VIEW) && !pressed.has(BTN.MENU)) comboUsed.delete(pad.index);
    previousPressed.set(pad.index, pressed);
  }

  if (!perfGame) renderPads(pads);
  requestAnimationFrame(pollPads);
}

function renderPads(pads: Gamepad[]): void {
  if (pads.length === 0) return;
  padsEl.innerHTML = pads
    .map((pad) => {
      const seen = report.pads[pad.index]?.buttonsSeen ?? [];
      const buttons = pad.buttons
        .map((b, i) => `<span class="btn ${b.pressed ? 'on' : ''} ${seen.includes(i) ? 'seen' : ''}">${BUTTON_NAMES[i] ?? i}${b.value > 0 && b.value < 1 ? ` ${b.value.toFixed(1)}` : ''}</span>`)
        .join('');
      const axes = pad.axes
        .map((v, i) => {
          const left = v < 0 ? 50 + v * 50 : 50;
          return `<span>${AXIS_NAMES[i] ?? `Achse ${i}`}</span><div class="bar"><i style="left:${left}%;width:${Math.abs(v) * 50}%"></i></div><span>${v.toFixed(2)}</span>`;
        })
        .join('');
      return `<div class="pad"><div class="pad-title">#${pad.index} · ${escapeHtml(pad.id)} · mapping: ${pad.mapping || '(leer)'}</div><div class="buttons">${buttons}</div><div class="axes">${axes}</div></div>`;
    })
    .join('');
}

// ---------- FPS-Test (Phaser, WebGL, wie das echte Spiel) ----------
let perfGame: Phaser.Game | null = null;
const perfEl = $('perf');
const perfHud = $('perf-hud');

function togglePerf(): void {
  if (perfGame) {
    perfGame.destroy(true);
    perfGame = null;
    perfEl.classList.remove('active');
    log(`FPS-Test beendet: ${report.perf.map((p) => `${p.sprites}→${p.avgFps}`).join(', ') || 'keine Werte'}`);
    return;
  }
  perfEl.classList.add('active');
  perfGame = new Phaser.Game({
    type: Phaser.AUTO,
    parent: perfEl,
    width: 1920,
    height: 1080,
    backgroundColor: '#223344',
    scale: { mode: Phaser.Scale.FIT, autoCenter: Phaser.Scale.CENTER_BOTH },
    scene: PerfScene,
  });
}

class PerfScene extends Phaser.Scene {
  private sprites: { s: Phaser.GameObjects.Image; vx: number; vy: number }[] = [];
  private stepIndex = 0;
  private stepStart = 0;
  private samples: number[] = [];
  private lastSample = 0;

  create(): void {
    report.perfRenderer = this.game.renderer.type === Phaser.WEBGL ? 'webgl' : 'canvas';
    const g = this.add.graphics();
    g.fillStyle(0xffd166).fillCircle(16, 16, 16).lineStyle(3, 0x000000).strokeCircle(16, 16, 15);
    g.generateTexture('ball', 32, 32).destroy();
    report.perf = [];
    this.startStep(0);
  }

  private startStep(index: number): void {
    this.stepIndex = index;
    this.stepStart = this.time.now;
    this.samples = [];
    const target = PERF_STEPS[index];
    while (this.sprites.length < target) {
      const s = this.add.image(Phaser.Math.Between(0, 1920), Phaser.Math.Between(0, 1080), 'ball');
      this.sprites.push({ s, vx: Phaser.Math.Between(-300, 300), vy: Phaser.Math.Between(-300, 300) });
    }
  }

  update(time: number, delta: number): void {
    const dt = delta / 1000;
    for (const p of this.sprites) {
      p.s.x += p.vx * dt;
      p.s.y += p.vy * dt;
      if (p.s.x < 0 || p.s.x > 1920) p.vx *= -1;
      if (p.s.y < 0 || p.s.y > 1080) p.vy *= -1;
      p.s.rotation += dt;
    }

    // Erste Sekunde pro Stufe verwerfen (Einschwingen), dann 4x pro Sekunde messen.
    if (time - this.stepStart > 1000 && time - this.lastSample > 250) {
      this.samples.push(this.game.loop.actualFps);
      this.lastSample = time;
    }

    const done = this.stepIndex >= PERF_STEPS.length;
    perfHud.textContent = done
      ? `Fertig · ${report.perf.map((p) => `${p.sprites}: ${p.avgFps} FPS`).join(' · ')} · View = zurück`
      : `${this.sprites.length} Sprites · ${Math.round(this.game.loop.actualFps)} FPS · Stufe ${this.stepIndex + 1}/${PERF_STEPS.length} · View = abbrechen`;

    if (!done && time - this.stepStart > (PERF_STEP_SECONDS + 1) * 1000) {
      const avg = this.samples.reduce((a, b) => a + b, 0) / Math.max(1, this.samples.length);
      report.perf.push({ sprites: this.sprites.length, avgFps: Math.round(avg), minFps: Math.round(Math.min(...this.samples)) });
      if (this.stepIndex + 1 < PERF_STEPS.length) this.startStep(this.stepIndex + 1);
      else this.stepIndex = PERF_STEPS.length;
    }
  }
}

// Zurück-Navigation (B-Taste auf der Xbox?) fängt installPageChrome ab – hier nur zählen.
installPageChrome({
  onBack: () => {
    report.backNavigations++;
    log('⚠ Zurück-Navigation ausgelöst (B-Taste?)');
    renderEnv();
  },
});
renderEnv();
log(report.gamepadApi ? 'Bereit. Taste auf einem Controller drücken.' : 'Gamepad API nicht verfügbar!');
requestAnimationFrame(pollPads);

if (import.meta.env.DEV) (window as unknown as { k3cReport: typeof report }).k3cReport = report;
