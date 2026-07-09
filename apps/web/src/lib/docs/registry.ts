import { getAllDocs } from './loader';
import type { DocPage } from './types';

const docs = getAllDocs();

const slugMap = new Map<string, DocPage>();

for (const doc of docs) {
	slugMap.set(doc.slug, doc);
}

export const registry = {
	docs,

	get(slug: string) {
		return slugMap.get(slug);
	},

	all() {
		return docs;
	}
};
