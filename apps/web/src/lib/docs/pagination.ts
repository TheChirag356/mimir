import { getNavigation } from './navigation';
import type { DocPage } from './types';

const pages: DocPage[] = getNavigation().flatMap((group) => group.pages);

export interface Pagination {
	previous?: DocPage;
	next?: DocPage;
}

export function getPagination(slug: string): Pagination {
	const index = pages.findIndex((page) => page.slug === slug);

	if (index === -1) {
		return {};
	}

	return {
		previous: pages[index - 1],
		next: pages[index + 1]
	};
}
