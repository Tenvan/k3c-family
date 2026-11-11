// Rig-Format des nativen Spiels (native/k3c-rig/RIG.md). Der Export schreibt es, das Go-Paket `rig` liest es.
import type { Animation, Proportionen, Punkt, Spec } from '../../../tools/paper/schema/types';

/** Bild eines Teils, Bands oder Elements: Datei, linke obere Ecke in Einheiten der Figur, Größe in Pixeln. */
export type BildOrt = { bild: string; versatz: Punkt; groesse: [number, number] };

/** Starres Teil; ohne Lagen (Bein-Knochen des Vierbeiners) fehlt das Bild. `ebene` = Zeichenreihenfolge. */
export type RigTeil = { id: string; eltern?: string; pivot: Punkt; ebene: number } & Partial<BildOrt>;

/** Platz für ein optionales Element oder eine andere Figur: hängt an einem Teil, Punkt und Drehung in dessen Rahmen. */
export type RigPlatz = { name: string; teil: string; punkt: Punkt; drehung: number; ebene: number };

/** Band (Umhang, Bein): Bild der Ruhelage, entlang der Stützpunkte gebogen. `mitte` = x der Mittellinie, `ruhe` = y je
 * Stützpunkt, `breite` = Abstand der linken und rechten Kante von der Mittellinie je Stützpunkt (samt Rand für den Schatten). */
export type RigBand = { id: string; teil?: string; ebene: number; mitte: number; ruhe: number[]; breite: [number, number][] } & BildOrt;

/** Rand des Bands neben der Papierkante: links wenig, rechts Platz für den Schlagschatten (Licht von rechts oben). */
export const BAND_RAND: [number, number] = [1.6, 3.6];

export type Rig = {
  id: string;
  rig: 'figur' | 'vierbeiner';
  version: number;
  /** Pixel der Teil-Bilder je Einheit der Figur */
  dichte: number;
  massstab: number;
  /** Rahmen und Pixelhöhe des Sheets (für den Vergleich) */
  rahmen: [number, number, number, number];
  teile: RigTeil[];
  plaetze: RigPlatz[];
  baender: RigBand[];
  umhang?: Spec['umhang'];
  bein?: { spitze: number };
  proportionen?: Proportionen;
  animationen: Record<string, Animation>;
  /** Variante → Bild-Id (Teil, Band) → Datei statt des Standardbilds */
  varianten: Record<string, Record<string, string>>;
  /** Elemente, die ohne Auswahl an den Plätzen sitzen */
  standard?: Record<string, string>;
  reiten?: { sitz: Punkt; ausblenden: string[] };
  zuegel?: { farbe: string; deckkraft: number; breite: number; durchhang: number; ebene: number };
};

/** Optionales Element an einem Platz; `versatz` relativ zum Punkt des Platzes (Drehung 0). */
export type Element = { id: string; platz: string; dichte: number } & BildOrt;

export type Dateien = Map<string, Buffer | string>;
