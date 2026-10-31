// Zoom der Bühne (Assets-Seite): feste Stufen um den Spielmaßstab (1 = Mini-Szene ×2 wie im Spiel), „Anpassen“ rechnet
// den Maßstab, mit dem die ganze Szene in die Bühne passt.

export const ZOOMS = [0.25, 0.5, 0.75, 1, 1.5, 2, 3, 4] as const;
export const ZOOM_MIN = ZOOMS[0];
export const ZOOM_MAX = ZOOMS[ZOOMS.length - 1];

/** Nächste Stufe nach oben (+1) bzw. unten (−1); ein Zwischenwert (nach „Anpassen“) springt auf die nächste feste Stufe. */
export function stepZoom(zoom: number, dir: -1 | 1): number {
  if (dir > 0) return ZOOMS.find((z) => z > zoom + 1e-6) ?? ZOOM_MAX;
  return [...ZOOMS].reverse().find((z) => z < zoom - 1e-6) ?? ZOOM_MIN;
}

/** Maßstab, mit dem `content` ganz in `box` passt (abzüglich Rand), geklemmt auf die Stufen; ohne Maße bleibt 1. */
export function fitZoom(content: { w: number; h: number }, box: { w: number; h: number }, pad = 16): number {
  if (content.w <= 0 || content.h <= 0 || box.w <= pad || box.h <= pad) return 1;
  const z = Math.min((box.w - pad) / content.w, (box.h - pad) / content.h);
  return Math.min(ZOOM_MAX, Math.max(ZOOM_MIN, Math.floor(z * 100) / 100));
}

/** Anzeige des Maßstabs in Prozent („150 %“). */
export const zoomLabel = (zoom: number): string => `${Math.round(zoom * 100)} %`;

/** Gemerkter Zoom als Text: unlesbar oder außerhalb der Stufen gilt 1. */
export function parseZoom(raw: string): number {
  const n = Number(raw);
  return raw.trim() !== '' && Number.isFinite(n) && n >= ZOOM_MIN && n <= ZOOM_MAX ? n : 1;
}
