# B-1180 · Die FPS-Anzeige oben rechts zeigt 0 FPS, ist zu gross und am Rand abgeschnitten

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** LA1
- **Projekt:** RSU
- **Erstellt:** 2026-11-21
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

🧑 2026-11-21 am PC: Die FPS-Anzeige oben rechts zeigt „0 FPS“, ist viel zu groß und am Bildrand abgeschnitten. Auch die Aufnahmen des Agenten (`--shots`) zeigen dort eine große, abgeschnittene „T <Takt>“-Anzeige (siehe `docs/messung/abnahme-b-1129.md`). Die Anzeige kommt aus dem Statistik-Overlay (`--stats`, F-Taste) im HUD; Ort noch nicht untersucht.

## Ziel

Die Leistungsanzeige zeigt die echten FPS in kleiner, lesbarer Schrift vollständig im Bild.

## Beteiligte und Zielgruppen

🧑 am PC.

## Anforderungen

- FPS-Wert stimmt (nicht 0), auch bei zwei Ansichten.
- Schriftgröße klein, Text bleibt innerhalb des Bildes bei 1280x720 bis 1920x1080.
- Test für Lage und Größe.

## Nicht-Ziele

Neue Messwerte.

## Regeln und Einschränkungen

HUD-Regeln aus `docs/standards/rust-port.md`; Datei ≤ 400 Zeilen.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/src/devtools/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

1080p: kleine Anzeige „60 FPS“ in der Ecke, ganz sichtbar.

## Ausnahme- und Fehlerfälle

Pause und Menü: zeigt weiter den letzten Wert, nicht 0.

## Akzeptanzkriterien

- **AC-01** Anzeige zeigt den gemessenen FPS-Wert (Test).
- **AC-02** Text liegt vollständig im Bild und ist klein (Test, Abnahme 🧑).

## Offene Fragen

keine

## Notizen

Ursache: (1) `Game::measure_fps` lief nur bei eingeschalteter zweiter Zeile (F2, `Settings::stats`), sonst blieb der Wert 0; jetzt läuft sie, sobald die Anzeige FPS zeigt (`StatsOverlay::shows_fps`). (2) `Game::draw` ruft die Anzeige ohne Schrift auf, die Pixel-Schrift wurde im Maßstab 5 gezeichnet und ragte über den Fensterrand: jetzt Maßstab 2 (`PIXEL_SCALE`), die Fläche passt zur längsten Zeile (`pixel_width`) und sitzt rechtsbündig. `stats_rect` bleibt unverändert, weil das HUD (und die Go-Golden `still_hud`) daran hängen. Tests: `pixel_flaeche_passt_zur_laengsten_zeile`, `fps_werden_ohne_zweite_zeile_gemessen`. Die Anzeige mit der Schrift des Spiels (statt Pixel-Schrift) bleibt offen. Abnahme 🧑 am PC steht aus.
