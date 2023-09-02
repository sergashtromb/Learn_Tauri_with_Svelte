<script>

    import Task from "./Task.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { get_last_id } from "../../tools/small_operation";

    export let current_user = {};
    let tasks = [];


    async function get_tasks(user_id) {
        await invoke("get_tasks", {userId: user_id}).then((result) => {
            tasks = JSON.parse(result);
        });
    }

    async function add_task() {
        let new_task = { id: get_last_id(tasks) + 1, text: "Новая задача", is_done: false}
        tasks = tasks.concat(new_task);

        console.log(tasks);

        await invoke("add_task", {
            userId: current_user.id,
            taskText: new_task.text,
            isDone: new_task.is_done,
            lastId: new_task.id
        }).then((result) => {

            let res_task = JSON.parse(result);
            tasks.find(task => task.id == new_task.id).id = res_task.id;
            console.log("добавление в дб выполнено");
            console.log(tasks);

        }).catch((err) => {

            console.log("не добавлено в бд");
            console.log(err);
            
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

    {:catch err}

        <h3>{err}</h3>

    {/await}

</div>

<button id="bt_add_task" on:click={add_task}>+ Добавить задачу</button>