import type { AssetKind } from './types';

// Zuordnungs-Vorschläge des Chats auf Sound- und Grafik-Seite (B-600, WZ30.5): Backend (Go: backend/chat_suggestions.go,
// RPC AssetSuggestions, AssetSuggestionAdopt, AssetSuggestionDrop, WZ29.7), im Mock api/mockSuggestions.ts.

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
  /** Adresse für Bild bzw. Hörprobe (leer, solange die Datei nicht im Vorrat liegt) */
  vorschau: string;
  /** Status aus dem Vorrat: nur „geladen“ ist übernehmbar, Downloads nur auf der Seite Quellen nach Bestätigung */
  status: SuggestionStatus;
}

export type SuggestionStatus = 'offen' | 'geladen' | 'gesperrt';

/** Antwort von AssetSuggestions: offline = Chat/Internet nicht erreichbar, die Liste ist dann leer. `stand` ist der
 *  Stand des Slot-Registers für assetSuggestionAdopt (Schreibregel wie assetsSet). */
export interface AssetSuggestionsView {
  offline: boolean;
  stand: string;
  vorschlaege: AssetSuggestion[];
}

export interface SuggestionApi {
  /** Vorschläge zu einem Slot, die des Chats zuerst (`''` = nur die vom Chat genannten, alle Slots) */
  assetSuggestions(slotId: string): Promise<AssetSuggestionsView>;
  /** Übernimmt einen Vorschlag auf dem Stand `stand` (schreibt Zuordnung und Credit im Backend) und liefert den neuen
   *  Stand; scheitert bei Lizenzverstoß, nicht geladener Datei und geändertem Register */
  assetSuggestionAdopt(slotId: string, id: string, stand: string): Promise<string>;
  /** Verwirft einen Vorschlag (Status „verworfen“ in der Vorschlagsliste) */
  assetSuggestionDrop(slotId: string, id: string): Promise<void>;
}
