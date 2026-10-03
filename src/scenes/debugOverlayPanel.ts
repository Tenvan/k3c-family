import type { SlotCommand } from './localSlots';
import { DEV_ACTIONS } from './debugActions';

/**
 * Bedienung der Dev-Aktionen im Debug-Overlay (B-179): DOM-Schaltflächen für Maus und Touch, Fokus für den Controller.
 * Controller: RB schaltet den Dev-Fokus (nur bei sichtbarer Liste), im Fokus D-Pad hoch/runter = Aktion, links/rechts = Spieler,
 * A = auslösen. B, View + Menu und X bleiben frei.
 */

/** Tasten der Fokus-Bedienung (standard mapping) */
export const PAD_FOCUS = { A: 0, RB: 5, UP: 12, DOWN: 13, LEFT: 14, RIGHT: 15 } as const;

export interface FocusState {
  focus: boolean;
  /** gewählte Zeile in `DEV_ACTIONS` */
  index: number;
  /** gewählter lokaler Spieler (Index in den lokalen Slots) */
  seat: number;
}

/** Tasten, die in diesem Frame neu gedrückt wurden */
export type FocusEdges = Partial<Record<keyof typeof PAD_FOCUS, boolean>>;

const wrap = (i: number, n: number): number => (n <= 0 ? 0 : ((i % n) + n) % n);

/** Nächster Fokus-Zustand; `fire` = Zeile, die ausgelöst wird. Ohne sichtbare Liste ist der Fokus aus. */
export function focusStep(s: FocusState, e: FocusEdges, visible: boolean, seats: number): { state: FocusState; fire: number | null } {
  if (!visible) return { state: { ...s, focus: false }, fire: null };
  const focus = e.RB ? !s.focus : s.focus;
  if (!focus) return { state: { ...s, focus }, fire: null };
  const n = DEV_ACTIONS.length;
  const index = wrap(s.index + (e.DOWN ? 1 : 0) - (e.UP ? 1 : 0), n);
  const seat = wrap(s.seat + (e.RIGHT ? 1 : 0) - (e.LEFT ? 1 : 0), seats);
  return { state: { focus, index, seat }, fire: e.A ? index : null };
}

/** Im Dev-Fokus stehen die Spieler am Controller still (keine Bewegung, kein Sprint, keine Münzen). */
export function muteFocused(commands: SlotCommand[], focus: boolean, isPad: (slot: number) => boolean): SlotCommand[] {
  if (!focus) return commands;
  return commands.map((c) => (isPad(c.slot) ? { slot: c.slot, moveX: 0, sprint: false, pay: false } : c));
}

/** Registry-Schlüssel: Dev-Fokus aktiv (GameScene sperrt dann die Controller-Spieler) */
export const DEV_FOCUS_KEY = 'devFocus';

const PANEL_CSS = `
.k3c-dev{position:fixed;right:12px;top:72px;z-index:11;display:flex;flex-direction:column;gap:6px;padding:10px;
  background:rgba(0,0,0,.72);border:2px solid #9be564;border-radius:10px;font:bold 15px system-ui,sans-serif;color:#9be564}
.k3c-dev .row{display:flex;gap:6px;flex-wrap:wrap}
.k3c-dev button{font:inherit;color:#fff;background:#2b3558;border:2px solid transparent;border-radius:8px;padding:6px 10px;touch-action:manipulation}
.k3c-dev button.sel{border-color:#ffd166}
.k3c-dev.focus{border-color:#ffd166}`;

/** DOM-Schaltflächen über Canvas und Touch-Overlay (z-index 11 > 10); `onAction(index)` bzw. `onSeat()` beim Antippen. */
export class DevActionPanel {
  private readonly root = document.createElement('div');
  private readonly style = document.createElement('style');
  private readonly title = document.createElement('div');
  private readonly seatButton = document.createElement('button');
  private readonly buttons: HTMLButtonElement[] = [];

  constructor(onAction: (index: number) => void, onSeat: () => void) {
    this.style.textContent = PANEL_CSS;
    this.root.className = 'k3c-dev';
    this.seatButton.addEventListener('pointerdown', (ev) => (ev.preventDefault(), onSeat()));
    this.root.append(this.title, this.seatButton);
    for (const group of ['gold', 'material', 'zeit'] as const) {
      const row = document.createElement('div');
      row.className = 'row';
      DEV_ACTIONS.forEach((a, i) => {
        if (a.group !== group) return;
        const b = document.createElement('button');
        b.textContent = a.label;
        b.addEventListener('pointerdown', (ev) => (ev.preventDefault(), onAction(i)));
        this.buttons[i] = b;
        row.append(b);
      });
      this.root.append(row);
    }
    this.root.hidden = true;
    document.head.append(this.style);
    document.body.append(this.root);
  }

  render(visible: boolean, s: FocusState, seatLabel: string): void {
    this.root.hidden = !visible;
    if (!visible) return;
    this.root.classList.toggle('focus', s.focus);
    this.title.textContent = s.focus ? 'Dev-Fokus · D-Pad wählen · A auslösen · RB zurück' : 'Dev-Aktionen · RB: Controller-Fokus';
    this.seatButton.textContent = `für ${seatLabel}`;
    this.buttons.forEach((b, i) => b.classList.toggle('sel', s.focus && i === s.index));
  }

  destroy(): void {
    this.root.remove();
    this.style.remove();
  }
}
