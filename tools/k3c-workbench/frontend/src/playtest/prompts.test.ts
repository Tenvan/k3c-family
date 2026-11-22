import { describe, expect, it } from 'vitest';
import { createTicketsPrompt, evaluatePrompt } from './prompts';

// B-1224/AC-02: Beide Prompt-Knöpfe füllen den Prompt mit der Session-ID und nennen die MCP-Tools.
describe('Prompts des Spieltests', () => {
  it('Auswerten nennt Session, Lesen, Duplikat-Abgleich und Speichern, aber kein Anlegen', () => {
    const p = evaluatePrompt('2026-11-22-1');
    for (const w of ['2026-11-22-1', 'playtest_get', 'plan_list', 'duplicate_of', 'playtest_findings_set', 'Keine Tickets']) expect(p).toContain(w);
  });

  it('Tickets anlegen nennt Session und das Tool, nur angehakte', () => {
    const p = createTicketsPrompt('2026-11-22-1');
    for (const w of ['2026-11-22-1', 'playtest_tickets_create', 'angehakten', 'Hinweis']) expect(p).toContain(w);
  });
});
