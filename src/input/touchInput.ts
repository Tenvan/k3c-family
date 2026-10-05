import { toggleFullscreen } from '../core/fullscreen';
import type { Action, PlayerInput } from './playerInput';
import { SLOT_ACTIONS, slotBindings, type SlotAction } from './slotBindings';

type TouchKey = 'left' | 'right' | 'sprint' | 'confirm' | 'fullscreen' | SlotAction;
/** Aktionen, die per Bildschirmtaste ausgelöst werden (Laufen und Sprint fragt `PlayerInput` eigens ab) */
const TOUCH_ACTIONS: readonly Action[] = ['confirm', ...SLOT_ACTIONS];

/** Touch-Steuerung nötig? Handy/Tablet, oder per ?touch=1 erzwingbar (zum Testen am PC). */
export function wantsTouchControls(): boolean {
  if (new URLSearchParams(window.location.search).has('touch')) return true;
  return navigator.maxTouchPoints > 0 && window.matchMedia('(pointer: coarse)').matches;
}

const CSS = `
  .k3c-zone { position: fixed; inset: 0; z-index: 9; touch-action: none; user-select: none; -webkit-user-select: none; -webkit-tap-highlight-color: transparent; }
  .k3c-zone i { position: absolute; top: 50%; translate: 0 -50%; font: normal 700 12vmin/1 system-ui, sans-serif; color: #fff; opacity: .18; transition: opacity 80ms; pointer-events: none; }
  .k3c-zone i.l { left: 4vmin; } .k3c-zone i.r { right: 4vmin; }
  .k3c-zone.left i.l, .k3c-zone.right i.r { opacity: .5; }
  .k3c-touch { position: fixed; inset: auto 0 0 0; z-index: 10; display: flex; justify-content: flex-end; align-items: flex-end;
    padding: 0 max(2vmin, env(safe-area-inset-right)) max(2vmin, env(safe-area-inset-bottom)) max(2vmin, env(safe-area-inset-left));
    pointer-events: none; touch-action: none; user-select: none; -webkit-user-select: none; -webkit-touch-callout: none; }
  .k3c-touch .grp { display: flex; gap: 2vmin; align-items: flex-end; }
  .k3c-touch button { pointer-events: auto; touch-action: none; font: 700 calc(var(--s) * 0.4)/1 system-ui, sans-serif; color: #f1f3f9;
    width: var(--s); height: var(--s); border-radius: 50%; border: max(2px, 0.4vmin) solid rgba(255,255,255,.35);
    background: rgba(12,16,36,.55); backdrop-filter: blur(4px); -webkit-tap-highlight-color: transparent; }
  .k3c-touch button.down { background: rgba(255,209,102,.55); border-color: #ffd166; }
  .k3c-touch { --s: 16vmin; }
  .k3c-touch .a { --s: 22vmin; background: rgba(63,185,80,.5); }
  .k3c-touch .small { --s: 10vmin; }
  .k3c-touch .col { display: flex; flex-direction: column; gap: 1.5vmin; align-items: center; }
  .k3c-touch .skills { display: grid; grid-template-columns: repeat(3, auto); gap: 1.5vmin; }
`;

const SKILL_ARIA: Record<string, string> = { skill1: 'Skill 1', skill2: 'Skill 2', skill3: 'Skill 3', skill4: 'Skill 4', skillMenu: 'Skill-Menü' };

/** Kleine Tasten für Skill-Slot 1–4 und das Skill-Menü (Belegung aus `slotBindings`) */
function skillButtons(): string {
  return slotBindings('touch')
    .filter((b) => b.action !== 'attack')
    .map((b) => `<button class="small" data-k="${b.key}" aria-label="${SKILL_ARIA[b.action]}">${b.label}</button>`)
    .join('');
}

/**
 * Eingabe per Touch: linke Bildschirmhälfte berühren = nach links laufen, rechte = nach rechts.
 * Dazu Bildschirmtasten für Münz-Taste (= A), Schlag, Skill-Slots 1–4, Skill-Menü und » sprinten. Die Tasten liegen über der Lauf-Fläche.
 * DOM-Overlay statt Phaser, damit mehrere Finger gleichzeitig funktionieren (laufen + Münzen geben).
 */
