// Gemerkte Einstellungen von k3c-dev (Schlüssel `k3c-dev.<seite>.<was>`), Logik in tools/webkit/prefs.ts.
import { createPrefs } from '@webkit/lib/prefs';

export { isStringList } from '@webkit/lib/prefs';
export const { loadPref, loadText, savePref, loadJSON, saveJSON } = createPrefs('k3c-dev.');

/** Ablage für useSplit (`@webkit/lib/split`): gemerkte Splitter-Anteile unter `k3c-dev.<key>`. */
export const prefsStore = { loadText, savePref };
