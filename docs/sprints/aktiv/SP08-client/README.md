# SP08 · CLI · Browser als reiner Client

- **Status:** aktiv
- **Domäne:** CLI
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-016, B-037, B-039, B-061, B-082
- **Start-Commit:** 98a4907
- **Spec:** freigegeben
- **Revision:** 4
- **Freigabe:** 2026-10-01 🧑 Chat (SP08 Revision 4)

## Ausgangslage

Der Browser rechnet die Simulation selbst (`src/world/`) oder spricht Protokoll v1 mit dem Node-Server. Der Go-Server
(SP07) spricht Protokoll v2 mit Räumen, Slots, `snap`/`delta`, Wiederverbinden und Fehler-Codes; kein Client nutzt ihn bisher.

## Ziel

Der Browser rechnet nichts mehr, er schickt Eingaben und zeichnet Snapshots, lokal wie online. Am Ende sichtbar: 2 Controller an der Xbox + 1 Handy im selben Raum.

## Beteiligte und Zielgruppen

Spieler an der Xbox (2 Controller) und am Handy; 🧑 prüft am TV und gibt die Spec frei.

## Anforderungen

B-016, B-037 (einfache Raumwahl) und B-039 › Anforderungen. Vertrag ist `docs/protocol.md` mit den Beispielen in
`testdata/protocol/` (Entscheidung 001, 002). Sprint-eigen:

- Couch-Spiel läuft ebenfalls über den Server: Jeder lokale Spieler ist ein Slot, `addSlot` bei A auf einem neuen Controller.
- Layout (B-016, Entscheidung 🧑 2026-10-01): 1 lokaler Spieler Vollbild, 2 Streifen übereinander, 3–4 ein 2×2-Raster
  mit einer Kamera je Spieler. Bei 1 lokalem Spieler bleibt der nächste Mitspieler eines anderen Geräts oben (⅓), wie heute online.
- Lobby (`game.html` startet immer dort): Raumliste, ein Eintrag „Spielen“ (öffnet oder erstellt den Spielstand `familie`,
  `?save=NAME` wählt einen anderen), Beitritt per Liste oder `?room=CODE`. Bedienbar mit Controller, Tastatur und Touch.
- Start-Parameter für Tests (B-082, Grundlage der Testseite B-081): `?autostart=1` überspringt die Auswahl, `?fresh=1` erstellt einen neuen
  Spielstand, `?mock=N` (0–3) legt N **Mock-Slots** am selben Gerät an, die ohne Eingabe stehen.
- Keine Vorhersage (B-039): nur Interpolation; ob die eigene Laufbewegung vorhergesagt werden muss, entscheidet die Beobachtung am TV.

## Nicht-Ziele

Löschen der TS-Simulation und des Node-Servers (SP09); laufende Räume im Detail und Spielstand-Auswahl (B-037/AC-03, AC-04,
später); Dev-Tasten (B-080); Dev-Server-Proxy (B-078); Kacheln der Landingpage (B-079); Änderungen am Protokoll.

## Regeln und Einschränkungen

- `src/scenes` rechnet nichts, es zeichnet Snapshots. Seiten-Regeln aus `CLAUDE.md`, B nicht belegen, View + Menu bleibt reserviert.
- Neue Client-Dateien liegen als `src/online/client*.ts` neben `client.ts`; `protocol.ts`, `room.ts`, `wsServer.ts` (v1) bleiben unverändert bis SP09.
- Der Client wird gegen den Go-Server mit `task serve:go` ausprobiert (`task dev` leitet `/ws` erst nach B-078 weiter).
  Automatische Prüfungen laufen in Vitest mit einem Fake-WebSocket und den Dateien aus `testdata/protocol/`.
- Prüfungen am TV nur mit Freigabe durch 🧑. Komplexitäts-Budget aus `docs/arbeitsweise.md`.

## Beispiele

2 Controller an der Xbox und 1 Handy im selben Raum → drei Monarchen, jeder eigen gesteuert, flüssige Bewegung.
Zweiter Tab im selben Browser tritt demselben Raum bei → der erste Tab zeigt „an anderer Stelle geöffnet“ und verbindet sich nicht neu.

## Ausnahme- und Fehlerfälle

Verbindung weg → Hinweis und Wiederverbinden (B-030) statt Standbild. Alle Fehler-Codes aus `docs/protocol.md` haben ein
festes Verhalten (AC-09). Raum voll, unbekannt oder geschlossen → Hinweis, zurück zur Raumliste.

