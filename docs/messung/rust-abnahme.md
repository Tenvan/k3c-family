# Abnahme am PC: Rust-Spiel gegen Go-Spiel (RU36, Gate G4)

Stand RU36.1 (Hauptrechner, 2026-11-19): Liste, Schwere und Messwerkzeug stehen; die Gate-Werte und die Abnahme der Stationen
erhebt 🧑 in RU36.2 am PC mit zwei Pads. Auf dem Hauptrechner liefen keine Messläufe (Sperre 2026-11-11, B-742), der
Probelauf ist darum offen (siehe Abschnitt Probelauf). Go gilt als Referenz: Weicht Rust ab, ist es ein Befund, nie ein neues Golden.

## Vorbereitung (🧑)

Am PC, ruhiger Rechner (keine Builds, keine Agenten-Sessions, `docs/messung/rust-client-1.md` › Bewertung 3):

```powershell
task gen:paper                # Grafik rendern (einmal)
task build                    # Rust-Spiel nach dist\k3c-game\ (k3c-game.exe)
task native:game:build        # Go-Spiel nach dist\k3c-game-go\ (k3c-game.exe, measure.ps1)
```

Beide Spiele nebeneinander starten und vergleichen: `dist\k3c-game-go\k3c-game.exe` (Referenz) und
`dist\k3c-game\k3c-game.exe` (Rust). Einen zweiten Controller anstecken (Spieler 2 = Pad, Spieler 1 = Tastatur wie im
Ende-zu-Ende-Ablauf). Eintrag je Zeile: `Rust ok` = `ja` oder `nein`; bei `nein` Befund und Schwere ausfüllen.

## Schwere

- **schwer:** Absturz oder Hänger, Datenverlust beim Speichern oder Laden, Station nicht erreichbar, Eingabe eines der zwei
  Pads falsch oder vertauscht, sichtbares Ruckeln, Regel anders als Go. Wird in RU36.3/RU36.4 behoben.
- **mittel:** sichtbar anders als Go ohne Folgen für das Spiel (Lage, Farbe, Text). Wird ein Ticket.
- **leicht:** Kosmetik. Wird ein Ticket.

Regel für jedes Menü: Controller-Taste **B** geht zurück (wie Esc); im laufenden Spiel bleibt sie unbelegt.

## Wiederprüfung aus der groben Abnahme (Smoke-Client)

🧑 hat den Smoke-Client grob abgenommen („sehr gut“). Zwei Befunde wurden dabei gefunden und werden im Sprint RU54
behoben (andere Session, Crate `k3c-game`: view, hud, Bildpfad); hier sind sie die Wiederprüfung nach dem Merge von RU54.

| Nr. | Befund | Prüfung | Rust ok | Befund neu | Schwere |
|---|---|---|---|---|---|
| W-1 | Pausenmenü fehlte | Im Spiel `Esc` bzw. Menu-Taste: Pause-Menü liegt auf dem Papier-Hintergrund, Einträge Fortsetzen, Optionen, Tasten, Spieler abmelden, Spiel verlassen (Station 50) | – | – | – |
| W-2 | Farben wirkten blass | Gleiche Szene (Tag und Nacht, Menüs) neben dem Go-Spiel: Farbton und Sättigung gleich, kein Grauschleier über Welt und HUD | – | – | – |

## Abnahme-Liste

Quellen: Ende-zu-Ende-Ablauf `native/k3c-game/e2e/flow_test.go` (14 Stationen, in Spalte „E2E“), Bildschirme in
`native/k3c-game/menu/`, HUD in `native/k3c-game/hud/`, Werkzeuge in `native/k3c-game/devtools/` und `view/devkeys.go`.
Bekannte Abweichung vorab (aus `docs/messung/rust-bildvergleich.md`, ungeprüft für die Bilder): Das Licht (Lichtkarte)
zeichnet Rust nicht (B-1056), Nachtstationen weichen darum voraussichtlich ab (mittel, Ticket B-1056).

