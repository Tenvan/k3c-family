// Negativ-Fixtures: je Datei genau ein Fehler, den check.ts mit genau einer Regel meldet (schema.test.ts).
import type { Spec, Teil } from '../schema/types';

const block = (id: string, y0: number, y1: number, eltern?: string): Teil =>
  ({ id, ...(eltern ? { eltern } : {}), pivot: [0, y1], lagen: [{ art: 'flaeche', form: `M-6 ${y1}V${y0}H6V${y1}Z`, material: 'holz' }] });

const requisite = (id: string, material = 'holz'): Spec => ({
  id, rig: 'requisite', version: 1, beschreibung: `Negativ-Fixture ${id}`,
  teile: [{ id: 'koerper', pivot: [0, 0], lagen: [{ art: 'flaeche', form: 'M-10 0V-20H10V0Z', material }] }],
  ausgabe: { hoehe_px: 64, rahmen: [-12, -22, 24, 24] },
});

/** Figur mit Pflicht-Teilen; `kopfHoehe` steuert die Proportion (Rahmenhöhe 150, Soll 0,286 ± 0,15). */
function figur(id: string, kopfHoehe: number, nurKern = false): Spec {
  const glieder = ['oberarm-v', 'unterarm-v', 'oberarm-h', 'unterarm-h', 'oberschenkel-v', 'unterschenkel-v', 'oberschenkel-h', 'unterschenkel-h'];
  return {
    id, rig: 'figur', version: 1, beschreibung: `Negativ-Fixture ${id}`,
    teile: [block('rumpf', -100, -50), block('kopf', -100 - kopfHoehe, -100, 'rumpf'),
      ...(nurKern ? [] : glieder.map((g, i) => block(g, -90 + i * 5, -60 + i * 5, 'rumpf')))],
    ausgabe: { hoehe_px: 192, rahmen: [-20, -148, 40, 150] },
  };
}

export function negativSpecs(): Record<string, object> {
  return {
    'hex-farbe': requisite('hex-farbe', '#9a9386'),
    'unbekanntes-feld': { ...requisite('unbekanntes-feld'), glanz: true },
    'fremdes-material': requisite('fremdes-material', 'neon-gruen'),
    'pflicht-teil-fehlt': figur('pflicht-teil-fehlt', 43, true),
    proportion: figur('proportion', 10),
  };
}
