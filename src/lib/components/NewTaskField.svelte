<script lang="ts">
  import { getHomeworkList } from "$lib/states";

  let list = getHomeworkList();
  let title = $state("");

  const onsubmit = (e: SubmitEvent) => {
    e.preventDefault();
    list.add(title);
    title = "";
  };
</script>

<form class="flex m-4 gap-x-4" {onsubmit}>
  <input
    required
    type="text"
    bind:value={title}
    class="w-full bg-gray-800/30 rounded-md p-2 sm:p-3 border-2 border-gray-800 focus:border-sky-600"
  />
  <button
    type="submit"
    class="bg-sky-600 rounded-md text-sm sm:text-base p-2 sm:p-3 hover:bg-sky-500 transition-colors cursor-pointer"
  >
    Додати
  </button>
</form>
