import { Tooltip } from '@radix-ui/themes';
import type { ReactNode } from 'react';

/** Tooltip; ohne Inhalt nur das Kind. */
export function Tip({ content, children }: { content: ReactNode; children: ReactNode }) {
  if (!content) return <>{children}</>;
  return (
    <Tooltip content={content}>
      <span>{children}</span>
    </Tooltip>
  );
}
