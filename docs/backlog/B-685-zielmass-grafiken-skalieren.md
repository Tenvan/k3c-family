# B-685 · Grafiken bekommen ein Zielmaß in der Ausgabe und werden dort passend skaliert

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** GRJ
- **Erstellt:** 2026-10-31
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** – (offene Fragen, entscheidet 🧑)

## Ausgangslage

Die Größe einer Grafik im Spiel ergibt sich heute aus der zugeordneten Datei und ihrem Ausschnitt. Wer eine Grafik mit anderer Auflösung zuordnet (32 statt 128 Pixel), ändert damit die Darstellungsgröße; die Grafik muss passend erzeugt werden. Ein Zielmaß je Slot und Stufe gibt es nicht.

## Ziel

Jeder Grafik-Slot hat ein Zielmaß in der Ausgabe (z. B. Gebäude X auf Stufe N belegt 300×200 Pixel). Die zugeordnete Grafik wird in dieses Maß passend skaliert; ob die Quelle 32 oder 128 Pixel hat, ist egal, die 128er werden feiner.

## Beteiligte und Zielgruppen

🧑 Ralf ordnet Grafiken zu und legt die Maße fest; Familie spielt; Sessions CLI, DEV setzen um.

## Anforderungen

- Zielmaß je Slot (und je Stufe, wo der Slot Stufen hat) als Daten (`data/`), Einheit Pixel in der Ausgabe oder Units; Vorbelegung aus den heutigen Maßen, damit die Darstellung ohne Änderung gleich bleibt.
- Der Client skaliert die Grafik (nach Ausschnitt) auf das Zielmaß; Seitenverhältnis-Regel (einpassen, füllen, strecken) je Slot wählbar, Standard einpassen am Boden ausgerichtet.
- Pixelgrafik bleibt scharf bei ganzzahligen Faktoren; sonst Regel für die Skalierung festlegen (nearest/linear).
- Studio: Zielmaß je Slot und Stufe bearbeitbar, Vorschau (Bühne) skaliert gleich wie das Spiel; Chat-Aktion dafür (B-679).
- Hitboxen und Spiellogik bleiben unberührt (reine Darstellung); Leistung: keine Neuberechnung je Frame.

## Nicht-Ziele

Neue Grafiken erzeugen oder Auflösung konvertieren; Animationen neu schneiden.

## Regeln und Einschränkungen

Werte gehören in `data/`; Protokoll und Spielstand nur ändern, wenn nötig (Version erhöhen); 2 Spieler; Atlas-Pipeline (`tools/atlas`) beachten.

## Beispiele

Slot „tower“ Stufe 3, Zielmaß 300×200: 32-px-Quelle wird gröber, 128-px-Quelle feiner dargestellt, beide belegen 300×200.

## Ausnahme- und Fehlerfälle

Zielmaß 0 oder fehlend → heutiges Maß. Quelle mit anderem Seitenverhältnis → Seitenverhältnis-Regel des Slots.

## Akzeptanzkriterien

- **AC-01** Vitest: Grafiken unterschiedlicher Auflösung belegen dasselbe Zielmaß.
- **AC-02** Daten: Zielmaße vorbelegt, Darstellung ohne Änderung (Vergleichstest).
- **AC-03** Studio bearbeitet das Zielmaß, die Vorschau entspricht dem Spiel.
- **AC-04** Abnahme durch 🧑 im Spiel und im Studio.

## Offene Fragen

- Q1 (🧑): Einheit des Zielmaßes: Pixel der Ausgabe oder Units (1 Unit = 32 px)? Empfehlung Claude: Units, damit es mit Kamera-Zoom und Auflösung stimmt.
- Q2 (🧑): Gilt das Zielmaß für alle Slots (Figuren, Gebäude, Umgebung, HUD) oder zuerst nur für Gebäude je Stufe? Empfehlung Claude: zuerst Gebäude, dann Rest.
- Q3 (🧑): Standard bei abweichendem Seitenverhältnis: einpassen (Empfehlung) oder füllen?

## Notizen

–
