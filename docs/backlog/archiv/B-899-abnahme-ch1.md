# B-899 · Abnahme CH1: Sichtprüfung der Entwickler-Tasten, Cheats, Debug-Werte und HUD-Icons und Gen-Läufe sind erledigt

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** verworfen
- **Sprint:** CH1
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint CH1 (B-888, B-889, B-890) ist umgesetzt und reviewt; offen sind die Sichtprüfungen durch 🧑 (B-888/AC-03, B-889/AC-05, B-890/AC-03) und die Gen-Läufe am Sprint-Ende (`docs/arbeitsweise.md` › Sim, Balance und Gen). Sim und Balancing sind gesperrt (Beschluss 2026-11-11).

## Ziel

CH1 ist abgenommen.

## Beteiligte und Zielgruppen

- 🧑 Ralf: Sichtprüfung am PC und TV.

## Anforderungen

- 🧑 prüft im nativen Spiel (`task native:game`): F1 Übersicht, F5/F6, F7 Pause und Einzelschritt, F8 Zeitraffer, F9 Cheat-Menü (Tastatur und Pad View + Y, zwei Spieler), F10 Geometrie und Werte (Klick wählt Objekt), F11 Vollbild, F12 Screenshot in `reports/`.
- 🧑 prüft die HUD-Icons auf dem TV im Split-Screen.
- Nach einem Spiel liegt `reports/native-session-*.json`; Logs ohne Spam je Takt.
- `task gen:paper` samt Grafik-Golden und `task check:race` einmal grün.

## Nicht-Ziele

Neue Funktionen; Befunde werden eigene Tickets.

## Regeln und Einschränkungen

Gen nur im Abnahme-Ticket (B-742); Sim und Balancing bleiben gesperrt.

- **Bereich:** `native/k3c-game/**`
- **Abhängig von:** B-888, B-889, B-890
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** golden

## Beispiele

nicht relevant: Abnahme ohne neue Logik.

## Ausnahme- und Fehlerfälle

Golden-Abweichung bei `gen:paper` → Ursache klären (beim Nachweis-Lauf in CH1.6 wichen `golden.json`/`index.paper.json` mehrerer Gruppen schon ab, vermutlich vorbestehend), kein blindes Neuschreiben.

## Akzeptanzkriterien

- **AC-01** Sichtprüfung durch 🧑 dokumentiert (je Taste und HUD ✅ oder Ticket).
- **AC-02** `task gen:paper` ohne Golden-Abweichung, `task check:race` grün (ohne gcc: als übersprungen nennen).

## Offene Fragen

Verworfen (B-1143, 2026-11-19, autonom, Claude): Die Sichtprüfung gilt dem Go-Spiel, das mit RU37 entfällt; Entwickler-Tasten und Cheats im Rust-Spiel prüft RU56, den Gesamteindruck die Abnahme am PC (RU36).

## Notizen

Quelle: CH1.7.
