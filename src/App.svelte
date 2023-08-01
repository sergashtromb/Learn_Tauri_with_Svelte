<script>

  import { invoke } from "@tauri-apps/api/tauri"
  import { open } from '@tauri-apps/api/dialog';

  import Task from "./lib/Task.svelte"
  
  // C:\Users\Sergio\Desktop\tasks.txt
  let tasks = [];
  let path = " ";

  async function get_tasks() {

    const selected = await open({
      multiple: true,
      filters: [{
        name: 'Task',
        extensions: ['txt']
      }]
    });

    if (Array.isArray(selected)) {
      path = selected[0];

      invoke("parse_file_tasks", {path: path} )
        .then((result) => {
          tasks = JSON.parse(result);
          console.log(tasks);
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

.modal_window {
  display: block;
  position: fixed;
  width: 100%;
  height: 100%;
  left: 0;
  top: 0;
  background-color:rgba(0, 0, 0, 0.4);
  overflow: auto;
  z-index: 1;
}

.modal_content {
  margin: 0 auto;
  width: 75%;
  background-color: #0f0f0f;
  padding: 20px;
  position: relative;
}

#main_container {
  margin: auto, 0;
}

li {
  list-style-type: none;
}

ul {
  padding-inline-start: 0px;
}


</style>

<div id="main_container">

  <dev class="modal_window">

    <dev class="modal_content">
      <p>content</p>
    </dev>

  </dev>

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
            on:change_task_status={(event) => {
              tasks.find(task => task.id == event.detail.task_id).is_done = event.detail.is_done;
            }}/> </li> 
        {/each}
        <li><button>+ Добавить задачу</button></li>
    </ul>

  </div>

</div>
























