import type { Backend, ServerState } from './types';

// Die Wails-Laufzeit legt window.go (Bindungen der App) an. Die generierten Dateien unter frontend/wailsjs/ bindet die
// Oberfläche bewusst nicht ein: so braucht der Typecheck keine Wails-CLI.
interface GoApp {
  State(): Promise<ServerState>;
  Start(): Promise<void>;
  Stop(): Promise<void>;
}

declare global {
  interface Window {
    go?: { main?: { App?: GoApp } };
  }
}

export function hasWails(): boolean {
  return window.go?.main?.App !== undefined;
}

export function wailsBackend(): Backend {
  const app = window.go!.main!.App!;
  return { mock: false, state: () => app.State(), start: () => app.Start(), stop: () => app.Stop() };
}
