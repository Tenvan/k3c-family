# B-1002 · Das Fest zeigt Jubel, Himmels-Effekt je Welt und spielt Festmusik

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** SI6
- **Projekt:** KMP
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Teil von B-973 (Ideenrunde k3c-07, 2026-11-15): Die Engine meldet nach SI6.3 den Zustand `festival` und das Ereignis `festivalStart`; das Spiel zeigt den Sieg über den Endgegner bisher nicht als Fest. Vorhanden sind Effekte (`native/k3c-game/fx/`), Himmel (`native/k3c-game/world/backdrop.go`), Musik-Slots je Tageszeit in `data/sounds.json` und Musik-Varianten (B-936).

## Ziel

Der Sieg fühlt sich endgültig an: Im Hub wird bis zur Abfahrt gefeiert, mit Himmels-Effekt der Welt und Festmusik.

## Beteiligte und Zielgruppen

Spieler: ein oder zwei Monarchen auf der Couch. Umsetzung: Claude in autonomen Sessions (SI6.2 Assets, SI6.4 Anzeige, SI6.5 Festmusik), Fachprüfung durch `k3c-grafik` und `k3c-sound`. Abnahme am TV: 🧑.

## Anforderungen

- Anforderungen aus `B-973 › Anforderungen` gelten; hier nur Bild und Ton: Die Bürger jubeln und tanzen in allen Welten gleich; nur der Himmels-Effekt ist je Welt eigen. Sengoku lässt Laternen steigen, am Nil brennen Fackeln und Blüten treiben auf dem Wasser. Dazu eine kurze Festmusik je Welt (🧑 2026-11-15).
- Solange `festival` gilt: freie Bürger im Hub tanzen (vorhandene Posen, gespiegelt und versetzt abgespielt), Truppen jubeln an ihrem Posten; arbeitende Bauern und Handwerker arbeiten sichtbar weiter.
- Himmels-Effekt je Welt aus `data/native.json` › `festival` (Welt-ID → Effekt): `sengoku` Laternen, `nil` Fackeln am Ufer und Blüten auf dem Wasser; Welten ohne Eintrag (`standard`, `britannien`) feiern ohne Himmels-Effekt.
- Assets als Papierschnitt, nur über `assets_*`: `fest-laterne`, `fest-bluete`, `fest-fackel` (Gruppe `objekte`).
- Festmusik: je Welt ein Musik-Slot `music:fest:<welt>` in `data/sounds.json`, als Schleife von Sieg bis Abfahrt statt der Tages- und Nachtmusik; Welten ohne eigenen Slot nutzen `music:fest`.
- Split-Screen: Beide Hälften zeigen das Fest, wenn der Hub im Bild ist; die Festmusik spielt für beide.

## Nicht-Ziele

Nordlicht und Polar (kommen mit der Polar-Welt), neue Tanz-Animationen, Belohnungen, Feuerwerk, Fest außerhalb des Hubs, Regeländerungen (B-973).

## Regeln und Einschränkungen

- **Bereich:** `public/grafik/objekte/**`, `public/audio/**`, `data/paper.json`, `data/native.json`, `data/sounds.json`, `native/k3c-game/**`
- **Abhängig von:** B-973
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –
- Drei Sessions: SI6.2 Assets (PAP, vor der Engine möglich), SI6.4 Anzeige (CLI), SI6.5 Festmusik (CLI). Grafik-Golden und `gen:paper` laufen im Abnahme-Ticket des Sprints (B-742).
- Musik entsteht auf dem Weg der vorhandenen Musik-Varianten (B-936, `docs/rules/klang.md`); keine neue Abhängigkeit. Geht das nicht ohne neues Werkzeug, hält SI6.5 an und meldet es.

## Beispiele

- Sengoku, Insel 1: Der Endgegner fällt, im Hub steigen Laternen auf, Bürger tanzen, die Festmusik löst die Nachtmusik ab; mit dem Ablegen endet beides.
- `britannien`: Jubel und Tanz, allgemeine Festmusik, kein Himmels-Effekt.

## Ausnahme- und Fehlerfälle

- Monarch ist beim Sieg unter Tage → er hört die Festmusik sofort, sieht das Fest erst im Hub.
- Spiel wird während des Fests geladen → Fest und Musik laufen sofort (Zustand `festival`).
- Wetternacht während des Fests → Wetter-Darstellung hat Vorrang am Himmel, Tanz und Musik bleiben.
- Slot `music:fest:<welt>` fehlt → `music:fest`; fehlt auch der, schlägt der Test fehl.

## Akzeptanzkriterien

- **AC-01** Die Assets `fest-laterne`, `fest-bluete`, `fest-fackel` (Gruppe `objekte`) sind über `assets_*` angelegt, in `data/paper.json` zugeordnet und von `k3c-grafik` ohne schweren Befund geprüft.
- **AC-02** Go-Test in `native/k3c-game`: Zustand `festival` → tanzende freie Bürger und jubelnde Truppen im Hub, Arbeitende unverändert; ohne `festival` nichts davon.
- **AC-03** Go-Test: `data/native.json` › `festival` ordnet `sengoku` und `nil` ihren Himmels-Effekt zu; Welt ohne Eintrag → kein Effekt, kein Fehler.
- **AC-04** Go-Test: Mit `festivalStart` wechselt die Musik auf `music:fest:<welt>` (Rückfall `music:fest`) und bleibt bis zum Inselwechsel; `data/sounds.json` enthält die Slots mit Dateien unter `public/audio/`; `k3c-sound` meldet keinen schweren Befund.
- **AC-05** Beobachtung am TV (🧑): Fest nach dem Sieg in Sengoku und Nil, beide Hälften, Festmusik bis zur Abfahrt.
- **AC-06** `check_run task:check:diff` grün.

## Offene Fragen

Mit Default entschieden (vorläufig, 🧑 bestätigt):

- Woher kommt der Himmels-Effekt je Welt? → Zuordnung in `data/native.json` (reine Darstellung, kein Feld der Welt-Datei); neue Welten feiern ohne Effekt, bis sie eingetragen sind.
- Woher kommt die Festmusik? → Wie die Musik-Varianten aus B-936 (Weg ungeprüft, SI6.5 prüft ihn zuerst).
- `standard` und `britannien`? → Jubel, Tanz und allgemeine Festmusik, kein Himmels-Effekt.

## Notizen

Am 2026-11-15 aus B-973 abgetrennt (Zerlegung von L): Regel und Engine bleiben in B-973.
