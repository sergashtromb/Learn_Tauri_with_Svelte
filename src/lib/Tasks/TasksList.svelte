<script>

    import Task from "./Task.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { get_last_id } from "../../tools/small_operation";
    import { TaskObj } from "../../tools/tasks";

    export let current_user = {};
    let tasks = [];
    let visible_done = false;

    async function get_tasks(user_id) {
        await invoke("get_tasks", {userId: user_id}).then((result) => {
            let data = JSON.parse(result);
            data.forEach(element => {
                tasks = tasks.concat(new TaskObj(element.id, element.text, element.is_done));
            });
        });
    }

    async function add_task() {
        let new_task = { id: get_last_id(tasks) + 1, text: "Новая задача", is_done: false}
        tasks = tasks.concat(new_task);

        await invoke("add_task", {

            userId: current_user.id,
            taskText: new_task.text,
            isDone: new_task.is_done,
            lastId: new_task.id

        }).then((result) => {

            let res_task = JSON.parse(result);
            tasks.find(task => task.id == new_task.id).id = res_task.id;

        }).catch((err) => {

            console.log(err);

        });
    }

    async function change_status(event) {

        let update_task = {
            id: event.detail.task_id,
            text: event.detail.tasks_text,
            is_done: event.detail.is_done 
        }

        tasks.find(task => task.id == update_task.id).set_atribute(update_task.text, update_task.is_done);

        await invoke("change_task", {

            userId: current_user.id,
            taskText: update_task.text,
            isDone: update_task.is_done,
            taskId: update_task.id

        }).then((result) => {

            console.log(result);

        }).catch((err) => {

            console.log(err);
            
        });

    }

    // function change_visible_done() {
    //     visible_done = visible_done ? false: true;
    //     document.getElementById("task_is_done").style.visibility = visible_done;        
    // }

</script>

<style>

li {
  list-style-type: none;
}

ul {
  padding-inline-start: 0px;
}

#visible-task {

    font: inherit;
    background-color: black;
    width: 1.15em;
    height: 1.15em;
    border: 0.15em solid black;
    border-radius: 0.15em;
    transform: translateY(-0.075em);
    margin-right: 10px;
    margin-top: 10px;
    margin-bottom: 0px;

}



</style>

<div id="tasks_list">

    {#await get_tasks(current_user.id)}

        <h3>Подождите....</h3>

    {:then value} 
        <h4>Текущие задачи</h4>
        <ul>
            {#each tasks.filter(task => task.is_done === false) as elem}

                <li>
                    <Task
                        tasks_text = {elem.text}
                        is_done = {elem.is_done}
                        task_id = {elem.id}
                        on:choose_task
                        on:change_task_status={change_status}
                    />
                </li>
            {/each}

        </ul>

        <button id="bt_add_task" on:click={add_task}>+ Добавить задачу</button>

        <hr>

        <!-- <h4><input type="checkbox" name="visible-task" id="visible-task" bind:value={visible_done} on:change={change_visible_done}>Выполненные задачи {tasks.filter(task => task.is_done === true).length}</h4> -->

        <ul id="task_is_done">
            {#each tasks.filter(task => task.is_done === true) as elem}
                <li>
                    <Task
                        tasks_text = {elem.text}
                        is_done = {elem.is_done}
                        task_id = {elem.id}
                        on:choose_task
                        on:change_task_status={change_status}
                    />
                </li>
            {/each}

        </ul>    

        
        
    {:catch err}

        <h3>{err}</h3>

    {/await}

</div>

