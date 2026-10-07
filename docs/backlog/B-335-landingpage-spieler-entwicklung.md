# B-335 · Die Landingpage zeigt nur Spieler-Kacheln, Entwicklungs-, Performance- und Balancing-Aufrufe liegen auf einer eigenen Entwicklerseite

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Landingpage (`index.html`, Kacheln aus `src/landing/pages.ts`) mischt Spieler- und Entwickler-Kacheln: Weiterspielen, Neues Spiel, Online spielen, Unsere Aufstellung, Alle Figuren, Lizenzen & Danksagung neben Gamepad-Test, Testing, Level-Betrachter, Alle Grafiken, Hörprobe und Monitor. Die Dungeon-Master-Seite `/dm` hat gar keine Kachel (DBG3). Mit B-334 kommen Performance-Modi hinzu, für Balancing gibt es bisher keinen Aufruf im Browser. Eine Seite mit Unterkacheln gibt es schon: `testing.html` (`src/tools/testTiles.ts`).

## Ziel

Die Familie sieht auf der Landingpage nur, was zum Spielen gehört. Alle Aufrufe für Entwicklung, Performance und Balancing liegen gesammelt auf einer zweiten Seite (wie `testing.html`), die von der Landingpage über eine kleine Kachel erreichbar ist.

## Beteiligte und Zielgruppen

Familie am TV (Xbox, Edge) und PC als Spieler; 🧑 als Entwickler, Tester und Spielleiter. 🧑 entscheidet die Zuordnung der Kacheln und nimmt ab.

## Anforderungen

- Landingpage: nur Spieler-Kacheln (Vorschlag: Weiterspielen, Neues Spiel, Online spielen, Unsere Aufstellung, Alle Figuren, Lizenzen & Danksagung) plus **eine kleine** Kachel zur Entwicklerseite (kleiner und unauffälliger als die Spieler-Kacheln, Wunsch 🧑).
- Entwicklerseite, gegliedert nach **Entwicklung** (Testing, Level-Betrachter, Gamepad-Test, Alle Grafiken, Hörprobe, Dungeon Master `/dm`), **Performance** (Monitor, Performance-Modi aus B-334) und **Balancing** (Aufrufe, sobald es welche gibt).
- Jede Kachel bleibt per Controller und Tastatur erreichbar (Fokus-Weiterschaltung wie `testTiles.ts`); Zurück zur Landingpage über `goHome()`.
- Seiten-Regeln aus `CLAUDE.md` gelten für die neue Seite (`installPageChrome()`, Eintrag in `pages.ts`, Vollbild über die Shell, Wechsel nur über `openPage()`).

## Nicht-Ziele

Neue Werkzeugseiten selbst (Performance-Modi: B-334; Balancing-Seite: eigenes Ticket); Passwortschutz der Entwicklerseite; Neugestaltung der Spieler-Kacheln; Fehler der Kachel „Neues Spiel“ (B-292).

## Regeln und Einschränkungen

Domäne PLAT (`src/landing/`, `src/tools/`, `*.html`). `CLAUDE.md` › Seiten & Navigation; `tests/projectRules.test.ts` muss grün bleiben (Seiten eingetragen, `installPageChrome()`). Controller-Taste B nicht belegen, View + Menu reserviert. Texte über `t()` in de und en. Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

- Familie öffnet die Landingpage am TV → sieht Weiterspielen, Neues Spiel, Online spielen, Aufstellung, Figuren, Lizenzen und eine kleine Kachel „Entwicklung“.
- 🧑 wählt „Entwicklung“ → Seite mit Abschnitten Entwicklung, Performance, Balancing; „Monitor“ öffnet die Monitoring-Seite, Home führt zurück zur Landingpage.

## Ausnahme- und Fehlerfälle

Aufruf einer Werkzeugseite direkt per URL bleibt möglich. Ein Abschnitt ohne Kacheln (Balancing heute) wird ausgeblendet oder zeigt „noch keine Aufrufe“. GitHub-Pages-Build ohne Server: Kacheln, die einen Server brauchen, verhalten sich wie heute (B-032).

## Akzeptanzkriterien

- **AC-01** Die Landingpage enthält nur die festgelegten Spieler-Kacheln und genau eine kleine Kachel zur Entwicklerseite (Test über `pages.ts`).
- **AC-02** Die Entwicklerseite listet alle bisherigen Entwickler-Kacheln plus `/dm`, gegliedert nach Entwicklung, Performance und Balancing; jede führt zur richtigen Seite (Test).
- **AC-03** `task check` grün, `tests/projectRules.test.ts` deckt die neue Seite ab.
- **AC-04** 🧑 hat Landingpage und Entwicklerseite am PC mit Tastatur abgenommen (Navigation hin und zurück).

## Offene Fragen

- Welche Kacheln zählen als Spieler-Kacheln (v. a. Unsere Aufstellung, Alle Figuren, Lizenzen)? 🧑
- Wird `testing.html` zur Entwicklerseite ausgebaut oder bleibt sie eine Kachel darin? 🧑
- `/dm` als Kachel: läuft heute außerhalb der Shell (DBG3); öffnet die Kachel sie im Rahmen oder als eigenes Fenster? Beim Einplanen klären.

## Notizen

Auftrag 🧑 (2026-10-07, nach N2.4/B-334): „Auf der Standard-Landingpage gehören reine Kacheln für die Spieler, dann muss eine zweite her – wie die Testing –, die alle Aufrufe für Entwicklung, Performance und Balancing-Modi enthält.“ Hohe Priorität.

Nachtrag 🧑 (2026-10-07): „Der Aufruf dazu kann aber auch noch über eine kleine Kachel auf der Spielerseite liegen.“
