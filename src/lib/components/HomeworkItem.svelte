<script lang="ts">
    import type { Task } from '$lib/types';
    import { EditIcon, TrashIcon } from '$lib/icons';
    import { subtractDates } from '$lib/utils';
    import { getHomeworkList } from '$lib/states';

    type Props = { task: Task; editMode: number | null };
    let { task, editMode = $bindable() }: Props = $props();

    const list = getHomeworkList();
</script>

<article
    class="relative flex flex-col sm:flex-row gap-4 p-6 bg-gray-800/30 rounded-md"
>
    <div class="flex flex-grow flex-row gap-4 items-center">
        <input
            checked={task.done}
            onchange={() => {
                task.done = !task.done;
                list.toggleTask(task.id);
            }}
            type="checkbox"
            class="w-5 h-5 shrink-0 rounded-sm bg-gray-700 checked:bg-emerald-300"
        />
        <p
            class={[
                'flex-grow text-xs sm:text-sm',
                task.done && 'line-through'
            ]}
        >
            {task.title}
        </p>
    </div>
    <p
        class={[
            'flex shrink-0 self-end text-xs sm:text-sm items-center',
            task.done ? 'text-gray-600' : 'text-red-300'
        ]}
    >
        {subtractDates(task.deadline)}
    </p>

    <div
        class="absolute -right-9 sm:-right-18 flex flex-col sm:flex-row items-center gap-2"
    >
        <TrashIcon
            title="delete task"
            className="text-gray-600 transition hover:scale-105 p-1 hover:text-red-300"
            onclick={() =>
                confirm(`Видалити ${task.title}?`) && list.remove(task.id)}
        />
        <EditIcon
            title="edit task"
            className="text-gray-600 transition hover:scale-105 p-1 hover:text-amber-200"
            onclick={() => (editMode = task.id)}
        />
    </div>
</article>
