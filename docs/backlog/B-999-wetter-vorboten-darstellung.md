# B-999 · Vorboten, Taifun und Flut sind im Spiel sichtbar und hörbar

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** RU62
- **Projekt:** RSU
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Teil von B-969 (Ideenrunde k3c-07, 2026-11-15): Die Engine meldet nach SI3.2 Wetterart, Phase (Vorboten, aktiv) und bei Flut die gesperrten Bereiche; das Spiel zeigt davon nichts. Vorhanden sind Himmel und Hintergründe (`native/k3c-game/world/backdrop.go`), Wasser (`native/k3c-game/world/water.go`) und Effekte (`native/k3c-game/fx/`).

## Ziel

Die Spieler erkennen am Vortag die Vorboten und in der Wetternacht Taifun oder Flut samt gesperrten Wegen.

## Beteiligte und Zielgruppen

Spieler: ein oder zwei Monarchen auf der Couch. Umsetzung: Claude in autonomen Sessions (SI3.3 Assets, SI3.4 Anzeige), Fachprüfung durch `k3c-grafik`. Abnahme am TV: 🧑.

## Anforderungen

- Anforderungen aus `B-969 › Anforderungen` gelten; hier nur Bild und Ton. Am Vortag zeigt der Himmel Vorboten (Wolken, steigendes Wasser) (🧑 2026-11-15).
- Vorboten Taifun: dunkle Wolkenbank am Himmel; Taifun: Regen und Windstreifen in Windrichtung, Bäume und Banner neigen sich nicht (keine neue Animation).
- Vorboten Flut: der Wasserspiegel am Ufer steigt sichtbar; Flut: die gesperrten Randzonen stehen unter Wasser, die Grenze ist klar erkennbar.
- Ein Wetter-Ambient je Art (Wind, Wasser) über den vorhandenen Ambient-Bus; Auswahl aus vorhandenen Quellen oder neuer Slot in `data/sounds.json`.
- Split-Screen: Jede Hälfte zeigt das Wetter nur, wenn ihr Monarch an der Oberfläche ist.
- Wolkenbank als Papierschnitt-Asset über `assets_*`; Regen, Wind und Wasserstand werden gezeichnet (kein Asset).

## Nicht-Ziele

Schneesturm (kommt mit der Polar-Welt), Wetter als reine Kulisse ohne Engine-Zustand, HUD-Vorhersage oder Kalender, Regeländerungen (B-969).

## Regeln und Einschränkungen

- Umgehängt auf Rust (B-1143, 2026-11-19): Anzeige und Ton in `rust/crates/k3c-game`; die Go-Sessions des Sprints SI3 sind verworfen; Sessions entstehen beim Aktivieren von RU62 (Assets PAP, Anzeige und Ton CLI). Grafik-Golden und `gen:paper` laufen im Abnahme-Ticket des Sprints (B-742). Ton prüft `k3c-sound`.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `public/grafik/hintergruende/**`, `public/audio/**`, `data/paper.json`, `data/sounds.json`, `rust/crates/k3c-game/src/**`
- **Abhängig von:** B-969, RU37
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

- Sengoku, Tag 6: Wolkenbank zieht auf. Nacht 7: Regen und Windstreifen, Wind-Ambient; am Morgen klar.
- Nil, Wetternacht: Wasser bedeckt die Randzonen; Monarch 2 unter Tage sieht und hört nichts davon.

## Ausnahme- und Fehlerfälle

- Monarch wechselt in der Wetternacht die Stufe → Bild und Ton wechseln mit der Stufe.
- Welt ohne Wetter → keine Änderung an Bild und Ton.
- Niedrige Bildrate: Regen und Wind halten das Bild-Budget ein (Messung im vorhandenen Test `native/k3c-game/world/measure_test.go` oder gleichwertig).

## Akzeptanzkriterien

- **AC-01** Das Asset `wetter-wolkenbank` (Gruppe `hintergruende`) ist über `assets_*` angelegt, in `data/paper.json` zugeordnet und von `k3c-grafik` ohne schweren Befund geprüft.
- **AC-02** Go-Test in `native/k3c-game`: Zustand `forecast` und `active` je Art erzeugen die erwarteten Zeichen-Eintrage (Wolkenbank, Regen und Wind, Wasserstand, überflutete Zonen); unter Tage keine.
- **AC-03** Go-Test: Wetter-Ambient startet mit `active` und endet am Morgen; `data/sounds.json` nennt die Slots.
- **AC-04** Beobachtung am TV (🧑): Vorboten am Vortag erkennbar, Taifun und Flut in beiden Hälften richtig, gesperrte Zone der Flut eindeutig.
- **AC-05** `check_run task:check:diff` grün.

## Offene Fragen

Mit Default entschieden (vorläufig, 🧑 bestätigt):

- Asset oder gezeichnet? → Nur die Wolkenbank ist ein Asset; Regen, Wind und Wasser zeichnet das Spiel.
- Eigener Ton? → Ja, je Art ein Ambient; Musik bleibt unverändert.

## Notizen

Am 2026-11-15 aus B-969 abgetrennt (Zerlegung von L): Regel und Engine bleiben in B-969, der Welten-Editor ist B-1003.
