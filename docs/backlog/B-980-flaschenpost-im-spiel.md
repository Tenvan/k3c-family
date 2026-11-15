# B-980 · Das Spiel zeigt Flaschenpost, Richtungspfeil und leuchtendes Ziel in beiden Spielerhälften

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** GS4
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

B-971 liefert die Flaschenpost in der Simulation: Flasche am Strand, Ereignisse `bottleFound`, `bottleLost`, `bottleTargetReached` und im Zustand jeder Stufe Flasche und Markierung (Zielart, Stufe, Ort). Das Spiel zeichnet davon noch nichts. Level-Objekte zeichnet `native/k3c-game/props/` und `world/`, das HUD je Spielerhälfte `native/k3c-game/hud/` (Radar in `radar.go`, Meldungen in `banner.go`, Geschichtszeile aus B-974). Abgespalten aus B-971 (🧑 2026-11-15, Markierung).

## Ziel

Die Flasche glitzert am Strand, nach dem Aufheben blendet ihre Geschichtszeile ein, und in beiden Spielerhälften führen ein Pfeil am Bildrand und ein leuchtendes Ziel ohne Text zum Ort.

## Beteiligte und Zielgruppen

Spielende am TV (1–2 Monarchen im Split-Screen). Claude legt das Asset über `assets_*` an und setzt um, 🧑 gibt frei und nimmt am TV ab.

## Anforderungen

Aus B-971 (🧑 2026-11-15, wörtlich):

- **Hinweis: Markierung in der Welt.** Die Flasche zeigt einen Ort. Ohne Text verständlich.
- **Erscheinen:** Sie glitzert, der Monarch hebt sie im Vorbeireiten auf; bleibt sie liegen, holt die Flut sie am Abend.
- **Markierung: Leuchtspur für beide, bis gefunden.** Nach dem Aufheben blendet die Geschichtszeile kurz ein (B-965), danach zeigt ein dezenter Pfeil am Bildrand beider Spielerhälften die Richtung, am Ziel leuchtet der Ort. Die Markierung bleibt, bis ein Monarch dort war; immer nur eine zur Zeit.
- Funktioniert mit 2 Spielern gleichzeitig. Asset über `assets_*`.

Ergänzt:

- **R1 Flasche:** Papierschnitt-Asset „Flaschenpost“ (Gruppe `objekte`), am Ort aus dem Zustand gezeichnet, mit Glitzern (wiederkehrendes Aufblitzen, rein aus der Uhr). Sie erscheint auch als Marker im Radar der Stufe 1.
- **R2 Aufheben und Flut:** `bottleFound` → Flasche verschwindet, Geschichtszeile blendet in allen Spielerhälften ein (Einblendung aus B-974, Zeilen aus dem Ereignis). `bottleLost` → Flasche verschwindet ohne Meldung.
- **R3 Pfeil:** Solange eine Markierung läuft, zeigt jede Spielerhälfte einen dezenten Pfeil am Bildrand: Ziel in der eigenen Stufe links oder rechts außerhalb des Bildes → Pfeil am linken oder rechten Rand; Ziel in einer tieferen oder höheren Stufe → Pfeil schräg nach unten oder oben in Richtung des nächsten Übergangs (Tiefen-Eingang oder Treppe). Ist das Ziel im Bild, entfällt der Pfeil.
- **R4 Leuchten:** Das Ziel leuchtet in der Welt (weicher Schein um den Ort), sichtbar für jeden Spieler in dieser Stufe; im Radar steht ein Marker.
- **R5 Ende:** `bottleTargetReached` → Pfeil und Leuchten erlöschen in allen Hälften im selben Frame.
- **R6 Ohne Text:** Pfeil, Leuchten und Radar-Marker kommen ohne Schrift aus; eine Meldung in der Meldungs-Zeile gibt es nicht.
- **R7 Nur zeichnen:** Richtung und Sichtbarkeit rechnet das Spiel aus Zustand und Kamera; es ändert keinen Zustand der Sim. Keine Allokation je Frame.
- **R8 Laden:** Nach dem Laden eines Stands mit Flasche oder Markierung erscheinen sie sofort wieder, ohne erneute Geschichtszeile.

