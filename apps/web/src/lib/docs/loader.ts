import type { DocPage } from './types';

const modules = import.meta.glob('/src/content/docs/**/*.md', {
	eager: true
});

const docs: DocPage[] = Object.entries(modules)
	.map(([path, module]) => {
		const slug = path.replace('/src/content/docs/', '').replace('.md', '');

		const md = module as {
			default: DocPage['component'];
			metadata: DocPage['metadata'];
		};

		return {
			slug,
			path: slug,
			component: md.default,
			metadata: md.metadata
		};
	})
	.sort((a, b) => {
		if (a.metadata.category !== b.metadata.category) {
			return a.metadata.category.localeCompare(b.metadata.category);
		}

		return a.metadata.order - b.metadata.order;
	});

export function getAllDocs() {
	return docs;
}

export function getDoc(slug: string) {
	return docs.find((doc) => doc.slug === slug);
}