### Gruppe Menüs

| Nr. | Gruppe | Station | So kommt man hin | Erwartung (wie Go) | E2E | Rust ok | Befund | Schwere |
|---|---|---|---|---|---|---|---|---|
| 01 | Menüs | Hauptmenü | Spiel starten | Einträge Weiter, Neues Spiel, Laden, Optionen, Testing, Lizenzen, Beenden; „Weiter“ grau ohne Spielstand; Version unter dem Titel | 01 hauptmenue | – | – | – |
| 02 | Menüs | Hauptmenü mit Pad | Pad hoch/runter, A bestätigt | Auswahl folgt dem Pad, A öffnet den Eintrag | – | – | – | – |
| 03 | Menüs | Weltwahl | Neues Spiel | Links/Rechts blättert durch die Welten, großes Weltbild mit Namen, A wählt, B zurück | – | – | – | – |
| 04 | Menüs | Lobby mit zwei Geräten | Neues Spiel › Welt wählen | Gerät A tritt bei (Spieler 1); zweites Gerät tritt mit A bei, Spieler 2 geht mit B; B von Spieler 1 bricht ab | 02 lobby | – | – | – |
| 05 | Menüs | Lobby: Grad, Ziel, Niederlage | In der Lobby Links/Rechts, Hoch/Runter | Grad Normal, Schwer, Ultra; neuer Grad bringt seine Standard-Niederlage; Welt und Seed einstellbar | – | – | – | – |
| 06 | Menüs | Spielstart zu zweit | Lobby › Starten | zwei Spieler-Hälften, Tastatur und Pad zugeordnet | 03 spielstart | – | – | – |
| 07 | Menüs | Einstellungen | Hauptmenü › Optionen | Lautstärke Musik/Effekte in 10-%-Schritten, Anzeige (Fenster, rahmenlos, Vollbild), Sprache, Blitze, Anleitung, Entwicklermodus; Links/Rechts ändert | – | – | – | – |
| 08 | Menüs | Steuerung | Optionen › Steuerung | je Spieler ein Eingabe-Profil (Tastatur WASD, Pfeile, Gamepad); Wechsel wirkt sofort; Tastatur-Profil nicht doppelt | – | – | – | – |
| 09 | Menüs | Pad-Test (Tasten-Test) | Optionen (Entwicklermodus an) › Testing › Tasten-Test | zeigt je Gerät live die Tasten mit Xbox-Namen, Achsen und letzte Aktion; B zurück | – | – | – | – |
| 10 | Menüs | Spielstände (Laden) | Hauptmenü › Laden | neueste zuerst; A lädt; Tab fragt nach Löschen (Vorwahl „Abbrechen“); nicht ladbare Stände mit Grund markiert | – | – | – | – |
| 11 | Menüs | Lizenzen | Hauptmenü › Lizenzen | Lizenz-Seite mit Spiel, Schrift und Crates (B-1121: im Paket fehlt noch `LIZENZEN.txt`, Prüfung siehe Gruppe Paket) | – | – | – | – |
| 12 | Menüs | Testing | Optionen › Entwicklermodus an, Hauptmenü › Testing | Vorgaben aus `data/stresstests.json`, Zeilen ändern, Starten startet den Stresstest; Zurück | – | – | – | – |
| 13 | Menüs | B geht zurück | In jedem Menü (01 bis 12) B drücken | jede Seite geht eine Ebene zurück; im laufenden Spiel ist B unbelegt | – | – | – | – |

### Gruppe Spiel

