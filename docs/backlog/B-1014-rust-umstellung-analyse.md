# B-1014 · Spiel und Simulation laufen komplett in Rust (Analyse, Empfehlung, Gates)

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

Anlass (🧑 2026-11-16): Im Go-Spiel gibt es immer wieder Ruckler, die trotz Beseitigungsversuchen bleiben (B-943, offen); vermutet sind nicht steuerbare GC-Läufe. Dazu soll das Spiel auf schwächeren PCs weniger einbrechen. Es geht zuerst um die Leistung des Clients; Sim und Balancing rücken nach hinten.

Stand 2026-11-16, gezählt (Code / Tests in Zeilen): `engine/sim` 12 506 / 17 200, `engine/level` 2 206 / 2 019, `engine/room` 2 919 / 4 419, `engine/store` 689 / 694, `engine/balance` 7 342 / 3 518, `engine/simcli` 1 571 / 606, `native/k3c-game` 25 413 / 17 034 (192 Dateien), `native/k3c-rig` 3 080 / 524. Zusammen rund 56 000 Zeilen Code und 46 000 Zeilen Tests in Go. Das Spiel importiert `engine/sim` in 57 Dateien direkt; 16 von 17 Paketen hängen an Ebitengine. Rust war zweimal geplant (006, 007) und wurde am 2026-11-11 mit 008 verworfen.

## Ziel

🧑 kann über die Umstellung entscheiden: mit belegten Gründen dafür und dagegen, einer Empfehlung, Gates mit Abbruchkriterien und einem Rückfallplan. Bei „go“ laufen am Ende Spiel und Simulation als ein Rust-Programm, ruckelfrei; Go bleibt nur für Workbench und Werkzeuge.

## Beteiligte und Zielgruppen

🧑 entscheidet (go, no-go, Gates, Crates, Rang). Agenten setzen um. Spieler merken im besten Fall nur, dass es nicht mehr ruckelt.

## Anforderungen

- **Zuerst die Ursache:** RU0 misst im Go-Spiel, was die Ruckler auslöst (GC, Bildtakt, Laden), bevor Rust-Code entsteht.
- **Client zuerst:** Bis G2 zeichnet der Rust-Client aus Aufzeichnungen des Go-Spiels (RU38); die Sim wird erst danach portiert, Balancing und die Läufe zuletzt.
- Port ohne Regel-, Wert- und Formatänderung. Die 23 Golden unter `testdata/golden/` sind das Orakel und werden nie zugunsten von Rust neu geschrieben.
- Die Sim bleibt deterministisch: gewürfelt wird nur über den Port von `engine/rng`; Gleitkomma-Funktionen liefern dieselben Bits wie Go (RU3).
- Zwei Spieler gleichzeitig mit Split-Screen und eigener Eingabe; B geht in Menüs zurück.
- Sim-, Balancing- und Level-Läufe kommen aus der Spiel-EXE, headless oder mit Fenster (B-1017); die Schleife ist dafür von Anfang an vom Fenster getrennt (RU4).
- Das Go-Spiel bleibt bis G4 unverändert lauffähig (Rückfall).
- Je Gate ein Bericht mit Messwerten unter `docs/messung/`; 🧑 entscheidet über weiter oder Abbruch.
- Komplexitäts-Budget auch in Rust: Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Jede Port-Session folgt dem Leitfaden (B-1018).

## Nicht-Ziele

Workbench, `assetkit`, `tools/paper`, `checkdiff`, `cleanup`, `ruletable`, `soundatlas` nach Rust. Neue Spielregeln. Online-Spiel. Bevy (008). Ein ausgeliefertes Spiel mit Brücke zwischen Go und Rust.

## Regeln und Einschränkungen

Dieses Ticket ist die Entscheidungsvorlage (Größe S); die Umsetzung steht in den Sprints RU0 bis RU41. Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (Vorschlag in den Notizen; hier nicht eingetragen). Sim-Sperre vom 2026-11-11 gilt weiter. Neue Abhängigkeiten nur mit Freigabe 🧑 je Crate.

- **Bereich:** `rust/**`
- **Abhängig von:** –
- **Braucht 🧑:** Entscheidung
- **Größe:** S
- **Sperre:** deps

## Beispiele

RU0 zeigt: Mit „GC aus“ ruckelt es genauso, mit `--pacing frame` nicht → Ursache ist der Bildtakt; der Fix gehört zu B-943, und 🧑 entscheidet, ob Rust dann noch gewollt ist. RU0 zeigt: Die langen Bilder fallen mit GC-Läufen zusammen → Welle 2 (RU1, RU2) startet.

## Ausnahme- und Fehlerfälle

Ein Gate wird verfehlt → Abbruch: `rust/` und die Rust-Schritte in Taskfile und CI werden entfernt (eine Session, Muster B-779); der Go-Stand ist unberührt. Eine Regel ändert sich in Go, nachdem ihr Bereich portiert ist → Nachzieh-Session in Rust oder Feature-Stopp (B-1015).

## Akzeptanzkriterien

