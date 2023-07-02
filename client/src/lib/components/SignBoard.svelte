<script lang='ts'>
    import type Board from "./Board.svelte";
    export let board: Board
    import {modalStore} from "$lib/stores/modal.store";
	import RegisterModal from "./RegisterModal.svelte";
    
    let tab : "keys" | "login" = "login"

    const publish = () => {
    }

    const register = () => {
        modalStore.add({
            title: "Register",
            component: RegisterModal,
            props: {}
        })
    }

</script>

<section style="margin-top: 1rem;">
   <nav>
         <button on:click={() => tab = "login"}> Publish with credentials </button>
         <button on:click={() => tab = "keys"}>  Publish with keys </button>
   </nav>
   <section>
        {#if tab === "login"}
            <div class="input-group">
                <input type="text" placeholder="Username"/>
                <input type="password" placeholder="Password"/>
            </div>
            <div class="button-group">
                <button on:click={publish}> Publish </button>
                <button on:click={register}> Want us to handle your keys? </button>
            </div>
        {:else if tab === "keys"}
            <div class="input-group">
                <input type="text" placeholder="Public key"/>
                <input type="text" placeholder="Private key"/>
            </div>
            <div class="button-group">
                <button on:click={publish}> Publish </button>
                <a href="/generate" target="_blank"> Don't have keys? </a>
            </div>
           
        {/if}
   </section>
</section>

<style lang='scss'>
    *{
        box-sizing: border-box;
    }

    section{
        width: 100%;
        height: 100%;
        display: flex; 
        nav{
            padding: 1rem;
            display: flex;
            flex-direction: column;
            border-right: 1px solid black;
            gap: 1rem;
            button{
                padding: 0.5rem;
                border: 1px solid black;
                border: none;
                outline: none;
                &:hover{
                    background: black;
                    color: white;
                }
            }
        }
        section{
            padding: 1rem;
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            .input-group{
                display: flex;
                flex-direction: column;
                gap: 1rem;
                input{
                    padding: 0.5rem;
                    border: 1px solid black;
                    border: none;
                    outline: none;
                }
            }
            .button-group{
                display: flex;
                gap: 1rem;
                button{
                    width: 50%;
                    padding: 0.5rem;
                    border: none;
                    outline: none;
                    &:hover{
                        background: black;
                        color: white;
                    }
                }
            }

        }
    }
</style>