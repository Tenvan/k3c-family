import { useEffect } from 'react';

// Seitenwechsel von einer Seite zur anderen ohne Kenntnis der Navigation (WZ62.4, B-1171): Die Seite meldet das Ziel,
// die App (useNavigation) wechselt. Parameter wie der Welt-Filter stehen vorher in der Adresse (lib/weltFilter.ts).

export const NAVIGATE_EVENT = 'k3c:navigate';

/** Bittet die App, die Seite `page` (Schlüssel eines Reiters) zu öffnen. */
export function navigateTo(page: string): void {
  window.dispatchEvent(new CustomEvent<string>(NAVIGATE_EVENT, { detail: page }));
}

/** Hört auf Seitenwechsel-Wünsche; `handler` bekommt den Schlüssel des Reiters. */
export function useNavigateRequests(handler: (page: string) => void): void {
  useEffect(() => {
    const on = (e: Event) => handler((e as CustomEvent<string>).detail);
    window.addEventListener(NAVIGATE_EVENT, on);
    return () => window.removeEventListener(NAVIGATE_EVENT, on);
  }, [handler]);
}
