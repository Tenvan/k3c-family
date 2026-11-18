# RSU · Rust-Umstellung: Spiel und Simulation komplett in Rust

- **Status:** aktiv
- **Rang:** 11
- **Ziel-Tickets:** B-1014

## Ziel

Das Spiel läuft ruckelfrei und bricht auf schwächeren PCs weniger ein (Anlass: B-943, Ruckler trotz stabiler 60 FPS). Weg laut Auftrag 🧑: Game-Client und später die Simulation als ein Rust-Programm (Workspace `rust/`, EXE `k3c-game`); Sim- und Balancing-Läufe kommen aus derselben EXE, headless oder mit Fenster (B-1017). Go bleibt für Workbench und Werkzeuge. Erledigt, wenn 🧑 das Rust-Spiel am PC abgenommen hat (G4) und die Go-Engine entfernt ist (RU41).

**Stand 2026-11-16: G0 entschieden (🧑): „Wir steigen definitiv auf Rust um.“** Beleg RU0.2 › Ergebnis: Ebitengine ruckelt schon ohne Spielcode, mit offizieller API nur noch leicht, fällt ab etwa 20 000 Sprites je Bild ab; Rust + wgpu bleibt flüssig bis 160 000. G1 (Spike) entfällt; der Stack (winit 0.30 + wgpu, Vulkan/DX12) ist damit gemessen.

**Erste Rust-Welle (R1), Ziel: so schnell wie möglich ein lauffähiger Client nur mit Grafik und einfachem Driver:**

| Schritt | Sessions | parallel |
|---|---|---|
| 1 | RU1.1 Workspace und Leitfaden | – |
| 2 | RU1.2 Tasks, RU2.1 Plattform-Kern, RU2.2 Grafik laden | ja (getrennte Bereiche) |
| 3 | RU1.3 CI, RU2.3 Demo-Szene mit Driver | ja |
| 4 | RU1.4 Review, RU2.4 Messung und Abnahme 🧑 (PC und Notebook) | ja |

Danach geht es mit RU3, RU4, RU5, RU7 weiter (Plan in den Notizen; RU4 und RU7 übernehmen Schleife, Canvas und Grafik-Laden aus RU2 statt sie neu zu bauen).

## Sprints

In Abarbeitungs-Reihenfolge; der erste nicht erledigte Sprint ist der nächste. Jeder Sprint nennt das Projekt im Feld `Projekt`.

| Sprint | Thema | Status |
|---|---|---|
| RU0 | Ruckler im Go-Spiel messen: GC, Bildtakt, Laden (Gate G0) | erledigt |
| RU1 | Rust-Workspace, Toolchain, Taskfile und CI | erledigt |
| RU2 | Spike: schlanker Rust-Client gegen die Ebitengine-Messung (Gate G1) | erledigt |
| RU3 | rng, Go-gleiche Zahlen und Daten-Schemas in Rust | erledigt |
| RU4 | Plattform-Schicht: Schleife mit und ohne Fenster, Eingabe, Ton, Text | erledigt |
| RU5 | Rig in Rust: Rechnung und Zeichnen der Rig-Figuren | erledigt |
| RU7 | Grafik laden und Ansicht: Papierschnitt-Index, Sheets, Kamera | erledigt |
| RU8 | Sim-Zustand in Rust: Zustandsmodell, Typen, Welt-Grundgerüst | erledigt |
| RU13 | Welt und Akteure zeichnen | erledigt |
| RU38 | Zustands-Aufzeichnung aus Go und Wiedergabe im Rust-Client | erledigt |
| RU16 | Gebäude und Objekte zeichnen | erledigt |
| RU17 | Effekte und Ton im Spiel | erledigt |
| RU20 | HUD Teil 1: Anzeigen je Spielerhälfte | erledigt |
| RU39 | Ruckler-Messung: Rust-Client in Wiedergabe gegen das Go-Spiel (Gate G2) | aktiv |
| RU6 | Level-Generator in Rust, Level-Golden bitgleich | geplant |
| RU9 | Menüs Teil 1: Hauptmenü, Lobby, Einstellungen | geplant |
| RU11 | Sim: Einheiten und Gegner | geplant |
| RU23 | HUD Teil 2: Overlays, Skills, Geschichte, Seekarte im Spiel | geplant |
| RU10 | Sim: Insel mit Stufen, Takt und Material-Vorrat | geplant |
| RU12 | Menüs Teil 2: Spielstände, Weltwahl, Pause, Test-Menüs | geplant |
| RU15 | Sim: Kampf, Bosse, Krone, Portale | geplant |
| RU14 | Sim: Welt-Schritt, Kampagne, Schiff und Vorschau | geplant |
| RU26 | Spielschleife Teil 1: Zustände, Split-Screen, Zeichnen | geplant |
| RU18 | Sim: Wirtschaft, Bau, Hub, Monarch, Skills | geplant |
| RU19 | Sim: Events, Geschichte, Dev-Eingriffe | geplant |
| RU21 | Spielstände und Raum Teil 1 | geplant |
| RU29 | Testing-, Debug- und Cheat-Menüs im Spiel | geplant |
| RU22 | Sim-Abgleich: alle Sim- und Kampagnen-Golden bitgleich (Gate G3) | geplant |
| RU24 | Raum Teil 2 und Spielstand-Format | geplant |
| RU27 | Lokales Spiel: Sitzung, Kampagne, Spielstände, Einstellungen | geplant |
| RU30 | Spielschleife Teil 2: echtes Spiel, Aufnahme-Modus, Start | geplant |
| RU32 | Ende-zu-Ende-Test und Bildvergleich gegen das Go-Spiel | geplant |
| RU35 | Build, Paket und CI auf das Rust-Spiel umstellen | geplant |
| RU36 | Abnahme am PC: Rust-Spiel gegen Go-Spiel (Gate G4) | geplant |
| RU33 | Läufe im Game-Client: Sim, Level, Welten und Hub, headless oder mit Fenster per Parameter | geplant |
| RU37 | Go-Spiel entfernen, Doku und Regeln nachziehen | geplant |
| RU34 | Workbench: MCP-Tools für Sim, Level, Welten und Hub rufen die Spiel-EXE | geplant |
| RU25 | Balancing Teil 1: Bots | geplant |
| RU28 | Balancing Teil 2: Messungen und Auswertung | geplant |
| RU31 | Balancing Teil 3: Einsichten, Berichte, Tuning | geplant |
| RU40 | Balancing-Läufe im Game-Client, sim_test und Balancing der Workbench über die Spiel-EXE | geplant |
| RU41 | Go-Engine entfernen, Entscheidung 011 abschließen | geplant |

