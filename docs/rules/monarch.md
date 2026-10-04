# Regelwerk: Monarch, Klassen, Skills

Beschlossen von 🧑 im Workshop R3.2 am 2026-10-02 (Grundlage: [`archiv/ist-monarch-buerger.md`](archiv/ist-monarch-buerger.md); Rahmen: [`wirtschaft.md`](wirtschaft.md), [`stufen.md`](stufen.md), [`materialien-gebaeude.md`](materialien-gebaeude.md)).
Abschnitt 7 (Reittier) **bestätigt von 🧑 am 2026-10-03** im Workshop F1.4.
Je Regel: **Regel · Begründung · Verweis auf `data/` · Zielkorridor**. Werte sind **Startwerte**, Feintuning mit dem Balancing-Tester (B-099).
Zielkorridore gelten im Standardszenario **Normal, Wald-Start, 2 Spieler, Bot „sparsam“, je 100 Seeds**. Jede Regel gilt für 2+ Spieler. Die Bürger beschließt R3.3 (`buerger.md`).

## 1. Basiswerte, Schlag und Level

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Kein Level, keine Erfahrung.** Die Stats je Level (`perLevel`, `maxLevel`) entfallen; der Monarch wächst nur über Skills (§ 3). | Weniger Mechanik, Fortschritt ist Entdecken und Verteilen. | `data/monarch.json` (SIM entfernt `perLevel`, `maxLevel`) | – |
| Basiswerte: HP 100, Tempo 5 Units/s (Sprint ×1,8, Beschleunigung 8), Verteidigung 5 (mindert Schaden auf mindestens 1), Respawn heute 5 s, Ziel 15 s mit dem Wiederbeleben (B-120, W4.1; § 5). | Heutiger Stand. | `data/monarch.json` › `base` | – |
| **Einfacher Schlag:** Der Monarch kann zusätzlich zu den Skills **zuschlagen** (Taste X), Nahkampf, Schaden 10, Reichweite 1,5 Units, Abklingzeit 0,7 s (Startwerte). Der Hauptschaden bleibt bei Bogenschützen und Kriegern. | Der Monarch ist von Anfang an im Kampf nützlich, ohne ihn zum Hauptkämpfer zu machen. | `data/monarch.json` › `attack` (SIM legt an) | Anteil am Gesamtschaden einer Welle (Monarch) höchstens 20 % (Kennzahl fehlt, B-099) |

## 2. Klassen und Presets

- **Freie Skillung**, keine feste Klasse (B-017 entschieden). Die Presets Tank, Zauberer, Heiler, (Dieb) sind **nur Startverteilungen** beim Beitritt (Basiswerte aus `monarch.json` › `presets`); danach Respec nach § 3.
- Die Presets sind **auswählbar beim Beitritt** (Standard: Tank für Spieler 1, Zauberer für Spieler 2, wechselbar), kein Zwang zur Ergänzung der Rollen.
- Begründung: Rollen entstehen durch Absprache der Familie, hybride Builds sind erlaubt.
- Daten: `data/monarch.json` › `presets`.

## 3. Skill-Punkte, Baum, Tiers, Respec

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Fund-Pool je Insel, gemeinsam gezählt, persönlich verteilt:** Ein gefundener Skill-Punkt zählt für **jeden** Spieler des Raums; jeder verteilt die Punkte **für sich** in seinem eigenen Baum (drei gefunden = jeder darf drei vergeben). Späte Beitretende bekommen alle bisher gefundenen Punkte. | Keine Konkurrenz um Punkte im Koop, jeder baut seinen Monarchen selbst. | `World.SkillPoints` bleibt der Zähler, Verteilung je Spieler (SIM) | – |
| **Quellen:** versteckt in der Welt 3–5 je Stufe (heute Mittel 3,9), **Miniboss** je Stufe 1, **Endboss** der Insel 3, **Meilenstein** Hub-Stufe 2, 3, 4, 5 erreicht (im ersten Hub, einmalig je Insel) je 1, **jede 3. gefundene Truhe** der Insel 1 Punkt. Etwa 30–35 Punkte je Insel. | Mehr Quellen als nur Verstecke: Erkunden, Kämpfen, Aufbauen. | `data/biomes/*.json` › `skillPoints`, `data/monarch.json` › `skillPointSources` (SIM legt an) | Punkte im Pool nach Tag 10: ≥ 10 in ≥ 70 % |
| **Tier-Gating:** Tier 2 ab 5, Tier 3 ab 10, Tier 4 (Ultimate) ab 15 Punkten in der Linie. | Wie GDD; Spezialisierung wird belohnt. | `data/monarch.json` › `tiers` | – |
| **Respec kostenlos, an der Burg jedes Hubs, nur am Tag** (nicht in Dämmerung und Nacht des globalen Zyklus). | Umskillen zwischen den Nächten, nicht mitten in der Welle. | `monarch.json` | – |
| **Linien zuerst: Tank, Zauberer, Heiler** (je Tier 1 bis 4, Skill-Tabelle in `game-design.md`); der **Dieb** folgt später als Inhalt, bleibt im Regelwerk. | Gruppenrollen zuerst; weniger Aufwand für das erste Spielbare. | `data/monarch.json` › `lines` | – |

