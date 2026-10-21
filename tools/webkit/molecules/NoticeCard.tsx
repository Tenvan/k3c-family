import { Callout } from '@radix-ui/themes';
import type { ReactNode } from 'react';
import { Text, TONE_COLOR, type Tone } from '../atoms';

/** Hinweis mit Titel und Text (Radix Callout in der Farbe des Tons). */
export function NoticeCard({ title, tone = 'info', children }: { title: string; tone?: Tone; children: ReactNode }) {
  return (
    <Callout.Root color={TONE_COLOR[tone]} role="note" className="wk-notice">
      <Callout.Text>
        <Text as="span" weight="bold" className="wk-notice-title">
          {title}
        </Text>
        <Text as="span" className="wk-notice-body">
          {children}
        </Text>
      </Callout.Text>
    </Callout.Root>
  );
}
