# SO2.3 · Einbau über den Audio-Kern, Rückfall bei fehlender Datei

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** so2/3-einbau-rueckfall
- **Abhängig von:** SO2.2
- **Tickets:** B-167
- **Kriterien:** AC-03, AC-04, AC-05, AC-06

## Ziel

Münze aufheben, Schlag, Gegner-Tod, Bauen fertig und Nacht naht (dazu die übrigen katalogisierten Ereignisse) lösen ihren Sound über den Audio-Kern aus; das Mapping Ereignis → Sound ändert keinen Spielzustand, und eine fehlende oder nicht ladbare Datei lässt das Spiel ohne Fehlermeldung weiterlaufen.

## Kontext

- **Audio-Kern (SO1, Voraussetzung):** `src/audio/` mit Mixer (Busse Musik/Effekte/Ambient), Entsperren per Geste, Format mit Fallback, Sound-Atlas, Dämpfungsfunktion im Split-Screen (Quelle-x, Kameras der lokalen Spieler, Warnungen global) und einem Demo-Ereignis auf `built` (SO1.3). Aufbau und Funktionsnamen aus dem Stand auf `develop` lesen; hier nur das **Mapping** ergänzen, den Kern nicht ändern (Änderungswunsch → Ticket CLI). Fehlt der Kern: `Status: blockiert`.
- **Ereignisse:** Der Client bekommt sie je Frame in `GameScene` (`frame.state.events`, dort wird auch `pendingEvents` gefüllt; die HudScene leert `pendingEvents` über `splice(0)`, **nicht** daraus lesen, sondern über den Weg, den SO1.3 für den Audio-Kern angelegt hat). Typen (`src/model/types.ts`, `docs/protocol.md` › Ereignisse): `coinPickup`, `coinGive`, `strike`, `arrow`, `hit`, `kill`, `buildProgress`, `built`, `revive`, `playerDown`, `dusk`, `arrived`, `skillPoint`. Nacht naht = `dusk`.
- **Mapping:** eine Tabelle Ereignis-Typ (und Unterart, z. B. `team`, `to`, `target`) → Sound-Schlüssel im Atlas, als reine Funktion mit Test (jedes Ereignis aus `docs/assets/sounds.md` mit Status `zugeordnet` hat einen Schlüssel; unbekannter Typ oder Lücke → kein Sound, kein Fehler). Das Mapping liest nur Ereignisse; keine Spiel-Regel im Audio-Code (`src/scenes/noSim.test.ts` bleibt grün).
- **Position und Dämpfung:** Ereignisse mit `x` (Units) gehen mit Ort in die Dämpfung (jeder lokale Spieler hört seinen Bereich); Warnungen (`dusk`, `wave`, `castleFallen`) sind global (Regel aus SO1.3). „Nacht leise statt laut“ (Q15) gilt für die Auswahl der Sounds, nicht für zusätzliche Lautstärkelogik.
- **Rückfall (AC-05):** Atlas-Eintrag fehlt oder Datei lädt nicht → stumm, höchstens einmal je Schlüssel ein Eintrag über `clientLog` (`src/core/clientLog.ts`), keine Ausnahme; Test mit gemocktem Atlas ohne den Schlüssel.
- **Dichte:** Viele Ereignisse je Tick (bis 32, `maxEventsPerTick`) dürfen den Ton nicht verstopfen: höchstens N gleichzeitige Instanzen je Schlüssel; die Grenze ist ein Wert in der Konfiguration des Kerns oder hier an einer Stelle, mit Test.
- **Messung am TV** (Lautheit, Hörbarkeit) macht 🧑 in SO2.5; sie ist keine Abhängigkeit dieser Session.

## Erlaubte Dateien

- `src/audio/` (Mapping, Anbindung, Tests)
- `src/scenes/GameScene.ts` (nur: Ereignisse an den Audio-Kern weitergeben, falls SO1.3 den Weg noch nicht erledigt hat)
- `public/audio/` (nur Atlas-Beschreibung, falls der Katalog sie ergänzt)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Änderungen am Mixer, an der Dämpfung oder am Format (SO1), Musik (SO4), neue Sounds (SO2.2), Hörprobenseite (SO3), Optionen-Szene (S5).

## Schritte

1. Branch anlegen, `Status: in Arbeit`. SO1-Ergebnisse und `docs/assets/sounds.md` lesen.
2. Reines Mapping Ereignis → Schlüssel mit Test (Zuordnung vollständig laut Katalog, Lücke und unbekannter Typ → kein Sound).
3. Anbindung an den Audio-Kern (Ort, Dämpfung, globale Warnungen); Begrenzung gleichzeitiger Instanzen.
4. Rückfall: fehlende Datei → stumm; Test mit gemocktem Atlas.
5. Im Browser-Pane mit Konsole prüfen (`read_console_messages`): vor der ersten Eingabe keine Audio-Fehler, nach der Eingabe laufen die Ereignisse ohne Fehler; Audio-Requests zählen (`read_network_requests`).
6. `task check`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [ ] AC-03: Das Mapping bildet `coinPickup`, `strike`, `kill`, `built` und `dusk` auf je einen Sound ab (Test); im Browser-Pane löst jedes Ereignis den Aufruf aus (Konsole oder Zähler); das Hören am TV ist SO2.5.
- [ ] AC-04: Das Mapping ist reine Funktion über Ereignisse, ändert keinen Spielzustand, `src/scenes/noSim.test.ts` grün.
- [ ] AC-05: Fehlt ein Atlas-Eintrag oder lädt die Datei nicht, läuft das Spiel ohne Fehlermeldung weiter (Test mit gemocktem Atlas, Konsole im Browser-Pane).
- [ ] AC-06: `task check` grün.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
