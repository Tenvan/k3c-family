# B-652 · Mauern und Türme stehen verteilt in jedem Chunk, das Tor ist die äußerste gebaute Mauer

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** BS12
- **Projekt:** BAS
- **Erstellt:** 2026-10-28
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-28, Chat, durch 🧑, Revision 1, mit den Vorschlägen der offenen Fragen (Radius 6, 60 % Türme nah, Chunk n sichtbar ab Hub-Stufe n)

## Ausgangslage

Heute (`docs/rules/materialien-gebaeude.md` § 3, `engine/level/lines.go`, `data/hub.json › wallLines`): je Seite 5 feste **Mauerlinien** mit je einem Mauer-Platz, einem Turm-Platz (8 Units innen) und einem **Tor als eigenes Gebäude** (`data/buildings.json › gate`, 4 Units außen), Streuung 0 bis +4 Units je Seed, Linie k ab Hub-Stufe k und nur nach gebauter Mauer der Linie k−1. Das Tor ist nur an der äußersten gebauten Linie bezahlbar (Q47). Der Aufbau ist linear und vorhersehbar; K2C (Kingdom Two Crowns) setzt Mauern und Türme dagegen dynamisch, Türme häufig hinter Mauern.
Beschluss 🧑 2026-10-28 (Chat): Das ist eine Grundlage des Balancings und kommt **vor** weiteren Balancing-Läufen (B-651 pausiert).

## Ziel

Mauern und Türme sitzen je Seed verteilt statt auf fester Linie, das Tor ist kein eigenes Gebäude mehr, sondern die äußerste gebaute Mauer; Verteidigungsaufbau wirkt dynamisch wie in K2C.

## Beteiligte und Zielgruppen

🧑 entscheidet Regeln und Werte; Spieler (Koop, Xbox/PC/Mobile) erleben den freieren Ausbau; Claude setzt um.

## Anforderungen

1. **Tor entfällt als Gebäude.** `gate` verschwindet aus `data/buildings.json`, Tor-Platz aus `data/hub.json`, den Regeln (`materialien-gebaeude.md` § 3 und § 4, Glossar ›Tor‹, ›Tor-Platz‹, ›Äußerste Sperre‹), Level-Generator, Sim, Client (Sprites, Texte, Hinweise) und Bots.
2. **Tor-Regel:** Die **äußerste gebaute Mauer je Seite** ist das Tor: Spieler, Taler und eigene Bürger und Truppen passieren, Gegner nicht. Sie wird wie jede Mauer ausgebaut (gleiche Stufen, HP, Kosten); baut man weiter außen, wird die bisher äußerste wieder eine normale Mauer, die neue äußerste das Tor. Zerstört Gegner die äußerste, ist die nächste innere das Tor.
3. **Mauer-Plätze:** In jedem Chunk der Seite (nicht nur 5 feste Linien) legt der Generator **1 bis 3 Mauer-Plätze** an, Anzahl und Ort je Seed aus einem eigenen RNG-Strom (wie `…:sites`, Golden-Level für Ressourcen, Portale, Camps bleiben stabil, `engine/rng`, kein `math/rand`).
4. **Turm-Plätze:** Ebenso **1 bis 3 Turm-Plätze je Chunk**, per Seed. **Nah beieinander:** Ein Teil der Türme (Startwert 60 %, Feld in `data/hub.json`) steht **2 bis 10 Units** neben einer Mauer (Beschluss 🧑), der Rest frei im Chunk; Mauern und Türme überlappen nicht (Mindestabstand Plätze, Test).
5. **Freischaltung (Beschluss 🧑):** Ein Mauer- oder Turm-Platz ist bezahlbar, **sobald in seiner unmittelbaren Nähe keine Bäume und Felsen mehr stehen** (Radius als Feld, Startwert 6 Units). Es muss **nicht jede innere Mauer gebaut sein**, um weiter außen zu bauen. Die Hub-Stufe bestimmt wie bisher das Material und die Ausbaustufe der Mauer/des Turms, nicht den Ort (zu klären: ab welcher Hub-Stufe Plätze in welcher Entfernung überhaupt sichtbar sind, siehe Offene Fragen).
6. **Abhängige Regeln neu fassen** (Spec/Tests): Linien-Begriff (`WallLine`, `Lines`, `ClearOf`, Plan-Prüfung `ValidatePlanFor`), Krieger-Posten an der äußersten Sperre (`buerger.md` § 1), Zerstörungs-Regel Q58, Camps/Portale/Lava-Abstand zu Linien (`level.go`), Wellen-Ziele der Gegner, Bot-Logik (`bot_wirtschaft.go`, `buildOrder`, `innerWallsBuilt`, Abstieg), Balancing-Ziele (›Burg hält‹, ›Erste Mauer‹, ›Erster Turm‹).
7. Deterministisch, 2+ Spieler, Leistung (viele Plätze je Chunk dürfen den Tick nicht merklich verlangsamen).

## Nicht-Ziele

Neues Wellenverhalten; neue Gebäudearten; das Balancing selbst (danach BS3/B-651); die Baustellen-Fehler und der Holzzaun (B-653).

## Regeln und Einschränkungen

Werte nur in `data/`; Generator deterministisch (`engine/rng`); Dateien ≤ 400 Zeilen, Funktionen ≤ 60; keine Rücksicht auf alte Spielstände (Spielstand-Format darf brechen, Version anheben); Protokoll `docs/protocol.md` anpassen, wenn Bauplatz-Listen sich ändern; Golden der Level (`level-*`) und Sim mit Begründung neu (`task go:golden`); Tests über Go-Tests, Balancing-Messung per `task balance:milestone`.

