# KR6 · REG · Kritische Fragen zu KMP

- **Status:** aktiv
- **Projekt:** KRT
- **Domäne:** REG
- **Reife:** bereit
- **Tickets:** B-343, B-386, B-328
- **Start-Commit:** 88640930
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-23, Chat (Ralf), Revision 1

## Ausgangslage

Das Projekt KMP (Kampf, Bosse, Events) wurde im autonomen Lauf nur teilweise abgearbeitet (K5, LV1, KB2 aktiv, KB1 geplant). Drei Frage-Tickets, die im Sprint-Betrieb Anfang Oktober entstanden sind, hängen bei 🧑 und haben Regel-, Spielstruktur- oder Protokollwirkung: B-343 (Lage des Endboss-Baus), B-386 (Netz-Budget Q08) und B-328 (Flammen-Fläche des Feuergeists). Sie lagen in KB1 bzw. KB2 und sind nach KR6 verschoben.

## Ziel

Ralf hat die kritischen Fragen zu KMP an einer Stelle entschieden. Am Ende sichtbar: kein offenes Frage-Ticket B-343, B-386, B-328, Folgearbeit mit klarer Entscheidung.

## Beteiligte und Zielgruppen

Entscheidet: 🧑 Ralf. Vorbereitet: Claude. Betroffen: Spielende (Endboss, Feuergeist), Entwicklung (Server, Client, Level-Generator).

## Anforderungen

Je Ticket `B-NNN › Offene Fragen` und `Beispiele` (Frage, Optionen, Folgen):

| Ticket | Thema | Stand |
|---|---|---|
| B-343 | Lage des Endboss-Baus an der inneren Kante des Ausgangs-Chunks, Mindestabstand zu Portalen | umgesetzt (`lairX`), 🧑 bestätigt oder ändert |
| B-386 | Q08 (≤ 200 Byte je Tick und Client) für den ganzen Zustand oder nur für Ereignisse | K4.2 hat auf Ereignisse beschränkt; `delta` Ø 913 B |
| B-328 | Flammen-Fläche des Feuergeists umsetzen (Werte) oder aus `docs/rules/gegner.md` § 1 streichen | nicht umgesetzt |

Nicht aufgenommen: B-327 (Golden-Läufe tiefe Stufen), eine reine Testfrage ohne Spielwirkung; bleibt in KB1.

## Nicht-Ziele

Umsetzung der Folgearbeit (eigene Tickets oder Sprints im Projekt KMP); B-327; Anzeige der Fläche und des Baus (K5).

## Regeln und Einschränkungen

Nur Ralf entscheidet. Spec-Änderungen nach `docs/arbeitsweise.md` › SDD: neue Revision, zurück auf Entwurf, neue Freigabe. B-386 ändert gegebenenfalls B-154/AC-05 (Sprint K4, erledigt); B-343 berührt B-261 (Camp-Abstand) und Q28 (Dichte); Werte nur in `data/`. KR6.1 ist die einzige Session.

## Beispiele

B-343 bleibt → erledigt. B-386 Variante A (Budget gilt für `snap` und `delta`) → neues SRV-Ticket im Projekt KMP für Kompression oder selteneres Senden, B-386 erledigt mit Verweis. B-328 „umsetzen“ → SIM-Ticket mit Radius, Dauer, Schaden je Sekunde.

## Ausnahme- und Fehlerfälle

Ralf kann eine Frage noch nicht beantworten → Ticket bleibt eingeplant in KR6, KR6.1 bleibt offen.

## Akzeptanzkriterien

- **AC-01** B-343, B-386 und B-328 tragen unter Notizen Ralfs Entscheidung.
- **AC-02** Bestätigte Tickets sind erledigt; zu jeder geänderten Entscheidung existiert ein Umsetzungs-Ticket oder Sprint im Projekt KMP.

## Offene Fragen

keine (die Fragen sind der Inhalt der Tickets). Ob K5, LV1 und KB2 weitere offene Fragen in ihren READMEs tragen, ist ungeprüft; dann KR6 erweitern.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KR6.1 | `KR6.1-fragen-kmp.md` | Workshop | Mensch | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
