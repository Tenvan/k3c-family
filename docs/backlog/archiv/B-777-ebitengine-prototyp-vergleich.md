# B-777 · Ein Ebitengine-Prototyp zeigt dasselbe Level wie der Bevy-Prototyp und liefert vergleichbare Messwerte

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** NV1
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat-Auftrag (Ebitengine-Prototyp als Vergleich, im Agenten)

## Ausgangslage

Der Bevy-Prototyp (B-776, `native/k3c-proto`) läuft und ist auf dem Entwicklungs-PC gemessen. Für die Entscheidung in B-775 (Rust + Bevy oder Go + Ebitengine) fehlt der direkte Vergleich; mit Go könnte die vorhandene Simulation (`engine/sim`, 10,4k Zeilen) ohne Portierung im selben Programm laufen.

## Ziel

Ein Go + Ebitengine-Prototyp, der dasselbe Level zeigt und dasselbe misst wie der Bevy-Prototyp, damit beide auf demselben Rechner nebeneinander stehen.

## Beteiligte und Zielgruppen

🧑 Ralf (Messung am Mini-PC, Entscheidung), Claude (Bau, Referenzmessung).

## Anforderungen

- Eigenes Go-Modul `native/k3c-ebiten/`, Ebitengine-Version in `go.mod` festgelegt, kein cgo.
- Gleiches Wald-Level, gleiche Sheets, gleicher Split-Screen, gleiche Eingabe (zwei Gamepads, Tastatur-Ersatz) und gleiche Kommandozeile wie `native/k3c-proto`.
- Gleicher Messbericht (Feldnamen wie B-776), Lastfälle `level` und `alle`; im Lastfall `alle` liegen alle Sheets wirklich auf der GPU.
- `native/measure.ps1` misst wahlweise eine oder beide Engines in einem Bericht, mit Zeitlimit je Lauf.

## Nicht-Ziele

Keine Simulationslogik, kein Netz, kein HUD; kein Entity- oder Szenen-System (Entscheidung dafür später in B-775).

## Regeln und Einschränkungen

Wurzel-`go.mod` bleibt unberührt. Befehle nur über `task native:*`. Logging mit Emojis, Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen, kein `math/rand`.

## Beispiele

`task native:measure` → ein Bericht mit vier Läufen (Bevy und Ebitengine, je `level` und `alle`).

## Ausnahme- und Fehlerfälle

Sheet lässt sich nicht laden → Abbruch mit Fehlermeldung statt Hängen. Lauf überschreitet das Zeitlimit → Prozess wird beendet, der Bericht nennt den Lauf als fehlgeschlagen.

## Akzeptanzkriterien

- **AC-01** `task native:ebiten:build` baut ohne Fehler, `go vet` sauber.
- **AC-02** Das Fenster zeigt dasselbe Wald-Level im Split-Screen wie der Bevy-Prototyp (Screenshot-Vergleich).
- **AC-03** Zwei Gamepads bewegen je einen Monarchen; Tastatur-Ersatz geht.
- **AC-04** `task native:measure` schreibt einen Bericht mit beiden Engines und beiden Lastfällen.
- **AC-05** Vergleichswerte vom Entwicklungs-PC stehen unter Notizen.
- **AC-06** Vergleichswerte vom Mini-PC stehen unter Notizen (Abnahme 🧑).

## Offene Fragen

keine

## Notizen

### Vergleich Entwicklungs-PC (AC-05, 2026-11-11, NV1.2)

Rechner STIRA-PC (Desktop, Ryzen 7 5800X, RX 6700 XT), 1920×1080, ohne VSync, `--auto`, je 20 s nach 5 s Aufwärmen, `task native:measure:beide`. Bericht: `reports/native-proto-STIRA-PC-20261111-074146.json`. **Kein Mini-PC** (AC-06 offen).

| Lauf | Backend | FPS Mittel | 1-%-Low | p99 | GPU dediziert | RAM |
|---|---|---|---|---|---|---|
| Bevy 0.20.1 level | Vulkan | 399 | 226 | 4,0 ms | 224 MiB | 316 MiB |
| Bevy 0.20.1 alle | Vulkan | 364 | 202 | 4,3 ms | 996 MiB | 948 MiB |
| Ebitengine 2.10.6 level | DirectX 11 | 5238 | 1894 | 0,4 ms | 104 MiB | 168 MiB |
| Ebitengine 2.10.6 alle | DirectX 11 | 5146 | 1784 | 0,4 ms | 1330 MiB | 608 MiB |

Weitere Werte: EXE 13,9 MB (Bevy 93,3 MB), Bau kalt 7,7 s (Bevy erster Bau mehrere Minuten, danach ca. 3 min Release), Go-Code 1099 Zeilen (Bevy-Prototyp ca. 600 Zeilen Rust).

Einordnung:
- Beide sind für das Spiel weit schnell genug; der Abstand (Faktor ≈ 13) kommt vor allem daher, dass Ebitengine alle Sprites aus seinem Atlas in sehr wenigen Zeichenaufrufen zeichnet. Im unoptimierten Bevy-Prototyp ist nichts gebündelt. Kein Beleg, dass Bevy nicht schneller ginge.
- Ebitengine braucht mit allen Sheets ein Drittel mehr Grafikspeicher (Atlas, 1330 statt 996 MiB) und der Wert hängt von der Lade-Reihenfolge ab (1330–2234 MiB beobachtet); jetzt feste Reihenfolge, breiteste Streifen zuerst. Eigene Texturen je Sheet (`--unmanaged`) waren schlechter (2148 MiB, halbe FPS).
- Die Backends sind verschieden (Vulkan gegen DirectX 11). Kurzversuche: Ebitengine mit DirectX 12 nur 13 FPS (Ursache nicht untersucht), mit OpenGL schneller als DX11.
- Ladezeiten der beiden sind nicht vergleichbar (verschiedene Messpunkte).

### Offen

- AC-03: echte Gamepads und Tastatur von Hand nicht geprüft, nur Auto-Fahrt.
- Zeitlimit im Messskript eingebaut, aber nie ausgelöst; Skript nur mit Windows PowerShell 5.1 gelaufen, nicht mit pwsh 7.
- Nicht sichergestellt, dass das Fenster während der Messung vorne lag (gilt für beide Engines).
