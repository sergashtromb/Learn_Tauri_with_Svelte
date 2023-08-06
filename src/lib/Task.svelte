<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { createEventDispatcher } from "svelte";
    import { clickOutside } from "../tools/event";

    export let tasks_text = "Новая задача";
    export let is_done = false;
    export let task_id = 0;
    

    

    const dispatch = createEventDispatcher();

    function change_status() {
        is_done = is_done ? false: true;
        dispatch('change_task_status', {
            is_done: is_done,
            task_id: task_id
        });
    }

    function choose_task() {
        dispatch('choose_task', {
            task_id: task_id,
            tasks_text: tasks_text
        })
    }

</script>

<style>

div  {
    border: 3px solid rgb(29, 29, 29);
    border-radius: 15px;
    margin-top: 5px;
    padding: 5px;
}

#main_checkbox {
    font: inherit;
    background-color: black;
    width: 1.15em;
    height: 1.15em;
    border: 0.15em solid black;
    border-radius: 0.15em;
    transform: translateY(-0.075em);
}

</style>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div on:click={choose_task}>
    <p><input type="checkbox" bind:checked={is_done} id="main_checkbox" on:click|stopPropagation={change_status} />{tasks_text}</p>
</div>


        


