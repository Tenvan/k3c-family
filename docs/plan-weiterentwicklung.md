# Plan: K3C nach den Grundlagen – Features, Simulator, Balancing, Grafik, Sound

- **Status:** Richtung von 🧑 gewählt (2026-10-02, Chat); Umsetzung in Tickets B-134 bis B-170 und den Sprints F1 bis RL1 (Entwürfe, Specs je Sprint von 🧑 freizugeben). Offene Entscheidungen: [`fragenkatalog.md`](fragenkatalog.md).
- **Leitlinie (von 🧑 gewählt):** früh spielbar, dann Tiefe. **Quellen für Assets (von 🧑 gewählt):** CC0 + CC-BY mit Credits.
- **Grundlage:** [`roadmap.md`](roadmap.md), [`backlog/README.md`](backlog/README.md) (≈30 offene Umsetzungs-Tickets), `docs/arbeitsweise.md`
  (ein Sprint = eine Domäne, SIM → SRV → CLI), Lückenanalyse unten.

## 1. Ausgangslage (belegt)

- **Fertig:** Insel/Stufen-Kern (SP12), Optionen/Grade/Material/Lager in Go (SP13), Raum auf Insel + Protokoll v3 (SP14), Regelwerk R1–R4 beschlossen (`docs/rules/*.md`), Go-Server, Diagnose-TUI, k3c-dev, Logging (heute ausgebaut).
- **Offen, Sim/Regeln:** Hub-Ausbau B-112, Plantage/Adern B-114, Stufenbreite B-115, Gebäude B-116, Monarch-Schlag/Pool B-118, Skills B-119, Wiederbeleben B-120, Berufe/Händler B-121, Elite/Limit B-122, Traits B-128, Gegner B-129, Bosse B-130, Events B-131, Siegvarianten B-102, Inseln B-103.
- **Offen, Protokoll/Client:** B-123 Protokoll Skills, B-124 Skill-Menü, B-125 Aktionen-Overlay, B-126 Bürger-UI, B-117/B-132 Anzeigen, B-105 Anlegen-Dialog, B-106 Kamera je Stufe, B-107 Debug-Panel.
- **Grafik:** Figuren vorhanden (`public/sprites/*`, 38 Packs, `data/sprites.json`), Pack-Grafiken für Gebäude/Ressourcen/Hintergründe liegen unter `public/grafik/*` (G1), sind aber nicht zugeordnet; Renderer zeichnet Bauplätze noch als Formen (`src/scenes/worldRenderer.ts`). Reittiere ohne Mechanik.
- **Sound:** nichts vorhanden (kein Audio-Code in `src/`), B-011 ist nur ein Satz.
- **Balancing:** Werte in `data/*.json`; B-099 (Tester) hat noch keine Zielzahlen; Spieleabend B-008 nie gespielt.
- **Betrieb:** SP11 (Pi) wartet auf 🧑; B-071 (Golden auf arm64) offen.

## 2. Prinzipien

1. **Jede Phase endet spielbar am TV** (Controller, 1–4 Spieler), nie nur „Code fertig“.
2. **Regel vor Code, Messung vor Gefühl:** Werte nur in `data/`, jede Mechanik bekommt Kennzahl und Zielkorridor, bevor sie gebalanced wird.
3. **Sim bleibt die Wahrheit:** Client zeichnet und spielt Ton nur aus Server-Ereignissen; keine Client-Würfel, deterministisch.
4. **Assets laufen als eigene Schiene**, getrieben von einer Zuordnungstabelle (Spielobjekt → Asset → Lizenz), nicht ad hoc.
5. **Kleine Sprints** (2–4 Sessions, ≤400 Zeilen je PR), Spec-Freigabe durch 🧑, manuelle Abnahmen nur durch 🧑.

## 3. Phasen und Sprints

Nummern sind Vorschläge; Tickets werden beim Bereitmachen des jeweiligen Sprints angelegt (Vorlagen Pflicht).