## Nicht-Ziele

Workbench, `tools/assetkit`, `tools/paper` und die übrigen Werkzeuge nach Rust (bleiben Go und TypeScript). Neue Spielregeln (KSY, WRT, KMP). Balancing-Läufe selbst (BAL; gesperrt seit 2026-11-11). Online-Spiel. Ein ausgeliefertes Spiel mit Rust-Client und Go-Sim über eine Brücke (B-1014 › Notizen).

## Notizen

### Konzept (2026-11-16)

- **Anlass (🧑):** Im Go-Spiel bleiben Ruckler trotz Beseitigungsversuchen; vermutet sind nicht steuerbare GC-Läufe. Dazu soll das Spiel auf schwächeren PCs weniger einbrechen. Es geht zuerst um die Leistung des Clients; Sim und Balancing rücken so weit nach hinten wie möglich und werden nur mitgedacht.
- **Analyse und Empfehlung:** B-1014. Die GC-Vermutung ist ungeprüft; B-943 beschreibt Ruckler bei stabilen 60 FPS auch auf schnellen Rechnern, was eher auf den Bildtakt zeigt (Sim 30 Hz, Bild 60 Hz, Zwischenbilder aus der Wanduhr). Ist der Bildtakt die Ursache, behebt Rust nichts von selbst. Deshalb zuerst RU0.
- **Reihenfolge „Client zuerst“:** Der Rust-Client zeichnet bis G2 aus Aufzeichnungen des Go-Spiels (RU38), ohne portierte Sim. So ist nach 69 Sessions belegt, ob Rust die Ruckler beseitigt, bevor die übrigen 139 Sessions anfallen.
- **Wellen- und Kostenplan:** B-1015. 42 Sprints in 26 Wellen, rund 208 Sessions (Spanne ±20 %, ungeprüft).
- **Gates:** G0 nach 3 Sessions (Ursache der Ruckler), G1 nach 11 (Spike), G2 nach 69 (Client ruckelfrei bei echter Last), G3 nach 143 (Sim-Golden mit gleichem Verhalten, Entscheidung 012; Level-Golden bleiben bitgleich), G4 nach 171 (Abnahme am PC). An jedem Gate entscheidet 🧑.
- **Port-Orakel (🧑 2026-11-18, Entscheidung 012):** Bitgleichheit zu Go gilt nur für rng, Ganzzahlen und Level-Aufbau (RU3, RU6). Sim-Fließkomma (RU22): gleiches Verhalten, also dieselben Ereignisse in derselben Reihenfolge, dieselben Ganzzahl-Werte und Kampagnen-Ergebnisse, Fließkommawerte mit Toleranz (1e-6 relativ, absolut 1e-9 nahe 0). Determinismus innerhalb von Rust bleibt Pflicht; nach RU41 schreibt Rust eigene Golden.
- **Rückfall:** Bis G4 bleibt das Go-Spiel unverändert spielbar, Rust liegt nur unter `rust/`.
- **Läufe im Game-Client (🧑 2026-11-16):** Sim und Balancing laufen mit der Spiel-EXE, headless oder mit Fenster per Parameter; die MCP-Tools der Workbench rufen die EXE (B-1017). Umsetzung nach G4: RU33, RU34; Balancing zuletzt: RU25, RU28, RU31, RU40.
- **Sonnet-tauglich (🧑 2026-11-16):** Port-Leitfaden B-1018, Tabelle „Session → Go-Dateien“ je Port-Sprint, Vorentscheidungen in RU4, RU8, RU22, RU33, RU34.
- **Entscheidung 011** (löst 008 ab) ist nur als Vorschlag formuliert (B-1014 › Notizen) und nicht eingetragen.
- **Rang 12** hat `plan_create` vergeben; den Rang setzt 🧑.

