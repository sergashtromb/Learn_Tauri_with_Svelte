<script>

    import Modal from "../MainComponent/Modal.svelte";

    import { TaskObj } from "../../tools/tasks";
    import { invoke } from "@tauri-apps/api/tauri";
    import { createEventDispatcher } from "svelte";
    import { clickOutside } from "../../tools/event";


    export let tasks_text = "Новая задача";
    export let is_done = false;
    export let task_id = 0;
    let showModal = false;
    let new_text = tasks_text;
    
    let data_task = new TaskObj(task_id, tasks_text, is_done);
    

    const dispatch = createEventDispatcher();

    function change_status(check) {

        if (check) {
            is_done = is_done ? false: true;
        }
    
        dispatch('change_task_status', {
            is_done: is_done,
            task_id: task_id,
            tasks_text: tasks_text
        });
    }

    function choose_task() {
        showModal = true;
    }

</script>

<style>

.style-task  {
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

input {
    width: 100%;
    margin-right: 10px;
    margin-top: 5px;
    margin-bottom: 0px;
}

hr {
    width: 100%;
}

#edit-task {
    padding-bottom: 20px;
}

</style>


{#if showModal}
    <Modal showModal={showModal} isJustClosing={true} on:dialog_out={() => {showModal = false}}>
        
        <!-- <ChangeTask slot="cont"/> -->
        <div id="edit-task" slot="cont">
            <h2>Редактирование задачи</h2>
            <hr />
            <h4>Текст задачи</h4>
            <input type="text" bind:value={new_text}>
        </div>

        <button slot="butt" on:click={() => {
            tasks_text = new_text
            change_status(false);
            showModal = false;
        }}>Сохранить</button>
    
    </Modal>    
{/if}

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="style-task" on:click={choose_task}>
    <p><input type="checkbox" bind:checked={is_done} id="main_checkbox" on:click|stopPropagation={() => change_status(true)} />{tasks_text}</p>
</div>


        


