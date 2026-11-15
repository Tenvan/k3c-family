# tools/k3c-workbench/frontend/src/assets/sound/

## Responsibility

Teil **Sound** der Seite „Sound & Quellen“ im Bereich Studio (WB1.4, D-03): Sound-Slots aus `data/sounds.json` mit Status,
Hörprobe, Filter und Prompt-Buttons je Slot (B-728). Nur Ansehen; Zuordnen, Recherche und Import laufen über die MCP-Tools
`assets_*`.

## Design

- `SoundPage.tsx`: lädt die Slots einmal, filtert nach Text, Kategorie und Status und gruppiert nach Kategorie. Ein
  gewählter Slot zeigt `PromptButtons` mit den Vorlagen des Orts `sound` (`useVorlagen`). „Anhören“ holt die Datei als
  Data-URL und spielt sie im Player ab. Fehler stehen rot über der Liste (`errorText` aus `@webkit/lib/errors`).

## Flow

1. Mount -> `backend.studioSounds()` -> Liste der Slots.
2. „Anhören“ -> `backend.studioSoundData(pfad)` -> `<audio src>`.
3. Slot wählen -> `useVorlagen().jeOrt('sound')` -> `PromptButtons` (`../prompts/`).

## Integration

- Eingehängt von `../SoundSourcesPage.tsx` (Umschalter „Sound“/„Quellen“, gemerkt unter `studio.soundTeil`).
- Backend: Bindings `StudioSounds`, `StudioSoundData` (`tools/k3c-workbench/app_studio.go`, Logik in `internal/studioview`).
- Abhängigkeiten: `backend` aus `src/api`, `../prompts/`, `@webkit/atoms|molecules`.
