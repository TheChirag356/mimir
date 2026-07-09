<script lang="ts">
	import {
		IconInfoCircle,
		IconAlertTriangle,
		IconCircleCheck,
		IconCircleX,
		IconBulb
	} from '@tabler/icons-svelte';

	type Variant = 'info' | 'tip' | 'warning' | 'danger';

	interface Props {
		type?: Variant;
		title?: string;
		children: import('svelte').Snippet;
	}

	let { type = 'info', title, children }: Props = $props();

	const icon = $derived.by(() => {
		switch (type) {
			case 'tip':
				// Note: You originally imported Lightbulb but used CircleCheckBig for 'tip'.
				// If you wanted the bulb, you can swap IconCircleCheck for IconBulb here.
				return IconCircleCheck;

			case 'warning':
				return IconAlertTriangle;

			case 'danger':
				return IconCircleX;

			default:
				return IconInfoCircle;
		}
	});

	const label = $derived(
		title ??
			(type === 'tip'
				? 'Tip'
				: type === 'warning'
					? 'Warning'
					: type === 'danger'
						? 'Danger'
						: 'Info')
	);
</script>

<div class="my-8 flex gap-4 rounded-xl border border-border bg-muted/40 p-5">
	<svelte:component this={icon} class="mt-0.5 h-5 w-5 shrink-0 text-primary" />

	<div class="flex-1">
		<p class="font-medium">
			{label}
		</p>

		<div class="prose prose-neutral dark:prose-invert mt-2 max-w-none">
			{@render children()}
		</div>
	</div>
</div>
