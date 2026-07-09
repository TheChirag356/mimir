import { registry } from './registry';
import type { NavigationGroup } from './types';

export function getNavigation(): NavigationGroup[] {
	const map = new Map<string, NavigationGroup>();

	for (const page of registry.all()) {
		const category = page.metadata.category;

		if (!map.has(category)) {
			map.set(category, {
				title: category,
				pages: []
			});
		}

		map.get(category)!.pages.push(page);
	}

	return [...map.values()];
}
