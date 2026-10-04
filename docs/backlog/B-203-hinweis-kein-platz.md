# B-203 · Über dem Hub erscheint ein Hinweis, wenn für ein Gebäude kein Platz ist

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Beschluss Q26 (2026-10-04, `docs/fragenkatalog.md`): Der Hub wächst mit dem Ausbau (Hub-Stufe und entsprechender Mauerausbau) auf eine Breite, die den freigeschalteten Gebäuden Platz gibt; erweitert wird nur mit entsprechendem Mauerausbau. Fehlt der Platz, soll über dem Hub ein Hinweis „Kein Platz für <Gebäude>“ erscheinen. Der Client zeigt heute nichts dergleichen (`src/scenes/worldRenderer.ts`).

## Ziel

Spieler sehen, warum ein freigeschaltetes Gebäude nicht baubar ist, und wissen, dass erst die Mauer ausgebaut werden muss.

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen, Split-Screen); Entwickler (CLI); 🧑 gibt die Spec frei.

## Anforderungen

- Über dem Hub erscheint der Hinweis „Kein Platz für <Gebäude>“, solange ein freigeschaltetes Gebäude mangels Platz keinen Bauplatz hat (Beschluss Q26, 2026-10-04).
- Der Client rechnet nichts: Welches Gebäude keinen Platz hat, kommt aus dem Snapshot.
- Text zentral (Deutsch und Englisch, B-172), lesbar im Split-Viertel (`docs/rules/bedienung.md` § 2).

## Nicht-Ziele

Hub-Wachstum und Platzvergabe in der Simulation (B-112, W1); Protokoll-Feld (W5); Grafik (B-010).

## Regeln und Einschränkungen

`CLAUDE.md` (Client zeichnet nur, Datei ≤ 400 Zeilen, Funktion ≤ 60); Domäne CLI; Beschluss Q26.

## Beispiele

Hub-Stufe 3 erreicht, Mauer noch auf Stufe 2 → über dem Hub „Kein Platz für Schmiede“; nach dem Mauerausbau verschwindet der Hinweis.

## Ausnahme- und Fehlerfälle

Mehrere Gebäude ohne Platz → ein Hinweis, der alle nennt oder nacheinander zeigt (Gestaltung im Sprint). Server liefert die Information nicht → kein Hinweis, kein Fehler.

## Akzeptanzkriterien

- **AC-01** Test: Eine reine Funktion bildet aus dem Snapshot den Hinweistext „Kein Platz für <Gebäude>“; ohne fehlenden Platz kein Text.
- **AC-02** Der Hinweis erscheint über dem Hub und verschwindet, sobald der Platz da ist.
- **AC-03** `task check` grün.

## Offene Fragen

Welches Feld des Snapshots die fehlenden Plätze trägt, legen W1 und W5 fest (ungeprüft, ob W1 es schon vorsieht).

## Notizen

Entstanden aus Beschluss Q26 (Fragenkatalog Block 5).