| Nr. | Gruppe | Station | So kommt man hin | Erwartung (wie Go) | E2E | Rust ok | Befund | Schwere |
|---|---|---|---|---|---|---|---|---|
| 20 | Spiel | Laufen | Spieler 1 Tastatur rechts, Spieler 2 Pad Stick | beide Monarche laufen unabhängig, nur horizontal, keine Sprünge | – | – | – | – |
| 21 | Spiel | Sprinten | Sprint-Taste halten (Spieler 2: Pad) | schneller als Laufen, Ausdauer wie Go | – | – | – | – |
| 22 | Spiel | Münzen geben | An der Mauer Spieler 1 „zahlt“ | Münze fliegt, Gold sinkt, Ton | 04 muenzen | – | – | – |
| 23 | Spiel | Bauer und Bau | Münzen an einen Bauplatz geben | Bauer holt Werkzeug, Gebäude (Mauer) wird gebaut, Meldung „gebaut“ | 07 gebaut | – | – | – |
| 24 | Spiel | Skill-Punkt finden | Spieler 2 sprintet nach rechts zum Skill-Punkt | Pickup einsammeln, Meldung | 05 skillpunkt | – | – | – |
| 25 | Spiel | Skill-Menü | Skill-Menü-Taste (Pad: Steuerkreuz runter) | Menü nur für den öffnenden Spieler; Links/Rechts wählt, A lernt, B schließt; B-1126: nur der wählende Spieler lernt | 06 skillmenue | – | – | – |
| 26 | Spiel | Skill lernen und Respec | Skill-Menü, ersten Skill lernen | Skill gelernt; Respec-Eintrag nur am Tag an der Burg (B-1126) | – | – | – | – |
| 27 | Spiel | Tag/Nacht-Wechsel | Spielzeit bis zur Dämmerung laufen lassen | Dämmerung, Nacht; Licht: bekannte Abweichung B-1056 | – | – | – | – |
| 28 | Spiel | Nacht mit Welle | Erste Nacht abwarten | Gegner kommen beim Monarchen an | 08 nacht | – | – | – |
| 29 | Spiel | Kampf und Truppen | Nacht, Truppen/Bogenschützen eingesetzt | Kampf-Anzeige, Schaden, Treffer-Feedback, Truppen kämpfen | – | – | – | – |
| 30 | Spiel | Meldungen (Banner) | bei Ereignissen (Bau, Skill, Welle) | Text je Spieler, eine Zeile, Warteschlange | – | – | – | – |
| 31 | Spiel | Split-Screen und Kamera | zwei Spieler | zwei Ansichten, Kamera folgt je Spieler, HUD je Hälfte | – | – | – | – |
| 32 | Spiel | Geschichtszeile | Wendepunkte der Kampagne | 1 bis 2 Zeilen je Hälfte ein- und ausblenden | – | – | – | – |

### Gruppe Boss, Seekarte und Reise

| Nr. | Gruppe | Station | So kommt man hin | Erwartung (wie Go) | E2E | Rust ok | Befund | Schwere |
|---|---|---|---|---|---|---|---|---|
| 40 | Boss | Boss-Leiste | Boss-Welle (Cheat-Menü F9 verkürzt den Weg) | Leiste mit Anteil, Name, Phase („Phase 2 von 3“), Warnkreis vor Flächenschlag | – | – | – | – |
| 41 | Seekarte | Seekarte am Hafen | Endboss-Sieg, Wechselpunkt der Insel offen | je Zelle Etappen untereinander, Inseln als Knoten (abgeschlossen, Ziel, gesperrt); Links/Rechts wählt, A bestätigt; Wahl gilt für alle (B-1126) | – | – | – | – |
| 42 | Reise | Schiffsfahrt | nach der Wahl der Zielinsel | Fahrt über die Karte, Schiff schaukelt, dann beide Spieler auf der neuen Insel (B-1126) | – | – | – | – |

### Gruppe Pause, Speichern, Laden, Niederlage

