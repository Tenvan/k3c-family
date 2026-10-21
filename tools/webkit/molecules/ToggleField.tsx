import type { ComponentProps, ReactNode } from 'react';
import { Checkbox, Flex, Switch, Text } from '../atoms';

type Size = '1' | '2' | '3';

interface Base {
  label: ReactNode;
  size?: Size;
  /** Klassen am Label, z. B. für das Layout einer Zeile. */
  className?: string;
  title?: string;
}

type CheckProps = Base & Omit<ComponentProps<typeof Checkbox>, 'size' | 'className' | 'title'>;
type SwitchProps = Base & Omit<ComponentProps<typeof Switch>, 'size' | 'className' | 'title'>;

/** Kontrollkästchen mit Beschriftung; ein Klick auf den Text schaltet mit. */
export function CheckField({ label, size = '2', className, title, ...box }: CheckProps) {
  return (
    <Text as="label" size={size} className={className} title={title}>
      <Flex as="span" gap="2" align="center">
        <Checkbox size={size} {...box} />
        {label}
      </Flex>
    </Text>
  );
}

/** Schalter mit Beschriftung (z. B. „Dark“ in der Kopfzeile). */
export function SwitchField({ label, size = '2', className, title, ...sw }: SwitchProps) {
  return (
    <Text as="label" size={size} className={className} title={title}>
      <Flex as="span" gap="2" align="center">
        <Switch size={size} {...sw} />
        {label}
      </Flex>
    </Text>
  );
}
