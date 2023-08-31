<script>
    import { onMount } from "svelte";

    import Task from "./Task.svelte";
    import { invoke } from "@tauri-apps/api/tauri";

    export let current_user = {};
    let tasks = [];

    // onMount(() => {
    //     // @ts-ignore
    //     tasks = get_tasks(current_user.id);
    // });

    async function get_tasks(user_id) {
        await invoke("get_tasks", {userId: user_id}).then((result) => {
            tasks = JSON.parse(result);
        });
    }


</script>

<style>

li {
  list-style-type: none;
}

ul {
  padding-inline-start: 0px;
}

</style>

<div id="tasks_list">

    {#await get_tasks(current_user.id)}
        <h3>Подождите....</h3>
    {:then value} 
        <ul>
            {#each tasks as elem}
                <li>
                    <Task
                        tasks_text = {elem.text}
                        is_done = {elem.is_done}
                        task_id = {elem.id}
                        on:choose_task
                        on:change_task_status
                    />
                </li>
            {/each}

        </ul>
    {/await}

</div>

<button id="bt_add_task">+ Добавить задачу</button>