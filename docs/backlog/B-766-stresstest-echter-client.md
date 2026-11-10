# B-766 · Stresstest des echten Clients: Dev-Aktion spawn und Lauf mit Rampe in der GameScene

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RX1
- **Projekt:** LST
- **Erstellt:** 2026-11-09
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-765 misst die Render-Last in einer eigenen Phaser-Szene ohne Server. Ein echter Spieltest braucht den vollen Client: GameScene mit Go-Server, echtem Hintergrund, Vordergrund, mehreren Stufen (Biome nach `depth`), Animationen, Licht und Netzverkehr. Der Server hat Dev-Aktionen (`boss`, `event` …), aber keine, die Gegner und Truppen in beliebiger Zahl setzt.

## Ziel

Mit `game.html?dev=1&stress=<szenario>` läuft ein echtes Spiel (1 oder 2 lokale Spieler, Stufe wählbar per `depth`), in dem eine Rampe alle 10 s mehr Gegner und Truppen setzt; das Client-Log liefert je Stufe FPS, Frame-Zeit, Texturspeicher, Latenz und die Zahl der Figuren im Zustand.

## Beteiligte und Zielgruppen

🧑 startet Kacheln auf Xbox und PC, der Pi hält den Server; Claude wertet `logs_query` und `reports/` aus.

## Anforderungen

- **Dev-Aktion `spawn`** (Go, nur Dev-Mode): `slot`, `kind` (`enemy`, `troop`), `unit` (Schlüssel aus `data/enemies.json`/`troops.json`), `amount` 1 bis 200; Figuren stehen gleichmäßig auf 40 Units um den Monarchen, ohne rng (`sim.DevSpawnUnits`, Protokoll-Beispiel `c2s-dev-spawn.json`, `docs/protocol.md`).
- **Lauf im Client** (`src/stress/realStress.ts`): `?stress=<szenario>` mit `?dev=1`; nutzt `data/stress.json` (Gegner, Truppen, Rampe, Faktor) und `StressRun` aus B-765; je Stufe die fehlenden Figuren per `spawn`, am Stufenende eine Log-Zeile mit Latenz, Puffer und Zahl der Gegner und Truppen im Zustand; Endzeile mit Dichte-Grenze; mit `?perf=1` Bericht nach `reports/`.
- Hintergrund, Vordergrund, Licht, Animation und Stufenwechsel sind die des Spiels. Mehrere Stufen: Kacheln und URL-Parameter `depth` (0 Wald, 1 Höhle, 2 Mine, 3 Eisenfeste, 4 Kristall).
- Kacheln auf `dev.html` (Wald 1 Spieler, Wald 2 Spieler, Mine 2 Spieler).

## Nicht-Ziele

Keine Balance- oder Regeländerung; kein Last-Werkzeug für den Pi-3-Server ohne Client (dort `sim_test`); kein Stufenwechsel während des Laufs (je Stufe ein Lauf).

## Regeln und Einschränkungen

`spawn` nur im Dev-Mode des Servers (sonst `forbidden`); Protokollversion bleibt (Dev-Aktion, wie KB2.6). Gegner und Truppen kämpfen wie im Spiel, Gegner sterben also und der Spieler kann fallen; gemessen wird das echte Spiel. Keine neue Abhängigkeit.

## Beispiele

`game.html?dev=1&autostart=1&fresh=1&save=stress-wald&seed=stress&players=2&depth=0&stress=worstCase` → Spiel startet, nach dem Beitritt Log „🚀 Stress worstCase“, alle 10 s „💓 Stufe n/8 … Latenz … ms, x Gegner, y Truppen“, am Ende „✅ Ergebnis“.

## Ausnahme- und Fehlerfälle

Server ohne Dev-Mode: `spawn` ergibt `forbidden`, die Stufe läuft ohne neue Figuren (Zahlen im Log zeigen es). Unbekanntes Szenario: Fehlerzeile. Raum verlassen: Endzeile mit Grund `aborted`.

## Akzeptanzkriterien

- **AC-01** Go-Test: `spawn` setzt Gegner und Truppen um den Spieler, lehnt unbekannte Art, Sorte, 0 und mehr als 200 ab (`sim.DevSpawnUnits`).
- **AC-02** Go-Test: Das Protokoll-Beispiel `c2s-dev-spawn.json` wird im Dev-Mode angenommen, ungültige Felder sind `bad_request`, ohne Dev-Mode `forbidden`.
- **AC-03** Test: Die Nachrichten je Stufe sind genau die fehlenden Figuren (in Summe die der letzten Stufe), höchstens 200 je Nachricht, Slots des Geräts.
- **AC-04** Test: Der Lauf wartet auf Raum und Zustand, schreibt Start- und Echt-Zeile mit `runId`, je Stufe eine Zeile mit Server-Werten.
- **AC-05** Lauf im Browser (Claude): Spiel mit Server, Rampe läuft bis zur Endzeile, Zeilen im Server-Log.
- **AC-06** Abnahme (🧑): Kacheln auf Xbox und PC, Dichte-Grenze notiert.

## Offene Fragen

keine

## Notizen

Umgesetzt in RX1.4. Gegner sterben in diesem Lauf schnell (Truppen kämpfen mit): Wer die Gegnerdichte halten will, braucht einen späteren Schalter (Nachlegen bis zur Sollzahl). Die FPS-Werte eines von Hand getakteten Browsers (Pane im Hintergrund) sind nicht aussagekräftig.
