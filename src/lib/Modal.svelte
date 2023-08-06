<script>
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

	export let showModal; // boolean
	export let typeModal; // string
	export let isJustClosing; // boolean
	export let outArgumetns; // объект который принимает внешние аргументы как данные при вызове модального окна


	let dialog; // HTMLDialogElement

	$: if (dialog && showModal) dialog.showModal();

	function dialog_close() {

		dispatch("dialog_out", {
			saveData: outArgumetns,
			type: typeModal
		});

		showModal = false;
		typeModal = "";
	}

	function click_with_dialog() {
		if (isJustClosing) {
			dialog.close();
		}
	}

</script>

<style>
	dialog {
		width: calc(100vw - 25vw);
        background-color: #2f2f2f;
        border-radius: 0.2em;
		border: none;
	}
    #content {
        padding: 0;
        color: #fff;
        position: relative;
    }
	dialog::backdrop {
		background: rgba(0, 0, 0, 0.3);
	}
	dialog[open] {
		animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
	}
	@keyframes zoom {
		from {
			transform: scale(0.95);
		}
		to {
			transform: scale(1);
		}
	}
	dialog[open]::backdrop {
		animation: fade 0.2s ease-out;
	}
	@keyframes fade {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}
	button {
		display: inline-block;
	}
	#button_closing {
		float: right;
	}

	hr {
		margin: 20px 0px;
	}
</style>


<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<dialog
	bind:this={dialog}
	on:close={dialog_close}
	on:click|self={click_with_dialog} >
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div id="content">
		{#if typeModal === "message"}
			<slot name="modal_header" />
			<hr />
			<slot name="modal_content" />
			<hr />
		{:else if typeModal === "edit-task"}
			<h2>Редактирование задачи</h2>
			<hr />
			<input
				type="text"
				bind:value={outArgumetns.tasks_text}>
			<hr />
		{/if}
		<!-- svelte-ignore a11y-autofocus -->
		<div id="buttons_container">

			{#if typeModal === "edit-task"}
				<button
					id="button_save"
					on:click={() => {
						dialog.close();
					}}>
					Сохранить задачу
				</button>
			{/if}

			<button
			id="button_closing"
            autofocus 
            on:click={() => {
                dialog.close();
            }}>Закрыть окно</button>
		</div>
		
	</div>
</dialog>