import { clearSizeCache } from '../size/sizeCache';
import { reviewApi } from './api';
import type { Frame, PixelDiff, Review, Spec } from './types';

/** Review-Cache (B-732): Ergebnis je Basis, Specs, Frames und Pixel-Diffs leben auf Modul-Ebene und überleben den Abbau
 *  der Seite beim Tab-Wechsel. „Neu laden“ leert alles (`clearReviewCache`). Ein Fehler bleibt nicht im Cache des
 *  Ergebnisses, damit der nächste Blick neu fragt; Frame- und Diff-Fehler bleiben bis zum Neuladen (wie bisher je Ansicht). */
const reviews = new Map<string, Promise<Review>>();
const fertig = new Map<string, Review>();
export const specCache = new Map<string, Promise<Spec>>();
export const frameCache = new Map<string, Promise<Frame>>();
export const diffCache = new Map<string, Promise<PixelDiff>>();

/** Fertiges Ergebnis einer Basis, sofort ohne Warten (undefined = noch nicht geladen). */
export function cachedReview(basis: string) {
  return fertig.get(basis);
}

/** Lädt das Review einer Basis einmal; ein zweiter Aufruf bekommt dasselbe Promise. */
export function loadReview(basis: string): Promise<Review> {
  let p = reviews.get(basis);
  if (!p) {
    p = reviewApi.review(basis);
    reviews.set(basis, p);
    p.then(
      (r) => reviews.get(basis) === p && fertig.set(basis, r),
      () => reviews.get(basis) === p && reviews.delete(basis),
    );
  }
  return p;
}

/** Spec aus dem Cache, sonst vom Backend. */
export function loadSpec(id: string): Promise<Spec> {
  let p = specCache.get(id);
  if (!p) {
    p = reviewApi.get(id);
    specCache.set(id, p);
    p.catch(() => specCache.get(id) === p && specCache.delete(id));
  }
  return p;
}

/** Leert den Cache; das letzte Ergebnis darf die Seite weiter zeigen, bis das neue da ist. */
export function clearReviewCache() {
  reviews.clear();
  fertig.clear();
  specCache.clear();
  frameCache.clear();
  diffCache.clear();
  clearSizeCache();
}
