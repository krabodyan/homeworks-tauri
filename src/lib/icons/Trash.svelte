<script lang="ts">
    import type { ClassValue } from 'svelte/elements';

    type Props = { title: string; onclick: () => void; className: ClassValue };
    let { title, onclick, className }: Props = $props();
</script>

<button aria-label={title} class={className} {onclick}>
    <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        ><path
            fill="currentColor"
            d="M19 4h-3.5l-1-1h-5l-1 1H5v2h14M6 19a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V7H6z"
        /></svg
    >
</button>
