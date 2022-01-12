<script lang='ts'>
// START Properties
export let Title: string = "";
export let Subtitle: string = "";
export let Link: string = "";
export let Count: number = 0;
// END Properties


// START Component Imports
import Card, {
    Content,
    PrimaryAction,
    Actions,
    ActionButtons,
    ActionIcons,
    Media,
    MediaContent,
} from '@smui/card';
import Button, { Label } from '@smui/button';
import IconButton from '@smui/icon-button';
import {Icon} from '@smui/common';
import Badge from '@smui-extra/badge';
// END Component Imports


// START Other Imports
import { push } from 'svelte-spa-router';
// END Other Imports


// START Local Vars
let clicked: number = 0;
let img: string = "background-image: url(/images/icon.png);";
// END Local Vars


// START Functions
// END Functions


// START Init
// END Init
</script>

<div class="card-container">
    <div class="sub-container">
    <Card>
      <div style="padding: 1rem;">
        <h2 class="mdc-typography--headline6" style="margin: 0;">
          {Title}
        </h2>
        <h3 class="mdc-typography--subtitle2" style="margin: 0; color: #888;">
          {Subtitle}
        </h3>
      </div>
      <PrimaryAction on:click={() => push(Link)}>
        <div class="banner-container">
          <Media class="card-media-16x9" aspectRatio="square"/>
        </div>
        <Content class="mdc-typography--body2">
          And some info text. And the media and info text are a primary action
          for the card.
        </Content>
      </PrimaryAction>
    </Card>
    {#if Count > 0}
    <Badge color="primary" aria-label="unread count">{Count}</Badge>
    {/if}
    </div>
</div>

<style>
    @media only screen and (min-width: 601px) {
        .card-container{
            width: 400px;
            padding-left: 20px;
            padding-right: 20px;
        }
    }
    @media only screen and (max-width: 600px) {
        .card-container{
            margin:auto;
            width: 300px;
            padding-left: 20px;
            padding-right: 20px;
            padding-top:10px;
            padding-bottom:10px;
        }
    }
    .sub-container{
        position: relative;
    }
    * :global(.card-media-16x9) {
        background-image: url(https://place-hold.it/320x180?text=16x9&fontsize=23);
    }
    
    * :global(.card-media-square) {
        background-image: url(https://place-hold.it/320x320?text=square&fontsize=23);
    }
    .banner-container {
        background-image: url("/images/icon.svg");
        background-repeat: no-repeat;
        background-size: 100% auto;
    }
</style>