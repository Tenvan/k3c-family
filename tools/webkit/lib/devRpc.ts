// Client-Server-Betrieb der Wails-Apps (k3c-workbench, k3c-studio; Go: tools/workkit/uikit): jede Methode der App ist
// ein POST auf /api/dev/rpc/<Methode> mit den Argumenten als JSON-Array, die Antwort ist {result} oder {error}.
// Ereignisse kommen als Server-Sent Events von /api/dev/events. Die Kopfzeile hält fremde Seiten fern (CORS-Preflight).
const API = '/api/dev/';
const HEADERS = { 'Content-Type': 'application/json', 'X-K3C-Dev': '1' };

/** Meldet sich nach einem Abbruch des Stroms (EventSource verbindet selbst neu); die Seiten laden dann einmal neu. */
export const RECONNECT = 'ui:reconnect';

interface Answer<T> {
  result?: T;
  error?: string;
}

/** Aufrufer der Methoden einer App; `app` steht in den Fehlertexten. */
export function devRpc(app: string) {
  const answer = async <T>(res: Response): Promise<T> => {
    const body = (await res.json().catch(() => ({ error: `Fehler ${res.status} von ${app}` }))) as Answer<T>;
    if (!res.ok || body.error) throw new Error(body.error || `Fehler ${res.status} von ${app}`);
    return body.result as T;
  };
  return <T>(method: string, ...args: unknown[]): Promise<T> =>
    fetch(`${API}rpc/${method}`, { method: 'POST', headers: HEADERS, body: JSON.stringify(args) }).then((res) => answer<T>(res));
}

/** Ein Strom für alle Ereignisse; je Name ein Listener, der die Abonnenten bedient. */
export function devEvents<Events>() {
  let source: EventSource | null = null;
  const listeners = new Map<string, Set<(data: unknown) => void>>();
  const open = () => {
    let opened = false;
    const es = new EventSource(`${API}events`);
    es.addEventListener('open', () => {
      if (opened) listeners.get(RECONNECT)?.forEach((f) => f(null));
      opened = true;
    });
    return es;
  };
  return <E extends keyof Events & string>(event: E, fn: (data: Events[E]) => void): (() => void) => {
    let set = listeners.get(event);
    if (!set) {
      const subs = new Set<(data: unknown) => void>();
      listeners.set(event, subs);
      set = subs;
      source ??= open();
      if (event !== RECONNECT) {
        source.addEventListener(event, (e) => {
          const data: unknown = JSON.parse((e as MessageEvent<string>).data);
          subs.forEach((f) => f(data));
        });
      }
    }
    const f = fn as (data: unknown) => void;
    set.add(f);
    return () => {
      set.delete(f);
    };
  };
}