### Wellen

| Welle | Sprints | Inhalt | Sessions | Gate |
|---|---|---|---:|---|
| 1 | RU0 | Ruckler im Go-Spiel messen | 3 | **G0** (🧑): Ursache der Ruckler gemessen |
| 2 | RU1, RU2 | Workspace, Leitfaden; Spike schlanker Client | 8 | **G1** (🧑): Schwellen aus B-1016 |
| 3 | RU3, RU4, RU5 | rng, Zahlen, Daten; Plattform-Schicht; Rig | 17 | – |
| 4 | RU7, RU8 | Grafik laden, Ansicht; Zustands-Typen | 12 | – |
| 5 | RU13, RU38 | Welt und Akteure zeichnen; Aufzeichnung und Wiedergabe | 10 | – |
| 6 | RU16, RU17 | Gebäude zeichnen; Effekte, Ton | 10 | – |
| 7 | RU20 | HUD 1 | 6 | – |
| 8 | RU39 | Ruckler-Messung Rust gegen Go | 3 | **G2** (🧑): Rust-Client ruckelfrei bei echter Last, auch auf schwachem PC |
| 9 | RU6, RU9, RU11, RU23 | Level-Generator; Menüs 1; Sim Einheiten, Gegner; HUD 2 | 23 | – |
| 10 | RU10, RU12, RU15 | Sim Insel; Menüs 2; Sim Kampf | 15 | – |
| 11 | RU14, RU26 | Sim Welt-Schritt; Spielschleife 1 | 11 | – |
| 12 | RU18, RU19, RU21, RU29 | Sim Wirtschaft bis Skills; Sim Events; Spielstände, Raum 1; Dev-Menüs | 19 | – |
| 13 | RU22 | Sim-Abgleich | 6 | **G3** (🧑): alle Sim- und Kampagnen-Golden mit gleichem Verhalten (Ereignis-Abgleich, Toleranz) |
| 14 | RU24 | Raum 2 | 5 | – |
| 15 | RU27 | Lokales Spiel | 4 | – |
| 16 | RU30 | Spielschleife 2 | 5 | – |
| 17 | RU32 | Ende-zu-Ende-Test | 5 | – |
| 18 | RU35 | Build, Paket, CI | 4 | – |
| 19 | RU36 | Abnahme am PC | 5 | **G4** (🧑): Abnahme am PC mit zwei Pads |
| 20 | RU33, RU37 | Läufe im Client; Go-Spiel entfernen | 8 | – |
| 21 | RU34 | Workbench ruft EXE | 4 | – |
| 22 | RU25 | Balancing 1 | 6 | – |
| 23 | RU28 | Balancing 2 | 6 | – |
| 24 | RU31 | Balancing 3 | 6 | – |
| 25 | RU40 | Balancing-Läufe im Client | 4 | – |
| 26 | RU41 | Go-Engine entfernen | 3 | – |
| | | **Summe** | **208** | |

### Offen bei 🧑 vor dem Start

Freigabe von RU0, Abstimmung mit B-943 (FB4), Ausnahme von der Sim-Sperre für Golden-Vergleiche und Aufzeichnungen, Stack und Crates, Rang (B-1014 › Offene Fragen).
