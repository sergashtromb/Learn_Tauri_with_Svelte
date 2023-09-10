<script>
    import TasksList from "../Tasks/TasksList.svelte";
    export let current_user = {};
    let content_type = 'tasks';

</script>

<style>

#content {
    margin-left: 160px;
    left: calc(100vw - 85vw);
    padding-bottom: 60px;
}


</style>

<div id="content">

  {#if content_type === 'tasks'}

    <TasksList
      current_user={current_user}
      on:choose_task
    />
    
  {/if}

</div>