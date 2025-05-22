<script lang="ts">
    import type { FilterType } from '$lib/types';
    let { value = $bindable() } = $props();
</script>

{#snippet filterButton(filter: FilterType, text: string)}
    <button
        class={[
            'w-full text-sm p-2 rounded-md transition-colors cursor-pointer',
            value === filter
                ? 'text-white'
                : 'text-gray-600 hover:text-gray-400'
        ]}
        onclick={() => (value = filter)}
    >
        {text}
    </button>
{/snippet}

<div class="flex flex-row">
    {@render filterButton('all', 'Усі')}
    {@render filterButton('todo', 'Не виконані')}
    {@render filterButton('done', 'Виконані')}
</div>
