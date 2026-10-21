# B-468 · Geplante Chunks tragen Ruinen und Gegnerlager

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** WZ14
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-21, Ralf im Chat (Revision 2)

## Ausgangslage

B-461 (Chunk-Planung, Sprint WZ8) macht geplante Chunks mit festen Objekten (Ressourcen, Adern, Truhen, Camps, Portale) und Chunk-Seed planbar. 🧑 wollte zusätzlich vorgebaute Gebäude (Ruinen) und feste Gegner (Gegnerlager) in geplanten Chunks; beides passt nicht in denselben Sprint und wurde am 2026-10-17 (Chat) als Teil 2 in dieses Ticket verlegt (B-461 › Offene Fragen 4).

Heute gibt es dafür keine Grundlage:

- **Gebäude** entstehen nur auf Bauplätzen des Hubs (`data/hub.json` › `sites`, `islandSites`, Mauerlinien) und auf Welt-Plätzen neben dem Hub (`worldSiteSpecs`, z. B. Farm). Ein Bauplatz mitten im Level, der schon gebaut ist oder repariert werden muss, existiert nicht. Der Spielstand ordnet Plätze über `SiteSave` und `kind@x` zu; `castleLosses` setzt beim Burgfall Bauplätze leer.
- **Gegner** kommen aus Portalen als Wellen (`docs/rules/gegner.md` § 3: Aggressionspool, Nacht-Welle, Skalierung je Tiefe). Eine zweite Quelle, die dauerhaft an einer festen Stelle steht, gibt es nicht; Camps (`recruitCamp`) liefern Landstreicher, keine Gegner.

Das Format aus B-461 reserviert dafür nur Felder, die der Generator mit „noch nicht unterstützt“ ablehnt.

## Ziel

Ein geplanter Chunk kann eine **Ruine** tragen: ein gebautes, stark beschädigtes Nutzgebäude an fester Lage. Sie lässt sich im Editor des Studios setzen, wird beim Laden geprüft, erscheint in Chunk- und Live-Vorschau und im Spiel, wirkt wie ein gebautes Gebäude ihrer Art und kehrt nach einem Burgfall in ihren geplanten Zustand zurück. Nutzen: handgeplante Sondergegenden mit Geschichte, die den Spielenden einen Vorsprung geben, den sie verteidigen und reparieren. (Gegnerlager: B-517.)

## Beteiligte und Zielgruppen

- 🧑 Ralf: hat die Regeln beschlossen (WZ14.1), plant im Editor, nimmt ab und gibt die Spec frei.
- Spielende (Couch-Koop, 2+ Spieler): treffen auf Ruinen an bekannter Stelle der Insel und verteidigen sie als Vorposten.
- Agenten: setzen Format, Prüfung und Sim (SIM), Protokoll und Vorschau (SRV), Grafik (CLI) und Editor (DEV) um; ein getrennter Reviewer prüft.

## Anforderungen

Beschlossen im Regel-Workshop WZ14.1 (2026-10-21, Ralf im Chat) samt Nachtrag nach dem Spec-Review desselben Tages; Regeltext: `docs/rules/geplante-chunks.md` § 8. B-468 umfasst ab Revision 2 nur noch die **Ruinen**; die Gegnerlager stehen in B-517 (eigener Folge-Sprint).

