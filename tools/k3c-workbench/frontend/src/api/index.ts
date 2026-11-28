import { httpBackend } from './http';
import { mockBackend } from './mock';
import { hasTauri, tauriBackend } from './tauri';
import type { Backend } from './types';
import { hasWails, wailsBackend } from './wails';

export type * from './types';

/** `?mock=1` oder ein Testlauf (Vitest): erfundene Daten statt k3c-workbench (wie der Starter). */
function wantsMock(): boolean {
  return import.meta.env.MODE === 'test' || new URLSearchParams(window.location.search).get('mock') === '1';
}

/** Das Backend dieser Sitzung: Tauri-Commands bzw. Wails-Bindings im Fenster, Mock mit `?mock=1` und in Tests, sonst k3c-workbench -headless/-dev über HTTP. */
export const backend: Backend = hasTauri() ? tauriBackend() : hasWails() ? wailsBackend() : wantsMock() ? mockBackend() : httpBackend();
