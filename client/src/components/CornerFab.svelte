<script lang="ts">
    import IconButton, { Icon } from '@smui/icon-button';
    import Fab from '@smui/fab';
    import { Label } from "@smui/button";
    // Events
    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher();

    function onClick(){
        dispatch('action', {});
    }
</script>

<div class="fab-container">
    <Fab on:click={() => onClick()} extended>
        <Icon class="material-icons">create</Icon>
        <Label>Create</Label>
    </Fab>
</div>

<style>
    .fab-container{
        position:fixed;
        bottom:25px;
        right:25px;
        cursor:pointer;
    }
</style>