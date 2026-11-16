# B-1019 · Die Ursache der Ruckler im Go-Spiel ist gemessen: GC, Bildtakt oder Laden (Gate G0)

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** RU0
- **Projekt:** RSU
- **Erstellt:** 2026-11-16
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-16 🧑 Chat (k3c-08)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Im Go-Spiel gibt es Ruckler, die trotz Beseitigungsversuchen bleiben (B-943: „ruckelt trotz stabiler 60 FPS, auch auf schnellen Rechnern“; Zwischenbilder in `native/k3c-game/game/blend.go` sind schon drin). 🧑 vermutet nicht steuerbare GC-Läufe und plant deshalb die Umstellung des Clients auf Rust (B-1014). Gemessen ist bisher: 47 % der Bilder ohne neuen Takt vor den Zwischenbildern (`blend.go`), Zahl der GC-Läufe und Summe der Pausen je Messung (`measure.go`). Nicht gemessen: ob ein langes Bild mit einem GC-Lauf zusammenfällt.

## Ziel

Belegt ist, was die Ruckler auslöst. Damit entscheidet 🧑 an G0, ob Rust das Problem trifft, bevor rund 200 Sessions Umstellung beginnen.

## Beteiligte und Zielgruppen

🧑 sieht die Läufe am PC und entscheidet G0. Sprint RU0 misst. Das Ergebnis nützt auch B-943 (Fix im Go-Spiel).

## Anforderungen

- Messung je Bild statt je Lauf: Zeitstempel, Dauer von Update und Draw, Takte in diesem Bild, alpha, GC-Lauf in diesem Bild.
- Die Messung selbst darf nicht stören: Puffer vorab anlegen, erst am Ende schreiben, kein `runtime.ReadMemStats` je Bild (hält die Welt an), dafür `runtime/metrics`.
- Vier Läufe derselben Szene (zwei Spieler, Nacht mit Gegnern, 5 Minuten): normal; GC aus; VSync an und aus; Takt aus eigener Zeitsammlung (`--pacing frame`).
- Je Lauf: Median, p99, p99,9, längstes Bild, Zahl der Bilder über dem 1,5-Fachen des Medians, Anteil davon mit GC, Anteil mit null oder zwei Takten.
- Prozess-Priorität und Energieschema je Lauf notieren (Entscheidung 006: Takt-Ausreißer kamen dort vom Scheduling).
- Sichtprüfung durch 🧑 zusätzlich zu den Zahlen.

## Nicht-Ziele

Den Fix (B-943). Rust-Code. Sim- oder Balancing-Läufe.

## Regeln und Einschränkungen

Die Versuchsschalter ändern das Standard-Verhalten nicht. Die Szene läuft über das Spiel selbst (`--seed`, `--tick`, `--input`), nicht über `sim_test`.

- **Bereich:** `native/k3c-game/game/measure*.go`, `native/k3c-game/game/pacing*.go`, `native/k3c-game/game/game.go`, `native/k3c-game/game/blend.go`, `native/k3c-game/main.go`, `native/measure.ps1`, `docs/messung/ruckler-go.md`
- **Abhängig von:** –
- **Braucht 🧑:** Entscheidung
- **Größe:** M
- **Sperre:** –

## Beispiele

Lauf „GC aus“ hat genauso viele lange Bilder wie „normal“, Lauf `--pacing frame` hat keine → Ursache Bildtakt; Rust ist dafür nicht nötig. Die langen Bilder fallen zu 90 % mit GC-Läufen zusammen und verschwinden mit „GC aus“ → Ursache GC; Welle 2 startet.

## Ausnahme- und Fehlerfälle

Kein Lauf ist ruckelfrei → der Bericht nennt, was übrig bleibt (Laden, Ton, Treiber, Scheduling), und schlägt den nächsten Versuch vor. Die Ruckler treten in 5 Minuten nicht auf → Szene und Dauer mit 🧑 abstimmen, nicht raten.

## Akzeptanzkriterien