Der Skill-Punkte-Pool ist **Teil des Spielstands** (`SAVE_VERSION`), die Verteilung je Spieler auch.

## 4. Slots, Tasten, Menü

Vier Skill-Slots. **Controller:** Schlag = **X**; Skill-Slots 1 bis 4 = **LB, RB, LT, D-Pad hoch**; Skill-Menü = **D-Pad runter**. **Tastatur:** Schlag = **E**; Slots **Q, R, T, Z**; Skill-Menü = **K**. **Touch:** eigene Skill-Tasten im Touch-Overlay (CLI-Ticket). B bleibt unbelegt, View + Menu gemeinsam sind reserviert, LS-Klick bleibt das Debug-Overlay (nur Dev).
**Aktionen-Overlay (überall):** Gültige Aktionen werden **in der Welt als Overlay am Ort angezeigt**, wie bei den Gebäuden die Preis-Münz-Slots: neben dem Spieler bzw. am Ziel erscheint die mögliche Aktion mit der passenden Taste (z. B. „A halten: Wiederbeleben“, „A halten: Bauen“, „X: Schlag“, „A: Eingang nutzen“). Es zeigt immer nur, was gerade gültig ist, passend zum zuletzt benutzten Gerät (Controller, Tastatur, Touch). Gilt für alle Aktionen des Spiels (Bauen, Zahlen, Eingänge, Wiederbeleben, Skills, Truhen).

Begründung: Alle Skills ohne Menü erreichbar, X bleibt die Hauptaktion im Kampf; das Overlay ersetzt Erklärtexte und macht die Steuerung auf dem TV ohne Handbuch verständlich. Dies ersetzt die R1-Aussage „X bleibt für Skills frei“ (`wirtschaft.md` § 6, `stufen.md` § 6).

## 5. Tod und Wiederbeleben

| Regel | Begründung | Zielkorridor |
|---|---|---|
| Ein gefallener Monarch bleibt als **Grabstein** liegen. Ein Mitspieler **belebt ihn wieder**, indem er daneben **A 3 s hält** (Interagieren; es gibt kein Zahlziel in der Nähe): Wiederbelebung am Ort mit **50 % HP**. Ohne Hilfe **Respawn an der Burg der Stufe nach 15 s** mit voller HP. Gilt in jeder Stufe der Insel. | Echtes Koop-Gefühl; der Tod hat Gewicht, ist aber keine Strafe. Bezahlte, nicht fertige Münzen werden wie heute erstattet. | `playerDown` je Welle: Median höchstens 0,5; Anteil Wiederbelebungen an Toden ≥ 30 % (Kennzahl fehlt, B-099) |
| Der Respawn-Wert ändert sich von 5 s auf 15 s (mit B-120, W4.1). **Ereignisse** (Q62, 2026-10-04): `revive` meldet den Respawn nach der Wartezeit, das neue `revived` das Wiederbeleben durch einen Mitspieler. | Zeit für Hilfe; Sound und Anzeige unterscheiden beides. | `data/monarch.json` › `respawnSeconds` |
| **Reichweite** des Wiederbelebens 2 Units (eigener Wert in `data/monarch.json`). **Mehrere Helfer** beschleunigen nicht; keiner lässt dabei eine Münze fallen. Ein **getrennter Monarch** (`Player.Free`) ist nicht wiederbelebbar (Q33, 2026-10-04). | Gleich weit wie Münzen; ein ausgeblendeter Grabstein ist nicht sichtbar. | `data/monarch.json` |