- **Ruine:** gebautes, stark beschädigtes **Nutzgebäude** an fester Lage eines geplanten Chunks; Arten `workshop`, `storage`, `farm`, `barracks`, `tavern`, `healer`, `smithy`, `armory`; Start-HP 25 % (aufgerundet); Arten und Anteil in `data/ruins.json`; höchstens **eine** je Chunk.
- **Wirkung:** wirkt ab Spielbeginn wie ein gebautes Gebäude ihrer Art (Kaserne → Truppenlimit, Lager → Kapazität des Vorrats); **Hub-neutral**; die Kennung `ruin` ist dauerhafte Herkunft des Platzes.
- **Reparatur und Vorposten:** Bauern reparieren kostenlos zwischen den Wellen; jenseits des äußersten Tors (Lage laut Layout der Stufe) reparieren und bauen sie nur ohne Gefahr, und Bürger holen dort bei Gefahr keine Waffen (neue Regel für alle Plätze außerhalb der Mauern). Gegner greifen Ruinen an wie jedes Gebäude.
- **Wiederaufbau:** eine zerstörte Ruine ist leer und erst ab der Hub-Stufe ihrer Art bezahlbar.
- **Burgfall:** `castleLosses` setzt Ruinen-Plätze auf den geplanten Ausgangszustand zurück (auch nach Zerstörung).
- **Format:** Feld `buildings` des geplanten Chunks lesen und prüfen (Art, Anzahl, Lage-Regeln § 8, `kind@x`); `enemies` bleibt reserviert (B-517). `engine/level` liest `data/ruins.json` selbst; die Signaturen von `ValidatePlanFor` und `PreviewChunkObjects` bleiben (das Studio nutzt sie), Neues kommt als neue Funktion.
- **Spielstand und Protokoll:** keine neue Spielstand-Version, Protokoll 7, Feld `ruin` additiv an `sites[]`.
- **Dev-Befehl „Burgfall“** in Sim und Protokoll, als Debug-Knopf im Spiel, damit 🧑 die Rückkehr der Ruine bei der Abnahme sieht.
- **CLI:** Ruinen unterscheidbar durch Schadens-Darstellung nach HP-Anteil plus Trümmer, keine neuen Sprites je Art. **DEV:** Editor und Vorschau des Studios setzen, zeigen und entfernen die Ruine.
- Deterministisch, 2+ Spieler, Benchmark der Level-Erzeugung ≤ +10 %.

## Nicht-Ziele

- Gegnerlager (B-517, Folge-Sprint); das Feld `enemies` bleibt reserviert und abgelehnt.
- Alles, was B-461 liefert (Format, Chunk-Seed, feste Objekte, Generator, Prüfung, Editor, Schreibregeln); B-468 baut darauf auf.
- Neue Sprites je Gebäudeart für Ruinen, neue Gebäudearten, Reparatur gegen Bezahlung, neuer Site-Zustand.
- Echte Planungen mit Ruinen einchecken (REG-Entscheidung von 🧑 nach dem Sprint).
- Mehrere Inseln und Inselwechsel (B-103); Freischaltung je Hub-Stufe (B-462).
- Migration bestehender Spielstände (keine Rücksicht auf alte Spielstände, Beschluss 🧑 bis auf Widerruf).

## Regeln und Einschränkungen

- **Abhängigkeit:** WZ8 (B-461) ist erledigt (Format, Generator, Editor, Vorschau-Option).
- **Domänen nacheinander, je Session eine:** SIM (Werte, Format, Prüfung) → SIM (Ruine als Platz) → SIM (Vorposten-Schutz, Dev-Burgfall) → SRV (Protokoll, Vorschau) → CLI (Grafik) → DEV (Editor), dazu Review und Abnahme; 9 Sessions inklusive Workshop.
- **Regeln aus B-461 gelten:** Zufall nur über `engine/rng` mit eigenem Strom je geplantem Chunk, Haupt-Strom ohne Planung unverändert, Golden ohne Planung byte-gleich, Test-Planung unter `testdata/` mit eigener Fixture (nicht `testdata/planned/forest.json`, die speist `level-planned.json`), Werkzeug schreibt REG-Daten nur atomar, format-stabil und mit Stand-Prüfung samt REG-Hinweis; der Sprint liefert keine echte Planung und keine Golden-Änderung aus.
- **Verträge:** Spielstand über `kind@x` (`SiteSave`) ohne neue Version; Protokoll Version 7 bleibt, nur additive Felder (`docs/protocol.md`, `testdata/protocol`). Öffentliche Funktionen von `engine/level` und `engine/room`, die das Studio nutzt, behalten ihre Signatur; Sessions, die sie berühren, fahren auch `task check:studio`.
- **Schichten und Budget:** `engine/level` importiert nichts aus `engine/sim`, `engine/room`, `engine/net`; Datei ≤ 400, Funktion ≤ 60 Zeilen; Logging mit Emoji (`CLAUDE.md`).
- **Mit 2 Spielern gleichzeitig** muss jede Mechanik funktionieren; Gamepad-Taste B bleibt unbelegt.

