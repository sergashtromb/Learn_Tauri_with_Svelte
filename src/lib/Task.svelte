<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { createEventDispatcher } from "svelte";
    import { clickOutside } from "../tools/event";

    export let tasks_text = "Новая задача";
    export let is_done = false;
    export let id = 0;
    export let is_active = false;

    let is_input_active = false;

    const dispatch = createEventDispatcher();

    function taskHandler() {

        console.log(id);  
        invoke("console_writeln", { text: String(id) });

        dispatch('checkTask', {
            status_done: is_done,
            tasks_id: id
        });
    }

    function change_status_activity() {

        is_active = true;

        dispatch('changeActiveTask', {
            tasks_id: id
        });
        
    }


    function set_text_task() {
        dispatch('changeTextTask', {
            tasks_id: id,
            task_text: tasks_text
        });
    
    }

    function break_current_focus() {
        
        is_active = false;

        dispatch('changeActiveTask', {
            tasks_id: -1
        });
    }

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
<div on:click={change_status_activity} use:clickOutside on:click_outside={() => {

        if(!is_active || is_input_active) {
            return;
        }
        break_current_focus();

    }

}
>
    <form>
        <p><input type="checkbox" checked={is_done} on:change={taskHandler}> 
            {#if is_active}

                <span id="visible_input_text">

                    <input
                        id="input_text"
                        type="text"
                        bind:value={tasks_text}
                        on:change={set_text_task}
                        on:focus={() => is_input_active = true}
                        on:focusout={() => is_input_active = false} >

                </span>  

            {:else}

                <span id="visible_text">{tasks_text}</span>  {id}

            {/if}
            </p>    
    </form>

</div>