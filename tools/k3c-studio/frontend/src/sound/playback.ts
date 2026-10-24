import { CROSSFADE_SECONDS } from '@game/audio/crossfade';
import { attenuation } from '@game/audio/events';
import type { FileOptions } from '@game/audio/audioCore';

// Abspiel-Zuordnung je Zuordnung eines Sound-Slots (umgezogen aus playSound der früheren Asset-Vorschau, B-459):
// reine Logik ohne Audio, damit sie testbar bleibt.

/** Vorschau-Wert für „fern“: Quelle eine halbe Bildschirmbreite neben dem eigenen Ausschnitt (Split-Screen), mit der
 *  Formel des Spiels (0,25); im Spiel ist die Dämpfung abstandsabhängig. Die frühere Asset-Vorschau nahm x = 100 (3,5 Breiten
 *  jenseits des Rands): spielgetreu stumm (Faktor 0), als Vorschau aber nutzlos. */
export const FAR_GAIN = attenuation(40, [{ center: 20, span: 20 }], false);

export interface PlaySlot {
  bus?: string;
}
export interface PlayChoice {
  datei: string;
  sprite?: string;
}

export type Plan =
  | { kind: 'sprite'; name: string; bus: 'sfx'; gain: number }
  | { kind: 'file'; url: string; opts: FileOptions };

/** Atlas-Ton auf SFX (fern gedämpft), Musik mit Überblendung, Umgebungsgeräusch als Schleife, sonst SFX einmal. */
export function playPlan(slot: PlaySlot, choice: PlayChoice, far = false): Plan {
  if (choice.sprite) return { kind: 'sprite', name: choice.sprite, bus: 'sfx', gain: far ? FAR_GAIN : 1 };
  if (slot.bus === 'music') return { kind: 'file', url: choice.datei, opts: { fade: CROSSFADE_SECONDS } };
  const ambient = slot.bus === 'ambient';
  return { kind: 'file', url: choice.datei, opts: { bus: ambient ? 'ambient' : 'sfx', loop: ambient } };
}

/** Statuszeile wie in der früheren Asset-Vorschau: „Beschreibung · Datei#Ton“ oder die fehlende Datei. */
export function playStatus(beschreibung: string, choice: PlayChoice, ok: boolean): string {
  if (ok) return `${beschreibung} · ${choice.datei}${choice.sprite ? `#${choice.sprite}` : ''}`;
  if (choice.sprite) return `Ton „${choice.sprite}“ nicht im Audio-Atlas (oder Atlas nicht geladen; Atlas bauen mit „task soundatlas“)`;
  return `Datei fehlt oder ist nicht abspielbar: ${choice.datei}`;
}
