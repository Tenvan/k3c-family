# TS1.4 · Server beobachtet Dateien, Neuladen oder verzögerter Neustart

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** ts1/4-datei-watch
- **Abhängig von:** TS1.3
- **Tickets:** B-507
- **Kriterien:** AC-04

## Ziel

Der Server beobachtet immer (auch produktiv) `data/**`, Grafiken und Sound; eine Änderung führt entprellt zu Neuladen oder verzögertem Neustart, laufende Spieler sehen vorher „Server startet neu“ (AC-04).

## Kontext

Spec `B-507 › Anforderungen › Datei-Watch`. Der Server lädt `data/` heute per `go:embed` (`data/embed.go`) und liefert `dist/` aus. Entscheidung von 🧑: Watch immer aktiv, Neustarts auch in laufenden Runden (kleiner Heimserver), Spielstände werden wiederhergestellt (🩹); keine eigenen Refresh-Wege je Dateityp, im Zweifel verzögerter Neustart. Der Server schreibt nie in die beobachteten Ordner. Einfachste Lösung: Daten zur Laufzeit von der Platte lesen statt per embed (Fallback embed, falls Ordner fehlt) und bei Änderung einen kontrollierten Neustart des Prozesses auslösen (Beendigung mit speziellem Exit-Code, Wrapper/`k3c-starter`/Dienst startet neu, oder Selbst-Neustart per `exec`); für Weltdateien optional Neuladen ohne Neustart nur, wenn trivial. Meldung an Clients: neuer Protokoll-Nachrichtentyp oder vorhandener Hinweis-Kanal (Protokoll-Version und `docs/protocol.md` anpassen, falls neu). Uhr und Dateisystem im Test einspeisbar (Entprellen). Logs mit Emoji (🔁, ⏳, ❌).

## Erlaubte Dateien

- `cmd/k3c-server/**`, `engine/net/**`, `engine/room/**` (nur Hinweis/Neustart), neues Paket `engine/watch/` samt Tests
- `data/embed.go` und Ladecode der Datendateien, `docs/protocol.md`, `src/online/**` und `src/scenes/hud/**` nur für den Hinweis „Server startet neu“
- `engine/conlog` (neuer ns/Emoji), `tools/k3c-starter/**` nur falls der Neustart dort ankommen muss

## Nicht-Ziele

Hot-Reload ohne Neustart als Pflicht; Änderungen an Regeln, Spielstand-Format; Studio-Code (TS1.1/TS1.2); Docs außer Protokoll.

## Schritte

1. Tests zuerst (rot): Änderung in temporärem Ordner löst nach Entprellfrist genau einen Neuladen-/Neustart-Vorgang aus; mehrere schnelle Änderungen nur einen; ungültige Datei lässt den Server laufen (Fehler im Log); Hinweis geht vor dem Neustart an alle Clients.
2. Paket `engine/watch` (Beobachten von `data/**`, Grafik- und Sound-Ordnern, Entprellen, Uhr einspeisbar).
3. Server lädt Daten von der Platte (Fallback embed), verdrahtet den Watch, löst Neustart aus, sendet Hinweis; Client zeigt „Server startet neu“.
4. Protokoll/Doku nachziehen, prüfen, Commit `feat(srv): Server beobachtet Dateien und startet bei Änderung neu (TS1.4)`.

## Fertig, wenn

