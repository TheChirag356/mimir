import { writable } from 'svelte/store';
import type { TocItem } from './types';

export const toc = writable<TocItem[]>([]);
