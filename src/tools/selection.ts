/**
 * Auswahl auf den Referenzseiten (figuren.html, aufstellung.html, grafiken.html): Jede Figur, jedes Pack und jedes Bild hat ein Kästchen mit stabiler ID.
 * Unten steht die Auswahl als Satz zum Kopieren („Auswahl Figuren: goblin, skeleton“), damit man im Chat einfach sagen kann, was mit welcher Grafik geschehen soll.
 * Die Auswahl bleibt im Browser (localStorage) erhalten.
 */

/** Text der Auswahl; leer, wenn nichts gewählt ist. IDs ohne Doppelte, in der Reihenfolge der Wahl. */
export function formatSelection(label: string, ids: readonly string[]): string {
  const unique = [...new Set(ids)];
  return unique.length ? `Auswahl ${label}: ${unique.join(', ')}` : '';
}

/** Gespeicherte IDs lesen; kaputte oder fremde Werte ergeben eine leere Liste. */
export function parseSaved(raw: string | null): string[] {
  try {
    const value: unknown = JSON.parse(raw ?? '[]');
    return Array.isArray(value) ? value.filter((v): v is string => typeof v === 'string') : [];
  } catch {
    return [];
  }
}

export interface Selection {
  /** Kästchen für eine ID; gleiche IDs auf einer Seite bleiben gleich geschaltet. */
  box(id: string): HTMLLabelElement;
}

const STYLE = `
.k3c-pick { position: absolute; top: 0.9rem; left: 0.9rem; z-index: 1; display: inline-flex; gap: 0.35rem; align-items: center; cursor: pointer;
  background: rgba(12, 16, 36, 0.88); color: #e8edf4; padding: 0.15rem 0.7rem; border-radius: 999px; font: 600 0.85rem system-ui, sans-serif; }
.k3c-pick input { accent-color: #ffd166; width: 1.1em; height: 1.1em; }
.k3c-bar { position: fixed; left: 0; right: 0; bottom: 0; z-index: 5; display: flex; gap: 0.75rem; align-items: center; padding: 0.6rem 1.25rem;
  background: #0b0e13; border-top: 1px solid #2e3a4d; font: 1rem system-ui, sans-serif; }
.k3c-bar textarea { flex: 1; height: 3.4rem; background: #1c2330; color: #e8edf4; border: 1px solid #2e3a4d; border-radius: 0.4rem; padding: 0.4rem; font: 0.85rem ui-monospace, monospace; }
.k3c-bar button { font: inherit; background: #ffd166; color: #11151c; border: 0; border-radius: 0.4rem; padding: 0.5rem 1rem; cursor: pointer; }
.k3c-bar button.k3c-clear { background: #2e3a4d; color: #e8edf4; }
.k3c-bar button:focus-visible { outline: 3px solid #8ecae6; }
`;

/** Baut die Leiste unten und liefert die Kästchen. `key` trennt die Seiten im localStorage, `label` steht im Auswahltext. */
export function installSelection(key: string, label: string): Selection {
  const ids = new Set(parseSaved(safeGet(key)));
  const boxes = new Map<string, HTMLInputElement[]>();

  const style = document.createElement('style');
  style.textContent = STYLE;
  const out = document.createElement('textarea');
  out.readOnly = true;
  out.placeholder = 'Kästchen „Auswahl“ ankreuzen, der Text erscheint hier zum Kopieren';
  const copy = document.createElement('button');
  copy.type = 'button';
  copy.textContent = 'Kopieren';
  const clear = document.createElement('button');
  clear.type = 'button';
  clear.className = 'k3c-clear';
  clear.textContent = 'Leeren';
  const bar = document.createElement('div');
  bar.className = 'k3c-bar';
  bar.append(out, copy, clear);
  document.head.append(style);
  document.body.append(bar);
  document.body.style.paddingBottom = '6rem';

  const refresh = () => {
    out.value = formatSelection(label, [...ids]);
    safeSet(key, JSON.stringify([...ids]));
  };
  const set = (id: string, on: boolean) => {
    if (on) ids.add(id);
    else ids.delete(id);
    boxes.get(id)?.forEach((b) => (b.checked = on));
    refresh();
  };
  copy.addEventListener('click', () => {
    out.select();
    void navigator.clipboard?.writeText(out.value);
  });
  clear.addEventListener('click', () => [...ids].forEach((id) => set(id, false)));
  refresh();

  return {
    box(id: string): HTMLLabelElement {
      const input = document.createElement('input');
      input.type = 'checkbox';
      input.checked = ids.has(id);
      input.addEventListener('change', () => set(id, input.checked));
      boxes.set(id, [...(boxes.get(id) ?? []), input]);
      const wrap = document.createElement('label');
      wrap.className = 'k3c-pick';
      wrap.title = `ID: ${id}`;
      wrap.append(input, 'Auswahl');
      return wrap;
    },
  };
}

function safeGet(key: string): string | null {
  try {
    return localStorage.getItem(key);
  } catch {
    return null;
  }
}

function safeSet(key: string, value: string): void {
  try {
    localStorage.setItem(key, value);
  } catch {
    // Speicher gesperrt: die Auswahl gilt bis zum Neuladen
  }
}
