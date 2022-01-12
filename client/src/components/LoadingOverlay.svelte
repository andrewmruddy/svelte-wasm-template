<script lang="ts">
// START Properties
export let visible: boolean;
// END Properties


// START Component Imports
import CircularProgress from '@smui/circular-progress';
// END Component Imports


// START Other Imports
// END Other Imports


// START Local Vars
// END Local Vars


// START Functions
// END Functions


// START Init
// END Init
</script>

{#if visible}
<div class="loading-container">
    <CircularProgress style="height: 32px; width: 32px;" indeterminate />
</div>
{/if}

<style>

    .loading-container{
        position:fixed;
        bottom:50%;
        right:50%;
        cursor:pointer;
    }
</style>