# B-974 · Das Spiel blendet die Geschichtszeilen an den Wendepunkten je Spielerhälfte ein

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** GS1
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

B-965 legt die Geschichte in die Welt-Datei und liefert Engine-Funktionen, die zu einem Wendepunkt die Zeilen nennen. Das Spiel zeigt sie noch nicht. Das HUD kennt Meldungen je Spieler (`native/k3c-game/hud/banner.go`, Warteschlange, 2,8 s), aber keine zweizeilige Einblendung; Ereignisse der Sim erreichen das HUD über `view.Event`. Abgespalten aus B-965 (🧑 2026-11-15, Anzeige).

## Ziel

An den Wendepunkten blenden ein bis zwei Zeilen der Geschichte in jeder Spielerhälfte kurz ein und wieder aus, ohne Tastendruck und ohne das Spiel anzuhalten.

## Beteiligte und Zielgruppen

Spielende am TV (1–2 Monarchen im Split-Screen). Claude setzt um, 🧑 gibt frei und nimmt am TV ab.

## Anforderungen

Aus B-965 (🧑 2026-11-15, wörtlich):

- **Anzeige: Einblendung an Wendepunkten.** Ein bis zwei Zeilen blenden kurz ein und wieder aus: bei Ankunft auf der Insel, beim Abschluss einer Etappe, vor dem Endgegner. Kein Tastendruck, das Spiel läuft weiter.
- Kurz: wenige Zeilen, nichts zum Durchlesen.

Ergänzt:

- **R1 Quelle:** Das Spiel fragt die Zeilen bei der Engine ab (Funktionen aus B-965) und rechnet keine Wendepunkte selbst: neue Kampagne, Ankunft nach `islandSwitch`, `bossSpawned` und `bossDefeated` des Endbosses. Beim Laden eines Spielstands erscheint nichts.
- **R2 Ort:** Eigene Geschichtszeile in jeder Spielerhälfte (Zelle), waagerecht mittig im oberen Drittel, getrennt von der Meldungs-Zeile (`banner.go`); Meldungen laufen unverändert weiter.
- **R3 Alle zugleich:** Ein Wendepunkt der Insel erscheint in allen Spielerhälften gleichzeitig, auch wenn ein Spieler in einer anderen Stufe steht.
- **R4 Ablauf:** Einblenden, stehen, ausblenden (Startwerte 0,5 s / 4 s / 0,5 s, vorläufig, als Konstanten neben `bannerSeconds`). Folgen zwei Einträge aufeinander (z. B. `done`, dann Etappenzeile), laufen sie nacheinander. Die Zeit kommt nur aus `Frame.Clock`.
- **R5 Kein Eingriff:** Kein Tastendruck, keine Pause, keine Eingabe wird verbraucht; Overlays (Seekarte, Menüs) verdecken die Zeile, sie läuft im Hintergrund ab.
- **R6 Schrift:** Papierschnitt-Schrift des HUD; passt eine Zeile nicht in die Zelle, wird sie kleiner gezeichnet (wie `fitPx` der Meldungen), nicht umbrochen und nicht gekürzt. Alle Zeichen der Welt-Geschichten sind in der HUD-Schrift vorhanden.
- **R7 Leistung:** Keine Allokation je Frame im Ruhezustand (wie die übrigen HUD-Teile).

## Nicht-Ziele

Format und Wendepunkt-Regeln (B-965), Texte (B-976), Vertonung, Nachlesen früherer Zeilen, Einblendung über die ganze Bildschirmbreite, Flaschenpost-Zeile (B-980).

## Regeln und Einschränkungen

- Das native Spiel zeichnet nur (CLAUDE.md › Regeln); jede Mechanik funktioniert mit 2 Spielern gleichzeitig.
- HUD-Prüfung durch den Agenten `k3c-hud` vor dem Commit.

- **Bereich:** `native/k3c-game/hud/**`, `native/k3c-game/view/**`, `native/k3c-game/game/**`, `native/k3c-game/local/**`
- **Abhängig von:** B-965
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Sengoku, neue Kampagne zu zweit: In beiden Hälften blendet die Startzeile der Welt ein und aus, danach die Ankunftszeile der Reisfeld-Ebene. Spieler 2 steigt in die Tiefe und löst den Endboss aus: Die Boss-Zeile erscheint bei beiden, auch bei Spieler 1 an der Oberfläche. Beide spielen währenddessen weiter.

## Ausnahme- und Fehlerfälle

- Welt oder Insel ohne Geschichte → keine Einblendung, kein Platzhalter.
- Neuer Wendepunkt während einer laufenden Einblendung → wird angehängt, nichts geht verloren, nichts erscheint doppelt.
- Spielstand geladen → keine Einblendung.
- Ein Spieler kommt später dazu (Lobby, zweites Pad) → sieht erst den nächsten Wendepunkt.
- Pause-Menü offen → die Uhr steht, die Einblendung läuft danach weiter.

## Akzeptanzkriterien

- **AC-01** Go-Test in `native/k3c-game/hud`: Die Ereignisse neue Kampagne, `islandSwitch`, `bossSpawned` und `bossDefeated` des Endbosses reihen die Zeilen der Engine in jeder Spielerhälfte ein; mit 2 Spielern in verschiedenen Stufen sehen beide dieselben Zeilen im selben Frame.
- **AC-02** Go-Test: Ablauf einblenden, stehen, ausblenden nach `Frame.Clock`; zwei Einträge laufen nacheinander; ein Miniboss und das Laden eines Spielstands lösen nichts aus.
- **AC-03** Go-Test: Die Einblendung verbraucht keine Eingabe und allokiert im Ruhezustand nicht (`testing.AllocsPerRun` = 0).
- **AC-04** Go-Test: Jede Zeile der Geschichten aller Welt-Dateien unter `data/worlds/` ist mit der HUD-Schrift darstellbar (kein fehlendes Zeichen) und passt nach dem Verkleinern in eine halbe Bildschirmbreite bei 1280 × 720.
- **AC-05** TV (🧑): Sengoku zu zweit gestartet, Start- und Ankunftszeile sind in beiden Hälften lesbar, stören das Spiel nicht und verschwinden von selbst.

## Offene Fragen

- Je Spielerhälfte oder ganze Breite? → Default: je Spielerhälfte (vorläufig, 🧑 bestätigt).
- Dauer → Default: 0,5 s ein, 4 s stehen, 0,5 s aus (vorläufig, Abstimmung am TV).
- Ort → Default: oberes Drittel der Zelle, mittig (vorläufig, Abstimmung am TV).

## Notizen

Abgespalten aus B-965 (2026-11-15, Zerlegung wegen Größe L).
