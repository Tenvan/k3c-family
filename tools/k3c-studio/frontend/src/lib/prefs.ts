// Gemerkte Einstellungen von k3c-studio (Schlüssel `k3c-studio.<seite>.<was>`), Logik in tools/webkit/prefs.ts.
import { createPrefs } from '@webkit/lib/prefs';

export { isStringList } from '@webkit/lib/prefs';
export const { loadPref, loadText, savePref, loadJSON, saveJSON } = createPrefs('k3c-studio.');

/** Ablage für useSplit (`@webkit/lib/split`): gemerkte Splitter-Anteile unter `k3c-studio.<key>`. */
export const prefsStore = { loadText, savePref };
