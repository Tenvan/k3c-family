# GR5.4 · Abnahme am TV

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Branch:** gr5/4-abnahme-tv
- **Abhängig von:** GR5.3
- **Tickets:** B-164
- **Kriterien:** AC-01, AC-02, AC-04

## Ziel

🧑 hat die Effekte auf der Xbox am TV gesehen: Treffer, Münzen, Tod und Bauen sichtbar, Screenshake nur in der Kamera des betroffenen Spielers, mit Screenshake und Blitz „aus“ ruhig, Vibration spürbar. Das Ergebnis steht in dieser Datei.

## Kontext

**Controller zurückgestellt (B-314, 2026-10-06):** 🧑 prüft vorerst nur am PC mit Tastatur und Maus (1 Spieler; 2 Spieler an einer Tastatur erst mit B-316). Controller-, Vibrations- und Xbox-Schritte dieser Session sind nach B-314 verschoben; die Kriterien bleiben, ihr Controller-Anteil gilt als `angenommen, Validierung offen (B-314)`.

Ein Agent nimmt diese Session nicht und bereitet sie nicht vor. Dauer etwa 15 Minuten. Hardware-Session nach `docs/arbeitsweise.md` › Hardware entkoppelt: keine Abhängigkeit des Reviews GR5.3. Bis zur Abnahme gelten die Sicht auf AC-01 und AC-04, der Blitz-Eindruck aus AC-02 und die Vibration als `angenommen, Validierung offen (GR5.4)`; die Tests decken die Logik schon ab.

## Erlaubte Dateien

- diese Datei (Ergebnis, Status)
- Sprint-README (nur Tabelle der Sessions und Abnahme), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Code-Änderungen; neue Effekte.

## Schritte

1. Stand im Heimnetz bereitstellen (`task serve` oder Pi), `index.html` auf der Xbox öffnen, Spiel mit zwei Controllern starten.
2. Treffer, Kill, Münze aufheben und geben, Bau fertig und Tod auslösen; in den Optionen Screenshake und Blitz ausschalten und wiederholen.
3. Ergebnis eintragen (Datum, Gerät, Befund); Mängel als Tickets; `Status: fertig` (bei Mängeln `blockiert`). Ist es die letzte offene Session, Sprint nach `erledigt/` verschieben.

## Fertig, wenn

- [ ] AC-01: 🧑 sieht am TV zu jedem Feedback-Event den Effekt.
- [ ] AC-02: 🧑 bestätigt, dass mit Screenshake und Blitz „aus“ beides nicht auftritt.
- [ ] AC-04: 🧑 sieht Screenshake im Split-Screen nur in der Kamera des betroffenen Spielers.

## Prüfen

Manuell durch 🧑 an der Xbox.

## Ergebnis

2026-10-06, **Tastatur und Maus (Sammelaussage):** 🧑 (Ralf) im Chat: „Alle Tastatur und Maussteuerungen liefen bisher wie definiert.“ Gilt für den Tastatur- und Maus-Anteil dieser Session am PC; Darstellung, Ton und Controller (B-314) sind damit nicht abgenommen, die Session bleibt `offen`.
