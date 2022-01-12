<script lang='ts'>
// START Properties
export let model: CardModel = new CardModel();
export let icons: Array<IconModel>;
// END Properties


// START Component Imports
import Card, {
    Content,
    PrimaryAction,
    Actions,
    ActionButtons,
    ActionIcons,
} from '@smui/card';
import Button, { Label } from '@smui/button';
import IconButton, { Icon } from '@smui/icon-button';
// END Component Imports


// START Other Imports
import { push } from 'svelte-spa-router';
import type { IconModel } from '../../models/ui/IconModel';
import { createEventDispatcher } from 'svelte';
// END Other Imports


// START Local Vars
// END Local Vars


// START Functions
const dispatch = createEventDispatcher();
function onPrimaryAction(message:string) {
    dispatch('primaryAction', 
    {
      eventMessage:message
    });
}
function onIcon(message:string) {
    dispatch('icon', {
      eventMessage: message
    });
}
// END Functions


// START Init
// END Init


// START WASM
import init, {Card as CardModel} from "../../../../shared/pkg/shared";
init().then(() => {});
// END WASM
</script>

<div class="card-container">
    <Card variant="outlined">
      <Content>{model.title}</Content>
      <Content>{model.description}</Content>
      <Actions>
        <ActionButtons>
          <Button on:click={() => onPrimaryAction(model.key)}>
            <Label>{model.primaryLabel}</Label>
          </Button>
        </ActionButtons>
        <ActionIcons>
          {#each icons as icon}
          <IconButton
            on:click={() => onIcon(icon.event)}
            toggle={icon.toggle}
            aria-label={icon.label}
            title={icon.label}
          >
            <Icon class="material-icons" on>{icon.toggleIcon}</Icon>
            {#if icon.toggle}
            <Icon class="material-icons">{icon.icon}</Icon>
            {/if}
          </IconButton>
          {/each}
        </ActionIcons>
      </Actions>
    </Card>
  </div>

<style>

</style>