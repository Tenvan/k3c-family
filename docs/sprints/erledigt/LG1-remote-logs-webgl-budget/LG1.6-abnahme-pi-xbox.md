# LG1.6 · Abnahme am Pi und auf der Xbox

- **Status:** verworfen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** live
- **Branch:** lg1/6-abnahme-pi-xbox
- **Abhängig von:** LG1.5
- **Tickets:** B-770, B-771, B-772, B-773
- **Kriterien:** AC-05, AC-06, AC-07, AC-08

## Ziel

🧑 nimmt am Pi und auf der Xbox ab: Logs vom Pi lesbar, WebGL 1 unter 130 MiB, Textur-Testseite auf einem Screen.

## Kontext

Voraussetzung am Pi: Log-Ordner gesetzt (`K3C_LOG_DIR`) und `K3C_DEV=1` (am Pi über `docker compose`). Danach liest Claude die Logs mit `logs_errors {server: "pi"}` selbst. Prüft, ob die Erinnerung (WebGL 1, 130 MiB) mit den Logwerten übereinstimmt.

## Erlaubte Dateien

- keine (reine Abnahme); Abweichungen als Ticket

## Nicht-Ziele

Keine Codeänderung in dieser Session.

## Schritte

1. 🧑 setzt den Log-Ordner am Pi und startet ihn neu.
2. Xbox-Lauf von Menü bis Spiel, danach Textur-Testseite öffnen (Seiten > Performance).
3. Claude liest `pi-gaming:8080/api/dev/log?file=client` und `logs_errors {server: "pi"}`; Werte gegen die Erinnerung prüfen.
4. Abweichungen als Ticket, B-772 bei Bedarf anpassen.

## Fertig, wenn

- [ ] AC-05: Client-Log des Pi ist über den Endpunkt lesbar.
- [ ] AC-06: `logs_errors {server: "pi"}` zeigt echte Pi-Zeilen.
- [ ] AC-07: Xbox-Lauf ohne Kontextverlust, Log zeigt WebGL 1 und Texturen unter 130 MiB.
- [ ] AC-08: Textur-Testseite passt auf der Xbox auf einen Screen.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Verworfen 2026-11-12 (Entscheidung Ralf): abgelöst durch B-823 (nur noch natives Spiel am PC, Entscheidung 009).
