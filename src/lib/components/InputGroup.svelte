<script lang="ts">
  let { name, label, inputType, value = $bindable() } = $props();
</script>

<div class="flex flex-col gap-2">
  <label for={name} class="text-sm text-gray-500">
    {label}
  </label>
  <input
    type={inputType}
    class="text-sm sm:text-base w-full bg-gray-800/30 rounded-md p-3 border-2 border-gray-800 focus:border-sky-600"
    autocomplete="off"
    id={name}
    {name}
    bind:value
    required
  />
</div>