### Phase 0 – Fundament (vor jedem R-Code; klein, überwiegend REG/INF/SIM)
| Sprint | Domäne | Inhalt | Am Ende sichtbar |
|---|---|---|---|
| **F1** | REG 🧑 | **Zielkorridore** als Zahlentabelle (z. B. Nacht 3, Grad Normal, 2 Bots: Überleben ≥ 80 % der Seeds; Gold/Tag; Bauzeit bis Mauer), **Mindest-Schriftgrößen** je Split-Viertel (TV-Regel), **Pause-Semantik** (wer pausiert, hält der Raum an), **Feedback-Event-Liste** (Treffer, Kill, Münze, Pfeil, Schlag, Bau, Tod …) mit Größenbudget je Tick | Regeln mit Pass/Fail-Zahlen |
| **F2** | INF | **Golden-Ablauf** („Golden aktualisieren“: Task + Begründung im Commit), B-071 arm64-Golden in der CI, **Spielstand-Migrationsregel** (jede Formatänderung: Versionssprung + Fixture `saves/v{n}` + Test „alter Stand lädt“), Lint/Test gegen `range` über Maps in `engine/sim` | Rote CI bei Drift/Datenverlust |
| **F3** | SIM + SRV | **Feedback-Events** in Sim und Protokoll (neue Event-Typen, Delta-tauglich), Benchmark `island_bench_test` mit Bytes/Tick und Tick-p99 bei 4 Spielern × 3 Stufen | Events im Debug-Overlay sichtbar |
| **SP11** | SRV 🧑 | Pi einrichten + Lastmessung (läuft, wartet auf 🧑); Ergänzung: Netzlast-Ziel (KB/s je Client), **Backup `saves/` auf zweites Gerät** (SD-Karte), Reports/Clientlog rotieren und deckeln | Server dauerhaft im Heimnetz |
| **F4** | INF | Doku-Drift: `docs/rules/ist-*.md` archivieren, `CLAUDE.md`-Satz zu Entscheidung 003 aktualisieren, Landing-Kacheln an Lobby anpassen (B-079), Version in Server-Status + Landing-Fußzeile, `index.html` no-cache | Konsistente Doku, sichtbare Version |

### Phase 1 – „Spieleabend-Build“ (früh spielbar)
Ziel: Die Familie kann **eine Insel-Stufe mit Schlag, Skills und Ton** spielen und Feedback geben (B-008).

| Sprint | Domäne | Inhalt |
|---|---|---|
| **S1** | SIM | Monarch: Schlag, Fund-Pool, freie Skillung (B-118), Skills Tank/Zauberer/Heiler (B-119), Monarch-Daten im Spielstand (B-022, Migration F2) |
| **S2** | SRV | Protokoll: Schlag, Skills, Pool, gültige Aktionen je Spieler (B-123) |
| **S3** | CLI | Skill-Menü + Tasten (nach X1/B-026: Tastenbelegung am Controller), Aktionen-Overlay (B-124, B-125), **Controller-Glyphen statt Text** |
| **S4** | CLI | Kamera je Stufe (B-106), Layouts 1–4 mit Mindestschrift, Radar-Anpassung |
| **S5** | CLI | **Optionen/Pause-Szene** (Lautstärke getrennt Musik/SFX, Screenshake/Flash aus, Farbschwäche-Symbole), Speichern beim Verlassen/letzter Gerät getrennt |
| **A1** | CLI | **Audio-Basis** (siehe Sound-Schiene): Mixer, Entsperren per erster Geste (A beim Beitreten), ~12 Platzhalter-SFX auf F3-Events |
| **S6** | CLI | **Onboarding „Erste Nacht geführt“**: kontextuelle Hinweise über Objekten, Freundlich-Grad ohne Verlust |
| **P1** | REG 🧑 | **Spieleabend 1** (B-008): Protokoll nach `docs/playtests/`, kindgerechter Fragebogen, Spielmetrik-Report (Tod durch was, Nacht überlebt, Zeit bis erstem Bau) |

### Phase 2 – Tiefe: Wirtschaft, Gebäude, Bürger (R2/R3-Rest)
| Sprint | Domäne | Inhalt |
|---|---|---|
| **W1** | SIM | Hub-Ausbau 1–5, Mauer-/Turmstufen (B-112) |
| **W2** | SIM | Plantage + Adern, Stufenbreite, Eisenstollen/Kristallhöhle (B-114, B-115, B-012) |
| **W3** | SIM | Gebäude-Wirkungen: Tor, Kaserne, Taverne, Heilplatz, Schmiede, Rüstkammer, Zaubertum (B-116) |
| **W4** | SIM | Wiederbeleben (B-120), Berufe + Händler (B-121), Elite/Limit/Heilung (B-122, B-014) |
| **W5** | SRV | Protokoll für Berufe, Händler, Lager, Hub-Stufe |
| **W6** | CLI | Anzeigen: Wartezeit, Lagerstand, Hub-Stufe, Adern (B-117), Bürger-UI (B-126) |
| **B1** | REG 🧑 | Balancing-Runde Wirtschaft (Gebäudewerte B-015) gegen Zielkorridore; **Spieleabend 2** |

