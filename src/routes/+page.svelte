<script lang="ts">
    import type { FilterType, Task } from '$lib/types';
    import { createHomeworkList, getHomeworkList } from '$lib/states';
    import { NewTaskField, Filter, Modal, HomeworkItem } from '$lib/components';
    import { flip } from 'svelte/animate';
    import { invoke } from '@tauri-apps/api/core';

    createHomeworkList();

    invoke('load')
        .then((tasks) => {
            for (const task of tasks as Task[]) {
                task.deadline = new Date(task.deadline);
                list.tasks.push(task);
            }
        })
        .catch((err: string) => console.log(err));

    let list = getHomeworkList();

    let editMode = $state<number | null>(null);

    let filter = $state<FilterType>('all');
    let filteredTasks = $derived.by(() => {
        switch (filter) {
            case 'all': {
                return list.tasks;
            }
            case 'done': {
                return list.tasks.filter((task) => task.done);
            }
            case 'todo': {
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
</svelte:head>

<main class="w-full h-full bg-gray-950/97 overflow-y-auto pb-10">
    <section class="flex flex-col gap-4 mx-auto max-w-2xl mt-6">
        <NewTaskField />
        <Filter bind:value={filter} />
        <ul class="flex flex-col gap-4 ml-4 mr-12 sm:mr-20">
            {#each filteredTasks as task (task.id)}
                <li animate:flip={{ duration: 150 }}>
                    <HomeworkItem {task} bind:editMode />
                </li>
            {/each}
        </ul>
    </section>
    {#if editMode != null}
        <Modal bind:editMode close={() => (editMode = null)} />
    {/if}
</main>
