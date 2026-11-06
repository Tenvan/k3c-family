// Getypte Aufrufe der Asset-RPC für die Review-Seite: im Wails-Fenster über die Bindings, sonst
// POST /api/dev/rpc/<Methode> (wie api/http.ts). Eigener Transport, bis WB1 backend.assets* liefert (B-701).
import type { Frame, PixelDiff, Review, Spec, Vorlage } from './types';

type Methode = (...args: unknown[]) => Promise<unknown>;

function wails(): Record<string, Methode> | undefined {
  return (window as unknown as { go?: { main?: { App?: Record<string, Methode> } } }).go?.main?.App;
}

async function rpc<T>(method: string, ...args: unknown[]): Promise<T> {
  const app = wails();
  if (app?.[method]) return (await app[method](...args)) as T;
  const res = await fetch(`/api/dev/rpc/${method}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json', 'X-K3C-Dev': '1' },
    body: JSON.stringify(args),
  });
  const body = (await res.json().catch(() => ({ error: `Fehler ${res.status} von k3c-workbench` }))) as { result?: T; error?: string };
  if (!res.ok || body.error) throw new Error(body.error || `Fehler ${res.status} von k3c-workbench`);
  return body.result as T;
}

export const reviewApi = {
  review: (basis: string) => rpc<Review>('AssetsReview', basis),
  frame: (id: string, animation: string, frame: number, variante: string, basis: string) =>
    rpc<Frame>('AssetsFrame', id, animation, frame, variante, basis),
  pixelDiff: (id: string, animation: string, frame: number, basis: string) => rpc<PixelDiff>('AssetsPixelDiff', id, animation, frame, basis),
  get: (id: string) => rpc<Spec>('AssetsGet', id),
  prompts: () => rpc<Vorlage[]>('AssetsPrompts'),
};
