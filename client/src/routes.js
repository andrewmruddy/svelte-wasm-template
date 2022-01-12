import Home from './routes/generic/Home.svelte';
import NotFound from './routes/generic/NotFound.svelte';
import About from './routes/generic/About.svelte';
import Login from './routes/generic/auth/Login.svelte';
import SignUp from './routes/generic/auth/SignUp.svelte';
import ThankYou from './routes/generic/auth/ThankYou.svelte';
import Register from './routes/generic/auth/Register.svelte';
import Dashboard from './routes/dashboards/Dashboard.svelte';

export default {
    '/': Home,
    '/About': About,
    '/Login': Login,
    '/SignUp': SignUp,
    '/Register/:token': Register,
    '/ThankYou': ThankYou,
    '/Dashboard': Dashboard,
    // The catch-all route must always be last
    '*': NotFound
};
