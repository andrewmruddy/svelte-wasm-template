<script lang="ts">
    import Dialog, { Title, Content, Actions, InitialFocus } from '@smui/dialog';
    import Button, { Label } from '@smui/button';
    import { push } from 'svelte-spa-router';
    import { ApplicationState } from '../../models/State';
    import { DEFAULT, state, token } from '../../services/LocalStorage';
    export let open = false;
    function onClickLogout(){
        state.set(ApplicationState[ApplicationState.LOGGED_OUT])
        token.set(DEFAULT)
        push('#/')
    }
    function onClickDashboard(){
        push('#/Dashboard')
    }
</script>

<Dialog
  bind:open
  aria-labelledby="default-focus-title"
  aria-describedby="default-focus-content"
>
  <Title id="default-focus-title">Advice</Title>
  <Content id="default-focus-content">
    Are you sure you want to log out?
  </Content>
  <Actions>
    <Button on:click={() => onClickLogout()}>
      <Label>Yes, I am sure.</Label>
    </Button>
    <Button
      defaultAction
      use={[InitialFocus]}
      on:click={() => onClickDashboard()}
    >
      <Label>Take me to my dashboard.</Label>
    </Button>
  </Actions>
</Dialog>

<style>

</style>