## 6. Offen und Annahmen

- Annahme: Die Presets sind beim Beitritt wählbar (Standard Tank für Spieler 1, Zauberer für Spieler 2); 🧑 hat „Freie Skillung, Presets als Start“ beschlossen, die Auswahl beim Beitritt ist nicht gesondert bestätigt.
- Startwerte für Schlag (Schaden 10, Reichweite 1,5, Abklingzeit 0,7 s), Meilenstein- und Truhen-Punkte, Respawn 15 s, Wiederbelebung (3 s, 50 % HP) und die Zielkorridore sind **Vorschläge des Agenten** ohne gesonderte Bestätigung; Feintuning mit B-099.
- Die Skill-Zahlen und Passive stehen in `game-design.md` (Skill-Tabelle); Heiler-Skill „Resurrection“ wirkt nur auf Truppen, die Wiederbelebung des Monarchen läuft nicht über einen Skill.
- Die Schwierigkeitsgrade (`wirtschaft.md` § 4) ändern den Monarchen nicht.
- Das Aktionen-Overlay ist eine Anforderung an den Client (CLI-Ticket in R3.4, mit Protokoll: gültige Aktionen je Spieler); Gestaltung und Reichweite legt das Ticket fest.
- Offen: Skill-Zahlen und Passive in Daten (SIM-Ticket), Heiler im Koop (kein Wiederbeleben-Skill), Verhalten bei Niederlage-Modus „Stufenverlust“ (Skills und Pool bleiben; Spielstand).

## 7. Reittier

Beschlossen von 🧑 am 2026-10-03 im Chat (Fragenkatalog Q23, Korrektur): Reittiere **von Anfang an**, jeder Monarch reitet ein Standard-Reittier wie im Vorbild. Tierart und Faktoren hat der Agent in F1.3 vorgeschlagen; **bestätigt von 🧑 am 2026-10-03** im Workshop F1.4 (unverändert). Umsetzung: B-152 (SIM, S1), Darstellung: B-173 (CLI, S7).

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| Jeder Monarch reitet von Beginn an ein **Standard-Reittier**; es ist für alle Monarchen gleich. | Wie im Vorbild Kingdom Two Crowns (beschlossen, Q23). | `data/monarch.json` › `mount` (SIM legt an) | – |
| **Tierart: Pferd**, Sprite-Schlüssel `horse` („Braunes Pferd (Galopp)“) aus `data/sprites.json` › `mounts` (bestätigt F1.4). | Das Pferd ist das Reittier des Vorbilds; der Schlüssel ist eines der 13 vorhandenen Tiere. | `monarch.json` › `mount.sprite`; Tiere in `sprites.json` › `mounts` | – |
| **Geschwindigkeitsfaktor 1,0** (bestätigt F1.4): Geschwindigkeit = Basis × Faktor, heute 5 Units/s (`base.speed`; Presets Tank, Zauberer, Heiler 5,0, Dieb 5,5) bleibt unverändert. | Das Reittier soll das Balancing nicht verschieben, nur die Bewegung an einen Datenwert hängen. | `monarch.json` › `mount.speedFactor` | Keine Verschiebung der Kennzahlen aus [`zielkorridore.md`](zielkorridore.md) durch Faktor 1,0 |
| **Sprintfaktor 1,0** (bestätigt F1.4): Sprint wie heute `sprintMultiplier` 1,8 auf das Reittier, Beschleunigung 8 bleibt. | Wie oben: gleiche Bewegung wie heute. | `monarch.json` › `mount.sprintFactor`, `sprintMultiplier`, `acceleration` | wie oben |
| **Immer beritten:** kein Auf- und Absteigen; das Reittier wird weder gekauft noch verloren. Beitritt (auch spät), Wiederverbinden und Stufenwechsel behalten es; ein gefallener Monarch kommt mit Reittier zurück (§ 5). | Standard-Reittier ohne eigene Mechanik; keine neue Taste. | – | – |
| **Weitere Reittiere** (die übrigen Tiere in `sprites.json` › `mounts`) gibt es erst später als Auswahl oder Belohnung; ob das eine Spieloption wird, ist offen (🧑). | Grafik ist vorhanden, Regeln dafür fehlen noch. | – | – |
