# B-994 · Das Spiel zeichnet Gegner und Objekte mit den Skins der Welt

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WE6
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Das Spiel lädt Figuren und Objekte über `data/paper.json` und die Gruppen-Indizes `public/grafik/<gruppe>/index.paper.json` (`native/k3c-game/art/manifest.go`, `actors/catalog.go`), für alle Welten gleich. Mit B-984 trägt die Welt eine Zuordnung `skins` (Basis-Asset → Ersatz-Asset).

## Ziel

In einer Welt mit Skins sehen Gegner, Objekte und Gebäude so aus, wie die Welt es festlegt, und heißen so, wie die Welt sie nennt.

## Beteiligte und Zielgruppen

Spieler (2 Spieler, Split-Screen). 🧑 gibt frei und prüft am PC.

## Anforderungen

- Beim Laden einer Welt löst das Spiel jedes Asset über `skins` auf: steht das Basis-Asset als Schlüssel darin, wird das Ersatz-Asset geladen.
- Fehlt das Ersatz-Asset oder passt es nicht (Animationen, Zustände), gilt das Basis-Asset; ein Log-Eintrag je Asset.
- Namen von Gegnern (HUD, Hinweise) kommen aus dem Wertesatz der Welt (`enemies.<art>.name`).
- Weltwechsel im selben Programmlauf lädt die Skins der neuen Welt; nichts bleibt von der alten hängen.
- Texturbudget: geladen werden nur die Assets der aktiven Welt.
- Gilt für beide Bildhälften gleich; keine Spiellogik.

## Nicht-Ziele

Die Assets selbst (B-988 bis B-990), Skins für Monarch, Truppen und Weltbilder, Editor (B-987).

## Regeln und Einschränkungen

Das native Spiel zeichnet nur.

- **Bereich:** `native/k3c-game/**`
- **Abhängig von:** B-984, B-993
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Sengoku mit `"gegner/wolf": "gegner/ninja"` → Wölfe erscheinen als Ninja und heißen im HUD Ninja. Standard-Welt → unverändert.

## Ausnahme- und Fehlerfälle

`skins` zeigt auf ein Asset, das es nicht gibt → Basis-Asset, Log `❌` mit Welt und Pfad. Ersatz-Asset ohne die Animation `attack` → Basis-Asset für die ganze Art (kein Mischen).

## Akzeptanzkriterien

- **AC-01** Test: Auflösung mit und ohne Skin, mit fehlendem und mit unpassendem Ersatz-Asset.
- **AC-02** Test: Weltwechsel tauscht die Skins.
- **AC-03** Test: Gegner-Name kommt aus dem Wertesatz der Welt.
- **AC-04** Sichtprüfung 🧑 am PC (Teil der Abnahme von B-996).

## Offene Fragen

- Umfang → Default: Skins für die Gruppen `gegner`, `objekte`, `gebaeude-paper` und das Schiff; Monarch und Truppen nicht (vorläufig, 🧑 bestätigt).

## Notizen

Zerlegt aus B-959 (Punkt Gegner-Varianten je Welt, Anteil Spiel).
