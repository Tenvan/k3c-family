# B-1015 · Wellen- und Kostenplan der Rust-Umstellung

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU1
- **Projekt:** RSU
- **Erstellt:** 2026-11-16
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

B-1014 beschreibt Anlass, Gründe und Empfehlung. Dieser Plan zerlegt die Umstellung in 42 Sprints (RU0 bis RU41) in 26 Wellen, in der Reihenfolge „Client zuerst“ (🧑 2026-11-16): Diagnose, Client mit Wiedergabe, Sim-Kern, Spiel, Läufe, Balancing.

## Ziel

🧑 liest Dauer, Aufwand und Ausstiegspunkte an einer Stelle ab und kann je Gate über weiter oder Abbruch entscheiden.

## Beteiligte und Zielgruppen

🧑 entscheidet. Agenten planen die Wellen mit `plan_waves`, sobald Specs freigegeben sind.

## Anforderungen

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
| 13 | RU22 | Sim-Abgleich | 6 | **G3** (🧑): alle Sim- und Kampagnen-Golden bitgleich |
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

Höchstens vier Sprints laufen gleichzeitig; Sessions eines Sprints laufen nacheinander. Die Wellen folgen aus den Abhängigkeiten der Sprint-READMEs.

### Aufwand je Block

| Block | Sprints | Sessions | Größe |
|---|---|---:|---|
| A · Diagnose und Messung (bis G1) | RU0, RU1, RU2 | 11 | M |
| B · Client mit Wiedergabe (bis G2) | RU3, RU4, RU5, RU7, RU8, RU13, RU16, RU17, RU20, RU38, RU39 | 58 | L |
| C · Sim-Kern (bis G3) | RU6, RU10, RU11, RU14, RU15, RU18, RU19, RU22 | 43 | L |
| D · Spiel fertig (bis G4) | RU9, RU12, RU23, RU21, RU24, RU26, RU27, RU29, RU30, RU32, RU35, RU36 | 59 | L |
| E · Go-Spiel raus, Läufe im Client, Workbench | RU37, RU33, RU34 | 12 | M |
| F · Balancing, ganz hinten | RU25, RU28, RU31, RU40, RU41 | 25 | L |
| **Summe** | 42 Sprints | **208** | |

Bis G2 sind es 69 Sessions: Dann steht fest, ob der Rust-Client die Ruckler beseitigt. Bis G4 (spielbares Rust-Spiel) 171, danach 37 für Rückbau, Läufe und Balancing.

### Annahmen (alle ungeprüft)

- 450 Zeilen Code samt zugehörigen Tests je Port-Session (007 rechnete mit 400–520), eine Review-Session je Sprint. Spanne ±20 %: 166–250 Sessions. Die Zeilen je Session stehen in den Sprint-READMEs (Tabelle „Go-Dateien“).
- Eine Session ≈ ½ Tag Agentenarbeit einschließlich Review durch 🧑 (Annahme aus 006). Bis G2 rund 8 Wellen: mindestens 2 Wochen, realistisch 3–4. Bis G4 weitere 11 Wellen: realistisch 7–10 Wochen ab Start. Block F hängt hintereinander und braucht weitere 2–3 Wochen.
- Token: 3–8 Mio. verarbeitete Token je Session (überwiegend Cache-Lesen) und 60–150 Tsd. Ausgabe-Token; zusammen 0.6–1.7 Mrd. verarbeitet und 12–31 Mio. Ausgabe. Nicht gemessen; RU0 bis RU2 liefern die ersten echten Werte.
- Parallelität im Sim-Crate (Wellen nach G2): Greifen die Gruppen stärker ineinander als angenommen, laufen RU10 bis RU19 nacheinander und der Weg bis G3 wächst um rund 3 Wellen.

### Parallelphase

- **Bis G2** entsteht nur neuer Rust-Code neben dem Go-Spiel. Go-Features laufen weiter. Kosten: Ändert eine CLI-Session Zeichen-Code, der schon portiert ist (`world`, `actors`, `props`, `fx`, `hud`), braucht sie eine Nachzieh-Session in Rust (Aufschlag rund 40 % je betroffener Session, ungeprüft).
- **Ab G2 bis G4** (Vorschlag): Feature-Stopp für SIM und CLI. Die Golden sind dann ein festes Ziel für G3. REG, PAP, DEV und Planung laufen weiter. Stand 2026-11-16: 20 offene autonome Sessions in SIM und CLI (6 SIM, 14 CLI); KSY ändert mit sieben aktiven Sprints den Kampf und sollte vor G2 fertig sein oder ruhen.
- **Nach G4** laufen Regeländerungen nur noch in Rust. Level, Welten-Prüfung und Hub-Planer der Workbench rechnen bis RU34 mit der alten Go-Engine, Balancing bis RU40. Deshalb folgen RU33 und RU34 direkt auf G4, und Balancing bleibt bis RU40 gesperrt.

