import { writable } from "svelte/store";
export const DEFAULT = "--empty--";

const storedToken = localStorage.getItem("token");
const storedAppState = localStorage.getItem("state");

export const token = writable(storedToken);
export const state = writable(storedAppState);

// token.subscribe(value => {
//     localStorage.setItem("token", value===null?DEFAULT:value);
// });

// state.subscribe(value => {
//     localStorage.setItem("state", value===null?ApplicationState[ApplicationState.LOGGED_OUT]:value);
// });