# B-686 · Der Studio-Chat liefert nur Welten, die die Prüfung bestehen, und korrigiert Fehler Schritt für Schritt

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ36
- **Projekt:** WZG
- **Erstellt:** 2026-10-31
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-31 autonom, Claude (Wunsch 🧑 im Chat)

## Ausgangslage

Der Chat des Studios hat eine Welt „Nordsee“ ohne Hafen auf Insel 1 erzeugt; der Welten Editor zeigte „ungültig“ mit
`kein Hafen in den geplanten Chunks der Tiefe 0 …` (`geplante-chunks.md` § 10), am Inselkopf mit dem Dateinamen
`standard.json`, in der Meldung mit `nordsee.json`. Ursachen: `PlanWorldAnswer` prüfte mit der id der Antwort statt mit
der gewählten Welt; die Instruktion (`tools/k3c-studio/backend/app_world_prompt.go`) nannte die Hafen-Regel nicht, sagte
bei `plannedChunks` „im Zweifel weglassen“ und gab nur die Standard-Welt als Beispiel; der Chat bekam den aktuellen
Entwurf nicht und lieferte bei jeder Korrektur die ganze Welt neu; `chunks_planen` prüfte nur die Tiefe, nicht die Welt.

## Ziel

Kein Weltvorschlag und keine Welt-Aktion des Chats ist ausführbar, wenn die Welt die Prüfung des Ladens nicht besteht.
Eine Welt entsteht Schritt für Schritt auf dem aktuellen Entwurf, Fehler korrigiert der Chat selbst oder auf Wunsch.

## Beteiligte und Zielgruppen

🧑 baut Welten im Studio mit dem Chat; Claude (CLI) entwirft.

## Anforderungen

- Vorabprüfung jeder Chat-Antwort mit derselben Prüfung wie Laden/Editor (`sim.ParseWorld`, welten.md § 3,
  `level.ValidatePlanFor`, Hafen-Regel): Welt-Entwurf (Auftrag `welt`) und die Aktionen `welt_anlegen`, `welt_kopieren`,
  `welt_speichern`, `chunks_planen` (ganze Welt nach der Änderung).
- Befunde gehen mit der vollständigen Fehlerliste zurück an den Chat, höchstens 2 Korrekturversuche, danach sieht 🧑 die Fehler.
- Ein fehlender Hafen (Tiefe 0, jede Insel außer der letzten) wird vom Studio deterministisch ergänzt (äußerster freier
  Platz links), mit Hinweis.
- Die Instruktion nennt Hafen-Regel, Endboss-Tiefen, Stufen je Insel, Faktoren > 0, Inselgrößen und ID-Muster aus den Daten,
  mit einem zweiten Beispiel unterschiedlicher Stufenzahl, und verlangt den Änderungsmodus (alles Bestehende behalten).
- Jeder Welt-Auftrag bekommt die aktuelle Welt als Datenbestand mit Prüfergebnis: Entwurf des Editors, sonst letzter
  Vorschlag des Chats, sonst die gespeicherte Datei.
- Im Welten Editor: Fehlerbanner mit der Fehlerliste der gespeicherten Datei und Knopf „Im Chat korrigieren“.

## Nicht-Ziele

Fortsetzen der Claude-Sitzung (B-687). Prüfung beim Speichern im Stufen-Editor ohne Chat (bleibt wie bisher).

## Regeln und Einschränkungen

Domäne DEV (Studio), `engine/` unverändert. Der Chat schreibt nie; ausgeführt wird nur per Knopf (B-679).

## Beispiele

„3 Inseln, jede eine Stufe mehr“ → Antwort ohne Endboss auf Insel 3 → Studio schickt den Fehler zurück → gültige Welt
`[0, 4]`, `[0, 1, 4]`, `[0, 1, 2, 4]` mit ergänzten Häfen.

## Ausnahme- und Fehlerfälle

Bleibt die Antwort nach 2 Korrekturen ungültig → Hinweis mit allen Fehlern am Ende der Antwort, „Als Entwurf laden“ lädt
nichts, Karten bleiben gesperrt.

## Akzeptanzkriterien

- **AC-01** Ungültige Welt-Antworten und Welt-Aktionen werden nicht angeboten, sondern mit Meldung zurückgegeben
  (`chat_world_test.go` › `TestChatAktionenWeltVorabGeprueft`, `TestChatWeltKorrekturHoechstensZweimal`).
- **AC-02** Die Meldung nennt die Datei der gewählten Welt (`TestAntwortDateinameUndHafen`, `TestAntwortMitFehlerBleibtEntwurfMitGrund`).
- **AC-03** Ein fehlender Hafen wird ergänzt (`TestAntwortDateinameUndHafen`).
- **AC-04** Die Instruktion enthält Hafen-Regel, Änderungsmodus und aktuelle Welt mit Fehlerliste; ein mehrschrittiger
  Verlauf setzt auf dem Vorschlag auf (`TestChatWeltSchrittFuerSchritt`, `TestKontextMitEntwurfUndFehlerliste`).
- **AC-05** Vollständige Fehlerliste (`TestWeltFehlerlisteVollstaendig`).
- **AC-06** „Im Chat korrigieren“ legt alle Fehler in die Chat-Eingabe; der Entwurf geht mit (`FixInChat.test.tsx`,
  `ChatDraft.test.tsx`).
- **AC-07** Abnahme durch 🧑: im Studio eine Welt mit mehreren Inseln im Chat bauen und einen Fehler über
  „Im Chat korrigieren“ beheben lassen.

## Offene Fragen

keine

## Notizen

–
