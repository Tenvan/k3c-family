# B-426 · Kleine Befunde aus dem Review von SV1 sind erledigt

- **Domäne:** SRV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** SV2
- **Projekt:** WRT
- **Erstellt:** 2026-10-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-25

## Ausgangslage

Geringe Befunde aus dem unabhängigen Review von PR #293 (Sprint SV1):
1. Biom-Wahl wird ignoriert: B-315 verlangt „Seed und Biom wie im Level-Betrachter gewählt“, der Voll-Stand startet immer in Tiefe 0 (`src/tools/levelApi.ts`, `fullSaveTarget`).
2. Tests von FullSave laufen nur gegen Speicher-Stores (`memSaves`, Raum-Fixture), nicht gegen `store.Saves` mit seinem Namens-Regex.
3. `POST /api/dev/fullsave` (`engine/net/dev.go`) prüft weder Content-Type noch Origin; im Dev-Mode kann jede Webseite im LAN-Browser `voll-*`-Stände anlegen (CSRF, nur Dev).
4. Taste V im Level-Betrachter (`src/tools/leveltest.ts`) löst auch mit Strg/Alt aus (Strg+V).

## Ziel

Die kleinen Lücken sind geschlossen oder bewusst verworfen.

## Beteiligte und Zielgruppen

🧑 entscheidet bei Punkt 1 (Spec von B-315).

## Anforderungen

Je Punkt: umsetzen oder mit Grund verwerfen. Punkt 1 und 4 betreffen DEV/PLAT (`src/tools/`), Punkt 2 und 3 SRV; beim Einplanen nach Domänen trennen.

## Nicht-Ziele

B-424, B-425.

## Regeln und Einschränkungen

Domänen nicht in einer Session mischen.

## Beispiele

Strg+V im Level-Betrachter startet nichts; V allein startet.

## Ausnahme- und Fehlerfälle

nicht relevant (Sammel-Ticket)

## Akzeptanzkriterien

- **AC-01** Punkt 1: Entscheidung von 🧑 dokumentiert und umgesetzt (Startstufe nach Biom oder Spec von B-315 angepasst).
- **AC-02** Punkt 2: Ein Go-Test legt `voll-<seed>` über einen echten `store.Saves` (TempDir) an.
- **AC-03** Punkt 3: `/api/dev/fullsave` verlangt `Content-Type: application/json` (oder prüft Origin); Test für die Ablehnung.
- **AC-04** Punkt 4: V mit Strg, Alt oder Meta löst nichts aus.

## Offene Fragen

- Punkt 1: Startstufe nach gewähltem Biom (braucht Startstufe im Autostart, B-095) oder Spec von B-315 auf Tiefe 0 ändern?

## Notizen

Aus dem unabhängigen Review von PR #293 (2026-10-14); hängt an B-315.
Entscheidung 🧑 2026-10-18 (Abnahme WZ7.7): Punkt 1 und Punkt 4 „passt“ im Studio-Welt-Betrachter, nichts weiter zu tun; beide gelten damit als verworfen (`leveltest.ts` ist entfernt, AC-01 und AC-04 entfallen). Offen bleiben Punkt 2 (AC-02) und Punkt 3 (AC-03), beide SRV.
