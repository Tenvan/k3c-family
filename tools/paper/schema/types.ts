// Typen der Paper-Spec v1 und der Stil-Datei; Quelle der Wahrheit sind paper-spec.schema.json und stil.schema.json.

export type Punkt = [number, number];
export type Ton = 'licht' | 'basis' | 'schatten';

export type Lage =
  | { art: 'flaeche'; form: string; material: string; ton?: Ton; auflage?: boolean }
  | { art: 'falz'; form: string; breite: number; material: string; ton?: Ton }
  | { art: 'punkt'; mitte: Punkt; radius: number; material: string; ton?: Ton };

export type Teil = { id: string; eltern?: string; pivot: Punkt; lagen: Lage[] };

export type Animation = {
  frames: number;
  fps?: number;
  reittier?: string;
  gangart?: 'schritt' | 'trab' | 'galopp';
  keyframes?: { t: number; winkel: Record<string, number> }[];
};

export type Spec = {
  id: string;
  rig: 'figur' | 'vierbeiner' | 'gebaeude' | 'requisite' | 'hintergrund';
  version: number;
  beschreibung: string;
  teile: Teil[];
  umhang?: { anker: string; punkt: Punkt; material: string; laenge: number; breite: [number, number]; segmente?: number };
  bein?: { breiten_vorn: number[]; breiten_hinten: number[] };
  animationen?: Record<string, Animation>;
  varianten?: Record<string, { tausch: Record<string, string> }>;
  stufen?: { tausch?: Record<string, string>; zusaetze?: Teil[] }[];
  ausgabe: { hoehe_px: number; rahmen: [number, number, number, number]; massstab?: number };
};

export type Stil = {
  materialien: Record<string, Record<Ton, string>>;
  einzeltoene: Record<string, string>;
  papier: {
    lage: { dx: number; dy: number; unschaerfe: number; deckkraft: number; farbe: string };
    auflage: { dx: number; dy: number; unschaerfe: number; deckkraft: number };
    struktur: { frequenz: number; oktaven: number; staerke: number };
    falz_deckkraft: number;
  };
  licht: 'rechts-oben';
  proportionen: { figur: { kopf_anteil: number; toleranz: number }; vierbeiner: { rumpfhoehe_kopflaengen: number; toleranz: number } };
  skalierung: Record<string, number>;
};
