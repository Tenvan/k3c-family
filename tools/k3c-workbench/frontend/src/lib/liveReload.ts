// Nachladen bei Live-Ereignissen ohne Sturm (B-895): Im Fenster kommt jede Antwort als eigenes Skript (Wails,
// ExecuteScript). Eine Seite, die bei jedem Ereignis große Listen neu holt, schiebt so Hunderte KiB je Sekunde durch
// den WebView2-Renderer, der dabei stetig wuchs. liveReload lädt höchstens einmal je Takt und gar nicht,
// solange das Fenster verborgen ist; beim Sichtbarwerden holt es ein verpasstes Nachladen einmal nach.

/** Ausschnitt von document, den liveReload braucht (im Test ersetzbar). */
export interface Visibility {
  readonly hidden: boolean;
  addEventListener(type: 'visibilitychange', fn: () => void): void;
  removeEventListener(type: 'visibilitychange', fn: () => void): void;
}

export interface LiveReload {
  /** Wunsch nachzuladen; mehrere Wünsche innerhalb eines Takts ergeben ein Nachladen. */
  soon: () => void;
  stop: () => void;
}

/** Bündelt Nachlade-Wünsche: das erste nach einer Pause sofort, weitere höchstens einmal je `everyMs`. */
export function liveReload(run: () => void, everyMs: number, doc: Visibility = document, now = () => Date.now()): LiveReload {
  let timer: ReturnType<typeof setTimeout> | undefined;
  let last = -Infinity;
  let missed = false;
  const fire = () => {
    timer = undefined;
    if (doc.hidden) {
      missed = true;
      return;
    }
    missed = false;
    last = now();
    run();
  };
  const soon = () => {
    if (timer === undefined) timer = setTimeout(fire, Math.max(0, last + everyMs - now()));
  };
  const onVisible = () => {
    if (!doc.hidden && missed) soon();
  };
  doc.addEventListener('visibilitychange', onVisible);
  return {
    soon,
    stop: () => {
      clearTimeout(timer);
      timer = undefined;
      doc.removeEventListener('visibilitychange', onVisible);
    },
  };
}
