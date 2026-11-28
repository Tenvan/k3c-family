import { httpBackend } from './http';
import { mockBackend } from './mock';
import { hasTauri, tauriBackend } from './tauri';
import type { Backend } from './types';

export type * from './types';

/** `?mock=1` oder ein Testlauf (Vitest): erfundene Daten statt k3c-studio. */
function wantsMock(): boolean {
  return import.meta.env.MODE === 'test' || new URLSearchParams(window.location.search).get('mock') === '1';
}

/** Das Backend dieser Sitzung: Tauri-Commands im Fenster, Mock mit `?mock=1` und in Tests, sonst k3c-studio -headless/-dev über HTTP. */
export const backend: Backend = hasTauri() ? tauriBackend() : wantsMock() ? mockBackend() : httpBackend();
