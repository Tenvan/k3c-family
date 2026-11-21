# B-1212 · Gamepad funktioniert auch, wenn es nach dem Spielstart eingeschaltet wird; die Beitreten-Meldung im HUD entfällt

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WL1
- **Projekt:** WIR
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Wird ein Gamepad erst nach dem Spielstart eingeschaltet und verbunden, wird es nicht sauber unterstützt (Meldung Ralf 2026-11-21). Zusätzlich soll die Meldung „Beitreten“-Tasten im HUD ganz entfallen.

## Ziel

Ein später verbundenes (und wieder getrenntes) Gamepad funktioniert wie ein beim Start vorhandenes; die Beitreten-Tasten-Meldung im HUD gibt es nicht mehr.

## Beteiligte und Zielgruppen

Ralf am PC mit Pad; Hauptrechner.

## Anforderungen

- **AC-01** Hotplug: Verbinden und Trennen zur Laufzeit (Lobby, Menü, Spiel) wird erkannt; ein Pad belegt einen freien Spielerplatz, wie beim Start; Trennen im Spiel führt zu keinem Absturz oder hängenden Spieler.
- **AC-02** Die Beitreten-Tasten-Meldung im HUD ist entfernt (Text und Zeichnen), Beitreten funktioniert weiter.
- Controller-Taste B geht im Menü zurück (unverändert).

## Nicht-Ziele

Neue Belegungen, Rumble.

## Regeln und Einschränkungen

2 Spieler gleichzeitig, eigene Eingabe pro Spieler. Annahme: „Beitreten-Tasten Meldung“ ist der Hinweis im HUD, wie man dem Spiel beitritt; die Lobby-Auswahl bleibt.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-game/src/input/**`, `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-platform/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme | Gerät
- **Größe:** S
- **Sperre:** –

## Beispiele

Spiel läuft ohne Pad, Pad einschalten: Pad ist nutzbar und tritt bei. Pad aus: Spieler bleibt steuerbar per Tastatur oder wartet.

## Ausnahme- und Fehlerfälle

Pad wird getrennt und gleich wieder verbunden: derselbe Platz.

## Akzeptanzkriterien

- **AC-01** Test mit simuliertem Hotplug (Geräte-Ereignisse) in Menü und Spiel; Sicht am Pad durch 🧑.
- **AC-02** Test: HUD-Zeilenliste enthält die Beitreten-Meldung nicht mehr.

## Offene Fragen

keine.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
