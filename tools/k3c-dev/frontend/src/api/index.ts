import { mockBackend } from './mock';
import type { Backend } from './types';
import { hasWails, wailsBackend } from './wails';

export type * from './types';

/** Das Backend dieser Sitzung: Wails-Bindings im Fenster, sonst der Mock. */
export const backend: Backend = hasWails() ? wailsBackend() : mockBackend();