export class TouchInput implements PlayerInput {
  readonly label = 'Touch';
  private readonly down = new Set<TouchKey>();
  private queued = new Set<TouchKey>();
  private pressed = new Set<TouchKey>();

  constructor(parent: HTMLElement = document.body) {
    const style = document.createElement('style');
    style.textContent = CSS;
    const root = document.createElement('div');
    root.className = 'k3c-touch';
    root.innerHTML = `
      <div class="grp">
        <div class="col"><button class="small" data-k="fullscreen" aria-label="Vollbild">⛶</button><button class="small" data-k="sprint" aria-label="Sprinten">»</button></div>
        <div class="skills">${skillButtons()}</div>
        <button data-k="attack" aria-label="Schlag">${slotBindings('touch')[0]?.label ?? ''}</button>
        <button class="a" data-k="confirm" aria-label="Münzen geben / beitreten">🪙</button>
      </div>`;
    const zone = document.createElement('div');
    zone.className = 'k3c-zone';
    zone.innerHTML = '<i class="l">◀</i><i class="r">▶</i>';
    this.bindZone(zone);
    document.head.append(style);
    parent.append(zone, root);
    root.addEventListener('contextmenu', (e) => e.preventDefault());

    for (const button of root.querySelectorAll<HTMLButtonElement>('button')) {
      const key = button.dataset.k as TouchKey;
      const pointers = new Set<number>();
      const sync = () => {
        button.classList.toggle('down', pointers.size > 0);
        if (pointers.size > 0) this.down.add(key);
        else this.down.delete(key);
      };
      button.addEventListener('pointerdown', (e) => {
        e.preventDefault();
        button.setPointerCapture(e.pointerId);
        if (pointers.size === 0) this.queued.add(key);
        pointers.add(e.pointerId);
        sync();
      });
      const release = (e: PointerEvent) => {
        pointers.delete(e.pointerId);
        sync();
      };
      button.addEventListener('pointerup', release);
      button.addEventListener('pointercancel', release);
      button.addEventListener('lostpointercapture', release);
    }
  }

  /** Jeder Finger auf der Lauf-Fläche läuft in Richtung seiner Bildschirmhälfte (auch beim Wischen über die Mitte). */
  private bindZone(zone: HTMLElement): void {
    const fingers = new Map<number, 'left' | 'right'>();
    const sync = () => {
      const dirs = new Set(fingers.values());
      for (const dir of ['left', 'right'] as const) {
        if (dirs.has(dir)) this.down.add(dir);
        else this.down.delete(dir);
        zone.classList.toggle(dir, dirs.has(dir));
      }
    };
    const side = (e: PointerEvent) => (e.clientX < window.innerWidth / 2 ? 'left' : 'right');
    zone.addEventListener('pointerdown', (e) => {
      e.preventDefault();
      try {
        zone.setPointerCapture(e.pointerId);
      } catch {
        /* Zeiger schon weg */
      }
      fingers.set(e.pointerId, side(e));
      sync();
    });
    zone.addEventListener('pointermove', (e) => {
      if (!fingers.has(e.pointerId)) return;
      fingers.set(e.pointerId, side(e));
      sync();
    });
    const release = (e: PointerEvent) => {
      fingers.delete(e.pointerId);
      sync();
    };
    zone.addEventListener('pointerup', release);
    zone.addEventListener('pointercancel', release);
    zone.addEventListener('lostpointercapture', release);
    zone.addEventListener('contextmenu', (e) => e.preventDefault());
  }

  update(): void {
    this.pressed = this.queued;
    this.queued = new Set();
    if (this.pressed.has('fullscreen')) void toggleFullscreen();
  }

  moveX(): number {
    return (this.down.has('right') ? 1 : 0) - (this.down.has('left') ? 1 : 0);
  }

  sprint(): boolean {
    return this.down.has('sprint');
  }

  justPressed(action: Action): boolean {
    return TOUCH_ACTIONS.includes(action) && this.pressed.has(action as TouchKey);
  }

  held(action: Action): boolean {
    return TOUCH_ACTIONS.includes(action) && this.down.has(action as TouchKey);
  }
}
