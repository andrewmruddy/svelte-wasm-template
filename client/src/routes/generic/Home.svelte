<script lang="ts">
// START Properties
// END Properties

// START Component Imports
import Button, { Label } from '@smui/button';
import Fab from '@smui/fab';
import IconButton from '@smui/icon-button';
import NavBar from '../../components/NavBar.svelte';
import Banner from '../../components/Visualizations/Banner.svelte';
// END Component Imports


// START Other Imports
import {DEFAULT, state, token} from '../../services/LocalStorage';
import {push, pop, replace} from 'svelte-spa-router'
import { ApplicationState } from '../../models/State';
import { Routes } from '../../models/Routes';
// END Other Imports


// START Local Vars
// END Local Vars


// START Functions
function onClickLoginHome(){
	if($state != ApplicationState[ApplicationState.LOGGED_OUT] && $token != DEFAULT){
		push(Routes.Dashboard)
	}else{
		push(Routes.Login)
	}
}
// END Functions

// START Init
// END Init
</script>


<NavBar></NavBar>	

<div class="center top">
	<div class="center">
		<div class="sign-up">
			<Fab color="primary" on:click={()=>push("#/SignUp")} extended ><Label style="width:100px">Sign Up</Label></Fab>
		</div>
		<div class="log-in">
			<Fab color="primary" on:click={()=>onClickLoginHome()} extended ><Label style="width:100px">Log In</Label></Fab>
		</div>
	</div>	
</div>
<hr>

<style>
	.sign-up{
		padding-right: 10px;
	}
	.log-in{
		padding-left: 10px;
	}

	.center {
		margin: auto;
		display: flex;
		flex-direction: row;
	}
	.top{
		padding-bottom: 50px;
		padding-top:50px;
	}
	img{
		height: 400px;
	}
	hr{
		width: 98vw;
	}
</style>
