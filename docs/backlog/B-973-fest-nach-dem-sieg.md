# B-973 · Nach dem Sieg über den Endgegner feiert das Volk ein Fest

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** SI6
- **Projekt:** KMP
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ideenrunde k3c-07 (2026-11-15), Vorschlag von Claude, von 🧑 ausgewählt. Der Sieg über den Endgegner öffnet den Inselwechsel, wird aber nicht gefeiert (ungeprüft, was das Spiel heute zeigt).

## Ziel

Mit dem Sieg über den Endgegner beginnt im Hub ein Fest im Stil der Welt; es dauert, bis das Schiff ablegt. Der Sieg fühlt sich endgültig an.

## Beteiligte und Zielgruppen

Spieler: ein oder zwei Monarchen auf der Couch. Umsetzung: Claude in autonomen Sessions (SI6), Fachprüfung durch `k3c-reviewer`. Freigabe der Spec, Bestätigung der Defaults und Abnahme am TV: 🧑.

## Anforderungen

Dieses Ticket trägt **Regeltext und Engine** (friedliche Insel und Fest-Zustand); Darstellung und Festmusik stehen in B-1002.

- **Gestalt (🧑 2026-11-15): je Welt eigenes Fest.** Sengoku lässt Laternen steigen, am Nil brennen Fackeln und Blüten treiben auf dem Wasser, im Polar leuchtet Nordlicht über Lagerfeuern. Die Bürger jubeln und tanzen in allen Welten gleich; nur der Himmels-Effekt ist je Welt eigen.
- **Ablauf (🧑 2026-11-15): bis zur Abfahrt.** Das Fest beginnt mit dem Sieg im Hub und dauert, bis das Schiff ablegt; die Spieler bauen und beladen das Schiff in Ruhe (B-967).
- **Friedliche Insel (🧑 2026-11-15): Mit dem Sieg über den Endgegner enden alle Angriffe auf der Insel.** Neue Regel in `docs/rules/`; gilt bis zur Abfahrt.
- Dazu eine kurze Festmusik je Welt.
- Funktioniert mit 2 Spielern; beide Hälften zeigen das Fest.

Ergänzt (Defaults, siehe Offene Fragen):

- Der Frieden gilt ab dem Sieg über den Endgegner der Insel (`Island.EndbossDefeated`) für **alle Stufen** bis zum Inselwechsel; auf der letzten Insel der Welt bis zum Spielende.
- Im Frieden: keine neuen Wellen, kein Aggressionspool unter Tage, keine Nacht-Events mit Gegnern (Vollmond, Blutmond, Händler-Überfall); Portale und Camps schicken nichts mehr.
- Gegner, die beim Sieg noch leben, brechen ab und fliehen zum nächsten Portal oder Inselrand und verschwinden dort; sie greifen nicht mehr an und rauben keine Krone.
- Tag und Nacht, Händler-Besuche, Wetter (B-969) und die Arbeit von Bauern und Handwerkern laufen weiter; das Schiff wird normal gebaut und beladen.
- Der Zustand nennt `festival` (wahr von Sieg bis Abfahrt), damit B-1002 zeichnen kann; ein Ereignis `festivalStart` kommt genau einmal.
- Der Frieden wird aus `EndbossDefeated` abgeleitet; kein neues Feld im Spielstand.
- Der Korridor „Schiff fertig“ (`docs/rules/zielkorridore.md` § 6) bleibt im Wert unverändert; der Regeltext vermerkt, dass nach dem Sieg kein Druck durch Angriffe mehr besteht (Neubewertung im Balancing, derzeit gesperrt).

## Nicht-Ziele

Ein Fest für alle Welten, Fest nur für eine Nacht, kurzer Jubel-Moment ohne Zustand, Belohnungen durch das Fest.

## Regeln und Einschränkungen

