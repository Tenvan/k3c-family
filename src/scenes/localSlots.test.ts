import { describe, expect, it } from 'vitest';
import { ADD_SLOT_TIMEOUT_MS, LocalSlots, type SlotInputDevice } from './localSlots';

class Pad implements SlotInputDevice {
  confirm = false;
  x = 0;
  holding = new Set<string>();
  justPressed = () => this.confirm;
  held = (action: string) => (action === 'confirm' ? this.confirm : this.holding.has(action));
  moveX = () => this.x;
  sprint = () => false;
}

function host() {
  const calls: string[] = [];
  return { calls, client: { addSlot: (s: number) => void calls.push(`add${s}`), removeSlot: (s: number) => void calls.push(`remove${s}`) } };
}

const press = (pad: Pad) => {
  pad.confirm = true;
  return pad;
};

describe('LocalSlots', () => {
  it('der erste Spieler übernimmt den Platz aus create (Slot 0), ohne addSlot', () => {
    const { calls, client } = host();
    const slots = new LocalSlots<Pad>();
    const a = new Pad();
    expect(slots.waiting([0])).toBe(true);
    slots.join([press(a)], [0], client, 0);
    expect(slots.bound[0]).toBe(a);
    expect(slots.waiting([0])).toBe(false);
    expect(calls).toEqual([]);
  });

  it('A auf einem neuen Controller sendet addSlot für den ersten freien Slot (AC-11)', () => {
    const { calls, client } = host();
    const slots = new LocalSlots<Pad>();
    const [a, b] = [new Pad(), new Pad()];
    slots.join([press(a)], [0], client, 0);
    a.confirm = false;
    slots.join([a, press(b)], [0], client, 10);
    expect(slots.bound[1]).toBe(b);
    expect(calls).toEqual(['add1']);
    slots.join([a, b], [0, 1], client, 20); // bestätigt: nichts mehr offen
    expect(calls).toEqual(['add1']);
  });

  it('ein Gerät belegt nie zwei Plätze, und höchstens vier Spieler', () => {
    const { calls, client } = host();
    const slots = new LocalSlots<Pad>();
    const pads = [0, 1, 2, 3, 4].map(() => new Pad());
    slots.join([press(pads[0]!)], [0], client, 0);
    const seated = [0];
    for (const pad of pads.slice(1)) {
      slots.join([press(pad)], seated, client, 1);
      const added = calls.at(-1);
      if (added) seated.push(Number(added.slice(3)));
    }
    expect(calls).toEqual(['add1', 'add2', 'add3']);
    expect(slots.bound.filter(Boolean)).toHaveLength(4);
    slots.join([pads[0]!], seated, client, 2);
    expect(calls).toHaveLength(3);
  });

  it('wird addSlot nicht bestätigt, ist das Gerät nach dem Zeitlimit wieder frei', () => {
    const { client } = host();
    const slots = new LocalSlots<Pad>();
    const [a, b] = [new Pad(), new Pad()];
    slots.join([press(a)], [0], client, 0);
    slots.join([press(b)], [0], client, 100);
    expect(slots.bound[1]).toBe(b);
    slots.join([], [0], client, 100 + ADD_SLOT_TIMEOUT_MS + 1);
    expect(slots.bound[1]).toBeNull();
  });

  it('getrennter Controller: removeSlot; der letzte Spieler: leave (AC-11)', () => {
    const { calls, client } = host();
    const slots = new LocalSlots<Pad>();
    const [a, b] = [new Pad(), new Pad()];
    slots.join([press(a)], [0], client, 0);
    slots.join([a, press(b)], [0], client, 1);
    expect(slots.lose(b, [0, 1], client)).toBe('removed');
    expect(calls.at(-1)).toBe('remove1');
    expect(slots.bound[1]).toBeNull();
    expect(slots.lose(a, [0], client)).toBe('leave');
    expect(calls.filter((c) => c.startsWith('remove'))).toEqual(['remove1']);
    expect(slots.lose(new Pad(), [0], client)).toBe('none');
  });

  it('Mock-Slots fangen keine Bestätigung ab und senden „keine Bewegung“', () => {
    const { calls, client } = host();
    const slots = new LocalSlots<Pad>([1, 2]);
    const a = new Pad();
    a.x = 1;
    expect(slots.waiting([0, 1, 2])).toBe(true);
    slots.join([press(a)], [0, 1, 2], client, 0);
    expect(slots.bound).toEqual([a, null, null, null]);
    expect(calls).toEqual([]);
    expect(slots.waiting([0, 1, 2])).toBe(false);
    expect(slots.commands([0, 1, 2])).toEqual([
      { slot: 0, moveX: 1, sprint: false, pay: true },
      { slot: 1, moveX: 0, sprint: false, pay: false },
      { slot: 2, moveX: 0, sprint: false, pay: false },
    ]);
  });

  it('ein Platz ohne Spieler sendet keine Eingabe', () => {
    expect(new LocalSlots<Pad>().commands([0])).toEqual([]);
  });

  it('Schlag und gehaltener Skill-Slot gehen mit, nur wenn gehalten (B-124/AC-02)', () => {
    const { client } = host();
    const slots = new LocalSlots<Pad>();
    const a = new Pad();
    slots.join([press(a)], [0], client, 0);
    a.confirm = false;
    expect(slots.commands([0])).toEqual([{ slot: 0, moveX: 0, sprint: false, pay: false }]);
    a.holding = new Set(['attack', 'skill1']);
    expect(slots.commands([0])).toEqual([{ slot: 0, moveX: 0, sprint: false, pay: false, attack: true, skill: 1 }]);
    a.holding = new Set(['skill4']);
    expect(slots.commands([0])[0]).toMatchObject({ skill: 4 });
    expect(slots.commands([0])[0]).not.toHaveProperty('attack');
  });
});
