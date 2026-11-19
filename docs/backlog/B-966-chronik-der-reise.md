# B-966 · Abgeschlossene Inseln hinterlassen auf der Seekarte ein Andenken und eine Zeile der Geschichte

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** GS3
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ideenrunde k3c-07 (2026-11-15), Vorschlag von Claude, von 🧑 ausgewählt. Die Seekarte (B-964) zeigt Inseln, die Geschichte (B-965) blendet nur kurz ein.

Stand im Code: Der Spielstand trägt die abgeschlossenen Insel-IDs (`completed`) und das gewählte Ziel (`chosen`), `IslandSaveVersion` ist 9 (`engine/sim/island_save.go`). Tage und Spieler je abgeschlossener Insel werden nicht festgehalten. Es gibt einen Endboss in den Daten (`data/bosses.json` › `endBosses`: `crystalHeartWarden`); Bosse je Welt gibt es nicht.

## Ziel

Jede abgeschlossene Insel hinterlässt im Diorama der Seekarte eine Trophäe des Endgegners, die Abschlusszeile der Geschichte und zwei Zahlen. Die Karte wird zum Tagebuch des Spielstands.

Dieses Ticket liefert die Chronik im Spielstand, die Trophäen-Assets und die Anzeige auf der Seekarte im Spiel. Die Vorschau in der Spielstand-Auswahl ist B-978.

## Beteiligte und Zielgruppen

Spielende am TV (1–2 Monarchen). Claude setzt um und legt die Assets über `assets_*` an, 🧑 gibt frei und nimmt am TV ab.

## Anforderungen

Entscheidungen von 🧑 (wörtlich übernommen):

- **Andenken (🧑 2026-11-15): Trophäe des Endgegners.** Auf der Insel-Miniatur steht ein kleines Zeichen des besiegten Endgegners (Maske, Horn, Banner) samt Fahne der Spieler; je Welt wenige Motive.
- **Inhalt (🧑 2026-11-15): Zeile plus zwei Zahlen.** Die Abschlusszeile der Geschichte (B-965), darunter klein: Tage bis zum Sieg und wer dabei war.
- **Zugang (🧑 2026-11-15): Seekarte und Spielstand-Auswahl.** Im Spiel auf der Seekarte am Hafen (B-964); zusätzlich zeigt die Spielstand-Auswahl im Menü die Karte mit den Trophäen als Vorschau. (Vorschau: B-978.)
- Die Chronik steht im Spielstand (Tage und Spieler je abgeschlossener Insel).
- Assets nur über MCP `assets_*`.

Ergänzt:

- **R1 Chronik (Engine):** Beim Ablegen von einer Insel mit besiegtem Endboss hält die Insel einen Eintrag fest: Insel-ID, Tag der Insel beim Ablegen, Zahl der Monarchen, die den Wechsel mitgemacht haben. Die Einträge ziehen wie `completed` zur nächsten Insel mit und stehen im Spielstand (`chronicle`, optional). Die Reihenfolge ist die des Abschlusses.
- **R2 Spielstand:** `IslandSaveVersion` steigt um eins (nächste freie Version), neue Fixture; keine Rücksicht auf alte Spielstände: ältere Stände laden nach den bestehenden Regeln, ihre abgeschlossenen Inseln haben dann keinen Chronik-Eintrag.
- **R3 Trophäe:** Je Welt ein Trophäen-Motiv (heute gibt es einen Endboss), zugeordnet in `data/paper.json` › `seekarte` › `trophaeen` (Welt-ID → Asset); eine Welt ohne Eintrag nutzt das Motiv von `standard`. Die Trophäe ersetzt auf abgeschlossenen Inseln die schlichte Fahne aus B-964 durch Trophäe samt Fahne.
- **R4 Anzeige auf der Seekarte:** Steht der Zeiger des Overlays auf einer abgeschlossenen Insel oder ist keine wählbare Insel markiert, zeigt die Karte unter der Miniatur die erste Zeile von `done` der Insel (B-965) und darunter klein „<Tage> Tage · <n> Monarchen“ (Einzahl: „1 Tag“, „1 Monarch“). Der Zeiger darf dazu mit Links/Rechts auch abgeschlossene Inseln ansteuern; A wirkt dort nicht.
- **R5 Ohne Geschichte oder Chronik:** Fehlt `done`, erscheinen nur die Zahlen; fehlt der Chronik-Eintrag (alter Stand), nur Trophäe und Fahne.
- **R6 2 Spieler und Determinismus:** Beide Hälften zeigen dieselben Andenken; die Chronik entsteht nur aus dem Zustand der Engine (kein Zufall, keine Uhrzeit).

## Nicht-Ziele

Abbild der eigenen Burg je Ausbaustufe, ganze Geschichte der Insel zum Nachlesen, eigener Menüpunkt „Chronik“. Vorschau in der Spielstand-Auswahl (B-978). Namen oder Farben der Spieler. Eigene Trophäen je Endboss, solange es nur einen gibt.

## Regeln und Einschränkungen