### Phase 3 – Gegner, Bosse, Inseln (R4)
| Sprint | Domäne | Inhalt |
|---|---|---|
| **G1b** | SIM | Traits aoe/swarm/phases, Kiting, Angriffsrate in Daten (B-128), Gegner Eisen/Kristall (B-129, B-013) |
| **G2** | SIM | Minibosse + Endboss (B-130), Siegvarianten/Niederlage (B-102), Inseln + gemeinsamer Inselwechsel (B-103) |
| **G3** | SIM | Events Vollmond/Blutmond/Händler-Überfall (B-131) |
| **G4** | SRV | Protokoll Bosse/Events/Inselwechsel |
| **G5** | CLI | Boss-Leisten, Phasen, Event-Anzeige (B-132), Anlegen-Dialog (B-105), Debug-Panel (B-107, B-098) |
| **B2** | REG 🧑 | Balancing Kampf/Bosse; **Spieleabend 3** |

### Schiene B – Balancing und Simulator (läuft quer, SIM)
| Sprint | Inhalt |
|---|---|
| **BAL1** | B-099 Kern: Szenario-Matrix (feste Seeds × 1–4 Spieler × Bot-Profil × Tiefe), Bots nur über `PlayerCommand`, Kennzahlen-Report als JSON; `sim_run`/k3c-dev-Tool dafür |
| **BAL2** | Zielkorridore aus F1 als Prüfung (Pass/Fail pro Kennzahl), CI-Task `task balance` (nicht Pflicht-Gate, aber Bericht), Regressions-Vergleich „Wertänderung → welche Ziele kippen“ |
| **BAL3** | Bot-Profile erweitern (Wirtschaft zuerst, Mauern zuerst, Koop 2/4 Spieler, Kind-Bot mit Fehlern), Sensitivitäts-Läufe, Schwierigkeitsgrad-Kurven (Grade aus SP13) |
| **BAL4** | Spielmetrik aus echten Abenden (P1/B1/B2) mit Simulatorwerten abgleichen → Werte nachziehen |

Regel: Jeder Sprint der Phasen 2–3, der Werte ändert, nennt die betroffenen Kennzahlen im Ergebnis; ab BAL2 läuft der Bericht in der Review-Session.

### Schiene G – Grafik
1. **GR1 Zuordnungstabelle (PLAT/REG, Doku + Referenzseite):** `docs/assets/zuordnung.md` (oder in `data/sprites.json`/`public/grafik/index.json` ergänzen): je Spielobjekt (Gebäude je Hub-Stufe, Mauer/Turm je Materialstufe, Ressourcen, Truhen, Adern, Plantage, Portale, Gegner je Trait/Boss, Truppen je Beruf, Reittiere, UI-Icons, Skill-Icons, Hintergründe je Biom/Tiefe) → Asset-Pack, Frame, Lizenz. Lücken sind **fett** markiert („keine Grafik“, „Stil passt nicht“).
2. **GR2 Suche (Recherche, Agent + 🧑 wählt):** Lücken gezielt suchen bei OpenGameArt, itch.io (CC0/CC-BY), Kenney (CC0): Kandidaten je Lücke mit Vorschau, Lizenz, Stilpassung; Auswahl durch 🧑; Credits sofort in `public/*/CREDITS.md`. Stilregel: ein Grundstil (Pixel-Art, einheitliche Palette/Skalierung), Nachbearbeitung nur Skalieren/Palette.
3. **GR3 Renderer-Anbindung (CLI):** Bauplätze/Gebäude als Sprites statt Formen, Hub-Stufen sichtbar (1–5), Mauer-/Turm-Materialstufen, Ressourcen/Adern/Plantage, Parallax je Biom; Reittiere (Sattelpunkte vorhanden) sobald Mechanik da ist.
4. **GR4 Atlas + Lade-Szene (CLI/INF):** Atlas-Build (Task), Lade-Szene mit Fortschritt (B-029), Budget „Kaltstart < X s auf Xbox“ (Messung über Gamepad-Testseite, X1).
5. **GR5 Juice (CLI, hängt von F3):** Treffer-Blitz, Screenshake (abschaltbar), Münz-Partikel, Todes-/Bau-Effekte, optional Controller-Vibration.
6. **GR6 Credits-/Über-Seite** generiert aus den CREDITS-Dateien; Test „jedes Asset-Verzeichnis hat Credit-Eintrag“.

