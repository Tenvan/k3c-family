import type { AssetKind } from './types';

// Quellen und Download-Vorschlagsliste (B-601, WZ30.4). Endgültige Typen: WZ29.6 liefert sie im Backend (RPC Sources,
// SourceDownload, SourceProgress, SourceAdopt, SourceCleanupPreview, SourceCleanup; Liste `data/assets-vorschlaege.json`,
// Vorrat `.work/vorrat/`), bis dahin kommen sie aus api/mockSources.ts.

/** Status eines Eintrags in `data/assets-vorschlaege.json`; „gesperrt“ setzt das Backend, wenn die Quelle eine andere Lizenz nennt. */
export type SourceStatus = 'offen' | 'geladen' | 'verworfen' | 'gesperrt';

/** Ein Eintrag der Vorschlagsliste. */
export interface SourceProposal {
  id: string;
  /** Slot oder Gruppe, für den der Eintrag empfohlen wird */
  slot: string;
  art: AssetKind;
  titel: string;
  /** Name der Quelle (OpenGameArt, Kenney, itch.io, freesound …), Schlüssel zu `SourceInfo.name` */
  quelle: string;
  /** Seite des Angebots und Datei-URL */
  seite: string;
  dateiUrl: string;
  urheber?: string;
  lizenz: string;
  /** Lizenz nach Q70 erlaubt; sonst nur markiert und nicht ladbar */
  lizenzErlaubt: boolean;
  dateiname: string;
  /** Größe des Downloads in Bytes */
  groesse: number;
  /** 1 = hoch … 3 = niedrig */
  prioritaet: number;
  hinweis?: string;
  status: SourceStatus;
  /** Grund bei status = gesperrt oder lizenzErlaubt = false */
  sperrgrund?: string;
  /** Fehlertext des letzten Downloads (Eintrag bleibt „offen“, Wiederholen möglich) */
  fehler?: string;
  /** nur bei status = geladen: Prüfsumme (SHA-256, hex) und Adresse unter `/vorrat/…` für Hörprobe und Vorschau */
  pruefsumme?: string;
  vorschau?: string;
  /** Datei ist einem Slot zugeordnet (liegt unter `public/`); sonst nur im Vorrat */
  verwendet: boolean;
}

/** Übersicht je Quelle mit Lizenzregeln und Zählern. */
export interface SourceInfo {
  name: string;
  /** Lizenzregeln dieser Quelle in einem Satz */
  lizenzregeln: string;
  vorgeschlagen: number;
  geladen: number;
  verwendet: number;
}

/** Antwort von Sources: offline = kein Internet, die Liste bleibt sichtbar (Geladenes spielbar, Ungeladenes ausgegraut). */
export interface SourcesView {
  offline: boolean;
  quellen: SourceInfo[];
  vorschlaege: SourceProposal[];
}

/** Fortschritt des Downloads eines Eintrags. */
export interface SourceProgress {
  id: string;
  status: 'wartet' | 'laedt' | 'fertig' | 'fehler';
  /** Bytes geladen und Gesamtgröße */
  geladen: number;
  gesamt: number;
  fehler?: string;
}

/** Vorschau des Aufräumens: Vorrat-Dateien, die keinem Slot zugeordnet sind. */
export interface SourceCleanupView {
  dateien: { id: string; dateiname: string; groesse: number }[];
  groesse: number;
}

export interface SourceApi {
  sources(): Promise<SourcesView>;
  /** Startet den Download der Einträge (nur nach Bestätigung der Gesamtgröße und der Quellen); scheitert offline und bei gesperrten Einträgen */
  sourceDownload(ids: string[]): Promise<void>;
  /** Fortschritt aller laufenden und eben beendeten Downloads */
  sourceProgress(): Promise<SourceProgress[]>;
  /** Kopiert einen geladenen Eintrag an den Platz seines Slots unter `public/` und legt Credit und Katalog-Eintrag an */
  sourceAdopt(id: string): Promise<void>;
  /** Welche geladenen, unverwendeten Dateien das Aufräumen entfernen würde */
  sourceCleanupPreview(): Promise<SourceCleanupView>;
  /** Entfernt sie aus dem Vorrat (nach Bestätigung); Einträge werden wieder „offen“ */
  sourceCleanup(): Promise<SourceCleanupView>;
}
