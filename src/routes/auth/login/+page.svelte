<script lang="ts">
    import "$s/pages/register.scss";
    import { AccountStatus } from "$t/account";
    import { status } from "$d/account";
    import { goto } from "$app/navigation";
    import { autofill } from "$d/state";
    import { api } from "$api";

    let passwordClass = "form-auth-password-show";
    let passwordInput: HTMLInputElement;

    let username: string = $autofill.username || "";
    let password: string = $autofill.password || "";

    const handleLogin = async () => {
        const data = await api.login({ username, password });

        $status = AccountStatus.LOGGED_IN;

        goto("/");
    };

    const showPassword = () => {
        if (passwordClass == "form-auth-password-show hidden") {
            passwordClass = "form-auth-password-show";
            passwordInput.type = "password";
        } else {
            passwordClass = "form-auth-password-show hidden";
            passwordInput.type = "text";
        }
    };
</script>

<div class="form-auth-container">
    <form class="form-auth login" action="#" on:submit|preventDefault={handleLogin}>
        <p class="form-auth-title">Sign in to NodeForum</p>

        <hr class="form-auth-divider" />

        <input type="text" class="form-auth-input" placeholder="Email or username..." bind:value={username} />

        <div class="form-auth-password-group">
            <input
                type="password"
                class="form-auth-input password"
                placeholder="Your password..."
                bind:this={passwordInput}
                bind:value={password}
            />
            
            <button type="button" class={passwordClass} on:click={showPassword} />
        </div>

        <input type="submit" class="form-auth-submit" value="Continue" />
    </form>
</div>
