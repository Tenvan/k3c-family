# B-1003 · Der Welten-Editor wählt Wetter, Reittier und Figur einer Welt aus den Katalogen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** SI7
- **Projekt:** KMP
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Folge aus B-969, B-970 und B-972 (Ideenrunde k3c-07, 2026-11-15): Die Welt-Datei bekommt die optionalen Felder `weather`, `mount` und `hermit`, je eine ID aus einem Katalog (`data/weather.json`, `data/monarch.json` › `mounts`, `data/hermits.json`). 🧑 hat entschieden, dass neue Welten ihr Wetter im Welten-Editor aus der Liste wählen. Der Editor liegt in `tools/k3c-workbench/frontend/src/world/` (unter anderem `Overview.tsx`, `NewWorld.tsx`, `WorldValues.tsx`), das Backend in `tools/k3c-workbench/internal/world/`, die MCP-Tools heißen `world_get`, `world_set`, `world_check`.

## Ziel

Im Welten-Editor wählt man je Welt Wetter, Reittier und Figur aus den Katalogen oder „keins“, ohne die Welt-Datei von Hand zu bearbeiten.

## Beteiligte und Zielgruppen

Nutzer: 🧑 und Agenten beim Anlegen und Pflegen von Welten. Umsetzung: Claude in einer autonomen Session (SI7.1), Fachprüfung durch den Agenten `k3c-workbench`. Freigabe: 🧑.

## Anforderungen

- **Ablage (🧑 2026-11-15, B-969):** Die Welt-Datei nennt nur die Art oder „keins“. Neue Welten wählen im Welten-Editor aus der Liste.
- Die Übersicht einer Welt zeigt drei Auswahlfelder: Wetter, Reittier, Figur. Jede Liste kommt aus dem Katalog der Engine plus „keins“; nichts ist im Frontend fest verdrahtet.
- Speichern schreibt nur das jeweilige Feld der Welt-Datei (`weather`, `mount`, `hermit`); „keins“ entfernt das Feld.
- `world_set` nimmt die drei Felder an, `world_get` nennt sie, `world_check` meldet unbekannte IDs mit derselben Meldung wie die Engine.
- Der Dialog „Neue Welt“ legt Welten ohne die drei Felder an (keins).
- Oberfläche nach Atomic Design: Radix nur über `@webkit`, keine rohen Controls außerhalb von `atoms/`.

## Nicht-Ziele

Kataloge im Editor bearbeiten (Werte bleiben in `data/`), Vorschau von Wetter, Reittier oder Bauwerk im Editor, Himmels-Effekt des Fests (liegt in `data/native.json`, B-1002).

## Regeln und Einschränkungen

- **Bereich:** `tools/k3c-workbench/**`
- **Abhängig von:** B-969, B-970, B-972
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –
- Eine Session (SI7.1). Kein Sim-Code: Die Workbench liest Kataloge und Prüfung aus der eingebundenen Engine. Eine neue Workbench-EXE baut 🧑 nach dem Merge (`task buildall`), nicht die Session.

## Beispiele

- Welt `britannien` öffnen, Wetter `taifun` wählen, speichern → `data/worlds/britannien.json` hat `"weather": "taifun"`, `world_check` ist grün.
- Reittier auf „keins“ stellen → das Feld `mount` fehlt in der Datei.

## Ausnahme- und Fehlerfälle

- Die Welt-Datei nennt eine ID, die es im Katalog nicht gibt → das Feld zeigt den Wert als ungültig, Speichern verlangt eine gültige Wahl oder „keins“.
- Ein Katalog ist leer → die Liste zeigt nur „keins“.
- `world_set` mit unbekannter ID → abgelehnt, die Meldung nennt die gültigen IDs.

## Akzeptanzkriterien

- **AC-01** Go-Test im Backend: `world_get` nennt `weather`, `mount`, `hermit`; `world_set` setzt und entfernt sie; unbekannte ID wird mit den gültigen IDs abgelehnt.
- **AC-02** Go-Test: Die angebotenen Listen stammen aus den Katalogen der Engine (ein neuer Katalog-Eintrag erscheint ohne Codeänderung).
- **AC-03** Frontend-Test (Vitest): Die Übersicht zeigt die drei Auswahlfelder mit „keins“, eine Wahl löst das Speichern des Feldes aus, ein ungültiger Bestandswert wird markiert.
- **AC-04** `tests/atomicDesign.test.ts` und `check_run task:check:dev` grün.
- **AC-05** `check_run task:check:diff` grün.

## Offene Fragen

Mit Default entschieden (vorläufig, 🧑 bestätigt):

- Nur Wetter oder alle drei Kataloge? → Alle drei; 🧑 hat den Editor nur für das Wetter entschieden, für Reittier und Figur war die Frage offen (B-970, B-972).
- Wo sitzt die Auswahl? → In der Übersicht der Welt (`Overview.tsx`), nicht im Dialog „Neue Welt“.

## Notizen

Am 2026-11-15 aus B-969, B-970 und B-972 abgetrennt: eine Dateimenge (`tools/k3c-workbench/**`) für die Auswahl aller drei Kataloge.
