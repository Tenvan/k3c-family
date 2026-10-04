# Ist-Stand Materialien und Gebäude (Vorbereitung R2.2)

Stand: 2026-10-02 (Session R2.1). Vergleich von `docs/game-design.md` mit dem Go-Code (`engine/sim/`, `engine/level/`) und `data/*.json`.
Nur Tatsachen aus Code und Daten; was nicht geprüft wurde, steht als **ungeprüft**. Dieses Dokument beschließt nichts und ändert keinen Wert.
Beschlossene Rahmenregeln (nicht verhandelbar hier): Material gehört der **Insel**, Gold je Spieler (`stufen.md` § 1, `wirtschaft.md` § 1); Krieger, Elite, Tor, Farm, Kaserne und Truppen-Limit „später gebaut“ (`wirtschaft.md` § 5).

## 1. Materialien

| Material | Quelle (Chunk-Typ, Menge je Chunk) | Sammeln | Verwendung im Code | Verwendung in den Daten | Stufe |
|---|---|---|---|---|---|
| **Holz** | Baum: Wald `forest` 4–7, `clearing` 0–2, `rocks` 1–2, `stream` 1–3; Höhle `cavern` 0–1 | 10 Holz in 4 s, Markierung 1 Münze | Mauer 20, Turm 50, Werkstatt 40, Bogen 50 | + Tor 30, Farm 30, Krieger 50 (kein Code) | Wald Hauptressource, Höhle fast keins, Mine keins |
| **Stein** | Fels: Wald `rocks` 2–4; Höhle `tunnel` 2–4, `cavern` 3–5, `crystals` 1–3, `chasm` 0–2; Mine `shaft` 1–3, `rails` 1–2 | 10 Stein in 6 s, Markierung 1 Münze | Treppe hoch/runter 100 | + Kaserne 60, Elite-Bogenschütze 100 (kein Code) | Höhle Hauptressource |
| **Kupfer** | Kupfererz: Mine `rails` 0–1, `oreVein` 2–4 | 10 Kupfer in 8 s, Markierung 2 Münzen | **keine** | Elite-Krieger 100 (kein Code) | Mine Hauptressource |
| **Gold** | Truhen 10–25, Gegner-Drops (je Art), Tageseinkommen 5 je Spieler | Münzen einsammeln | alles Bezahlen (Bauplatz, Landstreicher, Markierung, Bogen) | – | überall |
| **Busch** | Wald `clearing` 1–3 | – | **keine** (kein Sammelobjekt, `economy.json` › `gatherables` kennt nur Baum, Fels, Kupfererz) | – | Wald |

- Vorrat: `World.Stock` je Welt (Holz, Stein, Kupfer als ganze Zahlen). **Kein Maximum** (anders als der Gold-Beutel, max. 100).
- Die Gegner lassen mit 10 % je 5 Einheiten der Hauptressource der Stufe fallen (`economy.json` › `enemyResourceDrop`).
- Nur Bauern sammeln, und nur markierte Ressourcen (Markierung kostet Gold, siehe Spalte Sammeln); nach dem Sammeln ist das Objekt verbraucht (kein Nachwachsen im Code, ungeprüft ob später im Level neu erzeugt wird).
- `game-design.md`: „Post-MVP: Tiefe 3 (Eisen, Lava), Tiefe 4 (Kristall)“; kein Eisen und kein Kristall in Daten oder Code.

## 2. Gebäude

`buildings.json` (HP, Bauzeit `buildSeconds`, Kosten, Freischaltung `unlockDepth`), Bauplätze `hub.json` (Offset zur Hub-Mitte, Hub 100 Units breit, ±50).

| Gebäude | Kosten | HP | Bauzeit | `unlockDepth` | Bauplatz in `hub.json` | Wirkung im Code |
|---|---|---|---|---|---|---|
| Burg/Thron | – | 1000 | – | 0 | steht in der Mitte | Hub-Kern; fällt sie, wirkt der Niederlage-Modus (`stufen.md` § 4) |
| Mauer | Holz 20 + Gold 5 | 300 | 6 s | 0 | 2: −44, +44 | blockiert Gegner (`ignoresWalls` umgeht) |
| Turm | Holz 50 + Gold 20 | 200 | 10 s | 0 | 2: −36, +36 | 2 Bogenschützen-Plätze, +3 Units Reichweite |
| Tor | Holz 30 + Gold 10 | 250 | 6 s | 0 | **keiner** | **keine** (nur Daten) |
| Werkstatt | Holz 40 + Gold 15 | 150 | 8 s | 0 | 1: −16 | Lager für bis zu 3 Bögen (`bowRack`), Bogen: Holz 50 + Gold 20 → Bogenschütze; **keine Schwerter** |
| Farm | Holz 30 + Gold 10 | 100 | 8 s | 0 | **keiner** | **keine** (nur Daten, „passive Ressourcen, optional“) |
| Kaserne | Stein 60 + Gold 30 | 200 | 12 s | 1 | **keiner** | **keine** (nur Daten, „+10 Truppen-Limit“; es gibt kein Limit) |
| Treppe hoch | Stein 100 + Gold 50 | 500 | 20 s | 1 | 1: +16, nur ab Tiefe 1 | Verbindung zur Stufe darüber |
| Treppe runter | Stein 100 + Gold 50 | 500 | 20 s | 1 | 1: +24, nur wenn es eine tiefere Stufe gibt | Verbindung zur Stufe darunter |

