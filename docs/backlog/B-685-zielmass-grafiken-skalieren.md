# B-685 · Grafiken bekommen ein Zielmaß in der Ausgabe und werden dort passend skaliert

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** GRJ
- **Erstellt:** 2026-10-31
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-31 🧑 Ralf im Chat (Q1 bis Q3 entschieden)

## Ausgangslage

Die Größe einer Grafik im Spiel ergibt sich heute aus der zugeordneten Datei und ihrem Ausschnitt. Wer eine Grafik mit anderer Auflösung zuordnet (32 statt 128 Pixel), ändert damit die Darstellungsgröße; die Grafik muss passend erzeugt werden. Ein Zielmaß je Slot und Stufe gibt es nicht.

## Ziel

Jeder Grafik-Slot hat ein Zielmaß in der Ausgabe (z. B. Gebäude X auf Stufe N belegt 300×200 Pixel). Die zugeordnete Grafik wird in dieses Maß passend skaliert; ob die Quelle 32 oder 128 Pixel hat, ist egal, die 128er werden feiner.

## Beteiligte und Zielgruppen

🧑 Ralf ordnet Grafiken zu und legt die Maße fest; Familie spielt; Sessions CLI, DEV setzen um.

## Anforderungen

- Zielmaß je Slot (und je Stufe, wo der Slot Stufen hat) als Daten (`data/`), Einheit **Units** (1 Unit = 32 px, Entscheidung 🧑); Vorbelegung aus den heutigen Maßen, damit die Darstellung ohne Änderung gleich bleibt.
- Der Client skaliert die Grafik (nach Ausschnitt) auf das Zielmaß; Seitenverhältnis: **einpassen** (Entscheidung 🧑), am Boden ausgerichtet, ohne Verzerrung.
- Pixelgrafik bleibt scharf bei ganzzahligen Faktoren; sonst Regel für die Skalierung festlegen (nearest/linear).
- Studio: Zielmaß je Slot und Stufe bearbeitbar, Vorschau (Bühne) skaliert gleich wie das Spiel; Chat-Aktion dafür (B-679).
- Hitboxen und Spiellogik bleiben unberührt (reine Darstellung); Leistung: keine Neuberechnung je Frame.

## Nicht-Ziele

Umfang zuerst nur **Gebäude je Stufe** (Entscheidung 🧑); Figuren, Umgebung und HUD folgen als eigenes Ticket.

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

keine. Entschieden von 🧑 am 2026-10-31: Q1 Units, Q2 zuerst Gebäude, Q3 einpassen.

## Notizen

2026-10-31 umgesetzt (Claude, Gebäude je Stufe), Abnahme AC-04 offen bei 🧑:

- Daten: `ziel` {w, h} in Units je Gebäude-Slot in `data/assets.json` (71 Slots: `hub:1–5`, `hub:camp:1–3`, `hub:2–5:2–3`, `wall:1–5`, `tower:1–5`, Hub-Gebäude `:1–5`, `ship:1–3`, Treppen), vorbelegt aus den 32-px-Dateien; Prüfung beim Laden in `tools/assetkit/target.go` (je Seite 0 bis 100). Regel: `docs/rules/materialien-gebaeude.md` § 3.4, Glossar „Zielmaß“. Protokoll und Spielstand unberührt.
- Client: `src/scenes/render/targetFit.ts` (einpassen, scharf nur bei ganzzahliger Vergrößerung, sonst Linear), `buildingSize.ts` (Schlüssel → Slot, aktive Zuordnung mit Ausschnitt), `buildingLook.ts` (einmal je Grafik nach dem Laden). Das Spiel lädt jetzt die **aktive Zuordnung** des Slots statt der festen Datei; Gerüste und Hintergrundmauer unverändert.
- Studio: Zielmaß-Feld im Inspektor der Assets-Seite (Gebäude-Slots), Bühne passt mit `fitTarget` ein, Binding `AssetsTargetSet`, Chat-Aktion `zielmass_setzen`.
- Tests: AC-01 `buildingSize.test.ts`, AC-02 `tests/buildingTargets.test.ts`, AC-03 `TargetField.test.tsx`, `target_test.go`, `chat_actions_target_test.go`.
- Hinweis für die Abnahme: `hub:5` hat im Arbeitsstand `burg-5-128.png` als Präferenz 1 (nicht committet); im Spiel erscheint sie damit 8 × 9 Units groß, weich verkleinert. Die Live-Vorschau im Studio und das Spiel lesen `data/assets.json` beim Bauen bzw. Neuladen der Seite (statischer Import).
