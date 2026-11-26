import { mockBackend } from './mock';
import type { Backend, EventName, Events, Info } from './types';

// RU65.1 (Spike, nur in .work/spike-tauri): Backend im Tauri-Fenster. `info` und alle Ereignisse kommen echt aus Rust
// (invoke/listen über window.__TAURI__, withGlobalTauri, kein npm-Paket), die übrigen Seiten zeigen weiter den Mock.

interface TauriGlobal {
  core: { invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> };
  event: { listen<T>(name: string, fn: (e: { payload: T }) => void): Promise<() => void> };
}

const tauri = (): TauriGlobal | undefined => (window as unknown as { __TAURI__?: TauriGlobal }).__TAURI__;

export function hasTauri(): boolean {
  return tauri() !== undefined;
}

/** Belege für das Log: UI bereit, jedes Ereignis von Rust wird quittiert. */
function probe(t: TauriGlobal): void {
  void t.core.invoke('ui_ready', { perfMs: performance.now(), href: window.location.href });
  for (const name of ['mcp:state', 'ui:notify', 'spike:pong']) {
    void t.event.listen(name, (e) => void t.core.invoke('ui_event_ack', { event: name, payload: e.payload }));
  }
}

export function tauriBackend(): Backend {
  const t = tauri()!;
  probe(t);
  (window as unknown as { spikePing: (msg: string) => Promise<string> }).spikePing = (msg) => t.core.invoke('spike_ping', { msg });
  return {
    ...mockBackend(),
    mock: false,
    info: () => t.core.invoke<Info>('info'),
    on: <E extends EventName>(event: E, fn: (data: Events[E]) => void) => {
      let off: (() => void) | undefined;
      let dead = false;
      void t.event.listen<Events[E]>(event, (e) => fn(e.payload)).then((u) => (dead ? u() : (off = u)));
      return () => {
        dead = true;
        off?.();
      };
    },
  };
}
