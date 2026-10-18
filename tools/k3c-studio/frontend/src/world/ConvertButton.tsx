import { Button, Text } from '@radix-ui/themes';
import { useRef, useState } from 'react';
import { CONVERT_HINT, convertChunk, convertLock, inSync, seedObjects } from './convert';
import { slotLabel } from './plan';
import type { PlanState } from './usePlan';

/** Hinweis, wenn sich der Entwurf während des Aufrufs geändert hat (Bearbeiten, Stufe, Laden) */
export const CONVERT_STALE = 'Der Entwurf hat sich inzwischen geändert, bitte erneut umwandeln';

/** Knopf „In feste Objekte umwandeln“ eines geplanten Chunks (B-485/AC-06): nur für einen Chunk mit Chunk-Seed, ohne
 *  feste Objekte und bei geprüftem Entwurf; sonst gesperrt, der Hinweis (title) nennt den Grund. Das Ergebnis gilt nur,
 *  wenn die Quelle der Vorschauen nach dem Aufruf noch dieselbe ist, danach meldet `onDone` den Platz; ein Fehler steht am Chunk, solange sein Seed gilt. */
export function ConvertButton({ plan, seed, index, onDone }: { plan: PlanState; seed: string; index: number; onDone?: (slot: number) => void }) {
  const [error, setError] = useState<{ seed: string; text: string } | null>(null);
  const [busy, setBusy] = useState(false);
  const latest = useRef(plan);
  latest.current = plan;
  const chunk = plan.draft[index];
  if (!chunk) return null;
  const lock = convertLock(chunk, plan.check.ok && inSync(plan.draft, plan.count, plan.source));
  const run = async () => {
    const at = plan.source;
    if (!at) return;
    const { slot } = chunk;
    setBusy(true);
    setError(null);
    const result = await seedObjects(seed, at, slot);
    setBusy(false);
    const now = latest.current;
    const target = now.draft.findIndex((c) => c.slot === slot);
    const fail = result.error || (now.source !== at || target < 0 || !inSync(now.draft, now.count, now.source) ? CONVERT_STALE : '');
    if (fail) { setError({ seed: chunk.seed ?? '', text: fail }); return; }
    now.edit(convertChunk(now.draft, target, result.objects));
    onDone?.(slot);
  };
  return (
    <>
      <Button variant="soft" disabled={lock !== undefined || busy} title={lock ?? CONVERT_HINT}
        aria-label={`In feste Objekte umwandeln Platz ${slotLabel(chunk.slot)}`} onClick={() => void run()}>In feste Objekte umwandeln</Button>
      {error && error.seed === (chunk.seed ?? '') && <Text size="1" color="red" role="alert" className="wd-error">{error.text}</Text>}
    </>
  );
}
