import { httpBackend } from './http';
import { mockBackend } from './mock';
import type { Backend } from './types';
import { hasWails, wailsBackend } from './wails';

export type * from './types';

/** `?mock=1` oder ein Testlauf (Vitest): erfundene Daten statt k3c-dev (wie der Starter). */
function wantsMock(): boolean {
  return import.meta.env.MODE === 'test' || new URLSearchParams(window.location.search).get('mock') === '1';
}

/** Das Backend dieser Sitzung: Wails-Bindings im Fenster, Mock mit `?mock=1` und in Tests, sonst k3c-dev -headless/-dev über HTTP. */
export const backend: Backend = hasWails() ? wailsBackend() : wantsMock() ? mockBackend() : httpBackend();