| Nr. | Gruppe | Station | So kommt man hin | Erwartung (wie Go) | E2E | Rust ok | Befund | Schwere |
|---|---|---|---|---|---|---|---|---|
| 50 | Pause | Pause-Menü | im Spiel Menu-Taste oder Esc (Wiederprüfung W-1) | Papier-Hintergrund; Fortsetzen, Optionen, Tasten, Spieler abmelden (grau bei einem Spieler), Spiel verlassen; B und Menu setzen fort; Simulation steht | 09 pause | – | – | – |
| 51 | Pause | Spieler abmelden | Pause › Spieler abmelden (zu zweit) | Spieler des bedienenden Geräts geht, der andere spielt weiter | – | – | – | – |
| 52 | Speichern | Spiel verlassen speichert | Pause › Spiel verlassen | Stand wird gespeichert, Hauptmenü, „Weiter“ aktiv | 10 verlassen | – | – | – |
| 53 | Laden | Weiter | Hauptmenü › Weiter | derselbe Stand wie beim Speichern, Spielzeit läuft weiter | 11 weiter | – | – | – |
| 54 | Laden | Spieler 2 tritt wieder bei | nach Weiter: Spieler 2 drückt A | Spieler 2 spielt wieder mit | 12 zu-zweit | – | – | – |
| 55 | Laden | Schnellspeichern / Neu laden | F5, F6 im Spiel | speichert und lädt denselben Stand | – | – | – | – |
| 56 | Niederlage | Kronenraub und Niederlage | Spieler ohne Gold vor das Portal, zweite Nacht abwarten | Gegner trägt die Krone ins Portal, Game Over | 13 niederlage | – | – | – |
| 57 | Niederlage | Zurück ins Hauptmenü | nach der Niederlage warten | nach `view.EndDelay` liegt das Hauptmenü oben | 14 ende | – | – | – |

### Gruppe Werkzeuge, Ton, Anzeige

| Nr. | Gruppe | Station | So kommt man hin | Erwartung (wie Go) | E2E | Rust ok | Befund | Schwere |
|---|---|---|---|---|---|---|---|---|
| 60 | Werkzeuge | Tastenübersicht | F1 | Liste aller Entwickler-Tasten mit Pad-Entsprechung | – | – | – | – |
| 61 | Werkzeuge | Leistungsanzeige | F2 | FPS und Zeiten ein/aus | – | – | – | – |
| 62 | Werkzeuge | Cheat-Menü | F9 oder View + Y | Gruppen, Werte setzen, Spielstand bleibt bedienbar; Pad bedienbar | – | – | – | – |
| 63 | Werkzeuge | Debug-Overlay | F10 | Geometrie, Werte, aus | – | – | – | – |
| 64 | Werkzeuge | Pause/Zeitraffer | F7 (Pause, ein Takt), F8 (1× 2× 4× 8×) | wie Go | – | – | – | – |
| 65 | Werkzeuge | Screenshot | F12 | Bild unter `reports/` | – | – | – | – |
| 66 | Anzeige | Vollbild | F11 oder rechter Stick | wechselt Fenster und Vollbild | – | – | – | – |
| 67 | Ton | Musik und Effekte | Spiel, Menüs; Lautstärke in Optionen | Musik, Münzen, Bau, Kampf hörbar; Regler wirkt | – | – | – | – |
| 68 | Anzeige | Farben und Bild | Menüs und Spiel neben Go | Wiederprüfung W-2 (Farben nicht blass) | – | – | – | – |

### Gruppe Paket und CI (Tickets B-1121, B-1123, B-1124)

