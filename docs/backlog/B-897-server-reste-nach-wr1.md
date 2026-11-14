# B-897 · Nach dem Server-Rückbau sind Server-Reste in engine/room, k3c-sim bot, simexe.StartBot und der Dienste-Infrastruktur der Workbench entfernt

- **Domäne:** SRV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** WR2
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

WR1 (B-887) hat Spielserver, `engine/net`, `engine/watch`, Lasttest, TUI, Starter und die Server-Tools der Workbench entfernt. Liegen geblieben, weil außerhalb des Umfangs oder noch von Tests genutzt: `engine/room` mit Server-Teilen (Monitor, Peer, Status/Summary, Preview-Kommentare zu `engine/net`), Unterbefehl `bot` in `cmd/k3c-sim` und `tools/k3c-workbench/internal/simexe/bot.go` (nur `sim_test` online nutzte sie), die leere Dienste-Infrastruktur der Workbench (`services.json` ist `[]`, `svc_*`, Seite Dienste, Port-Versatz für Vite/Spielserver), Mocks im Workbench-Frontend mit Spielserver und Vite, `testdata/protocol/` (nur noch `engine/room/autosave_test.go` liest eine Datei), Test-Fixtures in `tools/checkdiff/select_test.go` mit `engine/net`.

## Ziel

Kein toter Server-Code mehr; was nur dem Online-Spiel diente, ist weg oder bewusst behalten.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Prüfen und entfernen: `k3c-sim bot`, `simexe.StartBot`, Server-Teile in `engine/room` (sofern das native Spiel sie nicht nutzt).
- Entscheiden, ob Dienste-Infrastruktur und `svc_*` der Workbench bleiben (künftige Dienste) oder entfallen.
- Mocks und Fixtures ohne Spielserver und Vite.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `grep` nach `engine/net`, `Spielserver` und `k3c-server` in Go-Code findet nur noch Historie in Kommentaren mit Ticket-Verweis.
- **AC-02** `task check:all` grün.

## Offene Fragen

Bleiben Dienste-Infrastruktur und `svc_*` der Workbench? Default: bleiben (generisch), nur Port-Versatz-Texte anpassen.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