### Schiene S – Sound
1. **SO1 Architektur (A1 oben):** `src/audio/` (Mixer Musik/SFX/Ambient, Bus-Lautstärke pro Gerät im `localStorage`), Entsperren per Geste, Format ogg + m4a-Fallback (Edge auf Xbox: **Eintrag auf Gamepad-Testseite** für Autoplay und Dekodierung, X1), Sound-Atlas (Sprite-Sheet) gegen viele Requests, Positions-Dämpfung im Split-Screen (jeder hört seinen Bereich, Warnungen global).
2. **SO2 SFX-Katalog:** Tabelle Ereignis → Sound → Quelle → Lizenz (analog GR1): Münze aufheben/geben, Schlag, Pfeil, Treffer, Gegner-Tod, Bauen/Fertig, Hub-Ausbau, Nacht naht, Portal öffnet, Tod/Wiederbeleben, Skill je Klasse, Boss-Auftritt, UI-Klicks. Quellen: Kenney (CC0), OpenGameArt, freesound (CC0/CC-BY), selbst erzeugte Retro-SFX per Web Audio als Lückenfüller.
3. **SO3 Musik:** Zustände Tag, Abend, Nacht, Kampf, Höhle/Tiefe je Stufe, Boss, Niederlage/Sieg, Lobby; Übergänge per Crossfade; Lautstärke-Ducking bei Warnungen. Auswahl durch 🧑 (Hörproben auf einer Testseite `soundtest.html`, mit `installPageChrome()`, Eintrag in `src/landing/pages.ts`).
4. **SO4 Politur:** Mix-Pass am TV (Lautheit angleichen), Stummschalt-Taste, Credits (CC-BY) in der Über-Seite.

### Schiene R – Betrieb und Release (parallel, klein)
- SP11 (Pi) + Backup + Rotation + Netz-Ziele (siehe Phase 0).
- **Release-Checkliste** (`docs/arbeitsweise.md` ergänzen): Golden grün (amd64/arm64), `task check:all`, Save-Migration, Dev-Reste (B-098/B-107) aus, Pi-Image, Version im Status, Credits vollständig, Tag `v0.<n>.0` nach jeder Phase.
- Optional spät: itch.io (B-023, dann Lokalisierung/Englisch entscheiden), Xbox-Test X1 (B-006, B-026) – **früh nötig** für Skill-Tasten und Audio-Autoplay → X1 vor S3/A1 einschieben.

## 4. Lücken, an die vermutlich nicht gedacht war (Analyse, nach Risiko)

| # | Thema | Folge im Plan |
|---|---|---|
| 1 | Es gibt keine Feedback-Events (Treffer, Kill, Münze …) – ohne sie weder Sound noch Juice, ohne dass der Client rechnet | F1 + F3 |
| 2 | Pause/Optionen sind nicht definiert (Aktion `pause` existiert, keine Szene; Kollision mit View+Menu) | F1, S5 |
| 3 | Audio: Autoplay-Policy auf Edge/Xbox, Mixer, Split-Screen-Hören, Formate, Ladegröße | SO1, X1 |
| 4 | Snapshot-Größe und Pi-3-Leistung bei 4 Spielern × n Stufen + Events ungemessen | F3, SP11 |
| 5 | Lesbarkeit am TV im Viertel-Split (Texte 20 px → effektiv ~10 px) | F1, S4 |
| 6 | Onboarding für Kinder fehlt (Kern-Loop nicht selbsterklärend) | S6 |
| 7 | Spielstand-Migration (v1/v2/v3, neue Felder durch R2–R4) ohne Regel/Fixtures | F2 |
| 8 | Wie mit geänderten Golden-Hashes umgehen; arm64-Golden (Pi!) | F2 |
| 9 | Balancing-Ziele sind keine Zahlen → keine Abnahme von R2–R4 | F1, BAL2 |
| 10 | Dokumentations-Drift (`ist-*.md`, CLAUDE.md-Satz zu 003) | F4 |
| 11 | Asset-Ladezeit: ~40 Einzel-PNG-Ordner, kein Atlas, keine Lade-Szene | GR4 |
| 12 | Barrierefreiheit: Farbcodierung, Screenshake/Flackern, getrennte Lautstärken | S5, GR5 |
| 13 | WLAN-Latenz/Eingabe→Bild-Ziel, Verhalten bei langem Verbindungsverlust (Monarch schutzlos?) | Regel in F1, Messung in P1 |
| 14 | Heimnetz-Sicherheit: `/api/save/restore`, `/api/report`, `/api/clientlog` ohne Auth, `reports/` ohne Rotation (SD-Karte füllt sich) | SP11-Ergänzung |
| 15 | Backup liegt auf demselben Pi-Datenträger | SP11-Ergänzung |
| 16 | Version/Cache: Pi-Image und gecachter Xbox-Client können auseinanderlaufen | F4, Release-Liste |
| 17 | Credits im Produkt nötig (CC-BY) | GR6, SO4 |
| 18 | Determinismus: Map-Iteration in Sim, Float amd64/arm64, langsamer Tick auf Pi (verlangsamen statt überspringen) | F2, F3 |
| 19 | Playtest-Auswertung/Telemetrie fehlt (nur Logs) | P1, BAL4 |
| 20 | Nur Deutsch? Texte hart kodiert – bewusst festhalten | Entscheidung in F1 |
| 21 | Speicherpunkte: nur bei Tagesanbruch gespeichert, Kinder brechen mitten in der Nacht ab | S5 |
| 22 | Dev-Reste (B-098, B-080, B-107) vor Release | Release-Checkliste |
| 23 | Zusätzlich: Playtest-Gesundheit der Entwicklungsschleife – k3c-dev-Logging jetzt vorhanden; ein **Replay-/Repro-Format** (Seed + Eingaben als Datei) für Bug- und Balancing-Reproduktion fehlt | BAL1 (Bots liefern Eingabe-Aufzeichnung), Wiedergabe-Tool in k3c-dev |

