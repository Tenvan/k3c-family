/**
 * Vollbild für beliebige Seiten.
 * Achtung: Browser erlauben Vollbild nur direkt nach einer Nutzeraktion. Ob ein Controller-Tastendruck dafür zählt,
 * hängt vom Browser ab (das prüft der Gamepad-Test auf der Xbox). Außerdem endet Vollbild bei jedem Seitenwechsel.
 */
export function isFullscreen(): boolean {
  return !!document.fullscreenElement;
}

export function fullscreenSupported(): boolean {
  return typeof document.documentElement.requestFullscreen === 'function';
}

/** Manche Browser beantworten die Anfrage nie (weder erfüllt noch abgelehnt). Danach gilt sie als gescheitert. */
const RESPONSE_TIMEOUT_MS = 2000;

/** Schaltet um. Liefert eine Fehlermeldung, falls der Browser es verweigert oder nicht reagiert, sonst null. */
export async function toggleFullscreen(): Promise<string | null> {
  const request = isFullscreen() ? document.exitFullscreen() : document.documentElement.requestFullscreen({ navigationUI: 'hide' });
  const timeout = new Promise<never>((_, reject) =>
    setTimeout(() => reject(new Error('Browser hat nicht reagiert')), RESPONSE_TIMEOUT_MS),
  );
  try {
    await Promise.race([request, timeout]);
    return null;
  } catch (err) {
    return err instanceof Error ? err.message : String(err);
  }
}

export function onFullscreenChange(listener: (active: boolean) => void): void {
  document.addEventListener('fullscreenchange', () => listener(isFullscreen()));
}