| Nr. | Gruppe | Station | So kommt man hin | Erwartung | Rust ok | Befund | Schwere |
|---|---|---|---|---|---|---|---|
| 70 | Paket | Rust-Paket startet am Zielgerät | Ordner `dist\k3c-game` auf einen anderen PC kopieren, `k3c-game.exe` per Doppelklick und aus einer Konsole in einem fremden Ordner starten (B-1123 AC-06, RU57) | startet mit Grafik und Ton, `README.txt` erklärt Start und `saves\`, `saves\` entsteht neben der EXE | – | – | – |
| 71 | Paket | Lizenzdatei | `dist\k3c-game\LIZENZEN.txt`, `LICENSE`, `LICENSE-ASSETS` (B-1121, RU57) | nennt die Werke der Menü-Seite und rund 130 Rust-Crates mit Lizenztext; 14 Crates liefern keinen Text (u. a. `symphonia-*` MPL-2.0), dort stehen Quelle und Adresse des Standardtexts; Kopf nennt noch Go-Bibliotheken (B-1139). Entscheidung 🧑: genügt das für die Weitergabe | – | – | – |
| 72 | Paket | EXE-Größe und Paketliste | `exe_mib` im Messbericht; `task rust:game:dist:check` (B-1137) | EXE höchstens 40 MB (Schwelle G1); Prüfung meldet vollständig, keine Go-Reste (`*.go`, `assets.txt`) im Paket | – | – | – |
| 73 | CI | CI auf main | `gh run list --branch main --limit 5` (B-1124) | alle Jobs grün; heute rot: Natives Spiel, Go arm64, k3c-workbench (nicht Rust, Ticket B-1124) | – | – | – |
| 74 | CI | Prüfhinweise RU32 | `docs/messung/rust-bildvergleich.md` (B-1120) | Tabelle der 14 Stationen gefüllt, Stationen über der Schwelle mit Ticket | – | – | – |
| 75 | Skills | Prüfhinweise RU53 | B-1126 AC-03 | Station 25, 26, 41, 42 | – | – | – |

## Messanleitung (🧑, RU36.2)

Ruhiger Rechner, nichts anderes laufen lassen; Rust und Go abwechselnd messen (Go, Rust, Go, Rust, …). Ohne VSync wird
FPS gemessen, mit VSync die Bildzeit. Aufruf aus der Repo-Wurzel; vorher beide Spiele bauen (siehe Vorbereitung) und
`task native:game:exe` (Go-EXE `native\k3c-game\bin\k3c-game.exe`) oder das Go-Paket `dist\k3c-game-go\` verwenden.

```powershell
# 1. FPS, 1-%-Low, GPU- und Arbeitsspeicher, Startzeit: je Szene tag und nacht, 60 s, drei Läufe, ohne VSync
powershell -NoProfile -ExecutionPolicy Bypass -File native\measure.ps1 -Engine game     -Seconds 60 -Runs 3
powershell -NoProfile -ExecutionPolicy Bypass -File native\measure.ps1 -Engine rustgame -Seconds 60 -Runs 3

