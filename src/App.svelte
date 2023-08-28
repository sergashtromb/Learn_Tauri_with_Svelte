<script>

  import { invoke } from "@tauri-apps/api/tauri"
  import { open } from '@tauri-apps/api/dialog';
  import { documentDir } from '@tauri-apps/api/path';

  import Modal from "./lib/MainComponent/Modal.svelte";
  import Registration from "./lib/MainComponent/Registration.svelte";
  import MainMenu from "./lib/MainComponent/MainMenu.svelte";
  import Content from "./lib/MainComponent/Content.svelte";

  import { get_last_id } from "./tools/small_operation";

  const docDir = documentDir();
  
  let tasks = [];
  let path = " ";
  let showModal = false;
  let current_user = {};
  let is_sign = false;


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

</style>

<div id="main_container">

  {#if is_sign === false}
  
    <Registration on:log_in={(event) => {

      is_sign = true;
      current_user = event.detail.user;

    }}/>
    
  {:else}

    <Modal
      bind:showModal
      bind:typeModal={dialogSettings.for_modal}
      bind:isJustClosing={dialogSettings.is_just_closing}
      bind:outArgumetns={dialogSettings.argumetns}
      on:dialog_out={dialog_out}>
    </Modal>

    <MainMenu/>
    <Content/>

  {/if}

</div>

  
























