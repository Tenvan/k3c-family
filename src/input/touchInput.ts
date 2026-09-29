import { toggleFullscreen } from '../core/fullscreen';
import type { Action, PlayerInput } from './playerInput';

type TouchKey = 'left' | 'right' | 'sprint' | 'confirm' | 'fullscreen';

/** Touch-Steuerung nötig? Handy/Tablet, oder per ?touch=1 erzwingbar (zum Testen am PC). */
export function wantsTouchControls(): boolean {
  if (new URLSearchParams(window.location.search).has('touch')) return true;
  return navigator.maxTouchPoints > 0 && window.matchMedia('(pointer: coarse)').matches;
}

const CSS = `
  .k3c-touch { position: fixed; inset: auto 0 0 0; z-index: 10; display: flex; justify-content: space-between; align-items: flex-end;
    padding: 0 max(16px, env(safe-area-inset-right)) max(16px, env(safe-area-inset-bottom)) max(16px, env(safe-area-inset-left));
    pointer-events: none; touch-action: none; user-select: none; -webkit-user-select: none; -webkit-touch-callout: none; }
  .k3c-touch .grp { display: flex; gap: 14px; align-items: flex-end; }
  .k3c-touch button { pointer-events: auto; touch-action: none; font: 700 30px/1 system-ui, sans-serif; color: #f1f3f9;
    width: var(--s); height: var(--s); border-radius: 50%; border: 3px solid rgba(255,255,255,.35);
    background: rgba(12,16,36,.55); backdrop-filter: blur(4px); -webkit-tap-highlight-color: transparent; }
  .k3c-touch button.down { background: rgba(255,209,102,.55); border-color: #ffd166; }
  .k3c-touch { --s: clamp(64px, 16vmin, 104px); }
  .k3c-touch .a { --s: clamp(84px, 22vmin, 132px); background: rgba(63,185,80,.5); }
  .k3c-touch .small { --s: clamp(44px, 10vmin, 60px); font-size: 20px; }
  .k3c-touch .col { display: flex; flex-direction: column; gap: 12px; align-items: center; }
`;

/**
 * Eingabe über Bildschirmtasten (◀ ▶ laufen, Münz-Taste = A, » sprinten).
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
      <div class="grp"><button data-k="left" aria-label="Links">◀</button><button data-k="right" aria-label="Rechts">▶</button></div>
      <div class="grp">
        <div class="col"><button class="small" data-k="fullscreen" aria-label="Vollbild">⛶</button><button class="small" data-k="sprint" aria-label="Sprinten">»</button></div>
        <button class="a" data-k="confirm" aria-label="Münzen geben / beitreten">🪙</button>
      </div>`;
    document.head.append(style);
    parent.append(root);
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
    return action === 'confirm' && this.pressed.has('confirm');
  }

  held(action: Action): boolean {
    return action === 'confirm' && this.down.has('confirm');
  }
}
