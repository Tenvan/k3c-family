import { useEffect, useRef, useState } from 'react';

// Zeichnen im Pixel-Look für die Szenen (B-299/B-459): Bilder laden, ganzzahlig skalieren, ohne Glättung.

export function ladeBild(src: string): Promise<HTMLImageElement> {
  return new Promise((ok, fail) => {
    const img = new Image();
    img.onload = () => ok(img);
    img.onerror = () => fail(new Error(src));
    img.src = src;
  });
}

/** 2D-Kontext ohne Glättung nach dem Setzen der Größe (setzen der Größe setzt den Kontext zurück). */
export function pixelContext(c: HTMLCanvasElement, w: number, h: number): CanvasRenderingContext2D | null {
  c.width = w;
  c.height = h;
  const g = c.getContext('2d');
  if (g) g.imageSmoothingEnabled = false;
  return g;
}

/** Bild `kacheln`-mal nebeneinander im Maßstab `scale`; `boden` > 0 rückt die Unterkante auf diese Bodenlinie. */
export async function bildZeichnen(c: HTMLCanvasElement, datei: string, scale: number, kacheln = 1, boden = 0): Promise<void> {
  const img = await ladeBild(datei);
  const w = img.width * scale;
  const g = pixelContext(c, w * kacheln, img.height * scale);
  for (let i = 0; g && i < kacheln; i++) g.drawImage(img, i * w, 0, w, c.height);
  if (boden > 0) c.style.marginTop = `${Math.max(0, boden - c.height)}px`;
}

/** Canvas, das `draw` zeichnet, sobald sich `key` ändert; lädt ein Bild nicht, steht der Pfad als Lücke da. */
export function Pixel({ draw, drawKey }: { draw: (c: HTMLCanvasElement) => Promise<void>; drawKey: string }) {
  const ref = useRef<HTMLCanvasElement>(null);
  const [missing, setMissing] = useState('');
  useEffect(() => {
    const c = ref.current;
    if (!c) return;
    let live = true;
    setMissing('');
    draw(c).catch((e: unknown) => live && setMissing(e instanceof Error ? e.message : String(e)));
    return () => {
      live = false;
    };
    // draw ändert sich mit jedem Rendern; maßgeblich ist drawKey
  }, [drawKey]);
  return (
    <>
      <canvas ref={ref} className="sz-canvas" hidden={missing !== ''} />
      {missing && <div className="sz-gap">Datei fehlt: {missing}</div>}
    </>
  );
}
