<script lang='ts'>
// SMUI Imports
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

// Other Imports
import {User} from "../../../models/User";
import {DEFAULT, token  } from "../../../services/LocalStorage";
import {push} from 'svelte-spa-router';
import NavBar from '../../../components/NavBar.svelte';

import { Routes } from '../../../models/Routes';

// Vars + Functions
let promise = Promise.resolve([]);

async function validate(response){
    if (response == "Unauthorized" || response==false){
        text = "Invalid username/password";
        token.set(DEFAULT);
        invalid=true;
    }else{
        token.set(response.token);
        push(Routes.Dashboard)
    }
}

async function onClickLogin() {
    promise = login(user);
    await promise.then(response=>validate(response)).catch(error=>invalid=true)
}

let user = new User();
let text = new String();

let invalid: boolean = false;

</script>

<NavBar></NavBar>
<div class="center">
    <div class="card-container">
        <Card>
            <h1>Log In</h1>
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
                <div style="color:#B71C1C">
                    <Label>{text}</Label>
                </div>
            </Content>
            <Actions>
                <Button on:click={() => onClickLogin()}>
                <Label>Login</Label>
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

