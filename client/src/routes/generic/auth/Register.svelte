<script lang="ts">
// START Properties
export let params:{token:string} = {token:null};
// END Properties

// START Component Imports
import LoadingOverlay from "../../../components/LoadingOverlay.svelte";
import { push } from "svelte-spa-router";
// END Component Imports


// START Other Imports
import {register} from "../../../services/DataAccessService/DataAccessService";
import { Routes } from '../../../models/Routes';
// END Other Imports


// START Local Vars
let promise: Promise<boolean | void>;
// END Local Vars
function init(){
    promise = register(params.token).then(success => resolve(success)).catch(()=>error());
}
function resolve(success: boolean){
    if(success){
        push(Routes.ThankYou);
    }
}
function error(){
    push(Routes.Error);
}
init();
</script>

<!-- HTML Stuff -->
{#await promise}
    <LoadingOverlay visible={true}/>
{/await}

<style>
    
</style>