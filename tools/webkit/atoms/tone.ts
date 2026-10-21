import type { Badge } from '@radix-ui/themes';
import type { ComponentProps } from 'react';

/** Ton eines Status oder Hinweises; jeder Ton hat eine Radix-Farbe. */
export type Tone = 'ok' | 'warn' | 'error' | 'info' | 'neutral';

type Color = NonNullable<ComponentProps<typeof Badge>['color']>;

export const TONE_COLOR: Record<Tone, Color> = { ok: 'green', warn: 'amber', error: 'red', info: 'blue', neutral: 'gray' };