## 5. Entscheidungen, die 🧑 treffen muss (blockieren je einen Sprint)

1. Pause im gemeinsamen Raum: lokal oder raumweit, wer darf? (F1/S5)
2. Zielkorridore: konkrete Zahlen (F1; Vorschlag liefert Agent aus `docs/rules/*.md`)
3. Mindest-Schriftgröße je Split-Viertel (F1)
4. Skill-Tasten am Controller (B-026, X1) – vor S3
5. Registry für Pi-Image und Pi-Modell (SP11, offene Fragen der Spec)
6. „Nur Deutsch“ dauerhaft? (F1)
7. Grafik-/Sound-Auswahl je Lücke (GR2/SO3): 🧑 wählt aus Vorschlägen

## 6. Reihenfolge und Abhängigkeiten

```
F1 → F2 → F3 ──────────────┐
SP11 (🧑, parallel) ───────┤
F4 ────────────────────────┤
X1 (🧑, Xbox: Tasten, Audio-Autoplay) ─→ S1 → S2 → S3 → S4 → S5 → A1 → S6 → P1 (Spieleabend 1)
GR1 (früh), GR2 (parallel) ──→ GR3 ab Phase 2 (W1 braucht Hub-Grafiken)
BAL1 ab Phase 1 (parallel) → BAL2 vor B1
P1 → W1…W6 → B1 (Spieleabend 2) → G1b…G5 → B2 (Spieleabend 3) → Release v0.x
```
Ein aktiver Sprint je Domäne (ab F0, B-174; vorher einer insgesamt); Schienen GR/SO/BAL laufen als einschiebbare Sprints (`Einschiebbar: ja`) oder zwischen Sprints, nie als Nebenarbeit. Reihenfolge, Wellen und Zuordnung Mensch/autonom: § 11.

## 7. Akzeptanzkriterien dieses Plans (testbar)

- **AC-1** Nach F1–F4 existiert je Zielkorridor, Schrift- und Pause-Regel ein Eintrag in `docs/rules/` mit Zahl; `task check` grün, Golden-Ablauf und Migrationsregel stehen in `docs/arbeitsweise.md`.
- **AC-2** P1 findet statt, bevor Phase 2 beginnt; Spielmetrik-Report und Protokoll liegen in `docs/playtests/`.
- **AC-3** Nach Phase 1 spielt man am TV eine Stufe mit Schlag, einem Skill je Klasse, Ton (SFX auf ≥12 Ereignissen) und Optionen/Pause; Tick-p99 ≤ 10 ms im Ziel aus B-042 mit 2 Räumen × 3 Spielern.
- **AC-4** GR1-Tabelle hat für jedes Spielobjekt aus `data/*.json` (Gebäude, Gegner, Truppen, Ressourcen) einen Eintrag „zugeordnet“ oder „Lücke“; keine Lücke ohne Ticket; jedes verwendete Asset hat Credit-Eintrag (Test).
- **AC-5** Musik für Tag, Nacht, Kampf, Boss und Lobby, Lautstärke getrennt einstellbar, Autoplay auf Xbox-Edge geprüft (Bericht in `reports/`).
- **AC-6** `task balance` (ab BAL2) liefert pro Kennzahl Pass/Fail für 100 feste Seeds; jede Wertänderung in `data/` nennt im Commit die betroffenen Kennzahlen.
- **AC-7** Spielstände älterer Version laden in CI aus Fixtures; eine unbekannt neuere Version ergibt eine klare Meldung, keinen Datenverlust.
- **AC-8** Nach Phase 3: Insel mit Endboss durchspielbar, Siegvariante wählbar, alle Features aus R2–R4 spielbar; Release-Checkliste abgehakt, Tag gesetzt.

## 8. Risiken und Gegenmittel

