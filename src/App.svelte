<script>

  import { invoke } from "@tauri-apps/api/tauri"
  import { open } from '@tauri-apps/api/dialog';

  import Task from "./lib/Task.svelte"
  import Modal from "./lib/Modal.svelte";

  // C:\Users\Sergio\Desktop\tasks.txt
  let tasks = [];
  let path = " ";
  let showModal = false;
  let tt = ""

  async function get_tasks() {
    // открывает проводник для выбора файла
    const selected = await open({
      multiple: true,
      filters: [{
        name: 'Task',
        extensions: ['txt']
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

  <div class="modal_window">

    <Modal bind:showModal on:modal_close={() => tt=""}>

      <h3 slot="modal_header">Задача</h3>
      <p slot="modal_content">{tt}</p>

    </Modal>

  </div>

  <div id="tools">
    <button on:click={get_tasks}>Загрузить задачи из файла</button>
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
              tt = event.detail.tasks_text;
              showModal = true;
              }}
            on:change_task_status={(event) => {
              tasks.find(task => task.id == event.detail.task_id).is_done = event.detail.is_done;
            }}/> </li> 
        {/each}
        <li><button>+ Добавить задачу</button></li>
    </ul>

  </div>

</div>
























