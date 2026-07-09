import { error } from '@sveltejs/kit';
import { registry } from '$lib/docs/registry';
import { getPagination } from '$lib/docs/pagination';

export function entries() {
	return registry.all().map((doc) => ({ slug: doc.slug }));
}

export function load({ params }) {
	const slug = params.slug ?? 'getting-started/introduction';

	const page = registry.get(slug);

	if (!page) {
		throw error(404, 'Page not found');
	}

	return {
		page,
		pagination: getPagination(slug)
	};
}
