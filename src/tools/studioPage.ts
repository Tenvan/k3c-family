import { installPageChrome } from '../core/shell';
import { t } from '../core/texts';

installPageChrome();

// Das Asset-Studio (tools/k3c-studio/frontend, Build nach dist/studio/) ist ein Teil des Go-Servers und nur im Dev-Mode
// auf dem Rechner des Servers da. Die Seite zeigt es im Vollflächen-Rahmen; fehlt es, steht ein Hinweis.
const note = document.getElementById('note')!;
note.textContent = t('studio.loading');

async function studioAvailable(): Promise<boolean> {
  try {
    const res = await fetch('/api/studio/ping');
    // Unter dem Vite-Port liefert die Fallback-Seite 200: nur die JSON-Antwort des Studios zählt.
    return res.ok && ((await res.json()) as { result?: string }).result === 'ok';
  } catch {
    return false;
  }
}

void studioAvailable().then((ok) => {
  if (!ok) {
    note.textContent = t('studio.unavailable');
    return;
  }
  note.remove();
  const frame = document.createElement('iframe');
  frame.src = 'studio/index.html';
  frame.title = t('landing.studio');
  document.body.append(frame);
});
