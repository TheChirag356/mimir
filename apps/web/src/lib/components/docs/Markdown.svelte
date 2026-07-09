<script lang="ts">
	let { children } = $props();
</script>

<article
	class="
	prose
	prose-zinc
	dark:prose-invert

	max-w-none

	prose-headings:scroll-mt-24

	prose-h1:hidden

	prose-h2:text-3xl
	prose-h2:font-semibold
	prose-h2:mt-16

	prose-h3:text-2xl
	prose-h3:mt-10

	prose-p:text-base
	prose-p:leading-8

	prose-a:text-primary
	prose-a:no-underline
	hover:prose-a:underline

	prose-img:rounded-xl

	prose-code:before:hidden
	prose-code:after:hidden

	prose-pre:border
	prose-pre:border-border
	prose-pre:rounded-xl
	prose-pre:bg-muted/30

	prose-blockquote:border-primary
	prose-blockquote:not-italic
	"
>
	{@render children()}
</article>