| Risiko | Gegenmittel |
|---|---|
| Phase 1 wird zu groß (Skills + Ton + Onboarding) | Spieleabend-Build ist **Minimum**: Schlag + 1 Skill je Klasse, SFX-Platzhalter; Rest in Phase 2 |
| Pi 3 schafft 30 Hz mit Events nicht | F3 misst früh; Gegenmittel: Event-Budget, Delta-Kompression, Tick-Verlangsamung statt Überspringen |
| Stilbruch durch gemischte Asset-Packs | GR1 mit Stil-Spalte, einheitliche Skalierung/Palette, nur Packs mit 16/32 px Grundraster |
| CC-BY-Pflichten vergessen | GR6/SO4 + Test |
| Balancing wird Geschmacksfrage | Zahlen aus F1, Simulator + Spieleabend (BAL4) abgleichen |
| Regeländerung bricht Golden/Saves | F2-Ablauf, Migration-Fixtures |
| Xbox-Edge-Audio anders als erwartet | X1-Test vor A1, Fallback-Format |
| Scope-Creep durch „Dinge, an die man denkt“ | neue Ideen sofort als Ticket (Arbeitsweise), Plan nur per Sprint-Planung ändern |

## 9. Nächste konkrete Schritte (nach Freigabe)

1. Tickets für F1–F4 und GR1/GR2/SO1 aus den Vorlagen anlegen, Sprint-READMEs `Spec: Entwurf`, Freigabe durch 🧑 je Sprint.
2. F1 als nächsten Sprint bereit machen (Workshop mit 🧑 für Zahlen, Schriftgröße, Pause).
3. X1 (Xbox-Test: Tasten, Audio) terminieren; SP11 parallel durch 🧑.
4. (erledigt) Roadmap und Fahrplan sind angepasst; Tickets und Sprint-Entwürfe liegen in `docs/backlog/` und `docs/sprints/geplant/`.

## 10. Hinweise

- Dieser Plan ändert nichts am Code; Umsetzung erst nach ausdrücklicher Freigabe.
- Nicht verifiziert (Annahmen): Edge/Xbox-Autoplay und ogg-Dekodierung, Pi-3-Leistung bei Events, Eignung der vorhandenen Packs in `public/grafik` für Hub-Stufen 1–5.

## 11. Zwei Spuren: Mensch (hier) und autonom (zweiter Account)

Stand 2026-10-03. Ziel: Auf dem zweiten Account laufen möglichst viele autonome Sessions parallel, hier laufen nur die Schritte, die 🧑 braucht (Freigaben, Workshops, Abnahmen, Tests am Gerät). Grundlage ist die Domänen-Regel der Arbeitsweise: Jeder Sprint ändert nur Dateien seiner Domäne, Sprints verschiedener Domänen können deshalb gleichzeitig laufen. Dafür braucht es die Regeländerung **F0 (B-174)**: je Domäne ein aktiver Sprint, Sessions per Branch beanspruchen. Ohne F0 darf neben SP11 (SRV, wartet auf den Pi) kein weiterer nicht einschiebbarer Sprint aktiv sein.

### 11.1 Bahnen (je Domäne eine Reihenfolge, nacheinander)

| Domäne | Reihenfolge der Sprints | Sperre durch 🧑 |
|---|---|---|
| INF | F0 → F2 → F5 → GR4 → RL1 | RL1.1, RL1.3 (Checkliste abnehmen) |
| REG | F1.1–F1.3 → **F1.4** → F1.5 … später BAL4 → BR1 → BR2 | F1.4, BAL4.2, BR1.2, BR2.2, P1 |
| SIM | H1 → F3 → BAL1 → S1 → BAL2 → W1 → W2 → W3 → W4 → K1 → K2 → K3 → BAL3 | BAL3.1 (Bot-Profile) |
| SRV | DBG1 → F4 → S2 → LT1 → W5 → K4 | LT1.3 (Messlauf am Pi) |
| CLI | DBG2 → S5 → S4 → S3 → S7 → S6 → SO1 → W6 → K5 | S3.4, S4.3, S5.4, S6.4, S7.3, W6.4, K5.4 (Abnahmen am Gerät) |
| PLAT | X1.1 (autonom, Audio-Abschnitt) → X1.2 (🧑 Xbox-Test) → X1.3 → GR6 → SO3 | X1.2 (Xbox-Test) |
| Asset-Schiene (einschiebbar, CLI/INF) | GR1 → GR2 → GR3 → GR5, SO2, SO4 | GR1.1, GR2.2, GR4.3, SO2.1, SO4.1 (Stil und Auswahl) |

Die Asset-Schiene zählt nach Regel nicht als aktiver Sprint, teilt sich aber Dateien mit der CLI-Bahn (`src/scenes/worldRenderer.ts`): GR3 läuft nie gleichzeitig mit S4, S7, W6 oder K5.

### 11.2 Wellen (Abhängigkeiten berücksichtigt)

