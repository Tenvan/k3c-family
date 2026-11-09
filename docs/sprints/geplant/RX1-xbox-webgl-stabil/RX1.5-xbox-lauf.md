# RX1.5 · Xbox-Lauf mit Render-Log und Kontextschutz

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** live
- **Branch:** rx1/5-xbox-lauf
- **Abhängig von:** RX1.2, RX1.4
- **Tickets:** B-761, B-762, B-763, B-765, B-766
- **Kriterien:** AC-08, AC-17, AC-23

## Ziel

🧑 startet Spiel und Stress-Szene auf der Xbox mit RX1.1 bis RX1.3; die Werte bestätigen oder widerlegen die Texturvermutung (B-764) und liefern die Grenzen.

## Kontext

Voraussetzung: RX1.1, RX1.2 und RX1.3 (Stress-Szene) sind auf `develop` und auf dem Pi ausgeliefert. Claude liest danach das Log über `logs_query` (Client 192.168.2.209). B-761 ist im Code fertig, offen ist dessen Abnahme am TV (AC-05).

## Erlaubte Dateien

- Keine Codeänderung; Ergebnis in B-764 › Notizen und in dieser Datei.

## Nicht-Ziele

Kein Beheben; Texturprofil ist RX1.6.

## Schritte

1. 🧑: Xbox starten, Technik-Anzeige am TV einschalten (Menü oder `?gfx=1`), Renderer und Werte notieren.
2. 🧑: Spiel ca. 2 Minuten laufen lassen, danach `stress.html` mit `typisch` und `worstCase`, je 1 und 2 Spieler, und die drei Kacheln „Stress echt“ auf `dev.html` (echter Client mit Go-Server, B-766; Dev-Mode des Servers an) (`task gen:paper` ist vorher gelaufen und die PNG ausgeliefert), danach Claude Bescheid geben.
3. Claude: Server-Log lesen (Startzeile, maxTex, Texturspeicher, Verlauf, Kontextereignisse, Stufen-Zeilen und Dichte-Grenze je `runId`) und auswerten.
4. Entscheidung: Vermutung bestätigt → RX1.6 mit den Xbox-Grenzen; nicht bestätigt → B-764 verwerfen, Ursache neu suchen (Sprint anpassen).

## Fertig, wenn

- [ ] AC-23: Die Kacheln „Stress echt“ liefen mit dem Pi-Server, Dichte-Grenze in der Endzeile notiert.
- [ ] AC-17: Stress-Szene mit beiden Presets, 1 und 2 Spielern gelaufen, Dichte-Grenze in der Endzeile notiert.
- [ ] AC-08: Technik-Anzeige am TV gesehen, Startwerte, Verlauf und Ladedaten im Pi-Log gelesen.
- [ ] Ursache bestätigt oder widerlegt, Grenzwerte in B-764 › Notizen.

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

–