# 2. Längstes Bild zu Median: Szene nacht, 300 s, mit VSync, ein Lauf je Spiel
powershell -NoProfile -ExecutionPolicy Bypass -File native\measure.ps1 -Engine game     -Szenen nacht -Seconds 300 -Vsync -Frames
powershell -NoProfile -ExecutionPolicy Bypass -File native\measure.ps1 -Engine rustgame -Szenen nacht -Seconds 300 -Vsync -Frames
```

Im Paket (Mini-PC): Ordner `dist\k3c-game-go` und `dist\k3c-game` nebeneinander kopieren und aus `k3c-game-go` mit
`measure.ps1 -Engine rustgame ...` messen (das Skript findet `..\k3c-game\k3c-game.exe`).

Ergebnis: `reports\native-game-<rechner>-<zeit>.json` und `reports\native-rustgame-<rechner>-<zeit>.json` (je Lauf
`szene`, `lauf`, `fps_mittel`, `fps_1pct_low`, `frametime_ms`, `gpu_dediziert_mib_max`, `gpu_geteilt_mib_max`,
`ram_mib_max`, `start_ms`, `exe_mib`, mit `-Frames` das Feld `bilder`). Eine Zeilen-Datei allein auswerten:
`measure.ps1 -StatsOnly <datei>`.

- **Startzeit** = Prozessstart bis sichtbares Fenster (`MainWindowHandle` ungleich 0, Abfrage alle 50 ms), `start_ms`. Für
  das Gate zählt die Szene `tag` (Vorlauf 300 Takte); bei `nacht` zählt der Vorlauf von 15 600 Takten mit.
- **GPU-Speicher** = Höchstwert dediziert (WMI `GPUProcessMemory`), **Arbeitsspeicher** = Höchstwert `WorkingSet64`.
- Rust nimmt dieselben Schalter wie Go (`--seed k3c --players 2 --tick … --input … --measure --warmup --report --size`,
  `--novsync`, `--measure-frames`) und schreibt denselben Bericht (`launch.rs`, `scene.rs`, `measure.rs`).

## Schwellen (G1, B-1016, Vorschlag; Entscheidung 🧑)

| Kennzahl | Schwelle | Quelle der Messung |
|---|---|---|
| längstes Bild zu Median | höchstens 2 × Median | `bilder.max_zu_median`, Szene nacht, VSync |
| GPU-Speicher | höchstens 1330 MiB | `gpu_dediziert_mib_max` (und geteilt) |
| EXE-Größe | höchstens 40 MB | `exe_mib` |
| FPS (Mittel, 1-%-Low) | Rust nicht schlechter als Go | `fps_mittel`, `fps_1pct_low` |
| Startzeit | Rust nicht schlechter als Go | `start_ms`, Szene tag |
| Arbeitsspeicher | Rust nicht schlechter als Go | `ram_mib_max` |

Die 1-ms-Schwelle des Spike-Modus `level` entfällt. Aus G2 zusätzlich (`rust-ruckler.md`): `lang_programm` = 0.

## Gate-Werte (🧑, RU36.2; leer bis zur Messung)

Mittelwert über drei Läufe je Szene (ohne VSync), längstes Bild aus dem 300-s-Lauf mit VSync.

| Rechner | Spiel | Szene | FPS Mittel | FPS 1-%-Low | Startzeit (ms) | GPU ded. (MiB) | RAM (MiB) | EXE (MB) |
|---|---|---|---|---|---|---|---|---|
| Entwicklungs-PC | Go | tag | – | – | – | – | – | – |
| Entwicklungs-PC | Rust | tag | – | – | – | – | – | – |
| Entwicklungs-PC | Go | nacht | – | – | – | – | – | – |
| Entwicklungs-PC | Rust | nacht | – | – | – | – | – | – |

| Rechner | Spiel | Median (ms) | längstes Bild (ms) | längstes zu Median | lang_programm |
|---|---|---|---|---|---|
| Entwicklungs-PC | Go | – | – | – | – |
| Entwicklungs-PC | Rust | – | – | – | – |

Ergebnis AC-02 (Rust in keinem Wert schlechter als die Schwellen): – (offen, 🧑).

## Probelauf (RU36.1)

Offen: Der Hauptrechner führt keine Messläufe aus (Sperre 2026-11-11, B-742; parallele Sessions verfälschen die Werte).
`tools/measure/measure.ps1` ist gebaut und auf Syntax geprüft (PowerShell-Parser), die EXE wurden nicht gestartet. 🧑 macht den
Probelauf als ersten Schritt in RU36.2 (je Spiel `-Seconds 20`, ein Lauf), damit eine fehlerhafte Kette vor der langen
Messung auffällt:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File native\measure.ps1 -Engine game     -Seconds 20
powershell -NoProfile -ExecutionPolicy Bypass -File native\measure.ps1 -Engine rustgame -Seconds 20
```

Erwartung: je Spiel zwei Läufe (tag, nacht) mit `fps_mittel`, `gpu_dediziert_mib_max`, `ram_mib_max`, `start_ms` (größer 0)
und `exe_mib`. Zeigt `start_ms` den Wert -1, kam das Fenster nicht in der Zeit oder der Prozess endete vorher.
