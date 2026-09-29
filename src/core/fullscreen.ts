import { isEmbedded, postToShell, SHELL_MESSAGE, type ShellMessage } from './shell';

/**
 * Vollbild für alle Seiten. In der Shell wird immer die Landingpage (oberstes Dokument) in Vollbild geschaltet,
 * damit es über Seitenwechsel erhalten bleibt. Unterseiten bitten die Shell per postMessage darum.
 *
 * Browser erlauben Vollbild nur nach einer echten Nutzeraktion (Klick/Taste). Chrome/Edge übertragen die
 * Aktion vom iframe auf die Shell. Ob ein Controller-Tastendruck zählt, misst der Gamepad-Test.
 */

/** Manche Browser beantworten die Anfrage nie (weder erfüllt noch abgelehnt). Danach gilt sie als gescheitert. */
const RESPONSE_TIMEOUT_MS = 2000;

export function isFullscreen(): boolean {
  return !!document.fullscreenElement;
}

export function fullscreenSupported(): boolean {
  return document.fullscreenEnabled || isEmbedded;
}

/** Schaltet um. Liefert eine Fehlermeldung, falls der Browser es verweigert oder nicht reagiert, sonst null. */
export function toggleFullscreen(): Promise<string | null> {
  return isEmbedded ? askShell() : toggleLocal();
}

/** Nur für die Shell selbst: schaltet das eigene Dokument um. */
export async function toggleLocal(): Promise<string | null> {
  // Wichtig: request/exit synchron aufrufen, solange die Nutzeraktion noch "frisch" ist.
  const request = isFullscreen() ? document.exitFullscreen() : document.documentElement.requestFullscreen({ navigationUI: 'hide' });
  try {
    await withTimeout(request);
    return null;
  } catch (err) {
    return err instanceof Error ? err.message : String(err);
  }
}

let nextId = 1;

function askShell(): Promise<string | null> {
  const id = nextId++;
  return withTimeout(
    new Promise<string | null>((resolve) => {
      const onMessage = (e: MessageEvent<ShellMessage>) => {
        if (e.origin !== location.origin || e.data?.type !== SHELL_MESSAGE.fullscreenResult || e.data.id !== id) return;
        removeEventListener('message', onMessage);
        resolve(e.data.error);
      };
      addEventListener('message', onMessage);
      postToShell({ type: SHELL_MESSAGE.fullscreen, id });
    }),
    RESPONSE_TIMEOUT_MS + 500,
  ).catch((err) => (err instanceof Error ? err.message : String(err)));
}

function withTimeout<T>(promise: Promise<T>, ms = RESPONSE_TIMEOUT_MS): Promise<T> {
  return Promise.race([
    promise,
    new Promise<never>((_, reject) => setTimeout(() => reject(new Error('Browser hat nicht reagiert')), ms)),
  ]);
}

export function onFullscreenChange(listener: (active: boolean) => void): void {
  document.addEventListener('fullscreenchange', () => listener(isFullscreen()));
}
