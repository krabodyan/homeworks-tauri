<script lang="ts">
  import { getHomeworkList } from "$lib/states/homeworkList.svelte";
  import { parseDate } from "$lib/utils";
  import InputGroup from "$lib/components/InputGroup.svelte";
  import { stopPropagation } from "svelte/legacy";

  let {
    editMode = $bindable(),
    close,
  }: {
    editMode: number;
    close: () => void;
  } = $props();

  let list = getHomeworkList();
  let task = list.tasks.find((task) => task.id === editMode)!;

  let title = $state(task.title);
  let deadline = $state(parseDate(task.deadline));

  const onsubmit = (e: SubmitEvent) => {
    e.preventDefault();
    task.deadline = new Date(deadline);
    task.title = title;
    list.updateTask(task.id, task.title, task.deadline);
    close();
  };
</script>

{#if editMode !== null}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="fixed inset-0 w-full h-full bg-gray-950/90 flex items-center justify-center z-50"
    onclick={() => close()}
  >
    <div
      class="p-10 rounded-md bg-gray-900 border-2 border-gray-800"
      onclick={(e) => e.stopPropagation()}
    >
      <form class="flex flex-col h-full gap-8" {onsubmit}>
        <InputGroup
          label="Назва"
          name="title"
          bind:value={title}
          inputType="text"
        />
        <InputGroup
          label="Термін здачі"
          name="deadline"
          bind:value={deadline}
          inputType="date"
        />
        <button
          type="submit"
          class="bg-sky-600 rounded-md text-sm p-2 hover:bg-sky-500 transition-colors cursor-pointer mt-auto self-end"
        >
          Зберегти
        </button>
      </form>
    </div>
  </div>
{/if}
