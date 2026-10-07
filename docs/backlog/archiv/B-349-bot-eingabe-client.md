# B-349 · Bots steuern im Client die Monarchen über die Bot-Eingabe

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** TR2
- **Erstellt:** 2026-10-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-07, Chat, durch 🧑 (mit B-348)

## Ausgangslage

Der Client nimmt Eingaben nur von Tastatur, Gamepad und Touch (`src/input/`). Für Testläufe mit laufendem Client (B-348, `sim_test` mit `clients` 1–4) muss ein Bot die Monarchen der lokalen Spieler steuern, ohne dass der Client selbst etwas entscheidet.

## Ziel

Ein Client, mit `?botfeed=<Adresse>` geöffnet, nimmt die Eingaben seiner 1–4 lokalen Spieler aus dem Bot-Feed der Workbench. Alles andere läuft wie im normalen Spiel.

## Beteiligte und Zielgruppen

Workbench (`sim_test`) als Quelle, Agenten und 🧑 bei Testläufen.

## Anforderungen

- `BotInput` als weitere `PlayerInput`-Quelle in `src/input/`: liest je Slot die Kommandos (Bewegung, Sprint, Bezahlen, Schlag, Skill) aus einem WebSocket des Bot-Feeds; der Spiel-Code fragt weiter nur Aktionen ab.
- `?botfeed=…` und `?players=n` öffnen das Spiel direkt mit n lokalen Spielern; ohne den Parameter ändert sich nichts.
- Die Adresse des Feeds ist nur Loopback oder die Adresse der Seite; andere Hosts werden abgelehnt.
- Der Client meldet FPS und Latenz wie bisher über `/api/clientlog`.

## Nicht-Ziele

Bot-Entscheidung im Client (bleibt in der Workbench), Browser starten (B-348), neue Anzeige.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md` › Seiten & Navigation und Eingabe: Taste B unbelegt, Home-Kombi unberührt, kein `Math.random()`. Domäne PLAT.

## Beispiele

`game.html?botfeed=ws://127.0.0.1:5180/bot/4/0&players=2` → zwei Monarchen bewegen sich nach den Bot-Kommandos.

## Ausnahme- und Fehlerfälle

Feed bricht ab → die Monarchen stehen (keine Eingabe), der Client loggt `👋` und versucht es erneut.

## Akzeptanzkriterien

- **AC-01** Test: `BotInput` liefert je Slot die Aktionen aus Feed-Nachrichten; ohne Feed keine Aktion.
- **AC-02** Test: `?botfeed` mit fremdem Host wird abgelehnt; ohne Parameter bleibt die Eingabe unverändert.

## Offene Fragen

keine

## Notizen

Feed-Nachricht (Server → Client, JSON je Kommando): `{"slot":0,"moveX":-1…1,"sprint":false,"pay":false,"attack":false,"skill":0}`, Felder wie `sim.PlayerCommand`; die Workbench schickt je Slot und Takt höchstens eine Nachricht.