- **AC-01** 🧑 hat an G0 entschieden, ob Rust nach der Messung weiterverfolgt wird; die Entscheidung steht in den Notizen des Projekts RSU.
- **AC-02** Bei go: Entscheidung 011 ist von 🧑 angenommen und löst 008 ab; die Crate-Liste ist freigegeben.
- **AC-03** Jedes Gate G0 bis G4 hat einen Bericht unter `docs/messung/` und eine Entscheidung von 🧑.
- **AC-04** Nach RU41 laufen `task build` und `task check:all` ohne `engine/sim` und `native/k3c-game`.

## Offene Fragen

1. **RU0 freigeben (🧑)?** Drei Sessions im Go-Spiel, unabhängig von Rust nützlich. RU0 berührt dieselben Dateien wie B-943 im aktiven Sprint FB4; welcher läuft zuerst?
2. **Was heißt „Sim nach hinten“ (🧑)?** Der Plan liest es so: Der Sim-Kern kommt nach G2 (ohne ihn ist das Rust-Spiel nicht spielbar), Balancing und die Läufe im Client ganz zuletzt nach G4. Wenn nur die Läufe gemeint waren, ändert sich nichts.
3. **Stack und Crates (🧑):** Vorschlag in den Notizen; je Crate eine Freigabe, eingetragen als Abschnitt „Freigegebene Crates“ in diesen Notizen (RU1.1 liest ihn und hält sonst an).
4. **Ausnahme von der Sim-Sperre (🧑):** Golden-Vergleiche und Aufzeichnungen sind Sim-Läufe als Tests (RU6, RU10, RU22, RU38). Ohne die Ausnahme ist der Port nicht prüfbar.
5. **Parallelphase (🧑):** bis G2 kein Stopp, ab G2 Feature-Stopp für SIM und CLI bis G4 (Vorschlag, B-1015); KSY vorher abschließen oder ruhen lassen.
6. **Zwei Sims nach G4 (🧑):** Zwischen G4 und RU41 rechnet das Spiel mit der Rust-Sim, Balancing noch mit der Go-Sim. Bis RU40 bleibt Balancing deshalb gesperrt; einverstanden?
7. **Rang von RSU (🧑):** angelegt mit 12.
8. **Schwacher PC (🧑, Gerät):** Welches Gerät ist die Messlatte für G1 und G2 (Mini-PC)?

## Notizen

### Empfehlung

**RU0 sofort freigeben (3 Sessions, im Go-Spiel). Über Rust an G0 entscheiden, über die volle Umstellung erst an G2.**

1. **Die GC-Vermutung ist ungeprüft.** Der Messmodus zählt GC-Läufe und die Summe der Pausen (`native/k3c-game/game/measure.go`), ordnet sie aber keinem Bild zu. Ob ein langes Bild mit einem GC-Lauf zusammenfällt, weiß niemand.
2. **B-943 zeigt in eine andere Richtung:** Ruckler bei stabilen 60 FPS, auch auf schnellen Rechnern. Das spricht gegen „zu wenig Leistung“ und für den Bildtakt: Die Sim taktet mit 30 Hz, gezeichnet wird mit 60 Hz und mehr, dazwischen schieben Zwischenbilder die Lage mit einem alpha aus der Wanduhr (`blend.go`). Ebitengine ruft Update dabei unregelmäßig (kein, ein oder zwei Takte je Bild).
3. **Ist der Bildtakt die Ursache, behebt Rust nichts von selbst:** Ein Port mit derselben Schleife ruckelt genauso. Umgekehrt lässt sich der Bildtakt in Go in wenigen Sessions richten (B-943).
4. **Ist GC die Ursache, hilft Rust wirklich:** keine Speicherbereinigung, volle Kontrolle über jedes Bild. Dann ist auch der Client-zuerst-Weg richtig.
5. **Schwächere PCs:** Im Repo gibt es dazu keine Messung (Mini-PC steht aus). Dass Rust dort weniger einbricht, ist ungeprüft; G1 und G2 messen auf dem schwächsten Gerät.

RU0 kostet 3 Sessions und entscheidet über bis zu 208. Der Lauf „GC aus“ und der Versuchsschalter `--pacing frame` trennen die beiden Ursachen sauber.

### Reihenfolge „Client zuerst“

- Bis G2 zeichnet der Rust-Client aus **Aufzeichnungen** des Go-Spiels (Level, Zustand je Takt, Ereignisse; RU38). Dafür braucht er die Zustands-Typen (RU8), nicht die Regeln. Nach 69 Sessions steht fest, ob er bei echter Last ruckelfrei läuft.
- Spielbar wird das Rust-Spiel erst mit dem Sim-Kern (Block C, nach G2). Ein ausgeliefertes Spiel aus Rust-Client und Go-Sim bleibt abgelehnt (siehe Alternativen).
- Balancing und die Läufe im Client kommen nach G4. Folge: Zwischen G4 und RU41 gibt es zwei Sims; Balancing bleibt so lange gesperrt.

### Gründe dafür

