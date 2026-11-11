# 008 · Natives Spiel in Go + Ebitengine, Rust entfällt

- **Status:** angenommen (Ralf, gemeldet im Chat am 2026-11-11; die Entscheidung fiel vorher auf einem anderen Rechner und war nicht im Repo angekommen)
- **Datum:** 2026-11-11
- **Projekt:** NAT · Tickets: B-775, B-776, B-777
- **Vorgänger:** 001 (Engine auf Go-Server), 005 (Papierschnitt-Grafik), 006 (Rust-Engine), 007 (Rust-Server, Entwurf)

## Anlass

Projekt NAT bringt K3C als natives Windows-Spiel auf den Mini-PC: lokal, 1–2 Spieler, ohne Server und Browser (B-775).
Sprint NV1 hat zwei Prototypen desselben Levels gebaut und gemessen: Rust + Bevy (`native/k3c-proto`, B-776) und
Go + Ebitengine (`native/k3c-ebiten`, B-777).

## Entscheidung

1. **Das native Spiel entsteht in Go + Ebitengine, als lokal laufender Monolith:** ein Prozess, ohne Server und ohne Netz;
   Simulation, Zeichnen, Eingabe und Spielstände liegen im selben Programm. Bevy wird nicht weiterverfolgt.
2. **Rust entfällt komplett.** Es gibt keinen Rust-Kern und keinen Rust-Server mehr; die Lösung ist rein Go.
   Entscheidung 006 (Simulationskern in Rust) und der Entwurf 007 (Rust-Server) sind damit überholt, Projekt RST ruht
   und wird mit dem Aufräum-Ticket B-779 geschlossen. Die Rust-Tickets B-725 und B-736 sind verworfen (Ralf, 2026-11-11).
3. **Die Go-Simulation bleibt, wie sie ist** (`engine/sim`, `engine/level`, `engine/rng`, Goldens). Das native Spiel bindet
   sie im selben Prozess ein; eine Portierung der Regeln entfällt. Entscheidung 001 gilt für das Online-Spiel im Browser
   weiter, bis NAT es ablöst.
4. **Nächster Schritt ist ein dritter Prototyp** (B-778, Sprint NV2): nur der Monarch, zur Laufzeit aus Rig-Teilen
   zusammengesetzt statt aus vorgerenderten Sheets.
5. **Zielbild sind Rig-Modelle statt PNG-Sheets, zuerst Monarch und Reittier** (vorbehaltlich der Prototypen): Die Leistung
   der Go-Engine reicht dafür mehr als aus; Monarch, Reittier, Rüstung und Waffe werden frei kombinierbar, was mit Sheets
   praktisch unmöglich ist. Waffen und Rüstungen kommen später als optionale Elemente bei Monarch, Gegnern und Bossen dazu;
   das Rig-Modell sieht dafür von Anfang an Plätze vor. Rigs können die Zahl gleichzeitiger Gegner begrenzen; das klären
   weitere Prototypen (B-780).
6. **Das Gen-Skript bleibt vorerst** (`gen:paper`) und schrumpft Figur für Figur: Eine Figur verlässt es erst, wenn ihr Rig
   im Spiel läuft. Für Rig-Figuren liegt alles Relevante im Repo.

## Gründe

- **Leistung im Spiel:** Entwicklungs-PC (RX 6700 XT), gleiches Level, gleicher Messlauf: Ebitengine 5238 FPS (`level`) und
  5146 FPS (`alle`), Bevy 399 und 364 FPS (B-777 › Notizen).
- **Kompilierung:** Ebitengine baut kalt in 7,7 s, Bevy beim ersten Mal mehrere Minuten und danach etwa 3 min im Release.
  EXE 13,9 MB statt 93,3 MB.
- **Kein Neubau der Regeln:** 10,4k Zeilen Simulation und 33k Zeilen Go-Tests bleiben gültig; die Schätzung sinkt von
  4–6 Wochen (Bevy) auf 2–4 Wochen.

## Folgen

- B-775 und Projekt NAT beschreiben den Weg mit Ebitengine (Revision 2).
- `rust/`, `native/k3c-proto`, die FFI-Tests und die Rust-Schritte in Taskfile und CI werden entfernt (B-779).
- Risiko: Ebitengine brauchte mit allen Sheets mehr Grafikspeicher als Bevy (1330 statt 996 MiB). Der Monarch belegt
  363 von 580 MiB RGBA aller Sheets; der Rig-Prototyp (B-778) zielt genau auf diesen Anteil.
- Die Messung am Mini-PC steht noch aus (B-776/AC-06, B-777/AC-06); sie ändert die Entscheidung nur, wenn Ebitengine dort
  unbrauchbar wäre.