## Akzeptanzkriterien

- **AC-01** Der Browser schickt nur Eingaben und zeichnet Snapshots nach Protokoll v2 mit Interpolation (B-039/AC-01).
- **AC-02** Mehrere lokale Spieler pro Gerät nach dem Layout aus B-016 (B-016/AC-02).
- **AC-03** Raum erstellen und beitreten funktioniert (B-037/AC-01, B-037/AC-02).
- **AC-04** entfällt (Revision 2: B-018 verworfen, der Zielwert 300 Zeilen gilt nicht mehr).
- **AC-05** Am TV: 2 Controller an der Xbox + 1 Handy spielen im selben Raum (Beobachtung durch 🧑).
- **AC-06** Handschlag: Geräte-ID dauerhaft im `localStorage` (ohne Speicher eine Zufalls-ID), `hello` mit `v: 2` ist immer die erste Nachricht, `welcome` liefert Takt und Grenzen. Bei `version` zeigt der Client „Seite neu laden“ und verbindet sich nicht neu (Test).
- **AC-07** Eingabe-Takt: `input` bei Änderung, sonst spätestens alle 500 ms, höchstens eine je Tick, `seq` zählt je Verbindung ab 1 (Test mit Fake-Uhr).
- **AC-08** Level aus `level` (Biom-Werte über die Biom-ID aus `data/`, kein Generator), `snap` ersetzt den Zustand, `delta` mit `{set, del}` rekonstruiert jeden vollen Zustand; nach einem neuen `level` beginnt der Zustand neu (Test mit `testdata/protocol/`).
- **AC-09** Jeder Fehler-Code aus `docs/protocol.md` › *Fehler-Codes* hat ein festes Verhalten: die Server-`message` wird angezeigt; `room_full`, `too_many_slots`, `too_many_rooms`, `room_not_found`, `save_exists`, `save_not_found` bleiben bei der Raumliste bzw. im Spiel; `room_closed` führt zur Raumliste; `replaced` zeigt „an anderer Stelle geöffnet“ ohne Neuverbinden; `version` siehe AC-06; `bad_request` wird protokolliert (Test je Code, Anzeige in der Lobby).
- **AC-10** Wiederverbinden (Client-Teil von B-030): Bricht die Verbindung ab, zeigt der Client einen Hinweis und verbindet sich mit derselben Geräte-ID, demselben Code und denselben Slots neu (Wartezeit wächst bis 4 s, höchstens 120 s); `room_not_found` oder `room_closed` führen zur Raumliste, `replaced` und `version` nie zu einem Neuverbinden (Test mit Fake-Uhr; Hinweis im Spiel).
- **AC-11** Slots: A auf einem neuen Controller sendet `addSlot`, ein getrennter Controller `removeSlot`, der letzte entfernte Slot `leave` und zurück zur Lobby; `seats` und `joined` bestimmen, welcher Monarch zu welchem Slot gehört (Test).
- **AC-12** Layout nach B-016: 1 lokaler Spieler Vollbild (mit Mitspieler oben ⅓), 2 Streifen, 3–4 ein 2×2-Raster, bei 3 Spielern zeigt das vierte Feld Raumcode und freie Plätze (Test der Layout-Funktion für 1–4).
- **AC-13** Der Browser rechnet nichts: `src/scenes` importiert keine Simulationsfunktionen aus `src/world/sim` (nur Typen und Biom-Daten), geprüft von einem Test.
- **AC-14** `game.html?autostart=1&fresh=1&save=NAME&mock=N` erstellt ohne Auswahl einen neuen Raum mit dem echten Spieler als Slot 0 und N Mock-Slots, die stehen und nichts senden außer „keine Bewegung“; ungültige Werte (`mock` nicht 0–3, `save` gegen `^[a-z0-9-]{1,32}$`) werden ignoriert (B-082/AC-01, B-082/AC-02; Test der Parameter-Auswertung).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SP08.1 | `SP08.1-protokoll-client.md` | Umsetzung | autonom | fertig |
| SP08.2 | `SP08.2-szene-slots.md` | Umsetzung | autonom | fertig |
| SP08.3 | `SP08.3-lobby.md` | Umsetzung | autonom | fertig |
| SP08.4 | `SP08.4-review.md` | Review | autonom | offen |
| SP08.5 | `SP08.5-tv-abnahme.md` | Workshop | Mensch | offen |

## Abnahme

–
