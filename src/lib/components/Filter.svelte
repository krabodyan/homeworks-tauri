<script lang="ts">
  import type { FilterType } from "$lib/types";
  let { value = $bindable() } = $props();
</script>

{#snippet filterButton(filter: FilterType, text: string)}
  {@const className = `
    w-full p-2 mt-2 rounded-md transition-colors cursor-pointer
    ${value === filter ? "text-white" : "text-gray-600 hover:text-gray-400"}
  `}
  <button class={className} onclick={() => (value = filter)}>
    {text}
  </button>
{/snippet}

<div class="flex flex-row gap-x-2">
  {@render filterButton("all", "Усі")}
  {@render filterButton("todo", "Не виконані")}
  {@render filterButton("done", "Виконані")}
</div>
