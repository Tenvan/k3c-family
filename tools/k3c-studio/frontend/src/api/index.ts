import { httpBackend, useMock } from './http';
import { mockBackend } from './mock';
import type { Backend } from './types';

export type * from './slotTypes';
export type * from './sourceTypes';
export type * from './suggestionTypes';
export type * from './types';
export type * from './worldTypes';
export { isStale, STALE } from './stale';

/** Das Backend dieser Sitzung: der Studio-Teil des Go-Servers, im Test und mit ?mock=1 der Mock. */
export const backend: Backend = useMock() ? mockBackend() : httpBackend();