- **Bauplätze sind fest** (7 Stück je Hub, `hub.json` › `sites`); `game-design.md` sagt „Platzierung auf einem Raster im Hub-Bereich“. Eine freie Platzierung gibt es im Code nicht, ein Bau-Menü (Taste Y) hat keine Funktion.
- `unlockDepth` in `buildings.json` wirkt im Code nicht; maßgeblich sind `fromDepth` und `needsDeeper` je Bauplatz in `hub.json`.
- `game-design.md` nennt „HP und Kosten außer bei Turm und Treppen sind Platzhalter“ (B-015). Die Werte der Treppen (Stein 100 + Gold 50) sind **in Wald und Höhle erreichbar** (Stein), im Wald erst über Fels-Chunks (`rocks`, Gewicht 1 von 10).
- Startbesetzung des Hubs: 1 Bauer und 2 Bogenschützen (`hub.json` › `startTroops`, „vorläufiges Balancing“).

## 3. Bau-Ablauf (aus dem Code)

1. Jeder Bauplatz startet `unpaid`. Spieler zahlen **Gold** (Taste A halten, ein Spieler oder mehrere): Betrag `cost.gold`, erst dann `waitingMaterial`.
2. Sobald der **Insel/Hub-Vorrat** das Material hat (`cost` ohne Gold), wird es **automatisch abgebucht**; der Bauplatz wartet auf einen Bauer (`waitingWorker`). Material wird nicht von Spielern gezahlt.
3. Ein freier **Bauer** geht zum Bauplatz und baut `buildSeconds`; fertig = `built` mit voller HP, Ereignis `built`.
4. Zerstört ein Gegner das Gebäude (`destroySite`), wird der Bauplatz wieder `unpaid` (Gold und Material sind verloren); Ereignis `destroyed`. **Reparatur gibt es nicht** (ungeprüft: kein Code gefunden).
5. Fällt die Burg, werden **alle** Bauplätze auf `unpaid` zurückgesetzt (heute; als „Stufenverlust“ beschlossen).
6. Wartet kein Bauer oder fehlt Material, bleibt der Bauplatz stehen (kein Hinweis im Code, ungeprüft was der Client anzeigt).

## 4. Abweichungen Soll (`game-design.md`) gegen Ist

| Thema | `game-design.md` | Ist | Bewertung |
|---|---|---|---|
| Gebäudeliste | Burg, Mauer, Turm, Tor, Werkstatt, Farm, Kaserne, Treppen | Tor, Farm, Kaserne ohne Bauplatz und Wirkung | Abweichung (als „später“ beschlossen) |
| Platzierung | Raster im Hub | 7 feste Bauplätze | Abweichung |
| Material-Verwendung | Holz, Stein, Kupfer, Elite-Upgrades mit Stein/Kupfer | Kupfer ohne jede Verwendung im Code; Stein nur Treppen | Abweichung: Kupfer ist heute wertlos |
| Material-Speicher | – | kein Maximum | GDD schweigt |
| Bauen | „gebaut wird von Bauern“ | Bauer baut | nein |
| Reparatur | – | keine | GDD schweigt |
| Werkzeuge | Bogen, Schwert (Werkstatt) | nur Bogen (max. 3 im Lager) | Abweichung (Krieger später) |

## 5. Kennzahlen für Zielkorridore

Aus `ist-abgleich.md` § 6 verfügbar: Ereignisse `built` und `destroyed` (Zeitpunkt aus dem Tick), `gathered` (Ressource, Menge), `chest`, `recruited`, Stand von `Stock`, `Player.Gold`, Burg-HP. **Fehlen** (Vorschlag für B-099): Material-Ausgaben je Bau (Ereignis `spent`), Zeit zwischen „bezahlt“ und „gebaut“ (Wartezeit auf Bauer oder Material), Material-Vorrat je Tagesbeginn (aus `Stock` ableitbar), Zerstörungen je Welle (aus `destroyed` zählbar).

