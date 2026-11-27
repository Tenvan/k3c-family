import { rpcBackend, type Rpc } from './http';
import type { Backend, EventName, Events } from './types';

// Das Fenster der Rust-Fassung (Tauri 2, withGlobalTauri, kein npm-Paket): jede Methode läuft über den Command `rpc`
// mit derselben Aufruf-Tabelle wie die HTTP-API, Ereignisse kommen über `event.listen`. Ein Ereignis vor dem Listener
// geht verloren; die Oberfläche holt den Stand beim Start per `info`.

interface TauriGlobal {
  core: { invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> };
  event: { listen<T>(name: string, fn: (e: { payload: T }) => void): Promise<() => void> };
}

const tauri = (): TauriGlobal | undefined => (window as unknown as { __TAURI__?: TauriGlobal }).__TAURI__;

export function hasTauri(): boolean {
  return tauri() !== undefined;
}

/** Ein abgelehnter Command liefert den Fehlertext; die Seiten erwarten wie bei den anderen Transporten ein `Error`. */
const failed = (e: unknown): never => {
  throw new Error(String(e));
};

export function tauriBackend(): Backend {
  const t = tauri()!;
  const rpc: Rpc = <T>(method: string, ...args: unknown[]) => t.core.invoke<T>('rpc', { method, args }).catch(failed);
  const on = <E extends EventName>(event: E, fn: (data: Events[E]) => void) => {
    let off: (() => void) | undefined;
    let dead = false;
    void t.event
      .listen<Events[E]>(event, (e) => {
        if (!dead) fn(e.payload);
      })
      .then((unlisten) => {
        if (dead) unlisten();
        else off = unlisten;
      });
    return () => {
      dead = true;
      off?.();
      off = undefined;
    };
  };
  return rpcBackend(rpc, on, (url) => {
    void t.core.invoke('open_url', { url }).catch((e: unknown) => console.error('open_url', e));
  });
}
