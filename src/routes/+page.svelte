<script lang="ts">
  import Filter from "$lib/components/Filter.svelte";
  import Modal from "$lib/components/Modal.svelte";
  import HomeworkItem from "$lib/components/HomeworkItem.svelte";
  import NewTaskField from "$lib/components/NewTaskField.svelte";
  import type { FilterType, Task } from "$lib/types";
  import {
    createHomeworkList,
    getHomeworkList,
  } from "$lib/states/homeworkList.svelte";
  import { flip } from "svelte/animate";
  import { invoke } from "@tauri-apps/api/core";

  createHomeworkList();

  invoke("load")
    .then((tasks) => {
      for (const task of tasks as Task[]) {
        task.deadline = new Date(task.deadline);
        list.tasks.push(task);
      }
    })
    .catch((err: string) => console.log(err));

  let list = getHomeworkList();

  let editMode = $state<number | null>(null);

  let filter = $state<FilterType>("all");
  let filteredTasks = $derived.by(() => {
    switch (filter) {
      case "all": {
        return list.tasks;
      }
      case "done": {
        return list.tasks.filter((task) => task.done);
      }
      case "todo": {
        return list.tasks.filter((task) => !task.done);
      }
    }
  });
</script>

<svelte:head>
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://fonts.gstatic.com" />
  <link
    href="https://fonts.googleapis.com/css2?family=Onest:wght@100..900&display=swap"
    rel="stylesheet"
  />
  <title>Homeworks</title>
</svelte:head>
<main class="w-full h-full bg-gray-950/97 overflow-y-auto">
  <section class="flex flex-col gap-5 mx-auto max-w-2xl">
    <Filter bind:value={filter} />
    <NewTaskField />
    <ul class="flex flex-col gap-4 ml-4 mr-12 sm:mr-20">
      {#each filteredTasks as task (task.id)}
        <li animate:flip={{ duration: 150 }}>
          <HomeworkItem {task} bind:editMode />
        </li>
      {/each}
    </ul>
  </section>
  {#if editMode !== null}
    <Modal bind:editMode close={() => (editMode = null)} />
  {/if}
</main>

<style lang="postcss">
  @reference "tailwindcss";
  :global(html),
  :global(body) {
    width: 100%;
    height: 100%;
  }
  :global(html) {
    color: theme(--color-white);
    font-family: "Onest", "sans-serif";
    font-weight: 400;
    font-size: theme(--text-xl);
  }
  :global(input) {
    outline: none;
    appearance: none;
  }
</style>
