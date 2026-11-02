import type { AssetSaved } from './types';

// Kandidaten je Asset-Slot für die Assets-Seite (B-677, WZ33): Spiel (Katalog und Zuordnungen), Vorrat und Netz
// (Vorschläge und Recherche-Treffer) in einer Liste. Backend: RPC AssetCandidates und AssetCandidateAssign
// (tools/k3c-studio/backend/app_candidates.go, tools/assetkit/candidates.go), im Mock api/mockCandidates.ts.

/** Stufe einer Datei auf dem Weg ins Spiel: Netz → Vorrat → Spiel (public/, zugeordnet oder im Katalog). */
export type CandidateStage = 'netz' | 'vorrat' | 'spiel';

/** Lizenz-Marke: frei = CC0/CC-BY/OGA-BY, sa = Share-Alike (übernehmbar mit Hinweis), pruefen = Lizenz unklar, gesperrt = NC/ND/nicht erlaubt. */
export type LicenseMark = 'frei' | 'sa' | 'pruefen' | 'gesperrt';

/** Woher der Kandidat kommt; ein Kandidat mit gleicher Datei-URL oder Prüfsumme erscheint einmal mit allen Herkünften. */
export type CandidateOrigin = 'katalog' | 'vorrat' | 'vorschlag' | 'recherche';

export interface AssetCandidate {
  /** `sug:<Vorschlags-ID>`, `hit:<Treffer-ID>`, `cat:<Datei>` (Katalog) oder `wahl:<Datei>` (schon dem Slot zugeordnet) */
  id: string;
  stufe: CandidateStage;
  /** 0 = nicht zugeordnet, n = Präferenz n dieses Slots (1 = im Spiel aktiv) */
  praef: number;
  titel: string;
  /** Datei unter public/ (Stufe Spiel, übernommener Vorrat) bzw. Dateiname im Netz */
  datei: string;
  quelle: string;
  urheber?: string;
  lizenz: string;
  marke: LicenseMark;
  /** Warum „Zuordnen“ gesperrt ist (leer, wenn ladbar) */
  grund?: string;
  /** Größe in Bytes, 0 = unbekannt */
  groesse: number;
  /** Adresse für Bild bzw. Hörprobe, leer = keine Vorschau (Netz) */
  vorschau: string;
  /** Seite des Angebots */
  link?: string;
  herkunft: CandidateOrigin[];
  /** „Zuordnen“ ist möglich: Spiel/Vorrat immer, Netz nur mit direktem Download und erlaubter Lizenz (online) */
  zuordenbar: boolean;
  /** Recherche-Treffer mit Lizenz „prüfen“: 🧑 kann sie bestätigen (AssetsLicenseOK mit slot und Treffer-ID ohne `hit:`) */
  lizenzBestaetigen?: boolean;
}

/** Antwort von AssetCandidates: `stand` ist der Stand des Slot-Registers für AssetCandidateAssign (Schreibregel wie AssetsSet). */
export interface AssetCandidatesView {
  offline: boolean;
  stand: string;
  kandidaten: AssetCandidate[];
}

export interface CandidateApi {
  /** Kandidaten eines Slots: Zuordnungen (Präf. n), Vorrat, Netz (Vorschläge, Recherche) und Katalog-Treffer zu `query`
   *  (leer = Beschreibung des Slots), höchstens `limit` Katalog-Treffer. Zuordnungen stehen zuerst in Reihenfolge der Präferenz. */
  assetCandidates(slotId: string, query: string, limit: number): Promise<AssetCandidatesView>;
  /** Ordnet den Kandidaten dem Slot zu (hängt ihn als letzte Präferenz an) und lädt ihn dafür bei Bedarf:
   *  Netz → Vorrat → Spiel in einem Aufruf. Nur auf dem Stand `stand`; `confirmedBytes` muss bei Netz-Kandidaten mit
   *  bekannter Größe deren `groesse` entsprechen (Bestätigung in der Zeile). Scheitert ein Schritt, ist nichts zugeordnet. */
  assetCandidateAssign(slotId: string, id: string, stand: string, confirmedBytes: number): Promise<AssetSaved>;
}
