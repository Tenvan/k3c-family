# B-897 · Nach dem Server-Rückbau sind Server-Reste in engine/room, k3c-sim bot, simexe.StartBot und der Dienste-Infrastruktur der Workbench entfernt

- **Domäne:** SRV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WR2
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: nächste autonome Welle; Entscheidung B-850)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

WR1 (B-887) hat Spielserver, `engine/net`, `engine/watch`, Lasttest, TUI, Starter und die Server-Tools der Workbench entfernt. Liegen geblieben, weil außerhalb des Umfangs oder noch von Tests genutzt: `engine/room` mit Server-Teilen (Monitor, Peer, Status/Summary, Preview-Kommentare zu `engine/net`), Unterbefehl `bot` in `cmd/k3c-sim` und `tools/k3c-workbench/internal/simexe/bot.go` (nur `sim_test` online nutzte sie), die leere Dienste-Infrastruktur der Workbench (`services.json` ist `[]`, `svc_*`, Seite Dienste, Port-Versatz für Vite/Spielserver), Mocks im Workbench-Frontend mit Spielserver und Vite, `testdata/protocol/` (nur noch `engine/room/autosave_test.go` liest eine Datei), Test-Fixtures in `tools/checkdiff/select_test.go` mit `engine/net`.

## Ziel

Kein toter Server-Code mehr; was nur dem Online-Spiel diente, ist weg oder bewusst behalten.

## Beteiligte und Zielgruppen

- Wer entwickelt: Claude (autonom) und Ralf; weniger toter Code in `engine/room`, `cmd/k3c-sim` und der Workbench.
- Entscheidet: 🧑 Ralf (Entscheidung B-850, Rückbau des Spielservers).

## Anforderungen

- Entfernen: Unterbefehl `k3c-sim bot` (`cmd/k3c-sim/bot.go` samt Test), `simexe.StartBot`, `Bot`, `Pool` (`tools/k3c-workbench/internal/simexe/bot.go`, `pool.go`) und der Fake-Bot der MCP-Tests.
- Entfernen in `engine/room`, was nur `engine/net` aufrief: Messreihen (`Monitor`, `Sample`), `Reopen`/`OpenRefs` und den Hook `Changed`.
- Behalten, was das native Spiel (`native/k3c-game/local`) oder die Workbench (`studioview`) nutzt: `Manager`, `Room`, `Peer`, `Snapshot`, `Status` (Abstürze), `CloseRoom`, `DeleteSave`, Dev-Aktionen, Zeitraffer, Fehler-Werte. Bei Zweifel behalten und im Ergebnis nennen.
- `testdata/protocol/` entfernen; `engine/room/autosave_test.go` hält das Beispiel selbst.
- Dienste-Infrastruktur und `svc_*` der Workbench bleiben (generisch, künftige Dienste); Kommentare zum Port-Versatz und die Frontend-Mocks nennen weder Spielserver noch Vite.
- Kommentare in Go-Code, die auf `engine/net` oder den Spielserver verweisen, nachziehen.

## Nicht-Ziele

- `native/k3c-game` (parallel EP1/ST2, BI2).
- Beobachter-Logik und Vorschau (`preview*.go`) in `engine/room`: im Zweifel behalten, eigene Entscheidung später.
- Spielmetrik-Report (`metrics.go`, B-150): Balancing-Thema, gesperrt.
- Test-Fixtures, die „Spielserver“ nur als erfundenen Dienstnamen nutzen (Controller-Tests der Workbench).

## Regeln und Einschränkungen

- Entscheidung B-850 (Spielserver, `engine/net`, Lasttest, TUI entfallen), Entscheidung 009 (nur natives Spiel).
- Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; Simulationen und Balancing gesperrt.

- **Bereich:** `engine/room/**`, `cmd/k3c-sim/**`, `tools/k3c-workbench/internal/simexe/**`, `tools/k3c-workbench/internal/mcpsrv/**`, `tools/k3c-workbench/internal/services/**`, `tools/k3c-workbench/frontend/src/api/**`, `tools/checkdiff/**`, `testdata/protocol/**`
- **Abhängig von:** WR1
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** protokoll

## Beispiele

- `k3c-sim bot --bot saver` → unbekannter Unterbefehl (Exit Usage).
- Das native Spiel startet einen Raum über `room.NewManager` → läuft unverändert, ohne Messreihen.

## Ausnahme- und Fehlerfälle

- Ein Teil von `engine/room` wird doch noch genutzt → der Compiler meldet es, der Teil bleibt und steht im Ergebnis.

## Akzeptanzkriterien

- **AC-01** Go-Code ohne `_test.go` in `engine/room`, `cmd/` und `tools/k3c-workbench` nennt `engine/net`, Spielserver und `k3c-server` nur noch als Historie mit Ticket-Verweis.
- **AC-02** `k3c-sim bot`, `simexe.StartBot`/`Pool` und die nur dem Server dienenden Teile von `engine/room` (Messreihen, Reopen, Changed) sind entfernt; Behaltenes ist im Session-Ergebnis begründet.
- **AC-03** `testdata/protocol/` ist entfernt; `engine/room` testet das Ereignis `gesichert` ohne die Datei.
- **AC-04** Dienste-Infrastruktur und `svc_*` bleiben; Port-Versatz-Texte und Frontend-Mocks der Workbench nennen weder Spielserver noch Vite.
- **AC-05** `task check:diff` je Commit grün; `task check:all` läuft im Abnahme-Ticket B-907.

## Offene Fragen

keine (Dienste-Infrastruktur bleibt, Default aus dem Entwurf; autonom entschieden).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
