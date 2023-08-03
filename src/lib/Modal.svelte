<script>
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

	export let showModal; // boolean
	export let typeModal; // boolean

	let dialog; // HTMLDialogElement
	//on:click|self={() => dialog.close()}
	$: if (dialog && showModal) dialog.showModal();
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
		display: block;
	}

</style>


<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<dialog
	bind:this={dialog}
	on:close={() => (showModal = false)} >
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div id="content">
		{#if typeModal === "message"}
			<slot name="modal_header" />
			<hr />
			<slot name="modal_content" />
			<hr />
		{/if}
		<!-- svelte-ignore a11y-autofocus -->
		<button
            autofocus 
            on:click={() => {
                dispatch("modal_close")
                dialog.close()
            }}>close modal</button>
	</div>
</dialog>