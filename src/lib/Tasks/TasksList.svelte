<script>

    import Task from "./Task.svelte";

    export let current_user = {};
    let tasks = [];

</script>

<style>

</style>

<div id="tasks_list">

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

</div>

<button id="bt_add_task">+ Добавить задачу</button>