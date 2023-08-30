<script>
    import TasksList from "../Tasks/TasksList.svelte";

    import { get_last_id } from "../../tools/small_operation";
    export let current_user = {};
    let tasks = [];
    let path = " ";
    // export let showModal = false;

    // export let dialogSettings = {
    //     "for_modal": "",
    //     "is_just_closing": false,
    //     "argumetns": {},
    //     "clear_param": function() { 
    //     this.for_modal = "";
    //     this.is_just_closing = false;
    //     this.argumetns = {};
    //     },
    // }

</script>

<style>

#content {
    margin-left: 160px;
    left: calc(100vw - 85vw);
}


</style>

<div id="content">

  <TasksList
    current_user={current_user}
    on:choose_task
  />

</div>