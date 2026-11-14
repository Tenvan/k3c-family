# RX1.2 · WebGL-Kontextverlust abfangen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** rx1/2-kontextverlust
- **Abhängig von:** RX1.1
- **Tickets:** B-763
- **Kriterien:** AC-05

## Ziel

Ein WebGL-Kontextverlust führt zu einer spielbaren Anzeige statt zur weißen Seite (B-763); ein kurzer Verbindungsverlust pausiert den Stresstest des echten Clients, statt ihn abzubrechen.

## Kontext

Xbox-Edge meldet nach 3–4 s dreimal „WebGL Context lost“. Phaser 4.2 ruft bei `webglcontextlost` `preventDefault()` und baut Texturen und Puffer bei `webglcontextrestored` selbst wieder auf (`WebGLRenderer.dispatchContextRestored`); kommt kein `restored`, bleibt die Seite weiß. Log-Haken aus RX1.1 (`renderLogInstall.ts`). Shell-Regel: Seitenwechsel nur über `openPage()`. Der Client hat keinen Spielzustand, der Raum bleibt auf dem Server (Beitritt per `?room=CODE&autostart=1`). Stresslauf: `src/stress/realStress.ts` (B-766).

## Erlaubte Dateien

- `src/core/contextGuard.ts`, `contextGuardInstall.ts` (neu), `src/main.ts`, `src/core/texts.de.ts`, `texts.en.ts`
- `src/stress/realStress.ts` (Pause bei Verbindungsverlust), zugehörige `*.test.ts`

## Nicht-Ziele

Keine Texturverkleinerung (RX1.6), keine Optimierung des Canvas-Renderers.

## Schritte

1. Entscheidung: Phaser baut den Kontext selbst auf; die Seite wartet 4 s auf `webglcontextrestored`, sonst Neustart mit `?renderer=canvas` und Beitritt zum offenen Raum (`room`, `autostart`), höchstens einmal je Sitzung (Sitzungsspeicher `k3c-ctx-restart`); ein zweiter Verlust nach einer Wiederherstellung startet sofort den Neustart; nach dem Neustart nur noch ein Hinweis (Notbetrieb).
2. Hinweisleiste unabhängig von Phaser (DOM), Texte de/en.
3. Stresslauf: `reconnecting` pausiert (Spielzeit der Stufe läuft nicht), nach 60 s `aborted`; `ended`/`lost` beenden sofort.
4. Tests, Browser-Prüfung mit `WEBGL_lose_context`.

## Fertig, wenn

- [ ] AC-05: Test: erster Verlust löst genau einen Neustart aus, ein zweiter nach dem Neustart keinen (`src/core/contextGuard.test.ts`).
- [ ] Verbindungsverlust im Stresslauf pausiert und endet nach dem Limit (`realStress.test.ts`).
- [ ] B nicht belegt, keine Schleife.

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

- AC-05: umgesetzt und getestet (`contextGuard.test.ts`, 8 Tests; `realStress.test.ts` +4). Entscheidung: Wiederherstellung durch Phaser abwarten (4 s), sonst genau ein Canvas-Neustart mit Raum-Beitritt; danach nur der Hinweis „Notbetrieb“.
- Browser (Vite + Go-Server, `WEBGL_lose_context`): `restoreContext` innerhalb der Frist → kein Neustart; ohne Wiederherstellung → Neustart nach 4 s mit `renderer=canvas`. Der Beitritt zum offenen Raum ist nur im Test der Adresse geprüft (das Pane im Hintergrund trat dem Raum nicht bei).
- Stresslauf: `reconnecting` pausiert bis 60 s (`RECONNECT_LIMIT_MS`), dann `aborted`.
- Offen: AC-09 (Xbox bleibt bedienbar) im Xbox-Lauf RX1.5/Abnahme RX1.8.
