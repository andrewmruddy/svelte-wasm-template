<script lang='ts'>
// START Properties
// END Properties


// START Component Imports
import Paper from '@smui/paper';
import { Icon } from '@smui/common';
import { Input } from '@smui/textfield';
import Fab from '@smui/fab';
// END Component Imports


// START Other Imports
// END Other Imports


// START Local Vars
let value = '';
// END Local Vars


// START Functions
function doSearch() {
    alert('Search for ' + value);
}
function handleKeyDown(event: CustomEvent | KeyboardEvent) {
    event = event as KeyboardEvent;
    if (event.key === 'Enter') {
        doSearch();
    }
}
// END Functions


// START Init
// END Init

</script>

<div class="solo-demo-container solo-container">
    <Paper class="solo-paper" elevation={6}>
      <Icon class="material-icons" style='margin:0%;padding:0;'>search</Icon>
      <Input
        bind:value
        on:keydown={handleKeyDown}
        placeholder="Search"
        class="solo-input"
      />
    </Paper>
    <Fab
      on:click={doSearch}
      disabled={value === ''}
      color="primary"
      mini
      class="solo-fab"
      style='margin:0%;padding:0;'
    >
      <Icon class="material-icons">arrow_forward</Icon>
    </Fab>
</div>


<style>
    .solo-demo-container {
        padding: 36px 18px;
        /* background-color: var(--mdc-theme-background, #f8f8f8); */
        /* border: 1px solid */
        /* var(--mdc-theme-text-hint-on-background, rgba(0, 0, 0, 0.1)); */
    }
    
    .solo-container {
        display: flex;
        justify-content: center;
        align-items: center;
    }
    * :global(.solo-paper) {
        display: flex;
        align-items: center;
        flex-grow: 1;
        max-width: 600px;
        margin: 0 12px;
        padding: 0 12px;
        height: 48px;
    }
    * :global(.solo-paper > *) {
        display: inline-block;
        margin: 0 12px;
    }
    * :global(.solo-input) {
        flex-grow: 1;
        /* color: var(--mdc-theme-on-surface, #000); */
    }
    * :global(.solo-input::placeholder) {
        /* color: var(--mdc-theme-on-surface, #000); */
        opacity: 0.6;
    }
    * :global(.solo-fab) {
        flex-shrink: 0;
    }
</style>