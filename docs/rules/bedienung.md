# Regelwerk: Bedienung (Pause, Schriftgröße, Verbindung, Sprache)

Beschlossen von 🧑 am 2026-10-03 im Chat (Quelle: [`../fragenkatalog.md`](../fragenkatalog.md) › Beschlüsse, Q01, Q03, Q04, Q05).
**Bestätigt von 🧑 am 2026-10-03** im Workshop F1.4: alle Vorschläge aus F1.1 unverändert übernommen (Pause, Schrift für 2–4 Spieler und Nebeninfo, Verhalten nach der Frist, Latenz p95).
Je Regel: **Regel · Begründung · Verweis auf Code oder Daten**. Jede Regel gilt für 2–4 Spieler, auf einem Gerät (Split-Screen) und auf mehreren Geräten (online).
Reserviert bleiben (`CLAUDE.md`): **B** ist unbelegt, **View + Menu** gemeinsam heißt „zurück zur Landingpage“.

## 1. Pause

| Regel | Begründung | Verweis |
|---|---|---|
| **Taste:** Menu kurz drücken (losgelassen nach < 600 ms, ohne View) öffnet das Pause-Menü; Esc an der Tastatur. Menu zusammen mit View bleibt „zurück zur Landingpage“ und öffnet keine Pause. | Menu allein und die reservierte Kombi lassen sich nur über die Zeit trennen. | Aktion `pause` in `src/input/playerInput.ts` (heute ohne Szene), Pause-Szene in S5 |
| **Hält der Raum an?** Sitzen alle Spieler des Raums an **einem** Gerät (Couch-Raum), hält die Simulation des Raums an. Sind **mehrere Geräte** im Raum (online), pausiert nur das Gerät: Sein Menü öffnet sich, der Raum läuft weiter, die Monarchen dieses Geräts stehen still und sind geschützt (unverwundbar). | Online darf ein Gerät die anderen nicht anhalten; auf der Couch stört niemand. | Beschluss Q01 |
| **Wer darf pausieren?** Jeder Spieler eines Geräts; die Pause gilt dann für alle Spieler dieses Geräts (Split-Screen). | Ein Bildschirm, ein Menü; wer weg muss, soll nicht fragen müssen. | bestätigt (F1.4) |
| **Wer setzt fort?** Jeder Spieler des pausierten Geräts (Menu oder „Weiter“ im Menü). | Wie beim Pausieren; kein Warten auf eine bestimmte Person. | bestätigt (F1.4) |
| **Höchstdauer:** Couch-Raum **unbegrenzt**. Online schützt die lokale Pause die Monarchen höchstens **60 s** (wie die Frist bei Verbindungsverlust, § 3); danach bleibt das Menü offen, die Monarchen stehen weiter still, sind aber verwundbar. | Ohne Mitspieler an anderen Geräten schadet eine lange Pause niemandem; online soll die Pause kein dauerhafter Schutz in der Nacht sein. | bestätigt (F1.4); Frist = `WaitFor` in `engine/room/room.go` |

## 2. Schriftgröße

Alle Größen in px bezogen auf die Spielfläche **1920 × 1080** (`GAME_WIDTH`, `GAME_HEIGHT` in `src/core/constants.ts`), gemessen **nach** der Skalierung der Zelle des Layouts (`src/scenes/layout.ts`). **Pflicht-Info** sind Gold, Vorrat, HP, Warnungen und Meldungen in der Mitte; **Nebeninfo** sind Hinweise, Tastenhilfen und Debug-Zeilen. Kontrast Text zu Hintergrund **≥ 4,5 : 1** (beschlossen). Bei 3–4 Spielern wird Text gekürzt, nicht verkleinert (beschlossen).

| Layout | Ansicht je Spieler (px) | Pflicht-Info mindestens | Nebeninfo mindestens |
|---|---|---|---|
| 1 Spieler | 1920 × 1080 (Vollbild; mit Partner-Ansicht 1920 × 720) | **28 px** (beschlossen) | 24 px (bestätigt) |
| 2 Spieler | 1920 × 540 (zwei Streifen) | 28 px (bestätigt: volle Breite, Abstand zum TV wie beim Vollbild) | 24 px (bestätigt) |
| 3 Spieler | 960 × 540 oben, 1920 × 540 unten | **24 px** in den Vierteln (beschlossen), 28 px im breiten Streifen (bestätigt) | 20 px (bestätigt) |
| 4 Spieler | 960 × 540 (2 × 2) | **24 px** (beschlossen) | 20 px (bestätigt) |

**Prüfverfahren:** Für jede Textanzeige gilt Fontgröße × Skalierung der Zelle ≥ Mindestgröße der Tabelle; das prüft ein Test je Layout in S4 (B-106), die Lesbarkeit am TV nimmt 🧑 in der Abnahme von S4 per Sichtprüfung ab.
Heute liegen Hinweise und Info-Zeile bei 20 px (`src/scenes/HudScene.ts`), die Anpassung ist Teil von S4.

## 3. Verbindung

| Regel | Begründung | Verweis |
|---|---|---|
| **Reservierung 60 s:** Bricht die Verbindung eines Geräts ab, bleiben seine Plätze 60 s für dasselbe Gerät reserviert (Zustand `waiting`); kommt es binnen 60 s zurück, steuert es seine Monarchen weiter. | Kurze WLAN-Aussetzer kosten nichts. | `WaitFor` = 60 s in `engine/room/room.go`, `docs/protocol.md` › Wiederverbinden |
| **Monarch während der Frist, bei Tag und Nacht gleich:** unverwundbar und ausgeblendet, er steht still. | Ein getrennter Spieler soll nachts nicht schutzlos sterben; ausgeblendet, damit niemand auf ihn wartet. | Beschluss Q04 |
| **Nach der Frist:** Der Platz ist frei (Zustand `free`), ein anderes Gerät kann den Monarchen übernehmen; bis dahin bleibt er ausgeblendet und unverwundbar. | Der Raum bleibt für Neue offen, ohne dass ein verwaister Monarch stirbt. | „frei“ beschlossen (Q04); Verhalten bis zur Übernahme bestätigt (F1.4) |
| **Latenz Eingabe → Bild** im Heim-WLAN: Mittel **≤ 100 ms** (beschlossen), p95 **≤ 150 ms** (bestätigt in F1.4: Spielraum für einzelne WLAN-Spitzen bei 30 Hz). | Ohne Vorhersage des eigenen Monarchen spürt man mehr als 100 ms im Mittel. | Beschluss Q04 |
| **Messweg:** 🧑 bzw. der Agent der Session misst am Gerät mit dem Debug-Overlay (`src/scenes/debugOverlay.ts`) über mindestens 60 s Spiel mit 2 Spielern; die Anzeige der Latenz fehlt dort noch (B-181). | Messung dort, wo gespielt wird, nicht am Server. | B-181 |

## 4. Sprache

| Regel | Begründung | Verweis |
|---|---|---|
| **Deutsch und Englisch** (entschieden am 2026-10-03), Auswahl in den Optionen. | Familie spielt deutsch, ein späteres Release auf itch.io (Q21) braucht Englisch. | Beschluss Q05, B-172 (S5) |
| **Neue Texte nur aus der zentralen Textquelle,** hart kodierte Texte sind für neuen Code nicht mehr erlaubt; bestehende Texte ziehen mit B-172 in S5 um. | Zwei Sprachen lassen sich nur pflegen, wenn alle Texte an einem Ort stehen. | B-172 |
