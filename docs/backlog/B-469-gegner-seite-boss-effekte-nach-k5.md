# B-469 · Die Gegner-Seite zeigt Warnkreis, Boss-Leiste, Phasen und Event-Banner aus K5.1

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ23
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

K5.1 (B-132, Domäne CLI, geplant) baut im Spiel die Boss-Anzeigen: Warnkreis (`enemies[].warn`), Boss-Leiste, Phasen-Text (`enemies[].phase`) und Event-Banner für `bossSpawned`, `bossPhase` und `bossDefeated`. Geplante Dateien laut K5.1 sind `src/scenes/kampfAnzeige.ts` (reine Funktionen) und `src/scenes/kampfView.ts` (Phaser); maßgeblich sind die Namen im Code nach K5.

Die Gegner-Seite im Asset-Studio (B-464, Sprint WZ11) läuft vor K5.1 (Projekt WZG hat Rang 1, KMP Rang 10). Sie zeigt Warnkreis, Boss-Leiste, Phasen-Text und Event-Banner deshalb nur als Lücke („Anzeige im Spiel fehlt (B-132, Anbindung B-469)“), je Boss nur für die zutreffenden Effekte (Warnkreis nur bei Flächenschlag, Phasen-Text nur beim Endboss mit Phasen, Boss-Leiste und Event-Banner bei jedem Boss) und ohne eigenen Ersatz. Entschieden von 🧑 am 2026-10-17: Die Anbindung der vier Anzeigen an die Gegner-Seite ist ein eigenes Folge-Ticket; die früheren Kriterien B-464/AC-09, AC-10 und AC-11 wandern hierher.

## Ziel

Die Gegner-Seite im Asset-Studio zeigt Warnkreis, Boss-Leiste, Phasen-Text und Event-Banner mit dem Zeichencode des Spiels aus K5.1, an Beispielzuständen aus `data/bosses.json`. Die vier Lücken-Markierungen aus WZ11 entfallen. Nutzen: 🧑 beurteilt Boss-Anzeigen im Asset-Studio, ohne einen Boss im Spiel auszulösen.

## Beteiligte und Zielgruppen

🧑 prüft die Boss-Anzeigen am PC im Asset-Studio und nimmt ab. Agenten setzen in DEV um, ein getrennter Reviewer prüft. Spieler sind nicht betroffen (Entwickler-Werkzeug); die Anzeigen im Spiel selbst gehören zu K5.1 (CLI).

## Anforderungen

- **Warnkreis:** Für jeden Boss mit Flächenschlag (`trollKing`, `crystalHeartWarden` ab Phase 2; Phase 3 erbt die Fähigkeit) zeichnet die Seite den Warnkreis mit dem Zeichencode des Spiels; der Radius kommt aus `ability.radius` (Units, mit `UNIT_PX` in Pixel).
- **Boss-Leiste:** Die Seite zeigt die Boss-Leiste des Spiels mit einem Beispielzustand (HP und Phase aus `data/bosses.json`).
- **Phasen:** Beim Endboss ist jede Phase einzeln wählbar und zeigt den Phasen-Text des Spiels und die Schwelle `belowHpPercent`. Eine Phase ohne `ability` zeigt die Fähigkeit der vorigen Phase (Regel aus `data/bosses.json` › `notes`).
- **Event-Banner:** Erscheinen, Phasenwechsel und Sieg eines Bosses (`bossSpawned`, `bossPhase`, `bossDefeated`) zeigen das Event-Banner des Spiels mit dessen Text.
- **Kein Nachbau:** Die Seite ruft den Zeichencode des Spiels über `@game` auf (Namen laut K5.1, voraussichtlich `src/scenes/kampfAnzeige.ts`, `kampfView.ts`); sie zeichnet nichts selbst.
- **Lücken entfallen:** Die Lücke „Anzeige im Spiel fehlt (B-132, Anbindung B-469)“ der Gegner-Seite verschwindet je Anzeige und Boss, sobald das Spiel diese Anzeige zeichnet; zeichnet das Spiel eine davon nicht, bleibt die Lücke genau an dieser Anzeige, mit demselben Text. Bosse, für die ein Effekt nicht zutrifft (z. B. Warnkreis beim `goblinLeader`), hatten dort nie eine Lücke.
- **Qualität:** Beispielzustände (Boss, Phase, Warnkreis-Radius, Banner-Anlass) als reine Funktionen mit Vitest-Test im Studio-Frontend; keine Zufallswerte; Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Nicht-Ziele

