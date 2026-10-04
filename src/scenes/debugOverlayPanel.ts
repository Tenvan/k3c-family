import type { SlotCommand } from './localSlots';
import { DEV_ACTIONS } from './debugActions';

/**
 * Cheat-Dialog (B-231, vorher Dev-Aktionen B-179): modal, 80 % der Fläche, leicht durchsichtig; der Raum steht, solange er offen ist.
 * Maus und Touch tippen die Schaltflächen, der Controller bedient ihn bei offenem Dialog: D-Pad hoch/runter = Aktion,
 * links/rechts = Spieler, A = auslösen. B, View + Menu und X bleiben frei.
 */

/** Tasten der Controller-Bedienung (standard mapping) */
export const PAD_FOCUS = { A: 0, UP: 12, DOWN: 13, LEFT: 14, RIGHT: 15 } as const;

export interface FocusState {
  /** gewählte Zeile in `DEV_ACTIONS` */
  index: number;
  /** gewählter lokaler Spieler (Index in den lokalen Slots) */
  seat: number;
}

/** Tasten, die in diesem Frame neu gedrückt wurden */
export type FocusEdges = Partial<Record<keyof typeof PAD_FOCUS, boolean>>;

const wrap = (i: number, n: number): number => (n <= 0 ? 0 : ((i % n) + n) % n);

/** Nächster Auswahl-Zustand; `fire` = Zeile, die ausgelöst wird. Bei geschlossenem Dialog gehören die Tasten dem Spiel. */
export function focusStep(s: FocusState, e: FocusEdges, open: boolean, seats: number): { state: FocusState; fire: number | null } {
  if (!open) return { state: s, fire: null };
  const index = wrap(s.index + (e.DOWN ? 1 : 0) - (e.UP ? 1 : 0), DEV_ACTIONS.length);
  const seat = wrap(s.seat + (e.RIGHT ? 1 : 0) - (e.LEFT ? 1 : 0), seats);
  return { state: { index, seat }, fire: e.A ? index : null };
}

/** Bei offenem Dialog stehen die Spieler am Controller still (keine Bewegung, kein Sprint, keine Münzen). */
export function muteFocused(commands: SlotCommand[], focus: boolean, isPad: (slot: number) => boolean): SlotCommand[] {
  if (!focus) return commands;
  return commands.map((c) => (isPad(c.slot) ? { slot: c.slot, moveX: 0, sprint: false, pay: false } : c));
}

/** Registry-Schlüssel: Cheat-Dialog offen (GameScene sperrt dann die Controller-Spieler) */
export const DEV_FOCUS_KEY = 'devFocus';

const CSS = `
.k3c-cheat{position:fixed;inset:0;z-index:11;display:flex;align-items:center;justify-content:center;background:rgba(0,0,0,.35)}
.k3c-cheat[hidden]{display:none}
.k3c-cheat .box{width:80vw;height:80vh;box-sizing:border-box;overflow:auto;display:flex;flex-direction:column;gap:2vmin;padding:3vmin;
  background:rgba(12,16,36,.82);border:3px solid #9be564;border-radius:16px;color:#9be564;font:bold clamp(15px,2.4vmin,28px) system-ui,sans-serif}
.k3c-cheat h2{margin:0;font-size:1.3em}
.k3c-cheat .hint{color:#c9d1e8;font-weight:normal}
.k3c-cheat .row{display:flex;gap:1.5vmin;flex-wrap:wrap}
.k3c-cheat button{font:inherit;color:#fff;background:#2b3558;border:3px solid transparent;border-radius:10px;padding:1.2vmin 2vmin;
  min-height:44px;touch-action:manipulation}
.k3c-cheat button.sel{border-color:#ffd166}
.k3c-cheat .close{margin-top:auto;align-self:flex-end;background:#5a2b2b}`;

/** DOM-Dialog über Canvas und Touch-Overlay (z-index 11 > 10). */
export class CheatDialog {
  private readonly root = document.createElement('div');
  private readonly style = document.createElement('style');
  private readonly note = document.createElement('div');
  private readonly seatButton = document.createElement('button');
  private readonly buttons: HTMLButtonElement[] = [];

  constructor(onAction: (index: number) => void, onSeat: () => void, onClose: () => void) {
    this.style.textContent = CSS;
    this.root.className = 'k3c-cheat';
    const box = document.createElement('div');
    box.className = 'box';
    box.innerHTML = '<h2>Cheats · Raum angehalten</h2><div class="hint">Controller: D-Pad wählen · A auslösen · LB + RB schließt. Tastatur: Ä schließt.</div>';
    this.note.className = 'hint';
    const tap = (b: HTMLButtonElement, fn: () => void): void => b.addEventListener('pointerdown', (ev) => (ev.preventDefault(), fn()));
    tap(this.seatButton, onSeat);
    box.append(this.note, this.seatButton);
    for (const group of ['gold', 'material', 'zeit'] as const) {
      const row = document.createElement('div');
      row.className = 'row';
      DEV_ACTIONS.forEach((a, i) => {
        if (a.group !== group) return;
        const b = document.createElement('button');
        b.textContent = a.label;
        tap(b, () => onAction(i));
        this.buttons[i] = b;
        row.append(b);
      });
      box.append(row);
    }
    const close = document.createElement('button');
    close.className = 'close';
    close.textContent = 'Schließen';
    tap(close, onClose);
    box.append(close);
    this.root.append(box);
    this.root.addEventListener('pointerdown', (ev) => ev.target === this.root && (ev.preventDefault(), onClose()));
    this.root.hidden = true;
    document.head.append(this.style);
    document.body.append(this.root);
  }

  /** `note`: Hinweis, z. B. Server ohne Dev-Mode; leer = keiner. */
  render(open: boolean, s: FocusState, seatLabel: string, note: string): void {
    this.root.hidden = !open;
    if (!open) return;
    this.note.textContent = note;
    this.seatButton.textContent = `für ${seatLabel}`;
    this.buttons.forEach((b, i) => b.classList.toggle('sel', i === s.index));
  }

  destroy(): void {
    this.root.remove();
    this.style.remove();
  }
}
