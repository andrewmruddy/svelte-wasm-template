<script lang='ts'>
// START Properties
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
import Textfield from '@smui/textfield';
import LinearProgress from '@smui/linear-progress';
// END Component Imports


// START Other Imports
import {User} from "../../../models/User";
import {token} from "../../../services/LocalStorage";
import {push, pop, replace} from 'svelte-spa-router'
import NavBar from '../../../components/NavBar.svelte';
import { Routes } from '../../../models/Routes';
// END Other Imports


// START Local Vars
let promise = Promise.resolve([]);
let user = new User();
let secondPassword: string = "";
let hint: string = "";
let invalid: boolean = false;
// END Local Vars


// START Functions
async function validate(response){
    console.log(response);
    if (response == "Unauthorized"){
        invalid=true;
        token.set('--empty--');
        hint="Error";
    }else{
        push(Routes.ThankYou)
    }
}
async function onClickSignUp() {
    hint = ""
    if(user.password != secondPassword){
        invalid=true;
        hint = "Passwords Must Match"
    }else{
        promise = signUp(user);
        await promise.then(response=>validate(response)).catch(error=>invalid=true)
    }
}
// END Functions
</script>

<NavBar></NavBar>
<div class="center">
    <div class="card-container">
        <Card>
            <h1>Sign Up</h1>
            {#await promise}
                <LinearProgress indeterminate />
            {/await}
            <Content>
                <div class="text-input">
                    <Textfield
                        invalid={invalid}
                        style="width: 100%;"
                        helperLine$style="width: 100%;"
                        variant="outlined"
                        bind:value={user.email}
                        label="Email"
                        required
                    />    
                </div>
                <div class="text-input">
                    <Textfield
                        type="password"
                        invalid={invalid}
                        style="width: 100%;"
                        helperLine$style="width: 100%;"
                        variant="outlined"
                        bind:value={user.password}
                        label="Password"
                        required
                    />    
                </div>
                <div class="text-input">
                    <Textfield
                        type="password"
                        invalid={invalid}
                        style="width: 100%;"
                        helperLine$style="width: 100%;"
                        variant="outlined"
                        bind:value={secondPassword}
                        label="Retype Password"
                        required
                    />    
                </div>
                <div style="color:#B71C1C">
                    <Label>{hint}</Label>
                </div>
            </Content>
            <Actions>
                <Button on:click={() => onClickSignUp()}>
                <Label >Sign Up</Label>
                </Button>
            </Actions>
        </Card>
    </div>
</div>

<style>
    .text-input{
        padding:10px;
    }
    .card-container{
        width: 400px;
    }
    .center{
        height: 100vh;
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
    }
</style>

