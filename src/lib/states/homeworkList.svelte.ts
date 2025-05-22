import { parseDate } from '$lib/utils';
import { type Task } from '$lib/types';
import { invoke } from '@tauri-apps/api/core';
import { getContext, setContext } from 'svelte';

export class HomeworkList {
    tasks = $state<Task[]>([]);

    add = (title: string) => {
        invoke('add_task', { title })
            .then((id) => {
                this.tasks.push({
                    id: id as number,
                    done: false,
                    deadline: new Date(),
                    title: title
                });
            })
            .catch((err) => console.error(err));
    };

    toggleTask = (id: number) => {
        invoke('toggle_task', { id }).catch((err) => console.error(err));
    };

    updateTask = (id: number, title: string, deadline: Date) => {
        invoke('update_task', {
            id,
            title,
            deadline: parseDate(deadline)
        }).catch((err) => console.error(err));
    };

    remove = (id: number) => {
        invoke('delete_task', { id })
            .then(() => {
                let index = this.tasks.findIndex((task) => task.id === id);
                this.tasks.splice(index, 1);
            })
            .catch((err) => console.error(err));
    };
}

const TOKEN = Symbol('token');

export function createHomeworkList() {
    setContext(TOKEN, new HomeworkList());
}

export function getHomeworkList(): HomeworkList {
    return getContext(TOKEN) as HomeworkList;
}