### Rückfall

Bis G4 bleibt der Go-Stand unverändert und spielbar; Rust liegt nur unter `rust/`. Abbruch an einem Gate: `rust/`, Rust-Tasks und CI-Job entfernen (eine Session, Muster B-779). Verlorener Aufwand: an G0 3 Sessions (die Messung bleibt für B-943 nützlich), an G1 11, an G2 69, an G3 143, an G4 171. Nach G4 markiert der Tag `go-final` den letzten Stand mit Go-Spiel; RU37 und RU41 sind eigene PRs und per Revert umkehrbar.

### Risiken

| Risiko | Wirkung | Umgang |
|---|---|---|
| Ursache der Ruckler ist der Bildtakt, nicht GC | Rust behebt sie nicht von selbst | RU0 vor allem anderen; Zeitsammlung in RU4 nach dem Ergebnis |
| Schlanker Stack verfehlt G1 | Ende nach 11 Sessions | Gegenprobe `wgpu` in RU2 |
| Rust-Client ist bei echter Last nicht sichtbar besser | Ende nach 69 Sessions | G2 mit Sichtprüfung und Messung auf schwachem PC |
| Go-gleiche Gleitkommazahlen nicht erreichbar | Golden nicht bitgleich | RU3 früh; RU6 prüft am Level-Generator |
| Zustandsmodell (Indizes statt Zeiger) passt nicht | Sim-Sprints dauern länger | Vorentscheidungen in RU8; Zeichnen aus Aufzeichnungen prüft die Typen schon vor dem Sim-Port |
| Regeln ändern sich in Go während des Sim-Ports | bewegliches Ziel | Feature-Stopp ab G2 |
| Zwei Sims zwischen G4 und RU41 | Balancing rechnet mit altem Stand | Balancing bleibt bis RU40 gesperrt |
| Headless-Lauf braucht doch einen Grafikkontext | B-1017 nicht erfüllbar | RU2/AC-04 und RU4/AC-02 prüfen es früh |
| Sim-Sperre verhindert Golden-Tests und Aufzeichnungen | Port nicht prüfbar | Ausnahme durch 🧑 vor RU3 |
| Aufwand läuft davon | Kosten | Review jedes Sprints nennt Ist gegen Plan; über dem 1,5-Fachen wird neu geschätzt |

## Nicht-Ziele

Die Umsetzung selbst. Eine Kostenrechnung in Euro (hängt am Tarif).

## Regeln und Einschränkungen

Wellen nach `docs/arbeitsweise.md` › Parallelität und Wellen; `plan_waves` rechnet erst mit freigegebenen Specs.

- **Bereich:** `docs/messung/rust-kosten.md`
- **Abhängig von:** –
- **Braucht 🧑:** Entscheidung
- **Größe:** M
- **Sperre:** –

## Beispiele

Nach G1 zeigen RU0 bis RU2 5 statt 3–8 Mio. Token je Session → die Hochrechnung in `docs/messung/rust-kosten.md` wird damit neu gerechnet und 🧑 vorgelegt.

## Ausnahme- und Fehlerfälle

Ein Sprint braucht mehr als das 1,5-Fache seiner Sessions → der Review nennt es, der nächste Gate-Bericht rechnet die Summe neu. Zwei Sprints einer Welle kollidieren doch → der mit der höheren Nummer rückt in die nächste Welle.

## Akzeptanzkriterien

- **AC-01** Alle 42 Sprints stehen mit Bereich, Abhängigkeit, Sperre und Kriterien im Fahrplan.
- **AC-02** Nach G1 ist die Hochrechnung (Sessions, Token, Dauer) mit den Ist-Werten aus RU0 bis RU2 in `docs/messung/rust-kosten.md` korrigiert.
- **AC-03** 🧑 hat die Regel für die Parallelphase entschieden.

## Offene Fragen

1. Parallelphase wie vorgeschlagen (🧑)? 2. KSY vor G2 abschließen oder ruhen lassen (🧑)? 3. Wie viele der vier Plätze bekommt RSU, wenn andere Projekte weiterlaufen (🧑)?

## Notizen

Sessions je Sprint stehen in den Sprint-READMEs unter Anforderungen, für Port-Sprints als Tabelle mit den Go-Dateien je Session. Session-Dateien gibt es für RU0 bis RU3; die übrigen entstehen beim Bereitmachen aus der Tabelle und dem Port-Leitfaden (B-1018). Modell-Empfehlung für alle Sessions: `sonnet`; Leitfaden und Vorentscheidungen nehmen die Entwurfsarbeit vorweg.
