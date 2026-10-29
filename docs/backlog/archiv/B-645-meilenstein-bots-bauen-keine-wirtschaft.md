# B-645 · Der Meilenstein-Lauf coop2 scheitert an den Bots, nicht an den Werten: Es entsteht keine Wirtschaft

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** verworfen
- **Sprint:** BS3
- **Projekt:** BAS
- **Erstellt:** 2026-10-26
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

BS3.4 (Meilenstein 1): Der Insel-Lauf `coop2`, Welt `standard`, Seed `k3c-dev`, 40 Tage bleibt auf der Münzskala rot, fünf Diagnose-Läufe (`run-1` bis `run-5`, Stand Commit nach BS3.3). Der Endboss erscheint nur, wenn ein Monarch die Kristallhöhle erreicht (60 % der Läufe mit dem alten Abstiegs-Notausstieg), und besiegt wird er in 0 %. Messwerte: Hub-Stufe am Ende 1 auf jeder Stufe, Kämpfer am Ende 0, Leerlauf der Bauern 94–100 %, Burg der Startstufe fällt ab Nacht 3 bis 5, auf der Kristallhöhle fielen im Mittel 700 bis 1700 Monarchen (Respawn-Schleife am Endboss). `wirtschaft` statt `coop2` (`run-4`) ist besser (Leerlauf 44–69 % an Tag 1–2), scheitert aber ebenso: Burg fällt ab Nacht 3, keine Hub-Stufe 2, keine Kämpfer.

## Ziel

Der Meilenstein-Lauf kann grün werden: Die Bots bauen auf der Münzskala eine Wirtschaft auf (Hub-Stufe, Kämpfer, Material), und der Endboss ist mit den Werten aus BS2 erreichbar.

## Beteiligte und Zielgruppen

🧑 entscheidet über Profil und Werte, Claude setzt um.

## Anforderungen

- Befund (belegt in `reports/simtest-run-1` bis `-5`): `coop2` markiert weder Bäume noch Felsen (die Rollen „Mauern zuerst“ und „Wirtschaft zuerst“ kennen nur Mauern und Anwerben); die Bauern arbeiten deshalb nie, Holz und Stein kommen nicht nach. Mit Startgold 14 und Tageseinkommen 2 trägt das keinen Hub-Ausbau (Stufe 2: 4 + 4 + 6 Gold, 120 Stein).
- Befund: Wer beide Monarchen absteigen lässt (Regel aus B-643), lässt den Hub unverteidigt; steigt nur einer ab, bleibt er in der Höhle hängen (Linie-1-Mauern allein mit 14 Gold, kein Bauer).
- Befund: Der Monarch allein schlägt den Endboss nie (10 Schaden je 0,7 s gegen einige 10 000 HP: `bosses.json` Faktor 30 mal Tiefen-Skalierung 1,5^4 mal Spielerfaktor); nur eine Armee aus Truppen kann das leisten.
- Entscheidung nötig: (A) `coop2` bekommt die Bausteine von `wirtschaft` (Markieren, Bögen, Hub) je Rolle, der Meilenstein misst dann dieses Profil; (B) eigener Bot „Insel“ mit Aufgabenteilung (Verteidiger am Hub, Material-Läufer in der Höhle); (C) zusätzlich Werte: Endboss-HP senken, Startgold oder Tageseinkommen anheben. Empfehlung (ungeprüft): A, danach höchstens zwei Werte-Runden.

## Nicht-Ziele

Neue Sim-Mechanik; Läufe über alle Seeds (BS6).

## Regeln und Einschränkungen

Bots nur über `sim.PlayerCommand`; deterministisch; Werte nur in `data/` nach `wirtschaft.md` § 8; Meilenstein verlangt `coop2` (`zielkorridore.md` § 6, Änderung nur durch 🧑).

## Beispiele

`sim_test {bots: coop2, world: standard, days: 40}` heute: Hub-Stufe 1, Kämpfer 0, Endboss 0 %. Ziel: Endboss besiegt, Schiff (Bedarf) mit Ampel.

## Ausnahme- und Fehlerfälle

Bleibt es nach A und zwei Werte-Runden rot: Befund an 🧑, kein weiteres Justieren ohne Entscheidung.

## Akzeptanzkriterien

- **AC-01** `sim_test {bots: coop2, world: standard, days: 40}` (Seed `k3c-dev`): Endboss Insel 1 besiegt (`endbossDefeated` 100 %), Bericht mit Schiff-Zeile.

## Offene Fragen

A, B oder C (siehe Anforderungen); ob der Meilenstein-Bot `coop2` oder ein neues Profil heißt (🧑).

## Notizen

2026-10-29 (Claude, Sichtung): verworfen, überholt. Seit den Bot-Änderungen BS12.4 baut coop2 eine Wirtschaft und besiegt den Titan (Tag 26 bis ecf262ec); der Rest ist B-651.
