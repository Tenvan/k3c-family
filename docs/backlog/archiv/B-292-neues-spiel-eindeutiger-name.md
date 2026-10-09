# B-292 · Die Kachel „Neues Spiel“ startet auch bei vorhandenem Spielstand familie

- **Domäne:** PLAT
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** LP1
- **Projekt:** –
- **Erstellt:** 2026-10-05
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-07, Chat, durch 🧑 (mit Sprint LP1)

## Ausgangslage

Die Kachel „Neues Spiel“ (`src/landing/pages.ts`) öffnet `game.html?fresh=1` ohne `save=`. Der Name fällt auf `DEFAULT_SAVE = 'familie'` zurück (`src/scenes/lobbyLogic.ts`). Der Server lehnt `fresh` auf einen vorhandenen Spielstand mit `save_exists` ab (`engine/room/manager.go`); er sichert den alten Stand nicht mehr. Ergebnis: Sobald `saves/familie` existiert, zeigt die Lobby „Spielstand gibt es schon“ und kein neues Spiel startet. Die Testing-Seite geht, weil sie mit `saveName(nonce)` je Start einen frischen Namen erzeugt. Die Kachelbeschreibung („Seed ‚k3c‘ … alter Spielstand wird gesichert“) stimmt nicht mehr.

## Ziel

„Neues Spiel“ startet bei jedem Klick ein neues Spiel, auch wenn schon Spielstände existieren; vorhandene Stände bleiben unberührt.

## Beteiligte und Zielgruppen

Spielende Familie (Landingpage, Xbox/Edge); 🧑 nimmt ab.

## Anforderungen

- Die Kachel übergibt beim Öffnen einen eindeutigen Spielstand-Namen im Format `^[a-z0-9-]{1,32}$` (`href` als Funktion, wird erst beim Öffnen ausgewertet).
- Die Beschreibung der Kachel nennt das tatsächliche Verhalten.
- Kein Server- oder Protokoll-Eingriff.

## Nicht-Ziele

Server sichert beim `fresh`-Start nicht selbst (Variante 2, bei Bedarf eigenes Ticket). Namensdialog in der Lobby: B-105.

## Regeln und Einschränkungen

Domäne PLAT (`src/landing/`). Kein `Math.random()` für Spiel-Logik: Der Name stammt aus der Uhrzeit, der Seed des Levels ist der Name (`engine/room/manager.go`) und damit deterministisch je Name.

## Beispiele

`saves/familie` existiert, Klick auf „Neues Spiel“ → Lobby „Spielen (neu-…)“ → Spiel startet; `familie` bleibt unverändert.

## Ausnahme- und Fehlerfälle

Zwei Klicks in derselben Millisekunde sind nicht möglich (Name aus Zeitstempel in Millisekunden); trifft der Name doch einen vorhandenen Stand, zeigt die Lobby wie bisher „Spielstand gibt es schon“.

## Akzeptanzkriterien

- **AC-01** `task test -- pages`: Der `href` der Kachel „Neues Spiel“ enthält `fresh=1` und einen `save`-Namen im Format `^[a-z0-9-]{1,32}$`; zwei Aufrufe zu verschiedenen Zeiten liefern verschiedene Namen.
- **AC-02** Im Browser (Main Checkout, Server mit vorhandenem Stand `familie`): „Neues Spiel“ erreicht die Lobby ohne Fehlerhinweis, „Spielen“ erzeugt einen Raum (Server-Log `🏰 Raum erstellt`, kein `save_exists`).
- **AC-03** Die Kachelbeschreibung nennt weder „Seed k3c“ noch „alter Spielstand wird gesichert“.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