- Zeichnen von Warnkreis, Boss-Leiste, Phasen-Text und Event-Banner im Spiel: K5.1 (B-132, CLI).
- Gegner-Seite selbst, Gruppen, Merkmale und Bosse ohne Anzeigen: B-464 (WZ11).
- Boss-Kampfablauf als Simulation (Beschwörung, Flammenspur, Splitter-Wurf); die Seite zeigt Beispielzustände.
- Neue Boss-Grafiken (Projekt GRA), Änderungen an `data/bosses.json`, `data/events.json`, `data/assets.json`.
- Änderungen an `src/scenes/`, `src/model/`, `src/online/`: reicht eine Funktion des Spiels nicht, entsteht ein CLI-Ticket.
- Nacht-Events (`fullMoon`, `bloodMoon`) als Banner, solange 🧑 es nicht entschieden hat (siehe Offene Fragen).

## Regeln und Einschränkungen

- **Domäne DEV:** `tools/k3c-studio/` (Studio-Frontend) und `src/tools/`; Spiel-Code aus `src/scenes/`, `src/core/` und `src/model/` nur lesend über `@game`.
- **Abhängigkeiten (beide vor Planung und Umsetzung):** K5.1 (B-132) ist erledigt und der Zeichencode für Warnkreis, Boss-Leiste, Phasen-Text und Event-Banner liegt im Code; WZ11 ist erledigt (Gegner-Seite, B-464; sie hängt von WZ10 ab: Abspiel-Bausteine aus B-463/WZ10.1, Bühne aus B-460/WZ7.3). Bis dahin bleibt das Ticket `offen`, ohne Sprint.
- **Kein Nachbau im Studio:** nur der Zeichencode des Spiels; der Client rechnet nichts (`src/scenes/noSim.test.ts`), Beispielzustände baut die Seite aus den Daten, nicht aus der Simulation.
- **Spec-Stand:** Die Namen und Signaturen des Zeichencodes stehen erst nach K5.1 fest; vor der Freigabe prüft die Planung sie im Code und passt die Kriterien an.
- **Begriffe** nach `docs/glossar.md` (Boss, Miniboss, Endboss, Tiefe). Die neuen Begriffe Warnkreis, Boss-Leiste, Event-Banner, Lücke und Gegner-Seite stehen vor der Freigabe im Glossar (wie in `WZ11 › Offene Fragen` und `B-464 › Regeln und Einschränkungen`), nicht erst in den Sessions.
- **Komplexitäts-Budget:** Datei ≤ 400, Funktion ≤ 60 Zeilen, Session ≤ ~400 geänderte Code-Zeilen; Fensterprüfungen nur mit Freigabe von 🧑 für den Lauf.

## Beispiele

- Bosse › Tiefe 4 › Kristallherz-Wächter, Phase 2 → Phasen-Text und Schwelle `belowHpPercent` stehen an der Karte, der Warnkreis hat den Radius aus `ability.radius` Units, die Boss-Leiste zeigt den Beispielzustand der Phase. In Phase 3 (ohne eigene `ability`) bleibt der Warnkreis mit demselben Radius, die Phase 1 hat keinen.
- Bosse › Tiefe 1 › Trollkönig → Warnkreis um den Boss mit Radius der Fähigkeit `aoe`.
- Ereignis `bossSpawned` gewählt → das Event-Banner des Spiels erscheint mit dem Text des Spiels; `bossPhase` und `bossDefeated` ebenso mit ihrem Text.
- Goblin-Anführer (Fähigkeit `summon`, kein Flächenschlag) → kein Warnkreis, wie im Spiel; keine Lücke.

## Ausnahme- und Fehlerfälle

- Boss ohne Flächenschlag in der gewählten Phase → kein Warnkreis, wie im Spiel (`warn` fehlt); keine Lücke.
- Das Spiel kennt für ein Boss-Ereignis keinen Banner-Text → Lücke „Banner-Text fehlt im Spiel“ statt eines leeren Banners.
- Das Spiel zeichnet eine der vier Anzeigen nach K5.1 doch nicht (Teilumfang) → an dieser Anzeige bleibt die Lücke „Anzeige im Spiel fehlt (B-132, Anbindung B-469)“ (je Boss nur dort, wo der Effekt zutrifft), die übrigen werden gezeigt.
- Endboss-Phase ohne `ability` → Fähigkeit der vorigen Phase (Regel aus `data/bosses.json` › `notes`).
- Zeichencode nicht über `@game` ladbar (Umbenennung) → Hinweis mit dem Namen statt eines leeren Bildes, kein Absturz.

## Akzeptanzkriterien

