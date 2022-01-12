<script lang='ts'>
    import Card, {
        Content,
        PrimaryAction,
        Actions,
        ActionButtons,
        ActionIcons,
    } from '@smui/card';
    import Button, { Label } from '@smui/button';
    import IconButton, { Icon } from '@smui/icon-button';
    import { push } from 'svelte-spa-router';
    import { CardModel } from '../../models/ui/CardModel';
    import type { IconModel } from '../../models/ui/IconModel';
    // Events
    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher();
    function onClick() {
        dispatch('click',{});
    }
    let style: string = "outline-style:dashed;outline-color:#395C6B;height:165px";
    export let title:string;
    export let description:string;
</script>

<div class="card-container" on:click={()=>onClick()}>
    <Card style={style}>
      <Content>{title}</Content>
      <Content>{description}</Content>
    </Card>
  </div>

<style>
    .card-container{
        cursor: pointer;
    }
</style>