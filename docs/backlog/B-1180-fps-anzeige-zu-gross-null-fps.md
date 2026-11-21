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

Gemeldet im Gespräch zu LA1.5.
