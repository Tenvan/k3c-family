# B-1016 · Ein schlanker Rust-Client hält die Messwerte der Ebitengine (Spike, Gate G1)

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU2
- **Projekt:** RSU
- **Erstellt:** 2026-11-16
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Entscheidung 008 verglich Rust + Bevy mit Go + Ebitengine am selben Level: 399 gegen 5238 FPS, Kaltbau mehrere Minuten gegen 7,7 s, EXE 93,3 gegen 13,9 MB, Grafikspeicher 996 gegen 1330 MiB (B-776, B-777). Ein schlanker Rust-Stack ohne Bevy wurde nie gemessen.

## Ziel

Eine belegte Antwort auf die Frage: Hält ein schlanker Rust-Client die Werte der Ebitengine, und läuft er auch ohne Fenster?

## Beteiligte und Zielgruppen

🧑 entscheidet Gate G1 anhand des Berichts. Sprint RU2 liefert ihn.

## Anforderungen

- Gleiches Level, gleiche Sheets, gleicher PC und gleiche Messmodi wie B-777 › Notizen.
- Gegen das heutige `native/k3c-game` am selben Tag gegenmessen, nicht nur gegen die Zahlen aus 008.
- Schwellen G1 (Vorschlag): Frame-Zeit ≤ 1 ms im Modus `level` ohne VSync, längstes Bild über 5 Minuten höchstens das Doppelte des Medians, Grafikspeicher ≤ 1330 MiB, Kaltbau ≤ 120 s, Neubau nach Einzeiländerung ≤ 15 s, EXE ≤ 40 MB.
- Rig-Figur als Dreiecksnetz, Text mit Umlauten, OGG-Ton, ein Shader, zwei Gamepads, zwei Bildhälften.
- Lauf ohne Fenster und ohne Grafikkontext (Voraussetzung für B-1017).

## Nicht-Ziele

Spielregeln, Sim, Menüs. Der Prototyp wird nach G1 gelöscht und nicht weitergebaut.

## Regeln und Einschränkungen

Der Prototyp hat einen eigenen Workspace unter `rust/spike/` und eigene Abhängigkeiten; sie gelten nicht als freigegeben für das Spiel.

- **Bereich:** `rust/spike/**`, `docs/messung/rust-spike.md`
- **Abhängig von:** –
- **Braucht 🧑:** Entscheidung
- **Größe:** M
- **Sperre:** –

## Beispiele

Modus `level`: 2400 FPS, 900 MiB, Kaltbau 45 s → alle Schwellen erfüllt, Bericht empfiehlt „weiter“. Modus `level`: 600 FPS → Schwelle verfehlt, Gegenprobe mit `wgpu`; verfehlt auch sie, empfiehlt der Bericht den Abbruch.

## Ausnahme- und Fehlerfälle

Der Stack kann nicht ohne Fenster laufen → im Bericht als verfehlt führen und den Weg nennen (Schleife ohne Zeichen-Stack bauen). Zwei Gamepads werden nicht getrennt erkannt → verfehlt, ohne Ausweichen auf Tastatur.

## Akzeptanzkriterien

- **AC-01** `docs/messung/rust-spike.md` zeigt FPS, Frame-Zeit (Median, p99, p99,9, längstes Bild), Grafikspeicher, Kaltbau, Neubau und EXE-Größe für Rust und für `native/k3c-game` am selben PC.
- **AC-02** Der Bericht nennt je Schwelle erfüllt oder verfehlt.
- **AC-03** Ein Lauf ohne Fenster und ohne Grafikkontext ist belegt (Befehl und Ausgabe im Bericht).
- **AC-04** 🧑 hat G1 entschieden: weiter, Abbruch oder Gegenprobe.

## Offene Fragen

0. Messung auf dem schwächsten PC (Mini-PC) gehört zu G1, sobald das Gerät da ist (🧑, Gerät). 1. Sind die Schwellen richtig gesetzt (🧑)? 2. Messung zusätzlich am Mini-PC (🧑, Gerät)? 3. Reicht der Vergleich gegen `native/k3c-game`, oder soll der Ebitengine-Prototyp aus der Git-Historie zurückgeholt werden (Vorschlag: reicht)?

## Notizen

Zahlen aus 008 und B-777 › Notizen.
