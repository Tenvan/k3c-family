# B-762 · Renderer-, Textur- und Leistungswerte im Client-Log (Start, Laden, Verlauf, Kontextverlust)

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** verworfen
- **Sprint:** RX1
- **Projekt:** LST
- **Erstellt:** 2026-11-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Technik-Anzeige (B-761, `src/core/renderInfo.ts`) zeigt Renderer, GPU, max. Texturgröße, Texturzahl und Texturspeicher nur auf dem Bildschirm. Ins Client-Log (`POST /api/clientlog`, `src/core/clientLog.ts`) gehen nur FPS und Latenz. Beim Xbox-WebGL-Fehler (dreimal 'WebGL Context lost' nach 3–4 s, Client 192.168.2.209) ließ sich vom Pi-Server aus nicht erkennen, woran es liegt.

## Ziel

Das Render-Verhalten jedes Clients ist ohne Blick auf den TV aus dem Server-Log lesbar: Start, Texturlast, Leistungsverlauf und jedes Renderer-Ereignis.

## Beteiligte und Zielgruppen

🧑 testet auf Xbox und Pi; Claude liest das Log über logs_query.

## Anforderungen

- **Start:** Nach dem Laden eine Zeile mit Renderer (WebGL2/WebGL/Canvas), GPU-String, max. Texturgröße, Texturzahl, geschätztem Texturspeicher, Puffer- und Fenstergröße, devicePixelRatio, User-Agent, Client-Version.
- **Ladevorgang:** Je Gruppe/Stufe Anzahl und Größe der geladenen Texturen und Ladezeit; fehlgeschlagene Ladevorgänge (Datei, Status) als Fehlerzeile.
- **Kontext-Ereignisse:** `webglcontextlost`, `webglcontextrestored` und ein Canvas-Fallback jeweils mit dem Zustand zum Zeitpunkt (Texturzahl, Texturspeicher, FPS, Zeit seit Start).
- **Leistungsverlauf:** Alle 30 s eine Zeile mit FPS (Ø und niedrigstem Wert), Ø Frametime, Texturspeicher und, wo der Browser es liefert, JS-Heap; bei FPS unter 20 sofort eine Warnzeile (höchstens eine je Minute).
- Wiederverwendung der Werte aus `renderInfo.ts`, keine zweite Messung; Gerätekennung (`device`) in jeder Zeile, damit sich Xbox und PC im Log trennen lassen.
- Emoji-Präfix laut CLAUDE.md; Mengen innerhalb des Rate-Limits von `/api/clientlog`.

## Nicht-Ziele

Keine neue Anzeige im Spiel, keine Leistungsoptimierung, kein eigener Metrik-Dienst; Log nur über den vorhandenen Weg `/api/clientlog`.

## Regeln und Einschränkungen

Kein Spiel-Zustand im Client; Rate-Limit von `/api/clientlog` beachten (eine Zeile je Ereignis).

## Beispiele

Xbox startet → '🚀 Renderer: WebGL2, GPU …, maxTex 4096, 212 Texturen, ~410 MiB'; danach alle 30 s '💓 FPS Ø 28, min 12, Texturen ~410 MiB'; bei Kontextverlust '💥 WebGL Context lost nach 3,8 s, 212 Texturen, ~410 MiB'.

## Ausnahme- und Fehlerfälle

Ohne `WEBGL_debug_renderer_info` steht die GPU als 'unbekannt'; Canvas loggt 'Canvas' ohne GPU-Werte.

## Akzeptanzkriterien

- **AC-01** Unit-Test: Die Startzeile enthält alle genannten Felder für WebGL und für Canvas.
- **AC-02** Unit-Test: Bei simuliertem Kontextverlust und -wiederherstellung wird je eine Zeile mit Zustand geschrieben.
- **AC-03** Unit-Test: Der Leistungsverlauf schreibt höchstens alle 30 s eine Zeile; die FPS-Warnung höchstens eine je Minute.
- **AC-04** Unit-Test: Ein fehlgeschlagener Textur-Ladevorgang erzeugt eine Fehlerzeile mit Datei und Status.
- **AC-05** Abnahme (🧑): Nach einem Xbox-Start sind Startwerte, Leistungsverlauf und Ladedaten im Server-Log des Pi lesbar.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Verworfen 2026-11-12 (Entscheidung Ralf): abgelöst durch B-823 (nur noch natives Spiel am PC, Entscheidung 009).
