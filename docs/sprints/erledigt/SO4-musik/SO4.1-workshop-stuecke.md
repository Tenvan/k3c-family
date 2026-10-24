# SO4.1 · Workshop: Stücke je Zustand hören und wählen, Lautheit am TV

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** so4/1-workshop-stuecke
- **Abhängig von:** SO1.4, SO3.2
- **Tickets:** B-168
- **Kriterien:** –

## Ziel

🧑 hat für jeden der acht Zustände ein Stück (oder bei „kein Treffer“ eine bestätigte Lücke) gewählt und die Lautheit der Auswahl am TV geprüft; die Tabelle Zustand → Stück → Quelle → Lizenz steht in `docs/assets/sounds.md` (Abschnitt Musik) und ist die Grundlage von AC-01 in SO4.2.

## Kontext

- **Schon entschieden (nicht neu verhandeln), Beschluss Q16** (`docs/fragenkatalog.md` › Beschlüsse vom 2026-10-03): **acht Zustände** Tag, Abend, Nacht, Kampf, Tiefe/Höhle, Boss, Lobby, Niederlage/Sieg; **1–2 Stücke je Zustand**; Crossfade; Ducking bei Warnungen; Lautheit am TV prüfen. Q15: Retro/Chiptune, kindgerecht, **Nacht leise**; Quellen Kenney, OpenGameArt, freesound, nur CC0 oder CC-BY.
- **Offen für 🧑:** welches Stück je Zustand gefällt und zur Stimmung passt; ob „Niederlage/Sieg“ ein gemeinsames oder zwei Stücke sind (Q16 nennt den Zustand einmal, Anzahl 1–2) und ob die Stücke gleich laut klingen. Dauer etwa 45 Minuten. Die Auswahl erfolgt auf `soundtest.html` (SO3: Kandidatenliste aus `public/audio/kandidaten.json`, Abspielen und Crossfade-Probe über den Audio-Kern aus SO1), am besten am TV; am PC geht dasselbe im Reiter „Hörprobe“ der Sound-Seite des Asset-Studios (WZ6.4, gleiche Logik, Kandidaten aus den Sound-Slots). Der Agent läuft mit und trägt die Entscheidungen ein (wie GR2.2, F1.4).
- **Vorbereitung durch den Agenten (diese Datei verlangt sie):** vor dem Termin je Zustand höchstens drei Kandidaten aus den Quellen von Q15 in `public/audio/kandidaten.json` eintragen (Gruppe „Zustand“, Name, Datei, Quelle, Urheber, Lizenz; Format der Datei legt SO3.2 fest). Nur CC0 oder CC-BY, nur kurze, schleifenfähige Stücke, Dateien klein halten (Format laut SO1.2/B-166, angenommen mp3 mit Fallback). GothicVania-Packs enthalten Musik von Pascal Belisle (CC-BY, nicht CC0, `public/grafik/CREDITS.md`): kein Kandidat ohne ausdrückliche Lizenzprüfung. Keine Stücke ohne Lizenzangabe.
- **Zustände und was sie später auslöst** (für die Auswahl wissen, nicht hier bauen): Tag, Abend, Nacht = `world.cycle.phase` (`day`, `dusk`, `night`, `src/model/types.ts`; das Morgengrauen ist bis B-470 nur das Ende der Nacht); Kampf = Welle mit Gegnern (`wave`, `enemies`); Tiefe/Höhle = `biome.depth` (je Stufe, `data/biomes/`); Boss = Ereignis `bossSpawned` (Bosse liegen in `data/bosses.json`), Sieg und Niederlage über die Simulation bzw. das Ereignis `castleFallen`; Lobby = `src/scenes/LobbyScene.ts`.
- **Lautheit:** Die Auswahl hört 🧑 am TV auf gleicher Lautstärke; zu laute oder zu leise Stücke werden notiert (Pegel anpassen geschieht in SO4.4 über Bus-Gain, nicht an der Datei).

## Erlaubte Dateien

- `docs/assets/sounds.md` (Abschnitt Musik, neu oder ergänzt)
- `public/audio/kandidaten.json` und `public/audio/kandidaten/` (nur Kandidaten-Einträge und -Dateien)
- diese Datei (Ergebnis), Sprint-README (nur Tabelle der Sessions), `docs/backlog/` (nur Status und neue Tickets für Lücken)

## Nicht-Ziele

Zustands-Automat und Crossfade (SO4.2), Dateien, Credits und Aufräumen (SO4.3), Effekte (SO2), eigene Kompositionen, dynamische Layer.

## Schritte

1. Vorbereitung (Agent): Kandidaten je Zustand eintragen, Tabelle `Zustand → Stück → Quelle → Urheber → Lizenz → Status` anlegen (Status zunächst `offen`).
2. Termin: 🧑 hört auf `soundtest.html` je Zustand, wählt ein bis zwei Stücke oder bestätigt „Lücke“; Crossfade-Probe Tag → Abend → Nacht; Lautheit am TV prüfen.
3. Agent trägt die Entscheidungen mit Datum ein; Lücken fett mit Ticket (neue Tickets nach `docs/vorlagen/ticket.md`, Index in `docs/backlog/README.md`).
4. Ergebnis eintragen (Zahl Stücke, Lücken, Lautheits-Anmerkungen), `Status: fertig`.

## Fertig, wenn

- [x] Je Zustand steht „gewählt“ oder eine Lücke in der Tabelle, bestätigt von 🧑 (2026-10-23, KR4.1).
- [x] Lautheits-Anmerkungen: keine Auffälligkeit (SO4.5, 2026-10-24).

## Prüfen

Manuell durch 🧑 (Browser, `soundtest.html`, am TV).

## Ergebnis

Gewählt und bestätigt von 🧑 (KR4.1, 2026-10-23; Tabelle in `docs/assets/sounds.md` › Musik): Tag Happy Adventure, Abend The Old Tower Inn, Nacht Haunting Chiptune Loop, Kampf Pixel Sprinter, Tiefe/Höhle 8-Bit Cave Loop, Boss NES Shooter Boss, Lobby NES Shooter Map, Niederlage eigenes Stück (B-573), Sieg 8-Bit Victory Loop (noch ohne Auslöser).

Entschieden am 2026-10-24 (Ralf): Morgengrauen läuft das Nacht-Stück weiter bis zum Tag (bis B-470); das Niederlage-Stück bleibt; die übrigen Kandidaten sind entfernt (10 Dateien, ca. 13,5 MB, Credit-Zeilen und Slot-Einträge). Lautheit: im Spiel passten die Sounds, nichts aufgefallen (Gerät nicht genannt). Keine neuen Tickets.
