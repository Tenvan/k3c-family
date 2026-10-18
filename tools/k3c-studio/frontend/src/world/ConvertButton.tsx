import { Button, Text } from '@radix-ui/themes';
import { useState } from 'react';
import { CONVERT_HINT, convertChunk, convertLock, inSync, seedObjects } from './convert';
import { slotLabel } from './plan';
import type { PlanState } from './usePlan';

/** Knopf „In feste Objekte umwandeln“ eines geplanten Chunks (B-485/AC-06): nur für einen Chunk mit Chunk-Seed, ohne
 *  feste Objekte und bei geprüftem Entwurf; sonst gesperrt, der Hinweis (title) nennt den Grund. Ein Fehler steht am Chunk. */
export function ConvertButton({ plan, seed, index }: { plan: PlanState; seed: string; index: number }) {
  const [error, setError] = useState('');
  const [busy, setBusy] = useState(false);
  const chunk = plan.draft[index];
  if (!chunk) return null;
  const lock = convertLock(chunk, plan.check.ok && inSync(plan.draft, plan.count, plan.source));
  const run = async () => {
    if (!plan.source) return;
    setBusy(true);
    setError('');
    const result = await seedObjects(seed, plan.source, chunk.slot);
    setBusy(false);
    if (result.error) { setError(result.error); return; }
    plan.edit(convertChunk(plan.draft, index, result.objects));
  };
  return (
    <>
      <Button variant="soft" disabled={lock !== undefined || busy} title={lock ?? CONVERT_HINT}
        aria-label={`In feste Objekte umwandeln Platz ${slotLabel(chunk.slot)}`} onClick={() => void run()}>In feste Objekte umwandeln</Button>
      {error && <Text size="1" color="red" role="alert" className="wd-error">{error}</Text>}
    </>
  );
}
