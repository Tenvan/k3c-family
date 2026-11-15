# B-976 · Die vier vorhandenen Welten haben Wortschatz und Geschichtszeilen

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** GS1
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Format (B-965) und Werkzeug (B-975) stehen, die Welten haben aber noch keine Texte. Vorhandene Welten: `standard` (eingecheckt), `sengoku`, `nil`, `britannien` (lokal in `data/worlds/`, nicht eingecheckt, `docs/rules/welten.md` § 7; sie liegen nur im Hauptcheckout und im Spielpaket `dist/k3c-game/`). Jede hat drei Inseln, `standard` eine. Abgespalten aus B-965 (🧑 2026-11-15, Machart und Entstehung).

## Ziel

`standard`, `sengoku`, `nil` und `britannien` haben je einen eigenen Wortschatz und Zeilen für Start, Etappen, Endgegner und Abschluss jeder Insel, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

Claude schreibt die Texte über `world_story_set`, 🧑 liest sie und nimmt ab. Spielende sehen sie über B-974.

## Anforderungen

Aus B-965 (🧑 2026-11-15, wörtlich):

- Geschichte je Welt und je Insel: Start, Etappen, Endgegner.
- Kunstsprache mit kleinem Wortschatz, dazwischen 2 bis 3 echte Wörter je Zeile als Andeutung.
- **Machart: je Welt eigener Klang.** Jede Welt hat eigene Laute und einen eigenen kleinen Wortschatz (Sengoku japanisch, Nil ägyptisch, Polar nordisch); die echten Wörter tragen den Sinn.
- **Entstehung: Claude schreibt, 🧑 nimmt ab.** Feste Zeilen in der Welt-Datei, geschrieben über die Welten-MCP-Tools.
- Kurz: wenige Zeilen, nichts zum Durchlesen.

Ergänzt:

- **R1 Umfang je Welt:** `lexicon`, `start`, `end`, je vorhandener Etappe eine Etappenzeile, drei Flaschenpost-Einträge in `bottles` (für B-971), je Insel `arrival`, `boss`, `done`; jeder Eintrag 1 bis 2 Zeilen.
- **R2 Klang:** Sengoku japanisch, Nil ägyptisch (wie von 🧑 gewählt); Britannien keltisch-altenglisch, Standard märchenhaft ohne Vorbildsprache (Default). Die Beispielzeilen aus B-965 für Sengoku und Nil werden übernommen.
- **R3 Inhalt:** Die echten Wörter jeder Zeile passen zum Wendepunkt und zur Insel (Name, Biome, Endboss); die Zeilen einer Welt ergeben zusammen einen Bogen von der Ankunft bis zum Sieg. Flaschenpost-Zeilen deuten einen Fund an (z. B. TRUHE, WEG, TIEF), ohne einen bestimmten Ort zu nennen. Keine echten Wörter aus fremden Sprachen, keine Namen realer Personen.
- **R4 Weg:** Geschrieben nur über `world_story_set` im **Hauptcheckout** (die lokalen Welten fehlen in Worktrees); `world_check` meldet danach für keine der vier Welten Fehler oder Hinweise zur Geschichte.
- **R5 Abnahme:** 🧑 bekommt alle Zeilen als Liste je Welt (Ergebnis der Session) und bestätigt oder ändert sie; erst danach gilt das Ticket als erledigt.

## Nicht-Ziele

Welt „Polar“ (gibt es nicht; bekommt ihre Geschichte beim Anlegen), Mechanik der Flaschenpost (B-971), Vertonung, Übersetzungen, weitere Welten einchecken.

## Regeln und Einschränkungen

- `data/worlds/standard.json` ändert sich: REG-Änderung mit Begründung im Commit (`welten.md` § 7); `story` ändert weder Simulation noch Golden (B-965/AC-05), `task go:golden` läuft nicht.
- Die drei lokalen Welten sind nicht eingecheckt; ihr Nachweis ist `world_check` und die Liste im Session-Ergebnis.
- Braucht die neu gebaute Workbench mit `world_story_set` (B-975).

- **Bereich:** `data/worlds/**`
- **Abhängig von:** B-965, B-975
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Sengoku, Start: „Oru kamae KRONE no sen. Shiro nu BRENNT.“ Nil, Start: „Akh'et ra KRONE sen-hotep. Ma'u FLUT kha, kha SAND.“ Britannien (Muster für den Klang): „Dun'ar wyth KRONE an mor. Cael NEBEL thu, thu STEIN.“

## Ausnahme- und Fehlerfälle

- `world_story_set` lehnt eine Zeile ab → Zeile oder Wortschatz anpassen, nicht die Prüfung.
- Lokale Welt fehlt im Checkout → Session bricht mit Hinweis ab (falscher Checkout), nichts wird neu angelegt.
- 🧑 ändert Zeilen bei der Abnahme → Claude trägt sie über `world_story_set` nach, `world_check` bleibt grün.

## Akzeptanzkriterien

- **AC-01** `world_check` meldet für `standard`, `sengoku`, `nil`, `britannien` weder Fehler noch Hinweise zur Geschichte.
- **AC-02** Jede der vier Welten hat `lexicon`, `start`, `end`, drei Einträge in `bottles` und je Insel `arrival`, `boss`, `done` (`world_get` zeigt keinen Eintrag als „fehlt“); die Wortschätze von je zwei Welten überschneiden sich in höchstens 2 Wörtern.
- **AC-03** `check_run task:check:go` grün, Golden unverändert (`git status testdata/golden` leer).
- **AC-04** 🧑 hat die Liste aller Zeilen je Welt gelesen und abgenommen (Vermerk im Session-Ergebnis).

## Offene Fragen

- Klang für Britannien und Standard → Default: Britannien keltisch-altenglisch, Standard märchenhaft-neutral (vorläufig, 🧑 bestätigt).
- Bekommt die eingecheckte Standard-Welt eine Geschichte? → Default: ja, wie jede Welt (vorläufig, 🧑 bestätigt).

## Notizen

Abgespalten aus B-965 (2026-11-15, Zerlegung wegen Größe L). Läuft im Hauptcheckout, weil die lokalen Welten in Worktrees fehlen.