- [ ] AC-04: Go-Tests (Entprellen, ungültige Datei, Hinweis vor Neustart, kein Schreiben in beobachtete Ordner); Client-Test für den Hinweis.
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task check
task check:go
```

## Ergebnis

- **AC-04 geprüft (Go und Client):** `engine/watch` (Tests mit temporärem Ordner und eingespeister Zeit): eine Änderung ergibt nach 2 s Ruhe genau einen Vorgang, mehrere schnelle Änderungen ergeben einen, ein fehlender Ordner ist kein Fehler, eine Änderung zwischen Laden der Daten und Grundstand geht nicht verloren, Run schreibt nichts. `cmd/k3c-server/restart_test.go`: Reihenfolge prüfen → Hinweis → 3 s → Neustart; ein ungültiger Stand ergibt ❌ im Log, keinen Hinweis und keinen Neustart; Ordner mit Schreib-Ordnern des Servers (saves, reports, logs) werden nicht beobachtet; Supervisor (Neustart, Rückfall nur bei frühem Absturz, Ende von außen, Signal), Kind-Umgebung, `-check-data` mit kaputter Welt. `engine/net/restart_test.go`: `restarting` geht an Geräte in der Raumliste und im Raum, der Raum bleibt bis `room_closed`. `data/embed_test.go`: Platte statt Einbettung, Fallback. Client: `src/online/clientRestart.test.ts`, `src/scenes/lobbyLogic.test.ts`.
- `task check` grün, `task check:go` grün; `go build` von `tools/k3c-studio` und `tools/k3c-dev` grün (Typ von `data.Files`).
- Von Hand am PC (Agent): Spielserver über k3c-dev gestartet, `touch public/atlas/atlas-0.png` → 🔁 → ⏳ → Hinweis → Neustart nach 3 s mit neuer PID, Supervisor-PID bleibt.

**Entscheidungen**
- Daten von der Platte: `data.Files` ist ein Interface (`fs.ReadFileFS` + `fs.ReadDirFS`), gewählt beim Laden des Pakets aus `K3C_DATA_DIR`; leer oder fehlender Ordner = eingebettet. Die Pakete lesen ihre Werte beim Start, deshalb übernimmt nur ein neuer Prozess die Änderung; Neuladen ohne Neustart gibt es nicht (einheitlich, keine Wege je Dateityp). `data.LoadedAt` (vor dem ersten Lesen) ist der Grundstand des Watch: Dateien, die danach geändert wurden, zählen als Änderung.
- Neustart als Selbst-Neustart: `k3c-server` startet sich als Kindprozess (`K3C_CHILD=1`, `K3C_DATA_DIR` vorher gesetzt) und startet es neu, wenn es mit Code 75 endet. Die PID bleibt, deshalb brauchen k3c-starter, k3c-dev, Docker und `task serve` keine Änderung (unter Windows liegt das Kind im Job-Objekt des Starters bzw. von k3c-dev). Der Supervisor hört Strg+C/SIGTERM für die ganze Laufzeit, reicht sie weiter und startet danach nie mehr neu. Nur ein Absturz mit Daten von der Platte binnen 10 s nach dem Start führt einmal zum eingebetteten Stand; ein Ende durch Signal oder Kill von außen wird nur weitergereicht.
- Prüfung vor dem Neustart: Kindprozess `k3c-server -check-data` (Zeitlimit 30 s) mit demselben Ladecode wie beim Start (Panic = ungültig), im Dev-Mode zusätzlich alle Welt-Dateien. Ungültig: alter Stand läuft weiter.
- Hinweis: neuer Fehler-Code `restarting` („Server startet neu“) an alle Geräte, additiv ohne neue Protokollversion (wie frühere additive Codes); danach normales Herunterfahren (`room_closed`). Der Client behält den Hinweis bis zum nächsten `welcome`.
- Beobachtet werden `data/` (`K3C_DATA_DIR`) und `public/` (`K3C_PUBLIC_DIR`), Takt 1 s; neuer conlog-ns `watch` mit 👀.

**Abweichungen**
- `src/scenes/lobbyLogic.ts` (+ Test) geändert, nicht in den erlaubten Dateien: `gameNotice` zeigt den Hinweis im Spiel auch vor dem Abbruch (`errorCode === 'restarting'`); anders kommt er nicht in die HUD.
- Spielstände sind gespeichert, Räume werden nach dem Neustart aber nicht automatisch wieder geöffnet (Geräte landen in der Raumliste) → B-508.
- Grafiken/Sound: Der Server startet bei Änderungen in `public/` neu, liefert aber `dist/` aus; die Clients sehen sie erst nach einem Build → B-509.
- k3c-dev startet den Spielserver bei Änderungen in `data/` zusätzlich selbst neu → B-510.
- Nicht alle Schreibwege des Studios sind atomar geprüft → B-511. `stopAdopted` in k3c-dev beendet einen übernommenen Server nach dessen Selbst-Neustart nicht (neue PID am Port) → B-512.

**Neue Tickets:** B-508 bis B-512. Nicht committet (Commit und Code-Review durch den Aufrufer).
