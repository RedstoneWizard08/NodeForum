<script lang="ts">
    import "$s/pages/register.scss";
    import { goto } from "$app/navigation";
    import { faker } from "@faker-js/faker";
    import { api } from "$api/index";
    import { autofill } from "$d/state";

    let passwordClass = "form-auth-password-show";
    let passwordInput: HTMLInputElement;

    let name: string;
    let username: string;
    let email: string;
    let password: string;

    const handleRegister = async () => {
        const info = await api.register({
            name,
            email,
            username,
            password,
        });

        if (info.status != 201) return alert("Error while registering!");

        $autofill = { username, password };

        goto("/auth/login");
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

    const fakeIt = () => {
        const firstName = faker.name.firstName();
        const lastName = faker.name.lastName();

        name = firstName + " " + lastName;
        username = faker.internet.userName();
        email = faker.internet.email(firstName, lastName, "test.nodeforum.io");
        password = faker.internet.password(20, false);
    };
</script>

<div class="form-auth-container">
    <form class="form-auth register" action="#" on:submit|preventDefault={handleRegister}>
        <p class="form-auth-title">Register for NodeForum</p>

        <hr class="form-auth-divider" />

        <input type="text" class="form-auth-input" placeholder="Your name..." bind:value={name} />
        <input type="text" class="form-auth-input" placeholder="Your username..." bind:value={username} />
        <input type="email" class="form-auth-input" placeholder="Your email..." bind:value={email} />

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
        <button type="button" class="form-auth-button" on:click={fakeIt}>Fake It</button>
    </form>
</div>