- Keine Speicherbereinigung, volle Kontrolle über Schleife, Speicher und jedes Bild. Das ist der Kern von 🧑s Begründung; ob es die Ruckler trifft, misst RU0.
- Das Typsystem (Summentypen, erschöpfendes `match`, kein `nil`) passt gut zu Regelcode.
- Die Zahlen aus 008 sprechen gegen Bevy, nicht gegen Rust: Ein schlanker Stack ist nie gemessen worden (RU2).
- Bevy brauchte weniger Grafikspeicher (996 statt 1330 MiB).

### Gründe dagegen

- **Umfang:** 56 000 Zeilen Code und 46 000 Zeilen Tests. Kein mechanischer Port: Der Sim-Zustand ist in Go ein Geflecht aus Zeigern (RU8); im Client wird jeder Ebitengine-Aufruf neu abgebildet.
- **Mittlere Leistung ist kein Problem:** 5238 FPS im Messlevel, Sim-Schritt 12–21 µs, Kaltbau 7,7 s (008, 006).
- **Kein Ersatz für Ebitengine aus einer Hand:** Fenster, Zeichnen, Text, Vektorpfade, Ton und Gamepad kommen in Rust aus mehreren Crates.
- **Determinismus:** `math.Pow` und `math.Exp` rechnen in Go im eigenen Code, Rust ruft die C-Laufzeit (Abweichung in der letzten Stelle möglich, ungeprüft); `math.Exp` läuft in jedem Schritt (`economy.go:61`). `math.Max` und `math.Min` behandeln NaN anders als Rust. Vier Sortierungen ohne Stabilität sind auf Gleichstand zu prüfen.
- **Zwei Sprachen bleiben:** Workbench (34 590 Zeilen Go, rund 28 500 Zeilen TypeScript), `assetkit` und `paper` bleiben.
- **Parallelphase:** 36 aktive Sprints, davon 20 offene autonome Sessions in SIM und CLI; KSY baut gerade den Kampf um.
- **Bauzeiten:** Rust baut langsamer als Go; wie viel beim schlanken Stack, ist ungeprüft (RU2 misst).

### Alternativen

| Variante | Urteil |
|---|---|
| A · Ruckler in Go beheben (B-943 nach RU0) | **Zuerst versuchen.** Wenige Sessions; trifft die Ursache, wenn es der Bildtakt ist. |
| B · Nur Sim in Rust, Client bleibt Go | **Abgelehnt.** Trifft das Problem nicht (Client) und RS1.6 maß die Brücke 4–8× langsamer als Go. |
| C · Rust-Client mit Go-Sim als Produkt | **Abgelehnt.** Die Go-Sim als DLL braucht cgo und einen C-Compiler (lokal nicht vorhanden); der Zustand müsste je Takt über die Grenze. Als Reihenfolge der Entwicklung mit Aufzeichnungen ist es der Plan bis G2. |
| D · Komplett, Client zuerst (dieser Plan) | Einzige stimmige Rust-Variante: am Ende ein Prozess, keine Brücke. Rund 208 Sessions, Ausstieg an fünf Gates. |

### Stack-Vorschlag (ungeprüft bis RU2; je Crate Freigabe 🧑)

- Fenster und Zeichnen: `macroquad` (schlank, kurze Bauzeit). Gegenprobe `wgpu` mit `winit` und eigenem Sprite-Sammler, falls G1 verfehlt wird. Bevy nicht (008).
- Gamepad: `gilrs` (XInput). Ton: `kira`. Text: Bordmittel des Zeichen-Stacks, sonst `fontdue`.
- Daten: `serde`, `serde_json`, `flate2` (Golden als `.json.gz`). rng: eigener Port ohne Crate.
- Aufteilung: Workspace `rust/` mit `k3c-rng`, `k3c-det`, `k3c-data`, `k3c-level`, `k3c-sim`, `k3c-room`, `k3c-balance`, `k3c-rig`, `k3c-game`. Jede Crate ist ein eigener Bereich für die Wellen.

### Vorschlag Entscheidung 011 (nicht eingetragen)

„011 · Spiel und Simulation in Rust, Go bleibt für Werkzeuge. Löst 008 Punkte 1 bis 3 ab, sobald G1 und G2 bestanden sind und 🧑 zustimmt. (1) Ein Rust-Programm `k3c-game` rechnet die Sim im selben Prozess. (2) Sim- und Balancing-Läufe kommen aus derselben EXE, headless oder mit Fenster; die Workbench ruft sie auf. (3) Die Golden bleiben das Orakel. (4) Der Go-Stand bleibt bis G4 der Rückfall. (5) Die Workbench bleibt Go. 008 Punkte 4 bis 6 (Rig-Zielbild, Gen-Skript) gelten weiter.“

### Quellen

006 (Messungen RS1.1 bis RS1.6), 007, 008, 009, B-776, B-777, B-924, B-925, B-943; `native/k3c-game/game/game.go` (`TPS = 30`), `blend.go`, `measure.go`. Zeilen gezählt mit `wc -l` am 2026-11-16 auf `sprint/planung-w1` (dc44f63eb).
