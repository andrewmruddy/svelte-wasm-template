<script lang="ts">
    import Dialog, { Title, Content, Actions, InitialFocus } from '@smui/dialog';
    import Button, { Label } from '@smui/button';
    import { push } from 'svelte-spa-router';
    import { ApplicationState } from '../../models/State';
    import { DEFAULT, state, token } from '../../services/LocalStorage';
    export let open = false;
    export let message: string;
</script>

<Dialog
  bind:open
  aria-labelledby="default-focus-title"
  aria-describedby="default-focus-content"
>
  <Title id="default-focus-title">Error Loading Content</Title>
  <Content id="default-focus-content">
    {message}
  </Content>
</Dialog>

<style>

</style>