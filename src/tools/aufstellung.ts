import { installPageChrome } from '../core/shell';
import { installSelection } from './selection';
import { installPadScroll, lineup, renderReference } from './spriteReference';

installPageChrome();
installPadScroll();
renderReference(document.getElementById('cards')!, lineup(), true, installSelection('k3c-auswahl-aufstellung', 'Aufstellung'));
