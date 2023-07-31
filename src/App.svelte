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
  <div id="tools">
    <button on:click={get_tasks}>Загрузить задачи из файла</button>
  </div>
  <div id="list_tasks">

    <ul>
        {#each tasks as elem}
          <li><Task tasks_text={elem.text} is_done={elem.is_done} task_id={elem.id}/> </li> 
        {/each}
        <li><button>+ Добавить задачу</button></li>
    </ul>

  </div>

</div>
























