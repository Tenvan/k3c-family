# B-468 · Geplante Chunks tragen Ruinen und Gegnerlager

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-461 (Chunk-Planung, Sprint WZ8) macht geplante Chunks mit festen Objekten (Ressourcen, Adern, Truhen, Camps, Portale) und Chunk-Seed planbar. 🧑 wollte zusätzlich vorgebaute Gebäude (Ruinen) und feste Gegner (Gegnerlager) in geplanten Chunks; beides passt nicht in denselben Sprint und wurde am 2026-10-17 (Chat) als Teil 2 in dieses Ticket verlegt (B-461 › Offene Fragen 4).

Heute gibt es dafür keine Grundlage:

- **Gebäude** entstehen nur auf Bauplätzen des Hubs (`data/hub.json` › `sites`, `islandSites`, Mauerlinien) und auf Welt-Plätzen neben dem Hub (`worldSiteSpecs`, z. B. Farm). Ein Bauplatz mitten im Level, der schon gebaut ist oder repariert werden muss, existiert nicht. Der Spielstand ordnet Plätze über `SiteSave` und `kind@x` zu; `castleLosses` setzt beim Burgfall Bauplätze leer.
- **Gegner** kommen aus Portalen als Wellen (`docs/rules/gegner.md` § 3: Aggressionspool, Nacht-Welle, Skalierung je Tiefe). Eine zweite Quelle, die dauerhaft an einer festen Stelle steht, gibt es nicht; Camps (`recruitCamp`) liefern Landstreicher, keine Gegner.

Das Format aus B-461 reserviert dafür nur Felder, die der Generator mit „noch nicht unterstützt“ ablehnt.

## Ziel

Ein geplanter Chunk kann zusätzlich eine **Ruine** (vorgebautes, ggf. beschädigtes Gebäude an fester Lage) und ein **Gegnerlager** (feste Gegner an fester Lage) tragen. Beides lässt sich im Editor des Welt-Betrachters (B-461) setzen, wird beim Laden geprüft, erscheint in Chunk- und Live-Vorschau und verhält sich im Spiel nach neuen Regeln. Nutzen: handgeplante Sondergegenden mit Geschichte und Gefahr, nicht nur Ressourcen und Truhen.

## Beteiligte und Zielgruppen

- 🧑 Ralf: entscheidet die Regeln für Ruinen und Gegnerlager (Workshop), plant im Editor, nimmt ab und gibt die Spec frei.
- Spielende (Couch-Koop, 2+ Spieler): treffen auf Ruinen und Gegnerlager an bekannter Stelle der Insel.
- Agenten: setzen Regeltext (REG), Sim und Format (SIM), Vorschau-Snapshot (SRV), Grafik (CLI) und Editor (DEV) um; ein getrennter Reviewer prüft.

## Anforderungen

Grob, bis WZ8 erledigt und der Regel-Workshop gelaufen ist; die Spec wird dann vervollständigt.

- **Ruine (REG):** Regeltext, welche Gebäude als Ruine erlaubt sind, Zustand und HP, ob sie dem Hub der Stufe gehört, ob sie für Hub-Stufe und Linien-Freischaltung zählt, Reparatur (Kosten, Zahlziel), Verhalten beim Burgfall (`castleLosses`), Abstand zu Bauplätzen und Zahlzielen.
- **Ruine (SIM/SRV):** neue Bauplätze neben `worldSiteSpecs`; Spielstand über `SiteSave`/`kind@x`, ggf. neue Spielstand-Version; Snapshot im Format von `level` und `snap` (Protokoll-Wechsel nur, wenn unvermeidbar, dann eigene Entscheidung).
- **Gegnerlager (REG):** neue Wellenquelle neben den Portalen (`rules/gegner.md` § 3), Verhältnis zu Aggressionspool und Nacht-Welle, Skalierung je Tiefe, besiegbar oder dauerhaft, Respawn.
- **Gegnerlager (SIM):** Spawn- und Kampf-Logik in `engine/sim`, deterministisch über eigene Ströme, mit 2 Spielern.
- **Format:** Die in B-461 reservierten Felder des geplanten Chunks werden gelesen, geprüft (Lage, Abstände, Biom-Summen, Zahl der Gegner) und mit Insel, Tiefe, Platz und Grund abgelehnt, wenn ungültig.
- **CLI:** Grafik für Ruine und Gegnerlager. **DEV:** Editor und Vorschau erweitern (Option „geplante Chunks“ der Vorschau-Funktion trägt die neuen Felder).
- Deterministisch, 2+ Spieler, Leistung wie B-461 (Benchmark der Level-Erzeugung ≤ +10 %).

