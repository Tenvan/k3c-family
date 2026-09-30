import type { Backend, EventName, Events, Info } from './types';

// Die Wails-Laufzeit legt window.go (Bindings der App) und window.runtime an. Die generierten Dateien unter
// frontend/wailsjs/ bindet die Oberfläche bewusst nicht ein: so braucht der Typecheck keine Wails-CLI.
interface GoApp {
  Info(): Promise<Info>;
}

interface WailsRuntime {
  EventsOn(name: string, fn: (...data: unknown[]) => void): () => void;
}

declare global {
  interface Window {
    go?: { main?: { App?: GoApp } };
    runtime?: WailsRuntime;
  }
}

export function hasWails(): boolean {
  return window.go?.main?.App !== undefined && window.runtime !== undefined;
}

export function wailsBackend(): Backend {
  const app = window.go!.main!.App!;
  const runtime = window.runtime!;
  return {
    mock: false,
    info: () => app.Info(),
    on: <E extends EventName>(event: E, fn: (data: Events[E]) => void) =>
      runtime.EventsOn(event, (data) => fn(data as Events[E])),
  };
}
