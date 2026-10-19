// Gemerkte Einstellungen von k3c-studio (Schlüssel `k3c-studio.<seite>.<was>`), Logik in tools/webkit/prefs.ts.
import { createPrefs } from '@webkit/prefs';

export { isStringList } from '@webkit/prefs';
export const { loadPref, loadText, savePref, loadJSON, saveJSON } = createPrefs('k3c-studio.');
