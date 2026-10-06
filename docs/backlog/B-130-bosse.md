# B-130 · Minibosse und Endboss sind spielbar

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** K2
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Es gibt keine Bosse, weder im Code noch in den Daten (`docs/rules/archiv/ist-gegner-bosse.md` § 3).

## Ziel

Je Stufe ein Miniboss, der mit einer Welle kommt, je Insel ein Endboss in seinem Bau mit drei Phasen; Belohnung, Skalierung und Spielstand wie beschlossen. Nutzen: Meilensteine, Ziel „Endboss“ und Inselwechsel (`docs/rules/bosse.md`).

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen); REG pflegt Werte, B-099 prüft.

## Anforderungen

- `data/bosses.json`: Miniboss je Stufe (Welle 5 im Wald, Welle 3 unten, etwa 8× HP, 2× Schaden, 1 Fähigkeit), Endboss (Bau, ausgelöst bei Ankunft, wartet, etwa 30× HP, 3 Phasen); Belohnung 100/500 Gold + 50/250 Material; Skill-Punkte 1/3 (B-118).
- Skalierung mit Insel-Tabelle und Spieleranzahl der Insel (HP × (1 + 0,5 je Zusatzspieler)).
- Fähigkeiten: Beschwörung (Goblins, Ratten, Kristallspinnen), Flächenschlag, Flammenspur, Splitter-Wurf, Wut (+50 % Tempo bei 25 % HP).
- Besiegte Bosse stehen im Spielstand und kehren nie zurück; Ereignisse `bossSpawned`, `bossDefeated`.
- Endboss-Sieg: schaltet den Inselwechsel frei und löst bei Ziel „Endboss“ den Kampagnensieg aus (B-102, B-103).

## Nicht-Ziele

Darstellung (B-132), Protokoll (B-123, B-104), Grafiken (B-010).

## Regeln und Einschränkungen

`docs/rules/bosse.md`; Werte nur in `data/`; deterministisch; mit 2+ Spielern. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Wald, Welle 5: der Goblin-Anführer kommt über ein Portal und ruft alle 10 s zwei Goblins; nach seinem Tod liegen 100 Gold und 50 Holz am Ort, alle Spieler bekommen den Skill-Punkt.

## Ausnahme- und Fehlerfälle

Spieler stirbt im Bosskampf → wie üblich Wiederbeleben/Respawn. Stufenverlust → Boss bleibt besiegt.

## Akzeptanzkriterien

- **AC-01** Test: Miniboss erscheint mit Welle 5 bzw. 3, Fähigkeit und Belohnung wie in den Daten.
- **AC-02** Test: Endboss wird bei Ankunft ausgelöst, wartet sonst, wechselt die Phasen.
- **AC-03** Test: Boss-HP skaliert mit der Spieleranzahl der Insel.
- **AC-04** Test: Spielstand merkt besiegte Bosse; sie kehren nie zurück.
- **AC-05** Golden-Daten aktualisiert; `task check:go` grün.

## Offene Fragen

Entschieden 2026-10-06 (🧑, `docs/rules/bosse.md` § 1.1 und Annahmen): Boss-Namen und Fähigkeiten-Zahlen sind vorläufige Startwerte, von 🧑 als „Vorschlag“ bestätigt; endgültige Werte über B-099.

## Notizen

Aus R4.3. Abhängig von B-100, B-115, B-128, B-118 (Skill-Punkte). Ergänzt B-103 (Inselwechsel).
