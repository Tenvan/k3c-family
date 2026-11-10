import type Phaser from 'phaser';
import { clientLog } from './clientLog';
import { ContextGuard, canvasRestartHref, type GuardEnv } from './contextGuard';
import { openPage } from './shell';
import { rendererRequest } from './renderInfo';
import { t } from './texts';
import { runSearch } from './runConfig';

/** Name der Seite für `openPage`: `name.html`, sonst `game.html` (ein Server ohne Endung in der Adresse) */
function pageName(): string {
  const last = location.pathname.split('/').pop() ?? '';
  return /^[\w.-]+\.html$/.test(last) ? last : 'game.html';
}

/** Hinweis-Leiste unten im Fenster, unabhängig von Phaser (der Renderer kann ausgefallen sein) */
function showNotice(text: string): void {
  let el = document.getElementById('k3c-ctx-notice');
  if (!el) {
    el = document.createElement('div');
    el.id = 'k3c-ctx-notice';
    el.setAttribute('role', 'status');
    el.style.cssText = 'position:fixed;left:0;right:0;bottom:0;z-index:10000;padding:12px 16px;background:#7a1f1f;color:#fff;font:600 20px system-ui,sans-serif;text-align:center';
    document.body.append(el);
  }
  el.textContent = text;
}

/**
 * Hängt den Schutz vor Kontextverlust an das Spiel (nur WebGL; mit `?renderer=canvas` gibt es keinen Kontext zu verlieren).
 * `roomCode` liefert den offenen Raum, damit der Neustart ihm wieder beitritt.
 */
export function installContextGuard(game: Phaser.Game, roomCode: () => string | null): ContextGuard | null {
  if (rendererRequest(runSearch()) === 'canvas') return null;
  const env: GuardEnv = {
    read: (k) => {
      try {
        return sessionStorage.getItem(k);
      } catch {
        return null;
      }
    },
    write: (k, v) => {
      try {
        sessionStorage.setItem(k, v);
      } catch {
        // ohne Sitzungsspeicher bleibt es beim Versuch; Canvas verliert keinen Kontext, eine Schleife entsteht nicht
      }
    },
    now: () => Date.now(),
    after: (fn, ms) => {
      const id = setTimeout(fn, ms);
      return () => clearTimeout(id);
    },
    restartWithCanvas: () => openPage(canvasRestartHref(pageName(), runSearch(), roomCode())),
    notice: (kind) => showNotice(t(kind === 'restart' ? 'ctx.restart' : 'ctx.canvas')),
    log: (msg, ctx) => clientLog('warn', msg, ctx),
  };
  const guard = new ContextGuard(env);
  const watch = (): void => {
    game.canvas.addEventListener('webglcontextlost', () => guard.lost());
    game.canvas.addEventListener('webglcontextrestored', () => guard.restored());
  };
  if (game.isBooted) watch();
  else game.events.once('ready', watch);
  return guard;
}
