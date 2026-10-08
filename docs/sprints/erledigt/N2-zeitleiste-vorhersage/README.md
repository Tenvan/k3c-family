# N2 · CLI · Flüssige Darstellung: Zeitleiste, Extrapolation, eigene Vorhersage

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** CLI
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-277, B-181
- **Start-Commit:** 33ae378
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat (Ralf, /team-Auftrag: bessere asynchrone Übertragung, Client wartet nicht, UI flüssig), Revision 1, aus dem Auftrag abgeleitet; umfasst B-277, B-181

## Ausgangslage

Der Client blendet nur zwischen den zwei neuesten Zuständen ab Eintreffen, extrapoliert nicht und wartet bei der eigenen Bewegung auf den Server (B-277 › Ausgangslage). Schwankende Zustände sehen wie starkes Ruckeln aus (B-194). Die Latenz ist nicht ablesbar (B-181).

## Ziel

Figuren laufen gleichmäßig trotz schwankender Zustände, der eigene Monarch reagiert sofort. Am Ende sichtbar: im Debug-Overlay Latenz (Mittel, p95) und Puffer-Verzögerung; am TV kein Ruckeln (🧑).

## Beteiligte und Zielgruppen

Familie am TV (Xbox, Edge) und Handy; CLI-Agent setzt um; 🧑 prüft am Gerät (AC-06).

## Anforderungen

`B-277 › Anforderungen`, `B-181 › Anforderungen`.

## Nicht-Ziele

Protokolländerung; Server-Tick (N1, B-276); Vorhersage fremder Figuren, von Truppen, Gegnern oder Aktionen (Bauen, Zahlen); Zeichen-Leistung je Kamera (B-194, bleibt offen bis zur Messung am Gerät).

## Regeln und Einschränkungen

Domäne CLI (`src/online/client*.ts`, `src/scenes/`). Kein Import von Simulations-Code (`src/scenes/noSim.test.ts`), kein `Math.random()`. Vorhersage nur für die x-Position der lokalen Monarchen (B-039). Neue Logik Phaser-frei in `src/online/` mit Vitest-Tests, die Szene ruft nur auf. Grenzwerte (Puffer, Extrapolation) „angenommen“ als benannte Konstanten. Datei ≤ 400 Zeilen (`GameScene.ts` hat 374: Logik auslagern), Funktion ≤ 60. 2 Spieler gleichzeitig.

## Beispiele

Zwei Zustände kommen im selben Frame, dann 80 ms keiner → Figuren laufen gleichmäßig weiter. Spieler drückt rechts → eigener Monarch bewegt sich im selben Frame, fremder erst mit dem Zustand.

## Ausnahme- und Fehlerfälle

Verbindung weg → Extrapolation endet nach der Obergrenze, Figuren stehen. Respawn/Teleport/Stufenwechsel → Server-Wert sofort, Puffer geleert. Server lehnt Bewegung ab (z. B. Rand) → Vorhersage wird weich zurückgeführt.

## Akzeptanzkriterien

- **AC-01** B-277/AC-01
- **AC-02** B-277/AC-02
- **AC-03** B-277/AC-03
- **AC-04** B-277/AC-04
- **AC-05** B-277/AC-05
- **AC-06** B-277/AC-06 (Hardware, keine Abhängigkeit des Reviews)
- **AC-07** B-181/AC-01
- **AC-08** B-181/AC-02

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| N2.1 | `N2.1-zeitleiste.md` | Umsetzung | autonom | fertig |
| N2.2 | `N2.2-vorhersage-latenz.md` | Umsetzung | autonom | fertig |
| N2.3 | `N2.3-review.md` | Review | autonom | fertig |
| N2.4 | `N2.4-abnahme-xbox.md` | Workshop | Mensch | fertig |

## Abnahme

- 2026-10-04, Review N2.3: AC-01, AC-02 (Ergebnis N2.1), AC-03, AC-04, AC-07, AC-08 (Ergebnis N2.2), AC-05 (`task check` grün) mit Nachweis; Ergänzungs-Tests in N2.3.
- AC-06: angenommen, Validierung offen (N2.4, 🧑 an der Xbox; Fahrplan „Offen am Gerät“).
- Behoben: Gummiband der Vorhersage unter Latenz (Vorlauf), Latenz-Fenster begrenzt, doppelter Tick, Zeitraffer-Schwelle, keine Vorhersage tot/angehalten, `moveX` gerastert, Glossar. B-278 → B-279 umbenannt (Kollision mit N1).
- Abhängigkeit: `ack` beim Einreihen statt beim Senden (N1); B-277 und B-181 archiviert, B-279 bleibt offen.
- Version: v0.11.0 vorgeschlagen (Minor: flüssigere Darstellung im Spiel); gesetzt erst nach Bestätigung durch 🧑.
- 2026-10-07, N2.4: AC-06 am PC durch 🧑 als flüssig abgenommen (sporadische Einbrüche, evtl. paralleler Build); Overlay-Werte nicht abgelesen, automatische Messung als B-334; Xbox/Split-Screen bleibt bei B-314 und B-194 (PF1). Sprint auf Entscheidung 🧑 erledigt.