## Nicht-Ziele

- Alles, was B-461 liefert (Format, Chunk-Seed, feste Objekte, Generator, Prüfung, Editor, Schreibregeln); B-468 baut darauf auf.
- Neue Gegner-Arten, Bosse und Balancing der Wellen (eigene Tickets); echte Planungen mit Ruinen oder Gegnerlagern einchecken (REG-Entscheidung von 🧑 nach dem Sprint).
- Mehrere Inseln und Inselwechsel (B-103); Freischaltung je Hub-Stufe (B-462).
- Migration bestehender Spielstände (keine Rücksicht auf alte Spielstände, Beschluss 🧑 bis auf Widerruf).

## Regeln und Einschränkungen

- **Abhängigkeit:** WZ8 (B-461) ist erledigt (Format, Generator, Editor, Vorschau-Option); Folge-Sprint im Projekt WZG nach WZ8.
- **Domänen nacheinander, je Session eine:** REG (Regeltext, Glossar „Ruine“, „Gegnerlager“) → SIM (Format lesen, Prüfung, Sim-Logik) → SRV (Vorschau-Snapshot, Spielstand) → CLI (Grafik) → DEV (Editor). Geschätzt 7–9 Sessions, also ein eigener Sprint, eher zwei (Ruinen, Gegnerlager).
- **Regeln aus B-461 gelten:** Zufall nur über `engine/rng` mit eigenem Strom je geplantem Chunk, Haupt-Strom ohne Planung unverändert, Golden ohne Planung byte-gleich, Test-Planung unter `testdata/`, Werkzeug schreibt REG-Daten nur atomar, format-stabil und mit Stand-Prüfung samt REG-Hinweis; Sprints liefern keine echten Planungen und keine Golden-Änderung aus.
- **Verträge:** Spielstand speichert Plätze über `kind@x` (`SiteSave`); eine neue Spielstand-Version nur mit eigener Entscheidung, keine Rücksicht auf alte Spielstände. Protokoll Version 3 bleibt, solange es geht.
- **Schichten und Budget:** `engine/level` importiert nichts aus `engine/sim`, `engine/room`, `engine/net`; Datei ≤ 400, Funktion ≤ 60 Zeilen; Logging mit Emoji (`CLAUDE.md`).
- **Mit 2 Spielern gleichzeitig** muss jede Mechanik funktionieren; Gamepad-Taste B bleibt unbelegt.

## Beispiele

Grob, Werte und Regeln legt der Workshop fest:

- Wald, Platz +4 mit einer Ruine „Schmiede“ → an fester Lage steht eine beschädigte Schmiede; ein Spieler repariert sie gegen Zahlung am Zahlziel, danach verhält sie sich wie die Hub-Schmiede (Regel offen, Offene Fragen).
- Höhle, Platz −3 mit einem Gegnerlager → feste Gegner stehen dort; sie greifen an, sobald ein Spieler in Reichweite kommt (Regel offen); mit 2 Spielern gilt dasselbe Verhalten.
- Ein Chunk ohne Ruine und Gegnerlager → verhält sich genau wie in B-461.

## Ausnahme- und Fehlerfälle

