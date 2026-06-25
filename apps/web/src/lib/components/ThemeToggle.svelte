<script lang="ts">
	import { mode, setMode } from 'mode-watcher';

	const options = [
		{ value: 'light', label: 'Light' },
		{ value: 'dark', label: 'Dark' },
		{ value: 'system', label: 'Auto' }
	] as const;
</script>

<div class="inline-flex h-9 items-center rounded-full border border-blue-500 bg-transparent p-0.5">
	{#each options as option}
		<button
			type="button"
			onclick={() => setMode(option.value)}
			class:bg-blue-500={mode.current === option.value}
			class:text-white={mode.current === option.value}
			class:shadow-sm={mode.current === option.value}
			class:text-blue-500={mode.current !== option.value}
			class="flex h-full items-center justify-center rounded-full px-4 text-[14px] font-medium tracking-wide transition-all duration-200 outline-none hover:bg-blue-500/10"
		>
			{option.label}
		</button>
	{/each}
</div>
