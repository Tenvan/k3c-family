# B-1210 · Welten liegen im dist-Paket; beim Start aus dem Repo-dist gelten die Repo-Welten (Dev-Symbol)

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WL1
- **Projekt:** WIR
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`task build` legt die Welten (`data/worlds`) nicht in `dist/k3c-game`. Das Spiel liest Daten aus `data/` per `K3C_DATA_DIR` oder eingebettet. Meldung Ralf 2026-11-21.

## Ziel

Das Paket `dist/k3c-game` enthält alle Welten. Startet das Spiel aus dem Repo-dist-Ordner, gelten die Welten aus dem Repo (`data/worlds`) statt der Kopie im dist, kenntlich an einem kleinen Dev-Symbol.

## Beteiligte und Zielgruppen

Ralf (Entwickler am PC); Hauptrechner.

## Anforderungen

- **AC-01** `task build` kopiert alle Welten nach `dist/k3c-game/data/worlds`, die Paketprüfung (`rustpaket pruefen`) kennt sie.
- **AC-02** Liegt `k3c-game.exe` in `<repo>/dist/k3c-game` (Repo-Wurzel an `.git`/`Taskfile.yml` erkennbar), liest das Spiel die Welten aus `<repo>/data/worlds`; sonst aus dem Paket.
- **AC-03** Im Menü (Welt-Auswahl) erscheint dann ein kleines Dev-Symbol an den Welten.

## Nicht-Ziele

Andere Daten (nur Welten); Workbench.

## Regeln und Einschränkungen

Entscheidung 013; Annahme: Repo-Erkennung über den Exe-Pfad (zwei Ebenen über der Exe liegt `Taskfile.yml`), keine Umgebungsvariable nötig. Gilt nur lokal, nie im weitergegebenen Paket.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-game/src/local/**`, `rust/crates/k3c-game/src/menu/**`, `rust/Taskfile.yml`, `tools/rustpaket/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Start aus `C:\WORKSPACE\K3C\dist\k3c-game`: neue Welt in `data/worlds` erscheint sofort, mit Dev-Symbol. Start aus einem kopierten Paket: Welten aus dem Paket, kein Symbol.

## Ausnahme- und Fehlerfälle

Repo-Welten unlesbar: Hinweis im Log, Welten aus dem Paket als Rückfall.

## Akzeptanzkriterien

- **AC-01** Test der Paketprüfung mit Welten.
- **AC-02** Test der Pfadwahl (Repo vs. Paket).
- **AC-03** Test, dass das Dev-Kennzeichen in der Welt-Liste gesetzt wird; Sicht am PC durch 🧑.

## Offene Fragen

keine.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