Grob; die Spec ergänzt sie nach dem Workshop:

- Ruine oder Gegnerlager an ungültiger Lage (Chunk-Grenze, Bauplatz oder Zahlziel zu nah, Portal-Mindestabstand) → beim Laden abgelehnt mit Insel, Tiefe, Platz und Grund (wie B-461).
- Ruine einer Gebäudeart, die es nicht gibt, oder Gegner-Art, die das Biom nicht kennt → abgelehnt.
- Burg fällt (`castleLosses`) → Verhalten der Ruinen laut Regel (offen).
- Spielstand mit Ruine, die danach aus der Planung verschwindet → keine Migration, der Platz geht verloren (Beschluss: keine Rücksicht auf alte Spielstände).

## Akzeptanzkriterien

Grob, Nummern und Nachweise werden mit der Spec vervollständigt.

- **AC-01** Ein REG-Beschluss von 🧑 (Workshop) legt die Regeln für Ruinen und Gegnerlager fest; Regeltext in `docs/rules/`, Glossar-Einträge „Ruine“ und „Gegnerlager“ (aus WZ8.1) ausgearbeitet, vor der ersten SIM-Session.
- **AC-02** Der Generator liest die reservierten Felder des geplanten Chunks, prüft sie unabhängig vom Seed (Lage, Abstände, Summen) und lehnt Ungültiges mit Insel, Tiefe, Platz und Grund ab; je Fall ein Go-Test.
- **AC-03** Ruinen und Gegnerlager verhalten sich in der Sim nach den neuen Regeln, auch mit 2 Spielern (Go-Test mit `CreateIsland`).
- **AC-04** Ohne Ruine und Gegnerlager bleiben alle Golden-Daten und der Benchmark (≤ +10 %) unverändert.
- **AC-05** Vorschau-Funktion, Grafik und Editor zeigen und bearbeiten Ruinen und Gegnerlager; Vorschau und Editor halten die Schreibregeln aus B-461 ein.
- **AC-06** 🧑 nimmt eine Sondergegend mit Ruine und Gegnerlager in Vorschau (und, falls als Planung eingecheckt, im Spiel) ab.

## Offene Fragen

Entscheidet 🧑 im Regel-Workshop (nach WZ8):

1. **Ruine:** welche Gebäude, Zustand und HP, gehört sie dem Hub der Stufe, zählt sie für Hub-Stufe und Linien-Freischaltung, Reparatur (Kosten, Zahlziel), Verhalten beim Burgfall, Abstand zu Bauplätzen und Zahlzielen?
2. **Gegnerlager:** neue Wellenquelle neben den Portalen, Verhältnis zu Aggressionspool und Nacht-Welle, Skalierung je Tiefe, besiegbar oder dauerhaft, Respawn?
3. **Spielstand:** neue Spielstand-Version für Ruinen nötig (Annahme: ja, `SiteSave` reicht eventuell nicht)? Protokoll-Wechsel nötig?
4. **Sprint-Zuschnitt:** ein Sprint (7–9 Sessions) oder zwei (Ruinen, Gegnerlager)?

## Notizen

- Herkunft: Teil 2 der Entscheidung 2 aus B-461 (Entschieden 4, 2026-10-17, Chat: Teilung angenommen, Projekt WZG, Folge-Sprint nach WZ8). Bezug: B-461 (Format, Editor, Vorschau-Option), B-460 (Welt-Betrachter), B-462 (Hub-Freischaltung), B-103 (mehrere Inseln).
- Fundstellen (Stand 2026-10-17, ungeprüft über die Notizen von B-461 hinaus): `worldSiteSpecs`, `SiteSave`/`kind@x`, `castleLosses` in `engine/sim`; Wellenregeln `docs/rules/gegner.md` § 3.
- Aufwand grob 7–9 Sessions (REG, SIM, SIM, SRV, CLI, DEV, Review, Abnahme).