Vorgeschlagene Korridore je Beschlussgröße (Normal, Wald, 2 Spieler, Bot „sparsam“, 100 Seeds):

| Größe | Kennzahl | Trägt der Korridor? |
|---|---|---|
| Mauer, Turm (Kosten, Bauzeit) | Tick der ersten Mauer, des ersten Turms | ja (`built`) |
| Materialfluss Holz | Holz am Tagesbeginn je Tag | ja (aus `Stock`) |
| Stein und Kupfer | Zeitpunkt der ersten Treppe, erste Kupfer-Verwendung | Treppe ja; Kupfer erst mit Verwendung |
| Gebäude-HP | Zerstörungen je Welle | ja (`destroyed`) |
| Wartezeit Bau | Zeit „bezahlt bis gebaut“ | **nein**, Kennzahl fehlt |

## 6. Fragen für den Workshop R2.2

Je Frage mit Optionen und Empfehlung; 🧑 entscheidet einzeln.

**Materialien**

- **M1 · Welche Materialien gibt es?** (a) Holz, Stein, Kupfer wie heute, Kupfer bekommt Verwendung (Empfehlung: Schmiede/Elite, z. B. Elite-Krieger und ein Gebäude); (b) zusätzlich Eisen und Kristall für Insel 2 und 3 schon festlegen; (c) Kupfer streichen, bis es gebraucht wird.
- **M2 · Wofür braucht man Kupfer konkret?** Elite-Upgrade, ein Gebäude (Schmiede), Tor/Treppen? Heute nichts.
- **M3 · Busch:** (a) streichen, nur Dekoration, (b) neues Sammelobjekt (z. B. Beeren/Nahrung für die Farm), (c) später.
- **M4 · Lagerlimit für Material:** (a) kein Limit wie heute, (b) Limit je Insel (z. B. 500) mit Lager-Gebäude, (c) Limit je Rohstoff. Empfehlung (a) für den Start.
- **M5 · Nachwachsen:** Wald wächst nach (Bäume neu, z. B. alle n Tage) oder endlich? Beeinflusst „alles abbauen“ als Siegvariante (`stufen.md` § 3).

**Gebäude**

- **G1 · Gebäudeliste in Insel 1 und Reihenfolge:** Welche Gebäude gibt es, welche kommen schon in Stufe 0, welche später? Neue Gebäude gewünscht (z. B. Schmiede, Lager, Taverne, Heilplatz)?
- **G2 · Platzierung:** (a) feste Bauplätze wie heute, mehr Plätze je Gebäudeart, (b) freie Platzierung auf einem Raster mit Bau-Menü (Taste Y), (c) Mischform (feste Plätze, Bau-Menü wählt das Gebäude).
- **G3 · Kosten- und HP-Raster:** Verhältnis zum Gold am Morgen (heute 5 je Spieler) und zu den Materialflüssen; Vorschlag: Mauer und Turm in Tag 1 bezahlbar, Treppe erst ab Tag 3–4.
- **G4 · Wirkungen:** Tor (Durchgang für eigene Truppen: schließt es Gegner aus, öffnet es per Hand?), Farm (was bringt „passiv“: Gold, Holz, Nahrung?), Kaserne (Truppen-Limit: Zahl, Basis ohne Kaserne), Werkstatt (Schwerter für Krieger), Turm (Bogenplätze und Reichweite), Mauer (Stufen von Holz zu Stein?).
- **G5 · Reparatur und Zerstörung:** Gebäude reparieren (Bauer, Kosten) oder bei Zerstörung neu bauen wie heute? Gold und Material beim Neubau verloren oder anteilig erstattet?
- **G6 · Wartezeit sichtbar:** Wartet ein Bauplatz auf Bauer oder Material, soll das in der Welt angezeigt werden (Hinweis für CLI-Ticket)?
- **G7 · Treppen:** Kosten Stein 100 + Gold 50 gelten je Treppe, in jedem Hub zweimal; passt das zur Erreichbarkeit von Stein im Wald (Fels-Chunks selten)?
- **G8 · Einfluss der Schwierigkeitsgrade:** Bleibt die Wirtschaft in allen Graden gleich (`wirtschaft.md` § 4), auch Kosten und HP der Gebäude? Empfehlung: ja.

## 7. Gliederung für `materialien-gebaeude.md`

1. Materialien (Liste, Quellen, Verwendung, Limit) 2. Gebäude (Liste, Kosten, HP, Bauzeit, Wirkung, Freischaltung, Platzierung) 3. Bau-Ablauf, Zerstörung, Reparatur 4. Zielkorridore 5. Offen und Annahmen.
