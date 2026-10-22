# B-506 · Spielmenü zeigt keine Welt und erlaubt kein Anlegen oder Löschen von Spielständen

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** BD5
- **Projekt:** BED
- **Erstellt:** 2026-10-20
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

Befund aus der Abnahme WZ18.8 (2026-10-20, 🧑 am PC): Welt anlegen, im Spiel starten und Inselwechsel funktionieren. Im Spielmenü fehlt aber:
- Die gewählte bzw. verfügbare Welt wird nicht angezeigt (kein Weltname, keine Auswahl).
- Spielstände lassen sich dort nicht neu anlegen und nicht löschen.
Ein weiterer Punkt des Berichts war leer; Rückfrage an 🧑 offen.

## Ziel

Spielende sehen im Spielmenü, in welcher Welt sie spielen, und legen Spielstände dort an und löschen sie, ohne Dateien von Hand anzufassen.

## Beteiligte und Zielgruppen

Spielende mit Controller und Tastatur; Ralf entscheidet und nimmt ab.

## Anforderungen

Siehe Sprint BD5 › Anforderungen (Lobby zeigt Welt, „Neuer Spielstand“, Löschen mit Rückfrage, `DELETE /api/save`).

## Nicht-Ziele

Welten im Spiel anlegen oder bearbeiten (Asset-Studio); Wiederherstellen gelöschter Stände im Spiel (geht über `/api/save/restore`).

## Regeln und Einschränkungen

Taste B frei, View + Menu reserviert, Texte zweisprachig; Löschen nur mit Rückfrage, Sicherungen bleiben (B-533).

## Beispiele

Spielstand `nord` in Welt `nordmeer` → Lobby-Zeile nennt „Welt nordmeer“. „Neuer Spielstand“ bei vorhandenen `spiel-1` → legt `spiel-2` an.

## Ausnahme- und Fehlerfälle

Stand in offenem Raum → `409`, Hinweis. Server-Fehler → Hinweis, nichts still. Stand ohne Welt-Feld → `standard`.

## Akzeptanzkriterien

- **AC-01** Server: Anlegen (bestehendes `create`) und Löschen (`DELETE /api/save`) mit Welt in der Liste; Löschen eines geladenen Stands abgelehnt (Go-Test).
- **AC-02** Lobby zeigt Welt, legt an und löscht mit Rückfrage (Vitest).
- **AC-03** Ralf hat es am PC abgenommen.

## Offene Fragen

Autonom entschieden: B-533, B-534.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