| Welle | Voraussetzung | Spur A (autonom, 2. Account, parallel je Bahn) | Spur M (🧑 hier) |
|---|---|---|---|
| **W0** jetzt | – | **F0.1 → F0.2** (INF) ‖ **X1.1** (PLAT, Audio-Abschnitt der Testseite, nach Freigabe X1). Währenddessen „Bereit machen“ (11.4) für F3, F4, S4, S5, S1. | Freigabe Spec **X1** (neue Spec mit Audio-Test, damit X1.1 starten kann); **SP11.2** Pi einrichten (läuft); **X1.2** Xbox-Test (Gamepad und Audio, B-166), sobald der autonome **X1.1** den Audio-Abschnitt gebaut hat |
| **W1** | F0 erledigt | **F1.1 → F1.2 → F1.3** (REG) ‖ **F2.1 → F2.4** (INF: Golden-Ablauf, Migration, Determinismus) ‖ SP11.1 liegt schon fertig | **F1.4** Workshop (Zielkorridore, Reittier, Bedienungszahlen); Freigabe-Paket 1: F2, F5, F3; **LT1.3** Messlauf am Pi (nach dem Lasttest-Werkzeug) |
| **W2** | F1.4, F2 erledigt | **F1.5** (REG) ‖ **F5.1 → F5.4** (INF) ‖ **F3.1 → F3.3** (SIM, Feedback-Events) ‖ **S5.1 → S5.3, S5.5** (CLI) ‖ **SP11.4** Review (SRV, nach SP11.3) | Freigabe-Paket 2: S5, S4, F4, S1; **S5.4** Abnahme; **X1.3** Auswertung beauftragen |
| **W3** | F3 erledigt | **BAL1** (SIM) ‖ **F4.1 → F4.4** (SRV, nach SP11 und F3) ‖ **S4.1, S4.2, S4.4** (CLI) ‖ **GR6** (PLAT, nach X1) ‖ **GR1.2 → GR1.4** (Asset) | **S4.3** Abnahme; **GR1.1** Stil-Zuordnung (Q13 ist entschieden: nur bestätigen); Freigabe-Paket 3: S2, S3, S7, GR1, GR2 |
| **W4** | F4, BAL1 erledigt | **S1.1 → S1.5** (SIM) ‖ **GR2.1, GR2.3, GR2.4** (Asset, nach GR2.2) ‖ **GR4** (INF) | **GR2.2** Auswahl der Grafik-Kandidaten (Referenzseite) |
| **W5** | S1 erledigt | **S2.1 → S2.4** (SRV) ‖ **S7.1, S7.2, S7.4** (CLI) ‖ **BAL2** (SIM, braucht F1.4) ‖ **SO1.1 → SO1.4** (Asset/CLI, wenn S7 nicht läuft) | **S7.3** Abnahme; **B-166**-Ergebnis in SO1 übernehmen |
| **W6** | S2, S5 erledigt | **S3.1 → S3.3, S3.5** (CLI) ‖ **W1** (SIM) ‖ **SO3** (PLAT) ‖ **GR3** (Asset, nur wenn S3 es nicht stört) | **S3.4** Abnahme am Gerät (Tasten aus Q06 bestätigen) |
| **W7** | S3, S5 erledigt | **S6** (CLI, Hinweise, Glyphen) ‖ **W2** (SIM) ‖ **SO2, SO4** (Asset, nach 🧑-Auswahl) ‖ **GR5** | **S6.4** Abnahme; **SO2.1**, **SO4.1** Hörproben und Auswahl |
| **W8** | S1–S7, SO1 erledigt | **RL1.2** Vorbereitung ‖ Doku für den Spieleabend | **P1.1, P1.2 Spieleabend 1**, Fragebogen auswerten |
| **W9** Phase 2 | P1 (nur für Werte) | SIM: **W2 → W3 → W4**, SRV **W5**, CLI **W6**, REG **BAL4** | **W6.4** Abnahme, **BR1.2** Balancing + Spieleabend 2 |
| **W10** Phase 3 | W4 erledigt | SIM **K1 → K2 → K3**, SRV **K4**, CLI **K5**, SIM **BAL3** | **K5.4** Abnahme, **BR2.2** Balancing + Spieleabend 3, **RL1.1, RL1.3** |

Die Wellen sind keine Termine, sondern Reihenfolgen: Eine Bahn rückt vor, sobald ihre Voraussetzung erledigt ist; die Spur M gibt vorher frei, was die Spur A als Nächstes braucht.

### 11.3 Was 🧑 hier priorisiert (Spur M, in dieser Reihenfolge)

1. **Freigabe F0** (ein Satz „F0 freigegeben“): entsperrt alle parallelen Bahnen.
2. **SP11.2 und X1.1** (Hardware, dauern; laufen im Hintergrund von allem anderen).
3. **F1.4 Workshop** (45 Minuten, sobald F1.1–F1.3 durch sind): entsperrt S1, S4, S5, BAL2.
4. **Freigabe-Pakete** (Spec lesen und freigeben; jedes Paket entsperrt die nächste Welle): Paket 1 F2, F5, F3 · Paket 2 S5, S4, F4, S1 · Paket 3 S2, S3, S7, GR1, GR2, SO1 · Paket 4 S6, SO2–SO4, W1–W6, BAL2, BAL3.
5. **Abnahmen am Gerät** (kurz, je Sprint): S5.4, S4.3, S7.3, S3.4, S6.4.
6. **Auswahl-Termine** Grafik und Sound (GR2.2, SO2.1, SO4.1), **Spieleabende** P1, BR1, BR2.