- **AC-01** (früher B-464/AC-09) Für jeden Boss mit Flächenschlag (`trollKing`, `crystalHeartWarden` ab Phase 2) zeigt die Gegner-Seite den Warnkreis des Spiels mit Radius `ability.radius` Units. Nachweis: Vitest-Test des Beispielzustands (Radius je Boss und Phase, beim Endboss ab Phase 2 einschließlich der erbenden Phase 3), Beobachtung durch 🧑 im Studio.
- **AC-02** (früher B-464/AC-10, erster Teil) Die Gegner-Seite zeigt die Boss-Leiste des Spiels mit einem Beispielzustand. Nachweis: Beobachtung durch 🧑, Review: Aufruf des Zeichencodes über `@game`.
- **AC-03** (früher B-464/AC-10, zweiter Teil) Beim Endboss ist jede Phase einzeln wählbar und zeigt den Phasen-Text des Spiels und die Schwelle `belowHpPercent`. Nachweis: Vitest-Test (Phasen aus `data/bosses.json`), Beobachtung durch 🧑.
- **AC-04** (früher B-464/AC-11) Erscheinen, Phasenwechsel und Sieg eines Bosses (`bossSpawned`, `bossPhase`, `bossDefeated`) zeigen das Event-Banner des Spiels mit dessen Text. Nachweis: Vitest-Test (Anlass → Banner-Eingabe), Beobachtung durch 🧑.
- **AC-05** Die Lücke „Anzeige im Spiel fehlt (B-132, Anbindung B-469)“ ist für jede der vier Anzeigen entfernt, die das Spiel nach K5.1 zeichnet (Warnkreis, Boss-Leiste, Phasen-Text, Event-Banner); sie bleibt, mit demselben Text, nur an einer Anzeige, die das Spiel nicht zeichnet (Teilumfang, siehe Ausnahme- und Fehlerfälle). Das Studio enthält keinen eigenen Zeichencode dafür. Nachweis: Review des Diffs (Aufrufe nur über `@game`); `git grep -n "Anzeige im Spiel fehlt (B-132, Anbindung B-469)" -- tools/k3c-studio/` findet nur Treffer für Anzeigen, die das Spiel nicht zeichnet, und ist leer, wenn das Spiel alle vier zeichnet.
- **AC-06** Die Tests der Beispielzustände laufen in `task check:all` und in der CI mit. Nachweis: Ausgabe von `task check:all`, CI-Lauf.

## Offene Fragen

Entschieden von 🧑 am 2026-10-17 (Chat):

1. Die Anbindung der Boss-Anzeigen (Warnkreis, Boss-Leiste, Phasen, Event-Banner) an die Gegner-Seite ist ein Folge-Ticket nach K5.1/B-132; WZ11 zeigt sie bis dahin als Lücke.

Offen (entscheidet 🧑, nicht blockierend, Vorschlag der Planung):

2. **Event-Banner:** nur für Boss-Ereignisse (Vorschlag) oder auch für Nacht-Events (`fullMoon` mit `alphaWolf`, `bloodMoon`)? Übernommen aus B-464 › Offene Fragen 5.
3. **Sprint:** Das Ticket hat keinen Sprint; ein neuer Sprint in Projekt WZG oder ein Platz in einem späteren Sprint wird beim Planen entschieden, nachdem K5.1 und WZ11 erledigt sind.
4. **Namen des Zeichencodes:** Die Namen aus K5.1 stehen erst nach der Umsetzung fest; die Kriterien werden vor der Freigabe daran angepasst.

## Notizen

- Verwandt: B-464 (Gegner-Seite, WZ11; AC-09, AC-10 und AC-11 sind hierher verschoben), B-463 (Abspiel-Bausteine, WZ10), B-460 (Phaser-Bühne, WZ7.3), B-132/K5.1 (Anzeigen im Spiel, CLI), B-329 (echte Figuren).
- Datenlage zu den Beispielzuständen: `data/bosses.json` › `bosses` je Tiefe 0–4 (Fähigkeiten `summon`, `aoe`, `flameTrail`, `shardThrow`) und `endBosses` (`crystalHeartWarden`, 3 Phasen); Zustand im Protokoll: `enemies[].phase`, `enemies[].warn` (`docs/protocol.md` › Kampf).
- Spec-Stand: Entwurf, noch nicht freigegeben. Das Ticket entstand am 2026-10-17 aus der Entscheidung von 🧑, die vier Boss-Anzeigen nicht in der Gegner-Seiten-Vorschau (WZ11) zu zeigen.
