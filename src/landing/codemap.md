# src/landing/

## Responsibility

Presentation-Schicht der Shell (`index.html`): dauerhaft geöffnete Landingpage mit Kachel-Navigation, die alle anderen Seiten in einem Vollflächen-iframe hostet, damit Vollbild auf der Xbox über Seitenwechsel erhalten bleibt.

## Design

- Registry: `pages.ts` – `PAGES: PageEntry[]` (title, description, icon, `href` als String oder Funktion, `section` `play|about`, `primary`, `small`), `SECTIONS` (Abschnittsnamen), `newGameHref()` (Start-URL „Neues Spiel“ mit `fresh=1` und eigenem Spielstandnamen aus der Uhrzeit). Nur Spieler-Kacheln plus die kleine Kachel „Entwicklung“ (`dev.html`, B-335); Werkzeug-Seiten stehen in `src/tools/devTiles.ts`.
- Shell/Host: `landing.ts` rendert Kacheln, erzeugt das iframe bei jedem Öffnen neu und entfernt es beim Schließen (keine Verlaufseinträge). `isOpenable()` aus `src/core/shell.ts` filtert Hash und `open`-Nachrichten.
- Räumliche Navigation: nächste Kachel in Blickrichtung (Abstand in Richtung einfach, seitlicher Versatz doppelt gewichtet); Eingaben: Gamepad (D-Pad/Stick mit Repeat, A, Y, View+Menu), Tastatur, Maus. Fokus wird per eigener Klasse markiert und in `sessionStorage`/`FOCUS_KEY` gemerkt.
- `serverCheck.ts`: `needsServer(page)` (Abschnitt `play`), `NO_SERVER_HINT`, `showNoServer(parent)` – Spielkacheln ohne Go-Server (GitHub Pages) deaktiviert.

## Flow

1. `landing.ts` startet: baut Kacheln aus `PAGES`; `fetchServerBuild()` (`GET api/health`) → ohne Server werden `play`-Kacheln gesperrt, Kopfzeile zeigt `versionLine(CLIENT, server)`.
2. Auswahl (A/Enter/Klick) → `openPage` im iframe `#frame`; Direktlink `index.html#game.html?seed=abc` öffnet beim Laden dieselbe Seite.
3. Unterseite sendet `postMessage` (`SHELL_MESSAGE.home`/`open`/`fullscreen`): `home` schließt das iframe, `open` öffnet eine andere Seite, `fullscreen` → `toggleLocal()` und Antwort `fullscreenResult` zurück ins iframe.
4. Zurück-Falle fängt Browser-Zurück (B) ab; beim Zurückkommen gelten noch gehaltene Tasten als verbraucht.

## Integration

- Eingebunden von `index.html` (`/src/landing/landing.ts`).
- Abhängigkeiten: `src/core/shell.ts` (Nachrichten, `isOpenable`), `src/core/fullscreen.ts`, `src/core/version.ts`; Endpoint `api/health`; Ziele sind die `*.html` im Repo-Root (`game.html`, `lizenzen.html`, `dev.html`; die Werkzeug-Seiten erreicht man über `dev.html`).
