# TS1.2 · Studio bindet die Engine ein, Vorschau und Spielstart ohne Server

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** ts1/2-studio-engine
- **Abhängig von:** TS1.1
- **Tickets:** B-507
- **Kriterien:** AC-02

## Ziel

Das Studio startet Vorschau und „Im Spiel öffnen“ mit eingebundener Engine und eigenen Testspielständen, ohne Spielserver und ohne Dev-Mode (AC-02).

## Kontext

Spec `B-507 › Anforderungen`. Heute nutzt das Studio für Vorschau und „Im Spiel öffnen“ (WZ18.6) den Spielserver im Dev-Mode (`app_game.go`, `app_preview.go`, `app_world*.go`). Neu: Das Studio bindet `engine/sim`, `engine/level`, `engine/room` (soweit nötig) als Bibliothek ein und hält einen eigenen Raum samt Testspielständen in einem eigenen Ordner (z. B. `tools/k3c-studio/.studio-saves/`, von git ignoriert), nie in `saves/` des Servers. Richtung: Studio → Engine, nie umgekehrt (Import-Test aus TS1.1 bleibt grün). Falls die Engine dafür Zugriffe braucht, die heute nur über `cmd/k3c-server` bestehen: Ticket anlegen und im Ergebnis nennen.

## Erlaubte Dateien

- `tools/k3c-studio/**` (Backend und Frontend-Aufrufe), `.gitignore` (eine Zeile für die Testspielstände)
- `tools/assetkit/**` nur wenn nötig

## Nicht-Ziele

Änderungen an `engine/` (Ticket statt Änderung), neue Studio-Funktionen, Server-Streichung (TS1.3), Watch (TS1.4).

## Schritte

1. Test zuerst (rot): Studio-Backend startet einen Raum mit Welt `standard` und einer Test-Welt ohne laufenden Server, Spielstand landet im Testordner, nicht in `saves/`.
2. Engine einbinden, die Aufrufe an den Spielserver (HTTP/WS, Dev-Mode) ersetzen.
3. Testspielstände anlegen/laden/löschen im Studio-Backend (nur Methoden, keine neue Oberfläche).
4. Prüfen, Commit `feat(dev): Studio mit eigener Engine und Testspielständen (TS1.2)`.

## Fertig, wenn

- [ ] AC-02: Go-Test: Vorschau und Spielstart ohne Spielserver; Spielstände nur im Testordner.
- [ ] `task check:go` und `task check:studio` grün.

## Prüfen

```bash
task check:go
task check:studio
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
