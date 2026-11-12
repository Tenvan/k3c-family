# B-813 · Burgfall-Banner, Skill-Menü, Hauptmenü nach dem Verlassen und Siegmeldung zeigen im nativen Spiel das Richtige

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** NG13
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

Befunde aus dem Ende-zu-Ende-Lauf NG9.1: (1) Das Burgfall-Banner ist abgeschnitten und nennt bei Niederlage `lost` die Verluste des Modus `stage`. (2) Die Kopfzeile des Skill-Menüs überlappt die erste Zeile. (3) Das Hauptmenü zeigt nach „Spiel verlassen“ ein Bild lang „Kein Spielstand vorhanden“, weil Draw vor dem ersten Update läuft. (4) `victory` wird nur gesehen, wenn ein lokaler Spieler auf Stufe 0 steht. Code in `native/k3c-game/hud` und `native/k3c-game/menu`.

## Ziel

Banner, Skill-Menü, Hauptmenü und Siegmeldung zeigen im nativen Spiel den richtigen Inhalt ohne Flackern.

## Beteiligte und Zielgruppen

Spieler, Claude (Umsetzung).

## Anforderungen

- Banner passt in das Bild und nennt bei `lost` die Verluste des passenden Modus.
- Skill-Menü: Kopfzeile und erste Zeile überlappen nicht.
- Hauptmenü zeigt im ersten Bild nach dem Verlassen den richtigen Spielstand-Zustand.
- `victory` wird unabhängig von der Stufe des lokalen Spielers gemeldet.

## Nicht-Ziele

Neue Texte, neue Regeln.

## Regeln und Einschränkungen

Entscheidung 008, Datei ≤ 400 Zeilen. Der Ende-zu-Ende-Lauf (`TestFlow`) dient als Regressionsschutz.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `native/k3c-game/hud/**`, `native/k3c-game/menu/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

Burg fällt bei Niederlage → Banner vollständig, Verluste des Modus `lost`.

## Ausnahme- und Fehlerfälle

Spielstand lädt langsam → Hauptmenü zeigt „lädt“ statt „Kein Spielstand vorhanden“.

## Akzeptanzkriterien

- **AC-01** Test: Banner-Rechteck liegt im Bild; Text enthält bei `lost` die Verluste des Modus `lost`.
- **AC-02** Test: Skill-Menü-Zeilen überlappen nicht (Rechteck-Vergleich).
- **AC-03** Test: erstes Draw nach „Spiel verlassen“ zeigt keinen falschen Zustand.
- **AC-04** Test: `victory` wird auch gemeldet, wenn kein lokaler Spieler auf Stufe 0 steht.

## Offene Fragen

keine

## Notizen

Gefunden in NG9.1 (2026-11-11).
