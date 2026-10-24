import { audioCore } from '@game/audio/audioCore';
import { playPlan, type PlayChoice, type PlaySlot } from './playback';

// Audio-Baustein des Studios: ein Audio-Kern für das ganze Fenster (audioCore() ist ein Einzelstück), damit ein
// Musikwechsel überblendet statt doppelt zu spielen. Jeder Aufruf kommt aus einem Klick oder einer Taste; onInput
// entsperrt den Kontext dabei (WebView2 verlangt wie Edge eine Nutzergeste).

export const core = () => audioCore();

/** Der Audio-Atlas lädt erst nach dem ersten Entsperren (onInput); so lange wiederholt ein Atlas-Ton, höchstens so lange. */
const ATLAS_WAIT_MS = 3000;
const ATLAS_STEP_MS = 100;
/** Zählt jedes Stopp hoch; eine noch wartende Wiederholung endet dann, statt den Ton später doch zu spielen. */
let generation = 0;

async function playSprite(name: string, bus: 'sfx', gain: number): Promise<boolean> {
  const c = core();
  const gen = generation;
  let ok = c.play(name, bus, gain);
  for (let t = 0; !ok && t < ATLAS_WAIT_MS && gen === generation; t += ATLAS_STEP_MS) {
    await new Promise((r) => setTimeout(r, ATLAS_STEP_MS));
    if (gen === generation) ok = c.play(name, bus, gain);
  }
  return ok;
}

/** Spielt eine Zuordnung nach playPlan; false, wenn gesperrt, Datei fehlt oder Ton unbekannt. */
export async function playChoice(slot: PlaySlot, choice: PlayChoice, far = false): Promise<boolean> {
  const c = core();
  await c.onInput();
  if (!c.running) return false;
  const plan = playPlan(slot, choice, far);
  if (plan.kind === 'sprite') return playSprite(plan.name, plan.bus, plan.gain);
  return c.playFile(plan.url, plan.opts);
}

/** Spielt eine Zuordnung mit festem Pegel auf dem Bus „Effekte“ (Abstandsprobe, Lautheit); Atlas-Ton oder vorgeladene Datei. */
export async function playAtGain(choice: PlayChoice, gain: number): Promise<boolean> {
  const c = core();
  await c.onInput();
  if (!c.running) return false;
  if (choice.sprite) return playSprite(choice.sprite, 'sfx', gain);
  return (await c.preload(choice.datei)) && c.playSample(choice.datei, 'sfx', gain);
}

/** Stoppt Musik, Effekte und Umgebungsgeräusche (Knopf „Stopp“, Verlassen der Seite Sound). */
export function stopAll(): void {
  generation += 1;
  const c = core();
  c.stop('music');
  c.stop('sfx');
  c.stop('ambient');
}