- **AC-01** Das Go-Spiel schreibt mit `--measure-frames <datei>` je Bild eine Zeile mit den genannten Werten; ohne den Schalter ändert sich nichts.
- **AC-02** Die Läufe liegen als Berichte unter `.work/messung/`; die Kennzahlen stehen in `docs/messung/ruckler-go.md`.
- **AC-03** Der Bericht nennt die Ursachen mit Anteil; 🧑 hat die Läufe gesehen und G0 entschieden.

## Offene Fragen

1. RU0 berührt dieselben Dateien wie B-943 im aktiven Sprint FB4: zuerst RU0, dann der Fix (Vorschlag), oder beides in FB4 (🧑)? 2. Welche Szene zeigt die Ruckler am deutlichsten (🧑)? 3. Zusätzlich auf dem schwachen PC messen (🧑, Gerät)?

## Notizen

Verwandt: B-943 (Fix, Sprint FB4, Projekt NAT), B-793 (30-Hz-Takt), Entscheidung 006 › „Was die Erstmessung zur Vermutung sagt“ (Scheduling, EcoQoS), B-1020 (voller Lauf auf ruhendem PC).

### Nachtrag 2026-11-16 (k3c-08): Hinweis auf die Schleife

🧑: Die Ruckler sind immer gleich, egal wie ausgelastet der Rechner ist, auch auf einem schwächeren Test-Notebook. Vermutung 🧑: ein Problem der Schleife. Die Bilddateien aus RU0.1 (`.work/messung/*.jsonl`) stützen das:

- **Rund zwei späte Bilder je Sekunde, immer:** 241, 279 und 345 Bilder über 25 ms in je 120 s (leichte Szene, leichte Szene mit GC aus, Last-Szene), bei 0,9 bis 2,4 ms Draw. Die Zahl hängt kaum an der Last.
- **Nachhol-Bilder trotz VSync:** 315 bis 524 Bilder unter 10 ms je Lauf, etwa die Hälfte der späten Bilder ist direkt von einem kurzen gefolgt. Mit sauberem VSync dürfte es bei 60 Hz kein Bild unter 16 ms geben.
- **Späte Bilder sind kein Vielfaches von 16,7 ms:** Nur etwa ein Fünftel liegt nahe an einem Vielfachen. Die Bildausgabe läuft also nicht im Takt des Bildschirms, sie schwankt frei.
- **Die Zwischenbilder rechnen richtig, solange die Bilder pünktlich kommen** (kurzer Lauf mit Zwischenbildern, rund 35 s nutzbar): Der Schritt der gezeigten Sim-Zeit weicht im Median 3 % von der Bilddauer ab. Bei einem späten Bild stimmt er nicht mehr (Beispiel: Bild 48 ms, gezeigter Schritt 33 ms, danach zwei Bilder zu 2 ms).

Folgerung (vorläufig): Die Ruckler kommen aus dem Takt der Bildausgabe (Schleife von Ebitengine mit 30 Takten je Sekunde, VSync, Fenster-Modus), nicht aus GC und nicht aus der Last. Nicht geprüft und damit Kandidaten für RU0.2: `ebiten.SetTPS(ebiten.SyncWithFPS)` mit eigener Zeitsammlung (`--pacing frame`), Vollbild statt Fenster, DirectX 12 oder OpenGL statt DirectX 11 (`EBITENGINE_DIRECTX`, `EBITENGINE_GRAPHICS_LIBRARY`), alpha aus der Zeitsammlung statt aus der Wanduhr.

Korrektur zur Messung: Mit `--input` waren die Zwischenbilder bisher aus (`game.go`: `newBlend(a.Shots == "" && !a.Script)`); die fünf Läufe aus RU0.1 zeigen deshalb jedes zweite Bild ohne Bewegung und beschreiben das Spiel ohne Zwischenbilder. Die Aussagen zu GC und zu den späten Bildern sind davon nicht betroffen. Seit dem Nachtrag bleiben die Zwischenbilder im Messmodus an.
