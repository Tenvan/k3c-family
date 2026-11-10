# B-773 · Die Textur-Testseite passt responsiv auf einen Screen

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** LG1
- **Projekt:** LST
- **Erstellt:** 2026-11-10
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude (Nutzer: „LST ganz abarbeiten“, 2026-11-10)

## Ausgangslage

Die WebGL-Performance-Seite ist nicht sauber responsiv (Hinweis von 🧑, 2026-11-10). Gemeint ist nach Annahme die Textur-Testseite `texturetest.html` (Kachel „Texturen“, Seiten > Performance, B-764). Der Inhalt braucht heute offenbar mehr als einen Screen. Auf der Xbox (TV) und im Panel (iframe unter der Leiste) ist Scrollen schlecht bedienbar.

## Ziel

Der gesamte Inhalt der Seite passt ohne Scrollen auf einen Screen, auf TV (Xbox), Desktop und kleinem Fenster, auch im Panel.

## Beteiligte und Zielgruppen

🧑 testet auf der Xbox; Entwickler nutzen die Seite am PC.

## Anforderungen

- Layout skaliert mit der Viewport-Größe (Breite und Höhe), Inhalt in einem Raster, das den Screen füllt, ohne vertikalen Seitenscroll.
- Die Diagnose (7 Kontext-Versuche, Renderer, maxTex, Messwerte) bleibt vollständig lesbar; zu viel Inhalt wird kompakter dargestellt (Spalten, kleinere Zeilenhöhe), nicht abgeschnitten.
- Oben ca. 70 px frei für den Home-Button (`installPageChrome()`), siehe CLAUDE.md Regel Seiten & Navigation.
- Die Prüfung gilt auch für die Schwesterseiten `stress.html` und `monitor.html`, sofern sie denselben Mangel zeigen (zuerst nur prüfen, Umbau nur bei Befund).

## Nicht-Ziele

Keine neue Messung, keine neue Funktion, kein Umbau der Diagnose-Logik (B-769).

## Regeln und Einschränkungen

Rahmen laut `src/core/shell.ts`; Vollbild nur über `src/core/fullscreen.ts`; Pad-Bedienung (B zurück) bleibt; Datei ≤ 400 Zeilen.

## Beispiele

- 1920×1080 (TV): alle Blöcke sichtbar, kein Scrollbalken.
- 1280×720 und 800×600: Inhalt rückt zusammen, nichts überläuft den Rand.

## Ausnahme- und Fehlerfälle

- Sehr kleines Fenster (unter 640×480) → Scrollen erlaubt, aber nichts abgeschnitten.
- Lange Fehlertexte aus `webglcontextcreationerror` → umbrechen oder kürzen mit Volltext im Bericht.

## Akzeptanzkriterien

- **AC-01** Test: Das Layout setzt Container auf Viewport-Höhe ohne Seitenscroll (Gerendert bei 1920×1080, 1280×720, 800×600; Prüfung `scrollHeight <= clientHeight`).
- **AC-02** Test: Home-Button-Abstand und `installPageChrome()` bleiben (`tests/projectRules.test.ts`).
- **AC-03** Abnahme (🧑): Auf der Xbox passt die Seite auf einen Screen, ohne zu scrollen.

## Offene Fragen

- Ist wirklich `texturetest.html` gemeint, oder `stress.html`/`monitor.html`? Default: `texturetest.html`, die anderen werden geprüft.

## Notizen

Quelle: 🧑 am 2026-11-10. Verwandt: B-764, B-769.
