import { useState } from 'react';
import { Flex, SelectField, Text, TextField } from '@webkit/atoms';
import type { WorldDetail } from '../api';
import { PromptRow } from './PromptRow';
import { PROMPTS, worldPrompt, type PromptId } from './worldPrompts';

// „Welt ändern lassen“ (B-1216): ein Prompt aus der Liste wählen (Insel hinzufügen, Größe, Biome, Skalierung, geplante
// Chunks, Natur, Etappe, Override, neue Welt), Insel und Wunsch eingeben, kopieren oder in Claude öffnen. Die Welt-Datei
// schreibt Claude über die MCP-Tools; hier entsteht nur der Auftrag.

export function WorldPrompts({ detail }: { detail: WorldDetail }) {
  const [id, setId] = useState<PromptId>('island-add');
  const [island, setIsland] = useState('');
  const [wish, setWish] = useState('');
  const spec = PROMPTS.find((p) => p.id === id)!;
  const chosen = detail.islands.some((i) => i.id === island) ? island : detail.islands[0]?.id ?? '';
  return (
    <section className="wd-card" aria-label="Welt ändern lassen">
      <Text size="2" weight="bold">Welt ändern lassen</Text>
      <Flex gap="2" align="center" wrap="wrap">
        <SelectField size="1" aria-label="Prompt" value={id} onChange={(v) => setId(v as PromptId)}
          options={PROMPTS.map((p) => ({ value: p.id, label: `${p.icon} ${p.name}` }))} />
        {spec.island && detail.islands.length > 0 && (
          <SelectField size="1" aria-label="Insel für den Prompt" value={chosen} onChange={setIsland}
            options={detail.islands.map((i) => ({ value: i.id, label: `${i.name} (${i.id})` }))} />
        )}
        <TextField.Root size="1" className="wd-wish" aria-label="Was soll sich ändern?" placeholder={spec.hint} value={wish} onChange={(e) => setWish(e.target.value)} />
        <PromptRow prompt={worldPrompt(detail, id, chosen, wish)} title={`${spec.name} in Claude ändern`} />
      </Flex>
    </section>
  );
}
