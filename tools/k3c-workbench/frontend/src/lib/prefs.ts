// Gemerkte Einstellungen von k3c-workbench (Schlüssel `k3c-workbench.<seite>.<was>`), Logik in tools/webkit/prefs.ts.
import { createPrefs } from '@webkit/lib/prefs';

export { isStringList } from '@webkit/lib/prefs';
export const { loadPref, loadText, savePref, loadJSON, saveJSON } = createPrefs('k3c-workbench.');

/** Ablage für useSplit (`@webkit/lib/split`): gemerkte Splitter-Anteile unter `k3c-workbench.<key>`. */
export const prefsStore = { loadText, savePref };
