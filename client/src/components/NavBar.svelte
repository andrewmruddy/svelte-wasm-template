<script lang="ts">
// START Properties
// END Properties


// START Component Imports
import IconButton from '@smui/icon-button';
import TopAppBar, { Row, Section, Title } from '@smui/top-app-bar';
import Checkbox from '@smui/checkbox';
import FormField from '@smui/form-field';
// END Component Imports


// START Other Imports
import {push, pop, replace} from 'svelte-spa-router'
import {token} from '../services/LocalStorage';
import {DEFAULT} from '../services/LocalStorage';
import {state} from '../services/LocalStorage';
// END Other Imports


// START Local Vars
let secondaryColor = false;
let visible = false;
// END Local Vars


// START Functions
// END Functions


// START Init
// END Init
</script>
<TopAppBar
	variant="standard"
	color={secondaryColor ? 'secondary' : 'primary'}
>
	<Row>
		<Section>
			<IconButton class="material-icons" style='margin:0%;padding:0;'>menu</IconButton>
			<div class="home" on:click={()=>push("#/")}>
				<Title>TITLE</Title>
			</div>
		</Section>
		<Section align="end" toolbar>
		</Section>
		</Row>
</TopAppBar>
<div class="app-bar-spacer">

</div>

<style>
	:global(app),
	:global(body),
	:global(html) {
		display: block !important;
		height: auto !important;
		width: auto !important;
		position: static !important;
	}
    @media only screen and (min-width: 601px) {
        .app-bar-spacer{
            height:64px;
        }
    }
    @media only screen and (max-width: 600px) {
        .app-bar-spacer{
            height: 56px;
        }
    }
	.home{
		cursor: pointer;
	}
</style>