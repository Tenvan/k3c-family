# B-761 · Renderer-Info (WebGL/Canvas, FPS, Texturen) im Spiel anzeigen, WebGL erzwingbar

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** LST
- **Erstellt:** 2026-11-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Client startet mit `Phaser.AUTO` (`src/main.ts`): WebGL, bei Fehlschlag stiller Canvas-Fallback. Auf der Xbox (Edge) ist die Leistung schlecht; ob WebGL oder Canvas läuft, ist im Spiel nicht sichtbar (nur `perfRenderer` in der Gamepad-Testseite, `src/tools/gamepadTest.ts`). Das Debug-Overlay (`?dev=1`) zeigt FPS und Frametime, aber weder Renderer noch Texturen.

## Ziel

Am TV ist ohne Entwicklerwerkzeug sichtbar, ob WebGL oder Canvas rendert und wie schnell das Spiel ist; Canvas fällt rot auf. WebGL lässt sich zum Test erzwingen.

## Beteiligte und Zielgruppen

🧑 testet auf Xbox und Pi; Claude wertet die Anzeige aus.

## Anforderungen

- Technik-Anzeige im Spiel (HudScene), Standard aus, schaltbar im Optionsmenü (Eintrag „Technik-Anzeige“) oder mit `?gfx=1`.
- Zeigt Renderer (WebGL2/WebGL/Canvas), GPU-String, max. Texturgröße, FPS, Ø Frametime, Zahl Texturen, geschätzten Texturspeicher, Puffer- und Fenstergröße, devicePixelRatio.
- `?renderer=webgl|canvas` erzwingt den Renderer, `?aa=0` schaltet die WebGL-Kantenglättung ab; `powerPreference: high-performance`.

## Nicht-Ziele

Kein Senken der Rendergröße (Phaser 4 kennt keine `resolution`-Option; der Rückpuffer ist fest 1920×1080), keine Leistungsoptimierung selbst.

## Regeln und Einschränkungen

Kein Spiel-Zustand im Client, nur Anzeige. Optionen je Gerät im `localStorage`. B nicht belegt.

## Beispiele

Xbox ohne WebGL → rote Zeile „CANVAS-FALLBACK (langsam)“. Mit WebGL → grüne Zeilen mit GPU-Name.

## Ausnahme- und Fehlerfälle

Ohne `WEBGL_debug_renderer_info` steht die GPU als „unbekannt“. `?renderer=webgl` ohne WebGL-Unterstützung lässt Phaser mit Fehler abbrechen (gewollt, zeigt das Problem).

## Akzeptanzkriterien

- **AC-01** `src/core/renderInfo.test.ts` grün: Texturspeicher, GPU/Texturgröße, Zeilen für WebGL.
- **AC-02** `?renderer=` und `?aa=0` werden gelesen (Test).
- **AC-03** Canvas zeigt als erste Zeile „CANVAS-FALLBACK“ (Test).
- **AC-04** Menüeintrag „Technik-Anzeige“ schaltet die Anzeige, Standard aus (`optionsLogic.test.ts`, `settings.test.ts`).
- **AC-05** Abnahme am TV (🧑): Anzeige im Spiel sichtbar, Wert auf Xbox notiert.

## Offene Fragen

keine

## Notizen

Verdächtig im Code: `antialiasGL` ist in Phaser 4 standardmäßig an (MSAA auf 1080p), `failIfMajorPerformanceCaveat` aus (Software-WebGL wird akzeptiert, kann sehr langsam sein). Die Log-Zeile in `main.ts` prüfte `config.renderType` statt des tatsächlichen `renderer.type`.
