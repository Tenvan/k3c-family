# B-513 · Im Welten-Editor lässt sich die Spiel-Adresse wählen: interne Engine oder Server

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-20
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-20, Ralf im Chat (Umsetzung für die Abnahme TS1.7 gewünscht, Revision 1)

## Ausgangslage

Wunsch von 🧑 bei der Abnahme TS1.7 (2026-10-20, Chat): „Interne URL oder Server-URL auswählbar im Welten-Editor, weil man die nicht im Kopf haben kann.“ Heute öffnet „Im Spiel öffnen“ (`WorldOpenGame`, `tools/k3c-studio/backend/app_game.go`) das Spiel über die eigene Engine des Studios (Loopback, Port 8091); der Parameter `base` bleibt nur aus Kompatibilität und muss Loopback sein. Den Spielserver (Port 8080, Heimnetz) kann man nicht wählen, die Adresse muss man kennen.

## Ziel

Im Welten-Editor wählt man vor „Im Spiel öffnen“ aus einer Liste, wo das Spiel läuft: die interne Engine des Studios oder ein erkannter Spielserver (Adresse wird angezeigt, nicht eingetippt).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

- Woher kommt die Server-Adresse (Standardport 8080 auf Loopback prüfen, Eintrag aus `.mcp.json`/k3c-dev, manuell gespeichert)? Entscheidet 🧑.
- Der Server kennt „voll ausgebaut“ und Dev-Aktionen nur im Dev-Mode; reicht die Welt-Wahl, oder soll der Studio-Spielstand auch dorthin? Entscheidet 🧑.
- Spec ist Entwurf, Freigabe fehlt.

## Notizen

**Umsetzung 2026-10-20** (Branch `b513/spiel-adresse`), Entscheidungen zu den offenen Fragen vom Koordinator:
- **Server-Erkennung:** `WorldGameServers()` (`tools/k3c-studio/backend/app_game_server.go`) prüft `GET /api/health` und `GET /api/dev` an `http://127.0.0.1:<K3C_HTTP_PORT|8080>` (2 s). Heimnetz-Adresse (erste private IPv4 mit Port) und Rechnername stehen nur zur Anzeige in der Liste.
- **Auswahl:** Im Welten-Editor wählt die Liste „Spiel läuft auf“ (`GameLauncher.tsx`) „Interne Engine des Studios (<eigene Adresse>)“ oder „Spielserver <Adresse> · im Heimnetz <LAN>“. Nicht erreichbare sind ausgegraut („nicht erreichbar“), ohne Dev-Mode steht „ohne Dev-Mode“ dabei, „Neu suchen“ prüft erneut. Die Wahl wird gemerkt (`k3c-studio.world.gameTarget`, localStorage über `@webkit/prefs` mit try/catch); ist der gemerkte Server nicht erreichbar, gilt die Engine.
- **Ziel Spielserver:** `WorldOpenGameServer(base, seed, biome, mode)` wie vor TS1.2: Gesundheitsprüfung, „voll ausgebaut“ über `POST /api/dev/fullsave` des Servers (Hinweis ohne Dev-Mode), andere Welten nur im Dev-Mode des Servers; Landingpage `<Adresse>/#game.html?…`. Der Studio-Spielstand wandert nicht mit.
- **Sicherheit:** `base` nur Loopback (`loopbackBase`), keine Weiterleitungen (`gameClient`); die LAN-Adresse wird nie angesprochen.
- **Nachweis:** Go-Tests `app_game_server_test.go` (Erkennung, nicht erreichbar, ohne Dev-Mode, Port, neu/voll auf dem Server, Hinweise, Loopback und Weiterleitung, andere Welt), Vitest `GameLauncher.test.tsx` (Liste, Server-Binding, gemerkte Wahl, ausgegraut mit Rückfall, Neu suchen). `task check`, `task check:go`, `task check:studio` grün. Manuelle Abnahme: TS1.7 durch 🧑.
