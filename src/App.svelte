<script>

  import { invoke } from "@tauri-apps/api/tauri"
  import { open } from '@tauri-apps/api/dialog';
  import { documentDir } from '@tauri-apps/api/path';

  import Task from "./lib/Task.svelte"
  import Modal from "./lib/Modal.svelte";
  import Registration from "./lib/Registration.svelte";

  import { get_last_id } from "./tools/small_operation";

  const docDir = documentDir();
  
  let tasks = [];
  let path = " ";
  let showModal = false;
  let current_user = {};
  let is_sign = false;
  
  let try_name = "";
  let try_pass = "";

  let dialogSettings = {
    "for_modal": "",
    "is_just_closing": false,
    "argumetns": {},
    "clear_param": function() { 
      this.for_modal = "";
      this.is_just_closing = false;
      this.argumetns = {};
    },
  }

  async function get_tasks_from_file(arrFiles) {
    console.log(docDir.finally);
    // открывает проводник для выбора файла
    const selected = await open({
      multiple: true,
      filters: [{
        name: 'Task',
        extensions: arrFiles
      }]
    });

    // после выбора возвращается массив с путями файлов
    if (Array.isArray(selected)) {
      path = selected[0];
      // берем нужный и отправляем на обработку на "сервер"
      invoke("parse_file_tasks", {path: path} )
        .then((result) => {
          tasks = JSON.parse(result);
        });
    } 
  }

  function reload_window() {
    let obj = {
      path: path,
      tasks: tasks
    };

    localStorage.setItem("all_info", JSON.stringify(obj));

    window.location.reload();
  }

  document.addEventListener("DOMContentLoaded", function() {

    let obj = JSON.parse(localStorage.getItem("all_info"));
    path = obj.path;
    tasks = obj.tasks;
    localStorage.clear();

  });


  function dialog_out(event) {

    dialogSettings.clear_param();
    
    if(event.detail.saveData === null) {
      return;
    }

    if(event.detail.type === "edit-task") {
      tasks.find(task => task.id == event.detail.saveData.task_id).text = event.detail.saveData.tasks_text;
      tasks = tasks;
    }
    
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

<div id="main_container">

  {#if is_sign === false}
  
    <Registration on:log_in={(event) => {

      is_sign = true;
      current_user = event.detail.user; 
      
    }}/>
    
  {:else}

    <div class="modal_window">

      <Modal
        bind:showModal
        bind:typeModal={dialogSettings.for_modal}
        bind:isJustClosing={dialogSettings.is_just_closing}
        bind:outArgumetns={dialogSettings.argumetns}
        on:dialog_out={dialog_out}>

      </Modal>

    </div>

    <div id="tools">
      <button on:click={() => invoke("get_all_tasks")}>Получить все задачи с Базы данных</button>
      <button on:click={() => get_tasks_from_file(["md"])}>Ипорт задач из .md файлов</button>
    </div>
    <div id="list_tasks">

      <ul>
          {#each tasks as elem}
            <li>
              <Task
              tasks_text={elem.text}
              is_done={elem.is_done}
              task_id={elem.id}
              on:choose_task={(event) => {

                dialogSettings.for_modal = "edit-task";
                dialogSettings.is_just_closing = false;

                dialogSettings.argumetns = {
                  "task_id": event.detail.task_id,
                  "tasks_text": event.detail.tasks_text
                };

                showModal = true;
                }}
              on:change_task_status={(event) => {
                tasks.find(task => task.id == event.detail.task_id).is_done = event.detail.is_done;
              }}/> </li> 
          {/each}
      </ul>
      <button 
        on:click={() => {
          tasks = tasks.concat({ id: get_last_id(tasks) + 1, text: "Новая задача", is_done: false});
        }}
        >+ Добавить задачу</button>
    </div>

  {/if}

</div>

  
























