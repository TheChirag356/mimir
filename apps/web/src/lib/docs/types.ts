export interface DocMetadata {
	title: string;
	description: string;
	category: string;
	order: number;
	lastUpdated?: string;
	editUrl?: string;
}

export interface DocPage {
	slug: string;
	component: unknown;
	metadata: DocMetadata;
}

export interface TocItem {
	id: string;
	text: string;
	level: number;
}
