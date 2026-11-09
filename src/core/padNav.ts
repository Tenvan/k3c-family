/**
 * Controller-Bedienung für Seiten mit Bedienelementen (B-767): D-Pad und linker Stick bewegen den Fokus zum nächsten Element in
 * der Richtung, A klickt, ohne Ziel in der Richtung scrollt die Seite. Gilt für jede Seite, die `installPageChrome()` aufruft, außer sie
 * hat eine eigene Pad-Bedienung (`padNav: false`) oder zeichnet mit Phaser. B (1) geht zurück (im Werkzeug-Panel ins Menü; der Edge-Zurück
 * der Xbox verfällt in der Zurück-Falle), View und Menu sind
 * reserviert (Home-Kombi). Die Zustandsmaschine und die Zielwahl sind rein; `installPadNav` verdrahtet sie mit der Seite.
 */

export type NavAction = 'up' | 'down' | 'left' | 'right' | 'accept' | 'back';

/** Pad-Tasten (standard mapping): A bestätigt, B geht zurück, D-Pad bewegt */
const A = 0;
const B = 1;
const UP = 12;
const DOWN = 13;
const LEFT = 14;
const RIGHT = 15;
const STICK = 0.55;
/** Erste Wiederholung nach … ms, danach alle … ms (Halten des Sticks) */
export const REPEAT_FIRST_MS = 420;
export const REPEAT_EVERY_MS = 130;

/** Was von einem Pad gelesen wird (Teil von `Gamepad`) */
export interface PadSnapshot {
  buttons: ArrayLike<{ pressed: boolean }>;
  axes: ArrayLike<number>;
}

/** Gehaltene Tasten eines Pads als Menge von Aktionen */
export function heldActions(pad: PadSnapshot): Set<NavAction> {
  const held = new Set<NavAction>();
  const on = (i: number): boolean => pad.buttons[i]?.pressed === true;
  const x = pad.axes[0] ?? 0;
  const y = pad.axes[1] ?? 0;
  if (on(A)) held.add('accept');
  if (on(B)) held.add('back');
  if (on(UP) || y < -STICK) held.add('up');
  if (on(DOWN) || y > STICK) held.add('down');
  if (on(LEFT) || x < -STICK) held.add('left');
  if (on(RIGHT) || x > STICK) held.add('right');
  return held;
}

/** Löst je gedrückter Taste eine Aktion aus; Richtungen wiederholen beim Halten, A nur einmal je Druck */
export class PadNav {
  private since = new Map<NavAction, number>();
  private last = new Map<NavAction, number>();

  /** Aktionen dieses Bildes aus den gehaltenen Tasten aller Pads zur Zeit `now` (ms) */
  update(pads: readonly (PadSnapshot | null | undefined)[], now: number): NavAction[] {
    const held = new Set<NavAction>();
    for (const p of pads) if (p) for (const a of heldActions(p)) held.add(a);
    const out: NavAction[] = [];
    for (const a of ['up', 'down', 'left', 'right', 'accept', 'back'] as const) {
      if (!held.has(a)) {
        this.since.delete(a);
        this.last.delete(a);
        continue;
      }
      const start = this.since.get(a);
      if (start === undefined) {
        this.since.set(a, now);
        this.last.set(a, now);
        out.push(a);
        continue;
      }
      if (a === 'accept' || a === 'back') continue; // einmal je Druck
      const wait = this.last.get(a)! === start ? REPEAT_FIRST_MS : REPEAT_EVERY_MS;
      if (now - this.last.get(a)! >= wait) {
        this.last.set(a, now);
        out.push(a);
      }
    }
    return out;
  }
}

export interface Box {
  left: number;
  top: number;
  right: number;
  bottom: number;
}

const center = (b: Box): { x: number; y: number } => ({ x: (b.left + b.right) / 2, y: (b.top + b.bottom) / 2 });

/**
 * Index des Elements, das nach `dir` den Fokus bekommt: das nächste, dessen Mitte in der Richtung liegt (Abstand in der Richtung plus
 * doppelter Versatz quer dazu); `-1` = keins; ohne aktuellen Fokus (`from` −1) das oberste, linkeste.
 */
export function pickTarget(boxes: readonly Box[], from: number, dir: 'up' | 'down' | 'left' | 'right'): number {
  if (boxes.length === 0) return -1;
  if (from < 0 || from >= boxes.length) {
    let first = 0;
    boxes.forEach((b, i) => {
      const f = boxes[first]!;
      if (b.top < f.top - 1 || (Math.abs(b.top - f.top) <= 1 && b.left < f.left)) first = i;
    });
    return first;
  }
  const c = center(boxes[from]!);
  let best = -1;
  let bestScore = Infinity;
  boxes.forEach((b, i) => {
    if (i === from) return;
    const p = center(b);
    const along = dir === 'up' ? c.y - p.y : dir === 'down' ? p.y - c.y : dir === 'left' ? c.x - p.x : p.x - c.x;
    const across = dir === 'up' || dir === 'down' ? Math.abs(p.x - c.x) : Math.abs(p.y - c.y);
    if (along <= 1) return;
    const score = along + across * 2;
    if (score < bestScore) {
      bestScore = score;
      best = i;
    }
  });
  return best;
}

const FOCUSABLE = 'a[href], button:not([disabled]), input:not([disabled]):not([type="hidden"]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';
/** Wie weit die Seite scrollt, wenn es in der Richtung kein Ziel gibt */
const SCROLL_PX = 140;

function visible(el: HTMLElement): boolean {
  const r = el.getBoundingClientRect();
  return r.width > 0 && r.height > 0 && getComputedStyle(el).visibility !== 'hidden';
}

/** Verdrahtet `PadNav` mit der Seite (`enabled` schaltet sie zeitweise ab, z. B. während ein Phaser-Spiel läuft): Fokus bewegen, A klickt, ohne Ziel scrollen (nur sichtbare Elemente zählen) */
export function installPadNav(enabled: () => boolean = () => true, onBack?: () => void): void {
  const nav = new PadNav();
  const step = (): void => {
    const pads = typeof navigator.getGamepads === 'function' ? Array.from(navigator.getGamepads()) : [];
    const actions = nav.update(pads, performance.now());
    if (enabled()) {
      for (const action of actions) {
        if (action === 'back') onBack?.();
        else apply(action);
      }
    }
    requestAnimationFrame(step);
  };
  requestAnimationFrame(step);
}

function apply(action: Exclude<NavAction, 'back'>): void {
  const els = Array.from(document.querySelectorAll<HTMLElement>(FOCUSABLE)).filter(visible);
  const at = els.indexOf(document.activeElement as HTMLElement);
  if (action === 'accept') {
    if (at >= 0) els[at]!.click();
    return;
  }
  const next = pickTarget(els.map((e) => e.getBoundingClientRect()), at, action);
  if (next >= 0) {
    els[next]!.focus();
    els[next]!.scrollIntoView({ block: 'nearest', inline: 'nearest' });
  } else if (action === 'up' || action === 'down') {
    window.scrollBy(0, action === 'down' ? SCROLL_PX : -SCROLL_PX); // kein Ziel in der Richtung: die Seite scrollt
  }
}
