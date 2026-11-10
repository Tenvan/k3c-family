import { describe, expect, it } from 'vitest';
import { panelAction, releasesGame } from './panelHost';
import { SHELL_MESSAGE } from './shell';

describe('Nachrichten der Seite im Panel (B-768/AC-03)', () => {
  it('home schließt das Panel', () => {
    expect(panelAction({ type: SHELL_MESSAGE.home })).toEqual({ kind: 'close' });
  });

  it('eine andere Seite des Projekts ersetzt den Inhalt des Panels', () => {
    expect(panelAction({ type: SHELL_MESSAGE.open, href: 'monitor.html' })).toEqual({ kind: 'open', href: 'monitor.html' });
    expect(panelAction({ type: SHELL_MESSAGE.open, href: 'grafiken.html#kacheln' })).toEqual({ kind: 'open', href: 'grafiken.html#kacheln' });
  });

  it('eine Spiel-Adresse schließt das Panel und startet das Szenario im selben Dokument', () => {
    const action = panelAction({ type: SHELL_MESSAGE.open, href: 'game.html?autostart=1&fresh=1&save=test-x&mock=1' });
    expect(action).toEqual({ kind: 'run', params: { autostart: '1', fresh: '1', save: 'test-x', mock: '1' } });
  });

  it('Vollbild-Anfrage trägt die Kennung für die Antwort', () => {
    expect(panelAction({ type: SHELL_MESSAGE.fullscreen, id: 7 })).toEqual({ kind: 'fullscreen', id: 7 });
    expect(panelAction({ type: SHELL_MESSAGE.fullscreen })).toBeNull();
  });

  it('nicht öffnbare Ziele, unbekannte und ungültige Nachrichten werden ignoriert', () => {
    for (const href of ['../index.html', 'http://evil.example/x.html', '//evil.example/x.html', 'index.html', 'pfad/seite.html', 'x.js', '']) {
      expect(panelAction({ type: SHELL_MESSAGE.open, href }), href).toBeNull();
    }
    expect(panelAction({ type: SHELL_MESSAGE.open })).toBeNull();
    expect(panelAction({ type: 'k3c:other' })).toBeNull();
    expect(panelAction(null)).toBeNull();
    expect(panelAction('text')).toBeNull();
    expect(panelAction(undefined)).toBeNull();
  });
});

describe('Seiten, die den Speicher des Spiels brauchen (RX1)', () => {
  it('die Textur-Messung gibt das Spiel frei, auch mit Abfrage und Anker', () => {
    expect(releasesGame('texturetest.html')).toBe(true);
    expect(releasesGame('texturetest.html?x=1')).toBe(true);
    expect(releasesGame('/texturetest.html#a')).toBe(true);
  });

  it('andere Seiten lassen das Spiel im Speicher, ähnliche Namen zählen nicht', () => {
    expect(releasesGame('monitor.html')).toBe(false);
    expect(releasesGame('stress.html')).toBe(false);
    expect(releasesGame('texturetest.html.evil')).toBe(false);
    expect(releasesGame('xtexturetest.html')).toBe(false);
    expect(releasesGame('')).toBe(false);
  });
});
