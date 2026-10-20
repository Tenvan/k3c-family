# TS1.4 · Server beobachtet Dateien, Neuladen oder verzögerter Neustart

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
