import type { AssetKind } from './types';

// Zuordnungs-Vorschläge des Chats auf Sound- und Grafik-Seite (B-600, WZ30.5). Endgültige Typen: WZ29.7 liefert sie im
// Backend (RPC AssetSuggestions, AssetSuggestionAdopt, AssetSuggestionDrop), bis dahin kommen sie aus api/mockSuggestions.ts.

/** Ein Vorschlag des Chats für einen Slot; geschrieben wird erst nach „Übernehmen“ (Q70: nur erlaubte Lizenzen). */
export interface AssetSuggestion {
  id: string;
  /** Slot, dem der Vorschlag zugeordnet würde */
  slot: string;
  art: AssetKind;
  /** Bus des Slots (Sound: music, sfx, ambient), bestimmt die Hörprobe */
  bus?: string;
  titel: string;
  /** Datei nach der Übernahme, relativ zu public/ */
  datei: string;
  /** Ton im Audio-Atlas, wenn der Vorschlag ein Atlas-Ton ist */
  sprite?: string;
  quelle: string;
  urheber?: string;
  lizenz: string;
  /** Lizenz nach Q70 erlaubt; sonst bleibt „Übernehmen“ gesperrt */
  lizenzErlaubt: boolean;
  /** Grund der Sperre (nur bei lizenzErlaubt = false) */
  lizenzGrund?: string;
  /** Größe des Downloads in Bytes */
  groesse: number;
  /** Adresse für Bild bzw. Hörprobe */
  vorschau: string;
}

/** Antwort von AssetSuggestions: offline = Chat/Internet nicht erreichbar, die Liste ist dann leer. */
export interface AssetSuggestionsView {
  offline: boolean;
  vorschlaege: AssetSuggestion[];
}

export interface SuggestionApi {
  /** Vorschläge des Chats zu einem Slot (`''` = zu allen Slots) */
  assetSuggestions(slotId: string): Promise<AssetSuggestionsView>;
  /** Übernimmt einen Vorschlag (schreibt Zuordnung, Credit und Atlas im Backend); scheitert bei Lizenzverstoß und offline */
  assetSuggestionAdopt(slotId: string, id: string): Promise<void>;
  /** Verwirft einen Vorschlag, schreibt nichts */
  assetSuggestionDrop(slotId: string, id: string): Promise<void>;
}
