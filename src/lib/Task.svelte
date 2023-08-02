<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { createEventDispatcher } from "svelte";
    import { clickOutside } from "../tools/event";

    export let tasks_text = "Новая задача";
    export let is_done = false;
    export let task_id = 0;
    

    

    const dispatch = createEventDispatcher();

    function change_status() {

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
    padding: 10px 5px;
}

</style>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div on:click={choose_task}>
    <input type="checkbox" bind:checked={is_done} id="main_checkbox" on:change={change_status}><label for="main_checkbox">{tasks_text}</label>
</div>

        


