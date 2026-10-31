# B-680 · Eine Welt-Datei erbt die Basis-Werte und übersteuert nur, was sie ändern will

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** BAL
- **Erstellt:** 2026-10-31
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-31 autonom, Claude; Entscheidung extends 2026-10-31 🧑 im Chat

## Ausgangslage

Eine Welt-Datei (`data/worlds/<id>.json`) kennt heute nur Inseln mit `depths`, `depthScaling`, `plannedChunks`, `chunkCounts` (`engine/sim/island_data.go`, `docs/rules/welten.md` § 2). Ressourcen, Kosten, Material, Gegner-, Truppen-, Gebäude- und Monarch-Werte stehen global in `data/*.json` und gelten für alle Welten; im Welten-Editor und in der Engine gibt es keinen Override (geprüft 2026-10-31). Welten sind „für sich vollständig“ (§ 1), aber nur bei der Insel-Struktur.

## Ziel

Eine Welt kann beliebige Basis-Werte optional übersteuern (Ressourcen, Kosten, Material-Vorrat, Gegner, Truppen, Gebäude, Monarch, Coop-Multiplikatoren) und erbt alles andere unverändert aus `data/`. Eine Welt kann von der Basis oder von einer anderen Welt erben (`extends`).

## Beteiligte und Zielgruppen

🧑 Ralf entwirft Welten und balanciert sie; Sessions der Domänen REG, SIM, DEV setzen um.

## Anforderungen

- Welt-Datei bekommt optional `overrides`: gleiche Struktur wie die Basisdateien (`data/<datei>.json`), nur die geänderten Felder; Merge-Regel „Feld ersetzt Basiswert, Listen je Schlüssel/ID, nicht als Ganzes“ in `docs/rules/welten.md` festlegen.
- Fehlt `overrides`, ist das Ergebnis byte-gleich zu heute (Golden unverändert, Standard-Welt unverändert).
- Strikte Prüfung: unbekannte Datei, unbekanntes Feld, falscher Typ oder Wert außerhalb der Basis-Regeln werden mit Welt, Datei und Pfad abgelehnt (fail closed).
- Entscheidung 🧑 (2026-10-31): Eine Welt darf mit `extends: <welt-id>` von einer anderen Welt erben (Kette Basis → Welt A → Welt B; Auflösung von der Basis her). `extends` ist optional; ohne es gilt die Basis. Abgelehnt werden: Zyklen, unbekannte Eltern-Welt, Kettentiefe über 4 (Startwert), Löschen/Umbenennen einer Welt, die noch Kind-Welten hat (Meldung nennt die Kinder). Eigenständigkeit bleibt über „Welt kopieren“ (flach machen) möglich.
- `docs/rules/welten.md` § 1 („erbt nichts“) wird auf dieses Modell umgeschrieben; Änderungen an einer Eltern-Welt wirken bewusst auf ihre Kinder, der Editor zeigt die Kind-Welten vor dem Speichern.
- Format-Änderung: Spielstand/Protokoll nur, wenn nötig (Version erhöhen).

## Nicht-Ziele

Umbau der Sim auf Werte je Raum (B-681), Editor (B-682), Chat (B-683).

## Regeln und Einschränkungen

Determinismus (`engine/rng`), 2 Spieler gleichzeitig, `docs/rules/welten.md`, Datei ≤ 400 Zeilen, Funktion ≤ 60. Zusammenhang: B-680 → B-681 → B-682 → B-683.

## Beispiele

Welt „Nordmeer“ setzt nur `overrides` für die Kosten einer Brücke; alle anderen Werte kommen aus `data/`, die Standard-Welt bleibt unverändert.

## Ausnahme- und Fehlerfälle

Ungültiger Override → Welt wird nicht geladen, Meldung nennt Welt, Datei und Pfad.

## Akzeptanzkriterien

- **AC-01** Regelwerk `docs/rules/welten.md` beschreibt Format, Merge und Prüfung von `overrides`.
- **AC-02** Go-Test: Welt ohne `overrides` lädt identisch zur Basis; Welt mit Override ändert nur das genannte Feld.
- **AC-03** Go-Test: ungültige Overrides (unbekannt, Typ, Wertebereich) werden mit Pfad abgelehnt.
- **AC-04** Golden der Standard-Welt unverändert.
- **AC-05** Go-Test: Kette Basis → A → B löst richtig auf; Zyklus, unbekannte Eltern-Welt und zu tiefe Kette werden abgelehnt; Löschen einer Eltern-Welt mit Kindern wird verweigert.

## Offene Fragen

keine. Q1 (`extends`) entschieden von 🧑 am 2026-10-31: Welt von Welt erlaubt.

## Notizen

–
