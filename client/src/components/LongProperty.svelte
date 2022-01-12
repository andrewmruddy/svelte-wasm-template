<script lang="ts">
// START Properties
export let getter:string;
export let setter:string;
export let title:string;
export let subtitle:string;
export let edit: boolean;
// END Properties


// START Component Imports
import Paper, {Title, Subtitle, Content} from '@smui/paper';
import IconButton from '@smui/icon-button';
import Textfield from '@smui/textfield';
import HelperText from '@smui/textfield/helper-text';
// END Component Imports


// START Other Imports
import { ItemState } from '../models/Utils';
import {ItemState_toBool} from '../models/Utils';
// END Other Imports


// START Local Vars
let value:string;
let item_state: ItemState = ItemState.VIEW;
let disabled: boolean = false;
// END Local Vars


// START Functions
function toggleEdit(){
    if(item_state==ItemState.VIEW){
        item_state = ItemState.EDIT;
    }else{
        item_state = ItemState.VIEW;
    }
    disabled = ItemState_toBool(item_state);
}
function init(){
    if(edit){
        disabled = false;
    }else{
        disabled = true;
    }
}
$: setter = value;
$: value = getter;
// END Functions


// START Init
init();
// END Init
</script>

<Paper>
    <div class='action-bar'>
        <IconButton class="material-icons" style='margin:0%;padding:0;'>document_scanner</IconButton>
        <IconButton class="material-icons"  style='margin:0%;padding:0;' on:click={()=>toggleEdit()}>edit_note</IconButton>
    </div>
    <hr>
    <Title>{title}</Title>
    <Subtitle>{subtitle}</Subtitle>
    <Content>
        <Textfield
            style="width: 100%;"
            helperLine$style="width: 100%;"
            textarea
            bind:value
            disabled={disabled}
            label={title}
        >
            <HelperText>Helper Text</HelperText>
        </Textfield>
    </Content>
</Paper>

<style>
    .action-bar{
        display: flex;
        flex-direction: row;
        justify-content: end;
    }
    
    @media only screen and (min-width: 601px) {
        .action-bar{
            min-width: 500px;
        }
    }
    @media only screen and (max-width: 600px) {
        .action-bar{
            min-width: 300px;
        }
    }
</style>