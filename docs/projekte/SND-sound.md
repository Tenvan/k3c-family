# SND · Sound

- **Status:** aktiv
- **Rang:** 6
- **Ziel-Tickets:** B-011

## Ziel

Das Spiel hat Soundeffekte und Musik je Zustand auf einem Audio-Kern mit Lautstärke je Bus (Ziel-Ticket B-011).

## Sprints

In Abarbeitungs-Reihenfolge; der erste nicht erledigte Sprint ist der nächste. Jeder Sprint nennt das Projekt im Feld `Projekt`.

| Sprint | Thema | Status |
|---|---|---|
| SO1 | Audio-Kern | erledigt |
| SO3 | Hörprobenseite `soundtest.html` | erledigt |
| SO2 | SFX-Katalog und Einbau | aktiv |
| SO4 | Musik je Zustand | aktiv |
| SO6 | Sound-Credits auf der Credits-Seite, Sound-Atlas und Sounds für die übrigen Ereignisse | aktiv |
| SO7 | Niederlage-Stück, Hufschlag, UI-Klicks und Skill-Sounds je Klasse | erledigt |

## Nicht-Ziele

Das Asset-Studio und seine Sound-Seite als Werkzeug (WZG, WZ6); Figuren, Gebäude und Grafik (GRA, GEB); der Tageszyklus mit Morgengrauen als Phase (B-470, SIM).

## Notizen

Stand 2026-10-18: Die Sound-Slots werden im Asset-Studio verwaltet (Sound-Seite mit Reiter „Hörprobe“, WZ6.4); `soundtest.html` bleibt für die Auswahl am TV (Gamepad), die Logik teilen beide (`src/tools/soundtestLogic.ts`). SO2 (SFX-Katalog) ist noch nicht gestartet, SO4 (Musik) läuft mit SO4.2 und SO4.6 fertig.

Morgengrauen: Das Musikstück dafür ist eine offene Frage von SO4. Die Phase selbst baut B-470 (WRT, SIM); bis dahin endet die Nacht mit dem Ereignis `dawn`, der Client zeichnet das Morgengrauen als letzte 10 % der Nacht.