- **Bereich:** `docs/rules/kampf.md`, `docs/rules/bosse.md`, `docs/rules/zielkorridore.md`, `engine/sim/**`, `testdata/golden/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** golden
- Zwei Sessions: SI6.1 Regeltext (REG), SI6.3 Engine (SIM). Sim-Code im eigenen Worktree mit eigener Dev-Workbench (B-925, Agent `k3c-sim-dev`).
- Gewollte Regeländerung: Golden-Läufe, die über den Sieg hinausgehen, ändern sich (vermutet); `task go:golden` nur in SI6.3 mit Begründung im Commit (`docs/arbeitsweise.md` › Golden aktualisieren).
- Keine Simulation, kein Balancing (gesperrt). Keine Belohnungen durch das Fest.

## Beispiele

- Sengoku, Insel 1: Der Endgegner fällt. Im Hub steigen Laternen auf, die Bürger tanzen, Festmusik spielt. Die Monarchen bauen das Segel fertig und beladen das Schiff; mit dem Ablegen endet das Fest.
- Beim Sieg stehen noch fünf Gegner an der Mauer → sie lassen ab, laufen zum Portal und verschwinden.
- Nacht 14 wäre Vollmond, der Sieg war an Tag 12 → kein Vollmond-Angriff.

## Ausnahme- und Fehlerfälle

- Ein Gegner trägt beim Sieg eine geraubte Krone → er lässt sie fallen, bevor er flieht (kein Game Over nach dem Sieg).
- Sieg in einer tieferen Stufe, ein Monarch ist in einer anderen Stufe → der Frieden gilt sofort in allen Stufen.
- Letzte Insel der Welt → Fest und Frieden bis zum Spielende (Kampagnensieg wie heute).
- Speichern und Laden nach dem Sieg → Frieden und Fest gelten weiter (aus `EndbossDefeated`).
- Spielmodus ohne Endgegner (Ziel ohne Boss, falls vorhanden) → kein Fest, Verhalten wie heute.
- Dev-Aktionen, die Gegner erzeugen, bleiben erlaubt (Entwickler-Werkzeug).

## Akzeptanzkriterien

- **AC-01** `docs/rules/kampf.md` hat die Regel „Friedliche Insel“ (ab wann, für welche Stufen, was endet, was weiterläuft), `docs/rules/bosse.md` § 1 und § 2 verweisen darauf, `docs/rules/zielkorridore.md` § 6 trägt den Vermerk zum Korridor „Schiff fertig“.
- **AC-02** Go-Test: Nach dem Sieg entstehen in keiner Stufe neue Gegner aus Wellen, Portalen, Camps oder Aggressionspool, über mindestens drei Tage und eine Vollmond-Nacht.
- **AC-03** Go-Test: Beim Sieg lebende Gegner fliehen und verschwinden; sie verursachen danach keinen Schaden und rauben keine Krone; eine getragene Krone fällt.
- **AC-04** Go-Test: Bauern und Handwerker arbeiten weiter, der Schiffsbau schließt ab, Händler-Besuche finden ohne Überfall statt.
- **AC-05** Go-Test: `festival` ist von Sieg bis Inselwechsel wahr, `festivalStart` kommt genau einmal; nach `NextIsland` ist die neue Insel nicht friedlich.
- **AC-06** Go-Test: Speichern und Laden nach dem Sieg erhält Frieden und Fest ohne neue Spielstand-Version.
- **AC-07** Go-Test: zwei Spieler auf verschiedenen Stufen, der Frieden gilt in beiden.
- **AC-08** `check_run task:check:diff` grün; geänderte Dateien unter `testdata/golden/` sind im Commit begründet.

## Offene Fragen

Alle mit Default entschieden (vorläufig, 🧑 bestätigt):

- Gegner und Portale beim Sieg? → Gegner fliehen und verschwinden, Portale schicken nichts mehr.
- Alle Stufen? → Ja, der Frieden gilt für die ganze Insel.
- Korridor „Schiff fertig“, Wetter, Events? → Korridor bleibt im Wert (Vermerk); Wetter und Händler laufen weiter, Nacht-Events mit Gegnern entfallen.
- Arbeiten Bauern und Handwerker weiter? → Ja; nur freie Bürger im Hub tanzen (Anzeige, B-1002).
- Polar und Nordlicht? → Kommen mit der Polar-Welt, die es noch nicht gibt.

## Notizen

Grundzüge in der Ideenrunde k3c-07 (2026-11-15) mit 🧑 geklärt. Die ursprüngliche Annahme „ohne Regelwirkung“ ist überholt: 🧑 hat die friedliche Insel als Regel entschieden. 2026-11-15 von L zerlegt in B-973 (Regel und Engine) und B-1002 (Darstellung und Festmusik).
