<script lang="ts">
// START Properties
export let promise: Promise<Array<Card>>;
export let icons: Array<IconModel>;
export let createNewTitle:string;
export let createNewDescription:string;
// END Properties


// START Component Imports
import LayoutGrid, { Cell } from "@smui/layout-grid";
import GenericCard from "./Cards/GenericCard.svelte";
import CreateNewCard from "./Cards/CreateNewCard.svelte";
import LoadingOverlay from "./LoadingOverlay.svelte";
// END Component Imports


// START Other Imports
import type { IconModel } from "../models/ui/IconModel";
// END Other Imports


// START Local Vars
// END Local Vars


// START Functions
// END Functions


// START Init
// END Init

// START WASM
import init, {Card} from "../../../shared/pkg/shared";
init().then(() => {});
// END WASM
</script>

{#await promise}
    <LoadingOverlay visible={true}/>
{:then items}
    <div class="card-container">
        <LayoutGrid>
            <Cell spanDevices={{ desktop: 4, tablet: 6, phone: 6 }}>
                <CreateNewCard on:click title={createNewTitle} description={createNewDescription}></CreateNewCard>
            </Cell>
            {#each items as item}
            <Cell spanDevices={{ desktop: 4, tablet: 6, phone: 6 }}>
                <GenericCard  
                    icons={icons} 
                    model={item}
                    on:icon
                    on:primaryAction
                >
                </GenericCard>
            </Cell>
            {/each}
        </LayoutGrid>
    </div>
{:catch error}
    <p style="color: red">{error.message}</p>
{/await}



<style>
  .card-container{
    display: flex;
    flex-direction: row;
    justify-content: center;
  }
    
</style>
    
