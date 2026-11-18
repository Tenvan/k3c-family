# RU39 · CLI · Ruckler-Messung: Rust-Client in Wiedergabe gegen das Go-Spiel (Gate G2)

- **Status:** erledigt
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** ee0b5264f
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

Nach RU20 zeichnet der Rust-Client Welt, Figuren, Gebäude, Effekte und HUD aus einer Aufzeichnung. Damit lässt sich die Frage des Projekts beantworten, bevor die Sim portiert wird.

## Ziel

Belegt ist, ob der Rust-Client bei gleicher Szene ruckelfrei läuft und auf einem schwachen PC weniger einbricht als das Go-Spiel. Am Ende sichtbar: Bericht `docs/messung/rust-ruckler.md`, Entscheidung G2.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 8 von 26, 3 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `opus` für Werkzeug und Bericht, Session 2 ist Mensch.

### Sessions

1. Messwerkzeug: Bildmessung der Wiedergabe im Go-Format, `measure.ps1 -Engine replay` mit einer Auswertung für beide Programme, Aufzeichnung `messung-nacht` (5 Minuten), Messanleitung, Probelauf (AC-01, AC-02, Teil Werkzeug)
2. Messläufe auf ruhigem Entwicklungs-PC und Mini-PC, Sichtprüfung nebeneinander (Mensch, AC-01, AC-02, AC-03)
3. G2-Bericht mit Bewertung, Entscheidung 🧑, Review (AC-01 bis AC-03)

### Hinweise für die Umsetzung

- Schwelle (Vorschlag): längstes Bild höchstens das Doppelte des Medians, kein Bild über dem 1,5-Fachen des Medians, das das Programm verursacht (`lang_programm`: Rechenzeit des Bildes über dem halben Median oder GC im Bild).
- Ist Rust hier nicht sichtbar besser als das Go-Spiel nach B-943, empfiehlt der Bericht den Abbruch: Bis hier sind es rund 70 Sessions, danach noch rund 140.
- Go misst Sekunden (`--measure 300`), die Wiedergabe Takte (`--measure 9000`); das Messskript rechnet um.

## Nicht-Ziele

Regel- oder Wertänderungen. Die Workbench in Rust. Alles, was ein anderer Sprint des Projekts RSU nennt.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. Die Aufzeichnung `messung-nacht` ist ein Sim-Lauf als Test mit derselben Ausnahme wie RU38.1. Einzige Go-Änderung: eine weitere Szene im Test von `engine/internal/trace` (RU39.1). Gate-Messwerte erhebt 🧑 auf ruhigem Rechner (RU39.2), G2 entscheidet 🧑 (RU39.3 hält an).

- **Bereich:** `docs/messung/rust-ruckler.md`, `native/measure.ps1`, `rust/crates/k3c-game/src/replay/**`, `engine/internal/trace/**`
- **Abhängig von:** RU38, RU16, RU17, RU20
- **Braucht 🧑:** Entscheidung
- **Sperre:** –

## Beispiele

Siehe Akzeptanzkriterien; jedes nennt den Befehl oder die Datei, an der man das Ergebnis sieht.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Dieselbe Szene (Nacht mit Gegnern, zwei Spieler, 5 Minuten) läuft im Go-Spiel und in der Rust-Wiedergabe; je Programm stehen Median, p99, p99,9, längstes Bild, Bilder über dem 1,5-Fachen des Medians, Rechenzeit je Bild, Grafikspeicher und Arbeitsspeicher im Bericht.
- **AC-02** Dieselben Werte liegen für den schwächsten verfügbaren PC vor (Mini-PC); fehlt das Gerät, steht „angenommen, Validierung offen“ im Bericht.
- **AC-03** 🧑 hat beide Programme nebeneinander gesehen und G2 entschieden: weiter mit dem Sim-Port, Abbruch oder Nacharbeit.

## Offene Fragen

Entscheidung 🧑 am Gate G2 (RU39.3 hält an) und Gerät für den schwachen PC (B-1014 › Offene Fragen 8).

- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): Schnitt geändert, die Messläufe für G2 macht 🧑 in Session 2 statt ein Agent in Session 1, weil parallele Sessions die späten Bilder verfälschen (`docs/messung/rust-client-1.md` › Bewertung 3) und der Mini-PC ein Gerät braucht; Session 1 baut Werkzeug und macht einen Probelauf.
- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): „Dieselbe Szene“ ist die Szene `nacht` aus `measure.ps1` (Seed `k3c`, 2 Spieler, 15 600 Takte Vorlauf, beide stehen); die Wiedergabe spielt dazu die Aufzeichnung `messung-nacht` (RU39.1).
- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): Ein langes Bild gilt als „vom Programm verursacht“, wenn seine Rechenzeit über dem halben Median liegt oder ein GC-Lauf im Bild war; sonst als extern (Treiber, Betriebssystem). Beide Zahlen stehen im Bericht.
- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): Gemessen wird mit VSync (Ruckler bei 60 Hz), 5 Minuten, je Programm zwei Läufe abwechselnd; ausgewertet von einer gemeinsamen Funktion in `measure.ps1`.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU39.1 | `RU39.1-messwerkzeug-rust-gegen-go.md` | Umsetzung | autonom | fertig |
| RU39.2 | `RU39.2-messlaeufe-und-sichtpruefung.md` | Workshop | Mensch | verworfen |
| RU39.3 | `RU39.3-bericht-g2-und-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-18: RU39.1 Werkzeug fertig; RU39.2 verworfen und verschoben nach B-1074 (Ralf: Messungen zum Schluss); RU39.3 Bericht mit vorläufiger Messung (CPU-Teil, unruhiger Rechner), G2 vorläufig bestanden nur für den CPU-Teil, Gate-Messung am Ende (B-1074).
Prüfungen: task check, task check:go, task check:diff grün; Review-Befunde (Formulierungen) behoben.
Neue Tickets: B-1074 (Gate-Messung, RU36), B-1075 (Allokationsmessung, RU36). AC-02 und AC-03 offen bis B-1074. Abnahme-Ticket Sim/Balance/Gen: kein eigenes (keine Sim-, Balance- oder Gen-Änderung; Sim-Sperre gilt).
