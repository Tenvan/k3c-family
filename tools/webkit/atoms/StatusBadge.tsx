import { Badge } from '@radix-ui/themes';
import type { ReactNode } from 'react';
import { TONE_COLOR, type Tone } from './tone';

/** Status als Pille mit Punkt in der Farbe des Tons. */
export function StatusBadge({ tone, children }: { tone: Tone; children: ReactNode }) {
  return (
    <Badge color={TONE_COLOR[tone]} variant="soft" radius="full" size="2" className="wk-status" data-tone={tone}>
      <span className="wk-status-dot" aria-hidden />
      {children}
    </Badge>
  );
}