- Umgehängt auf Rust (B-1143, 2026-11-19): Sim in `rust/crates/k3c-sim`, Anzeige in `rust/crates/k3c-game` (Entscheidung 013); die Go-Engine bekommt keine neue Mechanik.
- Berührt das Spielstand-Format; keine Rücksicht auf alte Spielstände (`docs/arbeitsweise.md` › Spielstand-Format ändern: Version erhöhen, Fixture).
- Review vor dem Commit (Domäne SIM, Spielstand).
- Assets nur über `assets_*` (Entscheidung 005), Fachprüfung `k3c-grafik`.
- Regeltext: `docs/rules/welten.md` § 5/§ 6.1 nennt die Chronik im Spielstand.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `docs/rules/welten.md`, `rust/crates/k3c-sim/**`, `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/src/view/**`, `rust/crates/k3c-game/src/local/**`, `public/grafik/objekte/**`, `data/paper.json`
- **Abhängig von:** B-964, B-965, RU37
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** spielstand

## Beispiele

Sengoku, Insel 1 nach 14 Tagen zu zweit besiegt: Auf der Miniatur steht die Maske des Endgegners mit Fahne; darunter die Abschlusszeile in Kunstsprache und „14 Tage · 2 Monarchen“. Dieselbe Karte erscheint als Vorschau in der Spielstand-Auswahl (B-978).

Spielstand (Ausschnitt): `"completed": ["island1"], "chronicle": [{ "island": "island1", "days": 14, "monarchs": 2 }]`.

## Ausnahme- und Fehlerfälle

- Ein Spieler ist beim Ablegen ohne Krone oder tot → er zählt nicht als Monarch der Chronik; gezählt werden die lebenden Spieler am Steg (die Bedingung des Wechsels).
- Spielstand mit `completed`, aber ohne `chronicle` → lädt, Andenken ohne Zahlen.
- `chronicle` nennt eine Insel, die nicht in `completed` oder nicht in der Welt steht → Stand wird abgelehnt wie bei unbekannter Insel in `completed`.
- Letzte Insel der Welt (Kampagnensieg, kein Ablegen) → kein Chronik-Eintrag nötig; die Karte wird nicht mehr geöffnet.
- Trophäen-Asset fehlt → Fahne aus B-964, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Go-Test in `engine/sim`: Inselwechsel am Tag 14 mit 2 lebenden Spielern am Steg erzeugt den Eintrag `{island1, 14, 2}`; er zieht zur nächsten Insel mit; ein zweiter Wechsel hängt den nächsten Eintrag an.
- **AC-02** Go-Test: Speichern und Laden erhalten `chronicle` byte-gleich; `IslandSaveVersion` ist erhöht, die neue Fixture lädt; ein Stand ohne `chronicle` lädt; ein Eintrag mit unbekannter Insel wird abgelehnt.
- **AC-03** `docs/rules/welten.md` nennt `chronicle` (Felder, Version, Verhalten älterer Stände); `docs/glossar.md` führt Andenken, Chronik und Trophäe.
- **AC-04** Die Trophäen-Assets der vier Welten sind über `assets_*` angelegt und in `data/paper.json` › `seekarte` › `trophaeen` zugeordnet.
- **AC-05** Go-Test in `native/k3c-game/hud`: Eine abgeschlossene Insel zeigt Trophäe und Fahne; mit dem Zeiger darauf erscheinen die erste Zeile von `done` und „14 Tage · 2 Monarchen“; ohne `done` nur die Zahlen, ohne Chronik-Eintrag nur die Trophäe; A auf einer abgeschlossenen Insel wählt nichts.
- **AC-06** Go-Test: Zwei Spielerhälften zeigen dieselben Andenken; keine Allokation je Frame im Ruhezustand.
- **AC-07** TV (🧑): Sengoku, Insel 1 zu zweit abgeschlossen, an Insel 2 die Seekarte öffnen: Trophäe, Zeile und Zahlen sind lesbar.

## Offene Fragen

- Welche Trophäen-Motive je Welt und Endgegner? → Default: ein Motiv je Welt, weil es nur einen Endboss gibt: Standard Kristallsplitter, Sengoku Maske, Nil Skarabäus, Britannien Horn (vorläufig, 🧑 bestätigt).
- „Wer dabei war“: Anzahl der Monarchen oder Namen/Farben der Spieler? → Default: Anzahl („2 Monarchen“, wie im Beispiel von 🧑) (vorläufig, 🧑 bestätigt).
- „Tage bis zum Sieg“ → Default: Tag der Insel beim Ablegen, gezählt ab Ankunft auf dieser Insel (vorläufig, 🧑 bestätigt); zählt die Engine die Tage über Inseln fort, speichert die Chronik die Differenz.
- Wann sieht man Zeile und Zahlen? → Default: wenn der Zeiger auf der abgeschlossenen Insel steht; die Trophäe ist immer sichtbar (vorläufig).

## Notizen

Grundzüge in der Ideenrunde k3c-07 (2026-11-15) mit 🧑 geklärt. 2026-11-15 nach GS3 verschoben und zerlegt: Vorschau in der Spielstand-Auswahl → B-978.
