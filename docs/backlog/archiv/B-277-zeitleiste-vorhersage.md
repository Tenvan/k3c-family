# B-277 · Der Client zeichnet trotz schwankender Zustände flüssig und wartet bei der eigenen Laufbewegung nicht auf den Server

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** N2
- **Erstellt:** 2026-10-04
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, mit N2

## Ausgangslage

Der Client (`src/scenes/GameScene.ts`, `src/online/clientInterpolation.ts`) blendet nur zwischen den **zwei neuesten** Zuständen und startet die Überblendung beim **Eintreffen** (`blendAlpha(now, cur.receivedAt, tickMs)`). Folgen:

- Kommen zwei Zustände im selben Frame (Netz-Schwankung, WLAN, Pi-Tick-Spitzen aus B-276), springt die Figur; kommt einer zu spät, bleibt sie stehen (keine Extrapolation). Das sieht wie starkes Ruckeln aus (B-194).
- Die eigene Laufbewegung erscheint erst nach Eingabe → Server → Zustand → einem Tick Überblendung; der Client wartet bei jedem Schritt auf die Antwort.
- `sendInput` (`clientConnection.ts`) verwirft eine Änderung, die schneller als ein Tick nach der letzten kommt, bis zum nächsten Frame.

B-039 erlaubt ausdrücklich, die eigene Laufbewegung lokal vorherzusagen.

## Ziel

Figuren laufen auf dem Bildschirm gleichmäßig, auch wenn Zustände schwankend oder gebündelt ankommen, und der eigene Monarch reagiert im selben Frame auf die Eingabe, ohne auf den Server zu warten.

## Beteiligte und Zielgruppen

Familie am TV (Xbox, Edge) und Handy; CLI setzt um; 🧑 prüft am Gerät.

## Anforderungen

- **Zeitleiste:** Der Client hält einen kleinen Puffer empfangener Zustände und zeichnet zur Zeit „Server-Tick-Schätzung minus Puffer-Verzögerung“. Die Schätzung folgt `tick` und Empfangszeit geglättet (keine Sprünge bei einzelnen Ausreißern), die Verzögerung passt sich der gemessenen Schwankung an (Untergrenze ≈ 1 Tick, Obergrenze ≈ 150 ms, angenommen).
- **Extrapolation:** Läuft der Puffer leer, laufen bewegliche Einträge höchstens ~100 ms (angenommen) in ihrer letzten Richtung weiter, danach bleiben sie stehen; ein neuer Zustand korrigiert weich.
- **Vorhersage:** Der Monarch jedes lokalen Slots (`client.you`) läuft sofort mit der lokalen Eingabe (Laufrichtung × Geschwindigkeit). Die Geschwindigkeit stammt aus der beobachteten Bewegung des Monarchen im Zustand (Rückfall: `base.speed` aus `data/monarch.json`). Abweichung zum Server-Zustand wird weich abgebaut, großer Abstand (Teleport, Respawn, Stufenwechsel) übernimmt den Server-Wert sofort. Nur die x-Position; alles andere kommt unverändert vom Server.
- **Eingabe:** Eine geänderte Eingabe geht sofort raus (nicht erst nach einem Tick); Keepalive bleibt.
- Funktioniert mit 2+ lokalen Spielern (Split-Screen) und online; Client würfelt nicht und rechnet keine Spiel-Logik außer der Anzeige-Vorhersage (B-039).

## Nicht-Ziele

Protokolländerung; Server-Tick (B-276); Vorhersage fremder Figuren, von Truppen, Gegnern oder Aktionen.

## Regeln und Einschränkungen

Domäne CLI. Kein Import von Simulations-Code (`src/scenes/noSim.test.ts`), kein `Math.random()`. Grenzwerte als benannte Konstanten „angenommen“. Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Zwei Zustände kommen im selben Frame, dann 80 ms keiner → Figuren laufen gleichmäßig weiter. Spieler drückt rechts → eigener Monarch bewegt sich im selben Frame, fremder erst mit dem Zustand.

## Ausnahme- und Fehlerfälle

Verbindung weg → Extrapolation endet nach der Obergrenze. Respawn/Teleport/Stufenwechsel → Server-Wert sofort, Puffer geleert. Server lehnt Bewegung ab (Rand) → Vorhersage wird weich zurückgeführt.

## Akzeptanzkriterien

- **AC-01** Test: Zustände mit Schwankung (gebündelt, verspätet, Lücke) ergeben eine gezeichnete x-Position, die sich je Frame monoton und ohne Sprung > 1,5 × (Geschwindigkeit × Frame-Dauer) bewegt (`task test`).
- **AC-02** Test: Bei leerem Puffer extrapoliert die Zeitleiste höchstens die Obergrenze und bleibt dann stehen; ein Sprung > `TELEPORT_UNITS` wird nicht überblendet.
- **AC-03** Test: Die Vorhersage bewegt den lokalen Monarchen im ersten Frame nach der Eingabe; der Abstand zum Server-Zustand baut sich ab; großer Abstand übernimmt sofort; fremde Monarchen werden nicht vorhergesagt.
- **AC-04** Test: `sendInput` sendet eine geänderte Eingabe sofort, auch kurz nach der letzten; unveränderte Eingabe nur als Keepalive.
- **AC-05** `task check` grün; `src/scenes/noSim.test.ts` bleibt grün.
- **AC-06** Am Gerät (Xbox, 2 Spieler, Split-Screen) läuft das Spiel ohne sichtbares Ruckeln (🧑, angenommen bis zur Prüfung, B-194).

## Offene Fragen

keine

## Notizen

Angelegt 2026-10-04 aus dem /team-Auftrag „Netzwerk lagt, Client soll nicht auf Antwort warten“.
