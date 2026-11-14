# SO4 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Musik je Zustand
- **Projekt:** SND

## Ziel

Die Musik wechselt je Spielzustand mit Crossfade. Am Ende sichtbar: Die 8 Zustände Tag, Abend (= Dämmerung, Q65), Nacht, Kampf, Tiefe/Höhle, Boss, Lobby und Niederlage/Sieg haben je 1–2 eigene Stücke, Warnungen senken die Musik.

## Ergebnis

- 2026-10-22, Review SO4.4: keine schweren Befunde; `task check` grün. AC-01, AC-04, AC-05, AC-06, AC-08, AC-09 geprüft (SO4.2, SO4.3, SO4.6). - 2026-10-24 (SO4.1, SO4.5, Ralf im Spiel): AC-02, AC-03, AC-07 abgenommen (Übergänge, Absenken, Lautheit ok, Gerät nicht genannt). Stückwahl von Ralf bestätigt (B-554), Morgengrauen = Nacht-Stück bis B-470, übrige Kandidaten entfernt. B-168 erledigt. - Version v0.20.0 (Minor: Musik im Spiel) vorgeschlagen, gesetzt erst nach Bestätigung durch 🧑.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Der Zustands-Automat bildet jeden der 8 Zustände auf 1–2 Stücke ab (Q16) (B-168/AC-01). |
| AC-02 | – | Der Wechsel Tag → Dämmerung → Nacht → Morgengrauen (Q65) ist hörbar ohne Pause und Knacken … |
| AC-03 | – | Ein Warn-Sound senkt die Musik ab und stellt sie wieder her (B-168/AC-03). |
| AC-04 | – | Jede Musik-Datei hat einen Credit-Eintrag, CC-BY-Stücke stehen auf der Credits-Seite (B-16 … |
| AC-05 | – | Die Lautstärke des Busses „Musik“ ist getrennt von den Effekten einstellbar (B-168/AC-05). |
| AC-06 | grün | `task check` ist grün. |
| AC-07 | offen | Die Lautheit der Musik ist am TV geprüft (Q16) (B-168/AC-06). |
| AC-08 | geprüft | Der Audio-Kern spielt ganze Dateien mit Crossfade, die Hörprobe nutzt ihn (B-250/AC-01; au … |
| AC-09 | geprüft | Die Optionen-Szene regelt auch die Lautstärke des Ambient-Busses (B-218/AC-01; aus SO5 AC- … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-168 | erledigt | AC-01 geprüft, AC-02 –, AC-03 –, AC-04 –, AC-05 –, AC-07 offen | Die Musik wechselt je Spielzustand mit Crossfade |
| B-250 | erledigt | AC-08 geprüft | Der Audio-Kern spielt ganze Dateien mit Crossfade, die Hörprobe nutzt ihn |
| B-218 | erledigt | AC-09 geprüft | Die Optionen-Szene regelt auch die Lautstärke des Ambient-Busses |
