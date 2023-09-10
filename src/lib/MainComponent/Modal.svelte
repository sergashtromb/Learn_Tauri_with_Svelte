<script>
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

	export let showModal; // boolean
	// export let typeModal; // string
	export let isJustClosing; // boolean
	// export let outArgumetns; // объект который принимает внешние аргументы как данные при вызове модального окна


	let dialog; // HTMLDialogElement

	$: if (dialog && showModal) dialog.showModal();

	function dialog_close() {

		dispatch("dialog_out");

		showModal = false;
	
	}

	function click_with_dialog() {
		if (isJustClosing) {
			dialog.close();
		}
	}

</script>

<style>
	dialog {
		min-width: 350px;
		width: calc(100vw - 25vw);
        background-color: #2f2f2f;
        border-radius: 0.2em;
		border: none;
		display: flex;
		justify-content: center;
	}
    #content {
		margin: 0 auto;
        padding: 0;
        color: #fff;
        position: relative;
		width: 100%;
		display: block;
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

	#buttons_container {
		width: 100%;
		margin: 0 auto;
	}
</style>


<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<dialog
	bind:this={dialog}
	on:close={dialog_close}
	on:click|self={click_with_dialog} >
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div id="content">
		<slot name="cont"/>
		<!-- svelte-ignore a11y-autofocus -->
		<div id="buttons_container">

			<slot name="butt"/>

			<button
				id="button_closing"
            	autofocus 
            	on:click={() => {
                dialog.close();
            }}>Закрыть окно</button>
		</div>
		
	</div>
</dialog>