## Beispiele

Seed `k3c-dev`, Wald, rechte Seite: Chunk 1 hat 2 Mauer-Plätze (x 28, 61) und 1 Turm bei x 33 (5 Units neben der Mauer), Chunk 2 hat 3 Mauern und 2 Türme usw. Der Spieler baut Chunk 1 und 3, überspringt 2 (Felsen dort noch nicht abgebaut): Die Mauer in Chunk 3 ist das Tor, Chunk 1 eine normale Mauer.

## Ausnahme- und Fehlerfälle

Chunk ohne freien Platz (Lava, Portal, Camp): weniger als 1 Platz erlaubt, Test dokumentiert die Untergrenze je Biom; kein Platz auf einer Seite: Generator-Fehler beim Prüfen (`-check-data`). Gegner zerstören die äußerste Mauer: das Tor wandert nach innen. Baum/Fels wird erst später markiert: der Platz bleibt gesperrt (graue Anzeige).

## Akzeptanzkriterien

- **AC-01** Kein `gate` mehr in `data/`, Code, Client und Regeltexten; Test schlägt bei einem übrig gebliebenen Tor an.
- **AC-02** Die äußerste gebaute Mauer je Seite lässt Spieler und eigene Einheiten durch, Gegner nicht; nach Ausbau weiter außen oder Zerstörung wandert das Tor (Test, 2 Spieler).
- **AC-03** Je Chunk und Seite 1 bis 3 Mauer-Plätze und 1 bis 3 Turm-Plätze, per Seed deterministisch (500 Seeds, Test), Ressourcen/Portale/Camps der Golden-Level unverändert.
- **AC-04** Mindestens der Anteil Türme aus `data/hub.json` steht 2–10 Units neben einer Mauer (500 Seeds); Plätze überlappen nicht.
- **AC-05** Ein Platz ist erst bezahlbar, wenn im Radius keine Bäume und Felsen stehen; äußere Plätze sind ohne innere gebaute Mauern bezahlbar (Test).
- **AC-06** Bots (`wirtschaft`, `coop2`) bauen mit den neuen Plätzen; `task balance:milestone` läuft durch (Ergebnis offen, kein Grün-Ziel).
- **AC-07** Regeln (`materialien-gebaeude.md`, `buerger.md`, `gegner.md`), Glossar und Protokoll-Doku sind angepasst; `task check:diff` grün.

## Offene Fragen

- Welche Hub-Stufe schaltet welche Platz-Entfernung frei (alle Plätze sofort sichtbar oder gestaffelt nach Chunk n ab Hub-Stufe n)? Vorschlag: Sichtbarkeit je Chunk wie bisher Linie k ab Hub-Stufe k, Kauf nur nach Freiraum.
- Radius der „unmittelbaren Nähe“ (Vorschlag 6 Units) und Anteil Türme nahe Mauern (Vorschlag 60 %).
- Gilt die Regel in allen Biomen (auch Eisenstollen/Kristallhöhle mit 25-Units-Chunks, dort 1–2 statt 1–3)?
- Krieger-Posten: an der äußersten gebauten Mauer (Tor) wie bisher?

## Notizen

**Messung nach BS12.2 (2026-10-28, Claude, `task balance:milestone`, coop2, standard, k3c-dev, 40 Tage):** Der Frühspiel-Lauf bricht zusammen (Burg fällt in Nacht 1). Vorher (Mauerlinien): Stufe 5 ab Tag 22, vier Minibosse, Titan nie. Ursache: Freiraum-Radius sperrt im Wald jeden Platz.

**Schritte:** Variante A (Baugrund frei, `Layout.FreeGround`), Variante E (Zone-1-Baugrund fest bei Mauer 44 und Turm 36, `sitePlacement.zone1WallUnits/zone1TowerUnits`), Bot: `wanted()` (je Seite, Zone, Art ein Platz), `innerWallsBuilt` auf Zone 1 (vorher stieg der Bot nie ab: Stein fehlte, die endlichen Felsen im Wald geben nur etwa 20), Holz-Reserve vor dem Abstieg (`descentWood` 400: unter Tage wächst kein Holz, Mauern in der Höhle warteten ewig auf Material), `fighterTarget` (Stufe mit Endboss-Bau will 16 Kämpfer).

**Messung jetzt (coop2, k3c-dev, 40 Tage):** Goblin-Anführer Tag 6, Troll-König Tag 15, Ratten-Königin Tag 19, Lava-Golem Tag 22, **Splitter-Titan Tag 26 (erstmals)**; der Endboss fällt nicht. Vorher (Linien): Titan nie. Die Burg fällt zuletzt.
**Rest (Befund):** In der Kristallhöhle stehen Höchstens 10 Kämpfer (das Ziel 16 wird nicht erreicht, ein fester Deckel ist in `data/` nicht zu finden: `grep` nach maxArchers u. ä. leer; vermutet: Anwerbe-Grenze der Landstreicher oder Bauern-Limit des Bots 4, ungeprüft). Der Endboss (20 883 HP) wird um Tag 35–36 aktiv und die Beschwörung räumt das Heer ab (10 → 0). Nächster Schritt: Ursache des Kampfer-Plateaus bei 10 suchen (Bot oder Anwerbe-Regel), dann Mehr-Seed-Lauf.
