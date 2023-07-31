<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { createEventDispatcher } from "svelte";
    import { clickOutside } from "../tools/event";

    export let tasks_text = "Новая задача";
    export let is_done = false;
    export let id = 0;
    

    

    const dispatch = createEventDispatcher();

    // function taskHandler() {

    //     console.log(id);  
    //     invoke("console_writeln", { text: String(id) });

    //     dispatch('checkTask', {
    //         status_done: is_done,
    //         tasks_id: id
    //     });
    // }

</script>

<style>

div  {
    border: 2px solid black;
    border-radius: 15px;
    margin-top: 10px;
    margin-bottom: 10px;
    
}

</style>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div>
    <input type="checkbox" bind:value={is_done} id="main_checkbox"><label for="main_checkbox">{tasks_text}</label>
</div>

        