## Nicht-Ziele

Regeln und Simulation der Flaschenpost (B-971), Ton (eigenes Ticket bei Bedarf), Hinweis auf der Seekarte, Text am Ziel, Darstellung von Einsiedler und Gatter (B-972, B-970).

## Regeln und Einschränkungen

- Entscheidung 005: Asset nur über `assets_*`, Fachprüfung `k3c-grafik`; HUD-Prüfung `k3c-hud`.
- Gen-Läufe (`gen:paper`, Grafik-Golden) am Feature bzw. im Abnahme-Ticket (B-742).

- **Bereich:** `public/grafik/objekte/**`, `data/paper.json`, `native/k3c-game/props/**`, `native/k3c-game/world/**`, `native/k3c-game/hud/**`, `native/k3c-game/view/**`, `native/k3c-game/fx/**`
- **Abhängig von:** B-971, B-974
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Sengoku, Tag 5, Tagesanbruch: Am linken Inselende glitzert eine Flasche. Monarch 1 reitet darüber, eine Zeile Kunstsprache blendet in beiden Hälften ein, danach zeigt in beiden Hälften ein Pfeil nach rechts unten. Monarch 2 steigt in Stufe 2 ab, sein Pfeil zeigt nun nach rechts; die Truhe leuchtet, er reitet hin, Pfeil und Leuchten erlöschen bei beiden.

## Ausnahme- und Fehlerfälle

- Asset nicht gerendert → Flasche als einfacher Platzhalter (wie fehlende Objekte heute), Pfeil und Leuchten funktionieren.
- Beide Spieler in verschiedenen Stufen → jede Hälfte zeigt die Richtung aus ihrer Stufe.
- Ziel direkt im Bild beim Aufheben → kein Pfeil, nur Leuchten.
- Overlay (Seekarte, Skill-Menü) offen → Pfeil wird vom Overlay verdeckt, nicht darüber gezeichnet.
- Spieler tot oder ohne Krone → seine Hälfte zeigt Pfeil und Leuchten weiter.

## Akzeptanzkriterien

- **AC-01** Das Asset „Flaschenpost“ ist über `assets_*` angelegt, in `data/paper.json` zugeordnet und im Review der Workbench sichtbar.
- **AC-02** Go-Test: Aus dem Zustand mit Flasche zeichnet die Stufe 1 die Flasche am Ort und das Radar einen Marker; nach `bottleFound` oder `bottleLost` nicht mehr.
- **AC-03** Go-Test in `native/k3c-game/hud`: `bottleFound` reiht die Zeilen des Ereignisses in allen Spielerhälften in die Geschichtszeile ein; ohne Zeilen erscheint nichts.
- **AC-04** Go-Test: Pfeilrichtung für die Fälle Ziel links, rechts, tiefer, höher und im Bild (kein Pfeil); zwei Spieler in verschiedenen Stufen bekommen je ihre Richtung.
- **AC-05** Go-Test: `bottleTargetReached` löscht Pfeil und Leuchten in allen Hälften; ein geladener Zustand mit Markierung zeigt sie ohne Geschichtszeile; keine Allokation je Frame.
- **AC-06** TV (🧑): Zu zweit eine Flasche aufheben und dem Pfeil folgen: Flasche fällt auf, Pfeil ist dezent, das Ziel ist ohne Text zu finden.

## Offene Fragen

- Pfeil bei Ziel in anderer Stufe → Default: schräg nach unten oder oben in Richtung des nächsten Übergangs (wie „Pfeil nach rechts unten“ im Beispiel von 🧑) (vorläufig, 🧑 bestätigt).
- Radar-Marker für Flasche und Ziel → Default: ja, klein und ohne Text (vorläufig, Abstimmung am TV).
- Ton beim Aufheben → Default: keiner in diesem Ticket (vorläufig).

## Notizen

Abgespalten aus B-971 (2026-11-15, Zerlegung). Zwei Sessions: Asset (PAP), Darstellung (CLI).
