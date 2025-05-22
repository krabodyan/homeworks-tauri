<script lang="ts">
    import { getHomeworkList } from '$lib/states';
    import { parseDate } from '$lib/utils';
    import { InputGroup } from '$lib/components';

    type Props = { editMode: number; close: () => void };
    let { editMode = $bindable(), close }: Props = $props();

    const list = getHomeworkList();
    const task = list.tasks.find((task) => task.id === editMode)!;

    let title = $state(task.title);
    let deadline = $state(parseDate(task.deadline));

    const onsubmit = (e: SubmitEvent) => {
        e.preventDefault();
        task.deadline = new Date(deadline);
        task.title = title;
        list.updateTask(task.id, task.title, task.deadline);
        close();
    };

    let backgroundContainer = $state<HTMLDivElement | null>(null);
    function onBackgroundMousedown(e: MouseEvent) {
        if (e.target != backgroundContainer) return;
        close();
    }

    $effect(() => {
        const escapeHandle = (event: KeyboardEvent) => {
            if (event.key === 'Escape') close();
        };
        window.addEventListener('keydown', escapeHandle);
        () => window.removeEventListener('keydown', escapeHandle);
    });
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    class="fixed backdrop-blur-sm inset-0 w-screen h-screen flex justify-center items-center z-50"
    bind:this={backgroundContainer}
    onmousedown={onBackgroundMousedown}
>
    <div
        class="p-10 mx-5 max-w-sm md:max-w-md w-full rounded-md bg-gray-900 border-2 border-gray-800"
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