### 11.4 Startanweisungen für den zweiten Account (Spur A)

Es gibt **zwei getrennte Aufträge**. Jeder Lauf bekommt genau einen davon; kopiert wird nur der Inhalt des jeweiligen Codeblocks.

**Auftrag 1: Eine Session ausführen** (Standard, so oft wiederholen, wie Sessions bereitstehen):

```text
Lies CLAUDE.md und docs/arbeitsweise.md. Lies docs/sprints/README.md (Fahrplan) und alle docs/sprints/aktiv/*/README.md.
1. Suche in den aktiven Sprints die erste Session mit Status offen, Agent autonom und erledigten Abhängigkeiten, deren Branch (Feld Branch der Session-Datei) noch nicht auf origin existiert (git ls-remote --heads origin <Branch>).
2. Gibt es keine: Nimm aus docs/sprints/geplant/ den ersten Sprint in der Reihenfolge des Fahrplans mit Spec freigegeben, Reife bereit, erledigten Voraussetzungen (Plan, Abschnitt 11.2) und einer Domäne ohne aktiven Sprint. Aktiviere ihn nach docs/arbeitsweise.md (git mv nach docs/sprints/aktiv/, Status aktiv, Fahrplan anpassen) und nimm dessen erste autonome Session.
3. Gibt es auch das nicht: nichts tun und melden, welche Freigabe, welches Bereit-machen oder welche Mensch-Session fehlt.
4. Führe genau die gefundene Session nach docs/arbeitsweise.md, Abschnitt Autonomer Ablauf, aus. Vor dem PR: git fetch und git rebase origin/main. PR öffnen, nicht mergen.
```

**Auftrag 2: Einen Sprint bereit machen** (nur wenn ein Sprint `Reife: Entwurf` hat und als Nächstes gebraucht wird; `<ID>` durch die Sprint-Nummer ersetzen, z. B. `F4`; Reihenfolge der Sprints: F4, S5, S4, S1, S2, BAL1, GR1, W1):

```text
Lies CLAUDE.md und docs/arbeitsweise.md. Mache den Sprint <ID> nach docs/arbeitsweise.md, Abschnitt Sprint-Lebenslauf, Schritt 2 bereit.
Der Sprint liegt unter docs/sprints/geplant/<ID>-*/. Offene Fragen sind in docs/fragenkatalog.md, Abschnitt Beschlüsse, beantwortet; erfinde keine weiteren Entscheidungen.
Schreibe jede Session als Datei nach docs/vorlagen/session.md (Stilvorbild: docs/sprints/geplant/F2-golden-migration-determinismus/), ersetze in der Sprint-README den Abschnitt Sessions durch die Tabelle, setze Reife bereit.
Spec und Freigabe nicht ändern. Prüfe mit: npx vitest run tests/planning.test.ts. PR öffnen, nicht mergen.
```

Auftrag 2 braucht keinen aktiven Sprint und keine Freigabe; Auftrag 1 aktiviert nur Sprints, deren Spec 🧑 freigegeben hat und die `Reife: bereit` haben.

### 11.5 Regeln gegen Reibung zwischen den Accounts

- **Zuständigkeit:** Spur M ändert keine Dateien eines Sprints, der gerade in Spur A läuft, außer Ergebnis-Einträge der 🧑-Sessions.
- **Merge-Reihenfolge:** 🧑 mergt PRs der Spur A in der Reihenfolge der Bahn (Abhängigkeit), nicht nach Eingang.
- **Gemeinsame Dateien** (`docs/backlog/README.md`, `docs/sprints/README.md`, `docs/roadmap.md`): Eine Session ändert sie erst am Ende, vor dem PR `git rebase origin/main`; Konflikte löst der Agent, der zuletzt rebased.
- **Golden-Daten und Spielstand-Fixtures** ändert nur der Sprint, der gerade in der Bahn SIM/INF läuft (kein zweiter parallel).
- **Review-Sessions** laufen nie im selben Lauf wie die Umsetzung und nie vom selben Agenten.
- **Engpass CLI:** Die Bahn CLI hat die meisten Sprints (S5, S4, S3, S7, S6, SO1, W6, K5); Asset-Sprints (GR3, GR5, SO2, SO4) schieben sich nur dazwischen, wenn die Datei-Überschneidung (Renderer) es erlaubt.
