import { useState } from 'react';
import { ChipSwitch } from '@webkit/molecules';

/** Zoom-Stufen der Grafik in Katalog und Review (Wunsch 🧑 2026-11-07, B-733): Faktor auf die Bildgröße. */
const STUFEN = ['0.5', '1', '2', '4'] as const;
type Stufe = (typeof STUFEN)[number];
const OPTIONEN = STUFEN.map((s) => ({ value: s, label: `${s.replace('.', ',')}×` }));

/** Zuletzt gewählte Stufe je Ansicht; überlebt Asset- und Tab-Wechsel. */
const gemerkt = new Map<string, Stufe>();

/** Zoom einer Ansicht (`ansicht` z. B. „vorschau“, „review“, „groesse“) mit Startwert; liefert Faktor und Umschalter. */
export function useZoom(ansicht: string, start: Stufe) {
  const [stufe, setStufe] = useState<Stufe>(gemerkt.get(ansicht) ?? start);
  const waehle = (s: Stufe) => {
    gemerkt.set(ansicht, s);
    setStufe(s);
  };
  return { faktor: Number(stufe), schalter: <ChipSwitch label="Zoom" options={OPTIONEN} value={stufe} onChange={waehle} /> };
}
