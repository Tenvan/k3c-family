# humanizer

Lektor-Skill, der typische KI-Muster aus deutschen Texten entfernt: Floskeln, Em-Dash, fettgedruckte Listenpunkte, Konjunktiv-Hedging, blumige Einleitungen, Anglizismen.

## Aufruf

```bash
/humanizer "Text einfügen"   # konkreten Text humanisieren
/humanizer                   # aktuell offenen Text prüfen
```

## Output

Vier Sektionen: `## Entwurf`, `## KI-Audit`, `## Finale Fassung`, `## Änderungen`.

## Struktur

```text
humanizer/
├── SKILL.md                   # Trigger, Workflow, Schnellcheck
├── README.MD                  # diese Datei
├── evals/
│   └── evals.json             # 3 realistische Prompts
└── references/
    ├── patterns.md            # 25 Muster mit deutschen Beispielen
    ├── examples.md            # Vollbeispiele (Vor/Nach)
    └── eval-notes.md          # Bewertungskriterien
```

## Quellen

Wikipedia „Signs of AI writing" (WikiProject AI Cleanup) plus deutsche Erweiterungen aus `references/patterns.md` (siehe Quellenliste am Ende).