## Beispiele

- Wald, Platz +4 mit Ruine `smithy` bei `at: 6` → dort steht eine Schmiede mit 63 von 250 HP (25 %, aufgerundet); sie wirkt sofort als Schmiede; tagsüber ohne Gegner reparieren Bauern sie auf 250 HP.
- Nachts greifen Gegner die Schmiede auf Platz +4 an; die Bauern bleiben hinter den Mauern (Vorposten-Regel); fällt die Schmiede, ist der Platz leer und erst ab Hub-Stufe 3 (Stufe der Schmiede) wieder bezahlbar.
- Höhle, Platz −3 mit Ruine `barracks` → das Truppenlimit steigt ab Spielbeginn wie bei einer gebauten Kaserne; die Hub-Stufe bleibt unverändert.
- Die Burg fällt → alle Hub-Plätze werden leer wie heute; die zerstörte Schmiede auf Platz +4 steht wieder als Ruine mit 63 HP.
- Ein Chunk ohne Ruine → verhält sich genau wie in B-461; Golden-Daten byte-gleich.

## Ausnahme- und Fehlerfälle

Beim Laden abgelehnt mit Insel, Tiefe, Platz und Grund (fail closed wie B-461):

- Gebäudeart, die es nicht gibt oder die als Ruine nicht erlaubt ist (z. B. `castle`, `wall`, `stairsUp`).
- Mehr als eine Ruine im Chunk.
- Lage außerhalb des Chunks oder näher als 2 Units am Rand; näher als 4 Units an einem Bauplatz der Mauerlinien, dem Farm-Weltplatz, einer Ader, einem Portal oder einem festen Objekt; Farm-Ruine, deren Plantage (±3 Units) nicht in den Chunk passt.
- Gleicher Schlüssel `kind@x` wie ein anderer Platz derselben Stufe.
- Eintrag im Feld `enemies` → weiterhin „noch nicht unterstützt (B-517)“.
- `data/ruins.json` ungültig (Anteil nicht in (0, 1], unbekannte Art, Hub-Gebäude) → Laden der Daten scheitert.

Im Spiel:

- Spielstand mit einer Ruine, die die Planung nicht mehr kennt → der Platz wird verworfen (vorhandene Regel „Bauplatz gibt es in dieser Version nicht mehr“), keine Migration.
- Spielstand ohne Eintrag für einen Ruinen-Platz → die Ruine steht im geplanten Ausgangszustand.
- Gefahr in der Nähe eines Vorpostens → Bauern brechen Reparatur oder Bau ab und kehren hinter die Mauern zurück.

## Akzeptanzkriterien

