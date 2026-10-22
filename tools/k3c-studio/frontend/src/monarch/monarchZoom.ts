/** Zoom der Monarch-Ansicht (B-502): 1 = alle Figuren im Bild (Spielmaßstab der Ansicht), nur die Ansicht skaliert. */
export const ZOOM_MIN = 0.5;
export const ZOOM_MAX = 4;
export const ZOOM_STEP = 1.25;

export const clampZoom = (z: number): number => Math.min(ZOOM_MAX, Math.max(ZOOM_MIN, z));

/** Zoom nach einem Schritt (`direction` > 0 vergrößert, < 0 verkleinert), auf 2 Stellen gerundet, in den Grenzen. */
export const stepZoom = (z: number, direction: number): number =>
  clampZoom(Math.round((direction > 0 ? z * ZOOM_STEP : z / ZOOM_STEP) * 100) / 100);

/** Alt + Mausrad zoomt (nach oben vergrößert), alles andere lässt das Rad der Seite. */
export const wheelZoom = (z: number, e: { altKey: boolean; deltaY: number }): number | null =>
  e.altKey && e.deltaY !== 0 ? stepZoom(z, -e.deltaY) : null;
