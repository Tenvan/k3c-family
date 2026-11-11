# B-801 · Das native Spiel spielt Klänge und Musik zu den Ereignissen wie der Browser-Client

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NG8
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

## Ausgangslage

Der Browser-Client spielt Ton über `src/audio/` (`atlas.ts`, `audioCore.ts`, `events.ts`, `music.ts`, `ui.ts`) aus dem Sound-Atlas `public/audio/atlas.ogg` mit `atlas.json` (Start und Dauer je Slot) und Musik.

## Ziel

Paket `native/k3c-game/audio` spielt Klang-Effekte zu den Ereignissen des Frames, Menü-Klänge und Musik wie der Browser-Client.

## Beteiligte und Zielgruppen

Familie, Claude (Umsetzung), 🧑 Ralf (Hörprüfung NG9).

## Anforderungen

- Atlas laden (Ogg Vorbis), Slots nach `atlas.json`; Einzeldatei als Rückfall wie im Browser.
- Busse Effekte und Musik mit eigener Lautstärke (Optionen aus NG7 über `view`).
- Zuordnung Ereignis → Klang wie `events.ts`, Menü-Klänge wie `ui.ts`, Musik je Tageszeit oder Lage wie `music.ts`.
- Zuordnung als reine Funktion mit Tests.

## Nicht-Ziele

Neue Klänge oder Musik; Raum-Klang; Änderungen an `tools/soundatlas`.

## Regeln und Einschränkungen

Lizenzen wie `public/audio/README.md`. Ton-Dekoder aus Ebitengine (Festlegung in NG1, Sperre `deps` dort). Vertrag `view` unverändert.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `native/k3c-game/audio/**`
- **Abhängig von:** NG1
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –

## Beispiele

Monarch zahlt eine Münze → Münz-Klang; Nacht beginnt → Musik wechselt.

## Ausnahme- und Fehlerfälle

Kein Audio-Gerät → Spiel läuft stumm, eine Meldung im Log. Slot fehlt im Atlas → Einzeldatei, sonst stumm mit Meldung.

## Akzeptanzkriterien

- **AC-01** Atlas und Busse: Test lädt `atlas.json` und prüft Start und Dauer je Slot; Lautstärke je Bus wirkt (Test ohne Gerät).
- **AC-02** Jedes Ereignis aus `events.ts` und jeder Menü-Klang aus `ui.ts` hat dieselbe Zuordnung nativ (Test über alle Einträge); Musikwechsel nach `music.ts` (Test).

## Offene Fragen

keine

## Notizen

Angelegt 2026-11-11 bei der Planung der Umsetzung (NG8).