- **AC-01** Regeltext für Ruinen (`docs/rules/geplante-chunks.md` § 8) und Glossar-Eintrag „Ruine“ beschreiben die Beschlüsse aus WZ14.1; erlaubte Arten und HP-Anteil stehen in `data/ruins.json` und werden beim Laden geprüft (Go-Test je Fehlerfall: Anteil außerhalb, unbekannte Art, Hub-Gebäude); Glossar „Gegnerlager“ verweist auf B-517.
- **AC-02** Der Generator liest das Feld `buildings` des geplanten Chunks, prüft es unabhängig vom Seed und lehnt jeden Fall aus „Ausnahme- und Fehlerfälle“ mit Insel, Tiefe, Platz und Grund ab; je Fall ein Go-Test; `enemies` bleibt abgelehnt; die Signaturen der vom Studio genutzten Funktionen bleiben.
- **AC-03** In der Sim steht die Ruine gebaut mit 25 % HP, wirkt wie ein gebautes Gebäude ihrer Art, ist Hub-neutral, trägt die Kennung `ruin` dauerhaft, wird von Bauern repariert (jenseits des äußersten Tors nur ohne Gefahr, dort auch kein Waffen-Holen bei Gefahr), kann zerstört werden, ist danach erst ab der Hub-Stufe ihrer Art bezahlbar und kehrt nach `castleLosses` in den Ausgangszustand zurück (auch nach Zerstörung); eine Dev-Funktion „Burgfall“ löst `castleLosses` aus; Go-Tests mit 2 Spielern (`CreateIsland`); Spielstand speichert und lädt sie ohne neue Version.
- **AC-04** Ohne Ruine bleiben alle Golden-Daten byte-gleich und der Benchmark der Level-Erzeugung ≤ +10 %.
- **AC-05** Snapshot (`sites[].ruin`) und Vorschau-Funktion tragen die Ruine (Protokoll 7, `docs/protocol.md`, `testdata/protocol`, `src/model/types.ts`), der Dev-Befehl „Burgfall“ ist über das Protokoll und als Debug-Knopf im Spiel erreichbar; der Client zeichnet Ruinen unterscheidbar von gebauten Gebäuden; der Editor des Studios setzt, zeigt und entfernt sie und hält die Schreibregeln aus B-461 ein (Tests je Schicht).
- **AC-06** Das Review weist jedes Kriterium nach; `task check:all`, `task check:go` und `task check:studio` grün; keine echte Planung mit Ruine eingecheckt.
- **AC-07** 🧑 nimmt eine Sondergegend mit Ruine am PC ab: Editor, Chunk- und Live-Vorschau, „Im Spiel öffnen“ (Ruine wirkt, wird repariert) und Rückkehr nach dem Debug-Knopf „Burgfall“.

## Offene Fragen

Entschieden im Regel-Workshop (WZ14.1, 2026-10-21, Ralf im Chat, Vorschläge von Claude angenommen):

1. **Ruine:** gebautes, stark beschädigtes Nutzgebäude (25 % HP), Nutzgebäude-Arten (siehe Anforderungen), höchstens eine je Chunk, Hub-neutral, aber funktional; Reparatur kostenlos durch Bauern; Burgfall → zurück zur Ruine.
2. **Gegnerlager:** stationäre Wache, besiegbar, kein Respawn, keine Wirkung auf Wellen → ausgelagert nach B-517.
3. **Spielstand/Protokoll:** keine neue Spielstand-Version, Protokoll 7 bleibt (additive Kennung).
4. **Zuschnitt:** zwei Sprints – WZ14 Ruinen, Gegnerlager als Folge-Sprint (B-517).

Nachtrag nach dem Spec-Review (2026-10-21, Ralf im Chat):

5. **Lage:** Ruinen auch außerhalb der Mauern als Vorposten; neue Regel: jenseits des äußersten Tors reparieren und bauen Bauern nur ohne Gefahr.
6. **Wiederaufbau:** eine zerstörte Ruine ist erst ab der Hub-Stufe ihrer Art bezahlbar.
7. **Dev-Befehl „Burgfall“** für die Abnahme: ja.
8. **Vorschläge von Claude bestätigt:** Abstände (2 Units Rand, 4 Units zu Bauplätzen, Farm-Weltplatz, Adern, Portalen, festen Objekten), Kennung `ruin` als dauerhafte Herkunft, Darstellung über HP-Anteil plus Trümmer ohne neue Sprites, Lager-Ruine erhöht die Kapazität.

## Notizen

- Herkunft: Teil 2 der Entscheidung 2 aus B-461 (Entschieden 4, 2026-10-17, Chat: Teilung angenommen, Projekt WZG, Folge-Sprint nach WZ8). Bezug: B-461 (Format, Editor, Vorschau-Option), B-460 (Welt-Betrachter), B-462 (Hub-Freischaltung), B-103 (mehrere Inseln).
- Fundstellen (Stand 2026-10-17, ungeprüft über die Notizen von B-461 hinaus): `worldSiteSpecs`, `SiteSave`/`kind@x`, `castleLosses` in `engine/sim`; Wellenregeln `docs/rules/gegner.md` § 3.
- Aufwand grob 7–9 Sessions (REG, SIM, SIM, SRV, CLI, DEV, Review, Abnahme).
