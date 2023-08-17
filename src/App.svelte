<script>

  import { invoke } from "@tauri-apps/api/tauri"
  import { open } from '@tauri-apps/api/dialog';
  import { documentDir } from '@tauri-apps/api/path';

  import Task from "./lib/Task.svelte"
  import Modal from "./lib/Modal.svelte";

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

  async function get_user() {
    await invoke("get_user", 
    {
      userName: try_name,
      userPassword: try_pass

    }).then((result) => {
      let data = JSON.parse(result);
      
      if (data.length > 0) {

        is_sign = true;
        current_user = data[0];

      };
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

#singing {

  margin: 0 auto;
  width: calc(100vw - 25vw);
  background-color: #3e3e3e;
  border-radius: 15px;
  top: calc(100vw - 85vw);
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  flex-wrap: wrap;
  padding: 10px 0px;
}

.users {
  width: calc(100vw - 35vw);
  margin: 5px 5px;
}

#buttons {
  width: calc(100vw - 35vw);
  padding: 5px 0px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.sg_but {
  display: inline-block;
}

</style>

<div id="main_container">

  {#if is_sign === false}
  
    <div id="singing">
      <input type="text" id="user_name" class="users" placeholder="Ник" bind:value={try_name}>
      <input type="password" id="user_password" class="users" placeholder="Пароль" bind:value={try_pass}>
      <div id="buttons">
        <button on:click={get_user} class="sg_but">Войти</button>
        <button class="sg_but">Создать аккаунт</button>
      </div>
      
    </div>
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

  
























