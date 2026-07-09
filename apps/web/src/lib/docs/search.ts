import { registry } from './registry';

export interface SearchItem {
	title: string;
	description: string;
	category: string;
	href: string;
}

const searchIndex: SearchItem[] = registry.all().map((page) => ({
	title: page.metadata.title,
	description: page.metadata.description,
	category: page.metadata.category,
	href: `/docs/${page.slug}`
}));

export function getSearchIndex() {
	return searchIndex;
}
