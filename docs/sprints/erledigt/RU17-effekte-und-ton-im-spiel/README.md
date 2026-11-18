# RU17 · CLI · Effekte und Ton im Spiel

- **Status:** erledigt
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** be144f5d7
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

`fx` (1080 Zeilen) und das ganze Paket `audio` (1311 Zeilen: Register, Atlas, Bank, Mischen, Ducking, Wiedergabe, Klänge je Zustand, `data/sounds.json`). Die Ton-Plattform (Gerät, Dekodieren, Stimmen, `NullAudio`) liefert RU4 als `k3c_platform::AudioOut`.

## Ziel

Effekte und Klänge folgen dem Sim-Zustand. Am Ende sichtbar: Nacht mit Gegnern mit Effekten und Ton.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 6 von 26, 6 Sessions (B-1015; nach der Überarbeitung von RU4 zwei Sessions mehr, weil RU4 nur die Plattform `k3c_platform::AudioOut` liefert und das ganze Go-Paket `audio` hier portiert wird).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`; Session 5 `opus` (Haltepunkt Atlas, Schnittstelle zur Plattform).

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `native/k3c-game/fx/` → `rust/crates/k3c-game/src/fx/`, `native/k3c-game/audio/` → `rust/crates/k3c-game/src/audio/`; Dateinamen nach Leitfaden § 2. Ebitengine-Audio und oto (`sink.go`, `output.go`, Dekodieren in `bank.go`) gehen auf `k3c_platform::AudioOut` (RU4.8). Die Sessions laufen als Kette auf dem Branch `sprint/ru17`. Der Schnitt weicht von der ursprünglichen Tabelle in der Verteilung ab (gleiche Dateien, gleiche Zahl Sessions): `fx.go` braucht `paint`, `looks` und `status` (die Rust-Datei `mod.rs` kommt nach den reinen Regeln), `audio/output.go` braucht `mix` und `music`, `events` gehört vor `mix`.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `native/k3c-game/fx/`: effects (203, T), looks (117), status (172, T), texts (13) | 505 |
| 2 | `native/k3c-game/fx/`: fx (267), paint (215), paintmarks (93); dazu `textrule_test.go` | 575 |
| 3 | `native/k3c-game/audio/`: register (153), atlas (65, T), duck (52), events (127, T); `sink` (47) entfällt (AudioOut) | 397 |
| 4 | `native/k3c-game/audio/`: mix (237, T), music (143, T) | 380 |
| 5 | `native/k3c-game/audio/`: bank (140), output (299, T), audio (48, T), dazu `klang_test.go` | 487 |
| 6 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 2391 Zeilen Go (`sink.go` ohne Rust-Entsprechung). Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. `native/`, `engine/` und `k3c-platform` bleiben unverändert; eine Erweiterung von `k3c-platform` (Atlas-Ausschnitte, RU17.5) braucht 🧑.

- **Bereich:** `rust/crates/k3c-game/src/fx/**`, `rust/crates/k3c-game/src/audio/**`, `rust/crates/k3c-game/src/lib.rs`
- **Abhängig von:** RU13
- **Braucht 🧑:** Haltepunkt in RU17.5 (Atlas-Ausschnitte über `AudioOut`, siehe Offene Fragen)
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Effekte erscheinen bei denselben Ereignissen wie im Go-Spiel.
- **AC-02** Klänge aus `data/sounds.json` spielen je Ereignis und Spielerhälfte; die Paket-Tests sind portiert.

## Offene Fragen

Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- **Aura-Quelle:** `sim.AurasOf` (skills_status.go) liefert erst RU18. Default: `fx` bekommt die Aura-Funktion als Parameter (wie in Go schon für die Tests), Platzhalter `fx/pending.rs::aura_of`, RU18 löscht ihn.
- **Textregel-Test:** `textrule_test.go` parst Go-Quelltext. Default: Rust-Test mit Zeichenscan über `fx/*.rs` und derselben Regel.
- **Ladezeit der Bank:** synchron beim ersten hörbaren Mix, Messung im Ergebnis von RU17.5, Thread nur bei > 100 ms.
- **Schnitt:** `events` vor `mix`, `bank` mit `output` (Abhängigkeiten), siehe Anforderungen.

**Haltepunkt für 🧑 (RU17.5, Atlas-Ausschnitte):** `AudioOut::load` nimmt ganze Dateien; Go spielt Effekte als Ausschnitt eines dekodierten Atlas (`public/audio/atlas.*`). RU17.5 hält an und fragt 🧑, wenn die Plattform keinen Ausschnitt-Weg bietet: (a) `k3c-platform` um `load_range` erweitern (Vorschlag) oder (b) Effekte als Einzeldateien über den Go-Rückfall laden. Echte Gate-Frage (Änderung eines fertigen Crates), nicht selbst entschieden.

**Nachzug aus Go-Sprints** (Beschluss 🧑 2026-11-16: Das Go-Spiel wächst nicht weiter). Diese Arbeiten entstehen direkt im Rust-Client, sobald die Sim-Seite steht. Spec steht im Ticket:

- SI3.4 Vorboten, Taifun und Flut mit Ambient (B-999)
- SI6.4 Jubel, Tanz und Himmels-Effekt je Welt (B-1002)
- SI6.5 Festmusik je Welt (B-1002)
- WE8.4 Ankündigung und Schleier für Überfall, Sandsturm, Nebel (B-995)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU17.1 | `RU17.1-fx-effects-looks-status-texts.md` | Umsetzung | autonom | fertig |
| RU17.2 | `RU17.2-fx-fx-paint-paintmarks.md` | Umsetzung | autonom | fertig |
| RU17.3 | `RU17.3-audio-register-atlas-duck-events.md` | Umsetzung | autonom | fertig |
| RU17.4 | `RU17.4-audio-mix-music.md` | Umsetzung | autonom | fertig |
| RU17.5 | `RU17.5-audio-bank-output-audio.md` | Umsetzung | autonom | fertig |
| RU17.6 | `RU17.6-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-18. AC-01 und AC-02 geprüft: `task check` und `task check:go` grün, alle Go-Tests von `fx` und `audio` in Rust portiert (Ergebnisse RU17.1 bis RU17.5), ein ignorierter Test `load_bank_atlas_clips_match_sprites` mit Grund B-1060 (Atlas-Haltepunkt, RU30, Braucht 🧑). Behobener Befund: `aura_of` stürzte per `unimplemented!` im Standardpfad ab, liefert jetzt eine leere Liste bis RU18. Abnahme-Ticket: B-1062 (check:race, Sichtprüfung, RU39-Prüfpunkte). Nachzüge B-999, B-1002, B-995 nicht Teil dieses Sprints, Stand unverändert offen.
