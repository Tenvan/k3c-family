# B-847 · „Im Spiel ansehen“ der Workbench startet das native Spiel statt game.html

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ49
- **Projekt:** NAT
- **Erstellt:** 2026-11-13
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-13

## Ausgangslage

Teil von B-823 (Entscheidung 009, Punkt 5). „Im Spiel ansehen“ (`tools/k3c-workbench/internal/studioview/game.go:77-89`) braucht `dist/index.html` und öffnet `game.html` im Browser.

## Ziel

„Im Spiel ansehen“ startet das native Spiel (`native/k3c-game/bin/k3c-game.exe` bzw. `dist/k3c-game/k3c-game.exe`) mit dem Asset in einer passenden Szene; ohne EXE eine klare Meldung.

## Beteiligte und Zielgruppen

🧑 Ralf im Bereich Assets; Claude (Umsetzung).

## Anforderungen

- Kein Bezug mehr auf `dist/index.html` oder `game.html`.
- Start der nativen EXE mit festem Pfad (keine Firewall-Abfrage, kein `go run`), Argumente für Level oder Seed soweit vorhanden.
- Fehlt die EXE: Meldung mit dem Task, der sie baut (`task native:game:build`).

## Nicht-Ziele

Neue Startparameter im nativen Spiel (dann eigenes CLI-Ticket).

## Regeln und Einschränkungen

Klasse A (Schuld, DEV). Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; Oberfläche nur über `@webkit`.

- **Bereich:** `tools/k3c-workbench/internal/studioview/**`, `tools/k3c-workbench/frontend/src/assets/**` (nur der Knopf)
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

Asset `objekte/portal` → Knopf startet das native Spiel; EXE fehlt → Hinweis.

## Ausnahme- und Fehlerfälle

EXE läuft schon → zweiter Start erlaubt, kein Absturz der Workbench.

## Akzeptanzkriterien

- **AC-01** Go-Test in `studioview`: Befehlszeile zeigt auf die native EXE, kein `game.html`.
- **AC-02** Test: fehlende EXE liefert die Meldung mit `task native:game:build`.

## Offene Fragen

keine

## Notizen

Zerlegung B-823 am 2026-11-13 (Claude).
