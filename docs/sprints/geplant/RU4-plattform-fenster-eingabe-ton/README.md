# RU4 · CLI · Plattform-Schicht: Schleife mit und ohne Fenster, Eingabe, Ton, Text

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Heute liefert Ebitengine Fenster, Schleife, Eingabe, Text und Ton (16 von 17 Paketen in `native/k3c-game` importieren es). In Rust gibt es dafür keinen einzelnen Baustein.

## Ziel

Eine dünne Plattform-Schicht kapselt den in G1 gewählten Stack; die Spielschleife läuft per Parameter mit Fenster oder headless. Am Ende sichtbar: Testfenster mit zwei Bildhälften, zwei Pads, Text und Ton; derselbe Lauf mit `--headless` ohne Fenster.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 3 von 26, 6 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

1. Schnittstelle `platform` (Vorentscheidungen), Fenster, feste Schrittweite, Render-Ziele je Hälfte; dieselbe Schleife ohne Fenster (AC-01, AC-02)
2. Eingabe: `native/k3c-game/input` portieren, Gamepads, Xbox-Belegung, B geht zurück (AC-03)
3. Ton Teil 1: `output`, `sink`, `mix` aus `native/k3c-game/audio` (AC-04)
4. Ton Teil 2: `bank`, `atlas`, `duck`, `audio`; Text und Schrift (AC-04, AC-05)
5. Log mit Emoji-Regel, Probe-Programm für beide Modi, Test „1000 Schritte gleich“ (AC-02, AC-05)
6. Review

### Vorentscheidungen

Die Schnittstelle der Plattform-Schicht steht fest; die Session baut sie, sie entwirft sie nicht neu.

- `trait App { fn step(&mut self, input: &InputState, dt: f64); fn draw(&mut self, canvas: &mut dyn Canvas); fn done(&self) -> bool; }`
- `trait Canvas`: `size()`, `sprite(tex, src, transform, tint)`, `mesh(tex, vertices, indices)`, `rect(…)`, `circle(…)`, `line(…)`, `path(…)`, `text(font, s, pos, size, color)`, `begin_target(id)` und `end_target()` für die Bildhälften.
- `NullCanvas` erfüllt `Canvas` mit leeren Funktionen. `run_headless(app, steps, script)` ruft nur `step` (und `draw` gegen `NullCanvas`, damit Zeichen-Code mitläuft) und fasst weder Fenster noch Grafikkontext noch Tongerät an.
- `run_window(app, config)` öffnet das Fenster und sammelt Zeit: je 1/60 s ein `step`, danach ein `draw`. Die Sim sieht immer `dt = 1.0 / 60.0`.
- `InputState` ist ein Schnappschuss je Schritt: je Spieler Achsen und Tasten (gehalten, gerade gedrückt), dazu die Tastatur. Headless kommt er aus einem `InputScript` (Liste aus Schritt und Eingabe).
- Der Zeichen-Stack wird nur in `platform/backend/` importiert. Ein Test liest alle anderen Dateien der Crate und schlägt fehl, wenn sie den Namen der Stack-Crate enthalten (Muster `native/k3c-game/game/imports_test.go`).
- Ton hat dieselbe Trennung: `trait AudioOut`, `NullAudio` für headless.

### Hinweise für die Umsetzung

- Kein Fachpaket darf den Zeichen-Stack direkt importieren; nur `platform/` kennt ihn (Leitfaden § 9).
- Headless heißt: kein Fenster, kein Grafikkontext, kein Tongerät. Zeichenaufrufe gehen an eine leere Zeichenfläche und kosten nichts.
- Feste Schrittweite wie im Go-Spiel (`native/k3c-game/game/game.go`, Ebitengine-TPS 60): Die Sim rechnet je Schritt dieselbe Zeit, unabhängig von der Bildrate.
- Die Zeitsammlung der Schleife folgt dem Ergebnis aus RU0 (`docs/messung/ruckler-go.md`): Sim-Takt 30 Hz wie `game.TPS`, je Bild so viele Takte, wie die gesammelte Zeit hergibt, alpha ist der Rest geteilt durch die Takt-Länge. alpha geht an `draw`; die Wanduhr wird in `draw` nicht gelesen.
- Im Schritt und im Zeichnen wird nichts angelegt, was je Bild neu entsteht: Puffer wiederverwenden. Ein Test zählt die Speicher-Anforderungen je Bild im Leerlauf und verlangt null.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-game/src/platform/**`
- **Abhängig von:** RU1, RU2
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Die Schleife rechnet mit fester Schrittweite und zeichnet je Spieler in ein eigenes Render-Ziel; Vollbild und Fenstergröße sind umschaltbar.
- **AC-02** Mit `--headless` läuft dieselbe Schleife ohne Fenster und ohne Grafikkontext; ein Test rechnet 1000 Schritte in beiden Modi und vergleicht den Zustand.
- **AC-03** Zwei Gamepads und Tastatur liefern je Spieler eigene Eingaben; B geht in Menüs zurück; nur Xbox-Symbole.
- **AC-04** OGG-Ton spielt mit Lautstärke je Gruppe; headless gibt es keinen Ton und keinen Fehler.
- **AC-05** Text mit Umlauten in den Schriften des Spiels; Log-Meldungen beginnen mit einem Emoji aus der Liste in `CLAUDE.md`.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
