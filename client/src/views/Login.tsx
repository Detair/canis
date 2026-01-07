import { Component, createSignal } from "solid-js";
import { useNavigate } from "@solidjs/router";

const Login: Component = () => {
  const navigate = useNavigate();
  const [serverUrl, setServerUrl] = createSignal("");
  const [username, setUsername] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [error, setError] = createSignal("");

  const handleLogin = async (e: Event) => {
    e.preventDefault();
    // TODO: Implement login via Tauri command
    navigate("/");
  };

  return (
    <div class="flex items-center justify-center h-screen">
      <div class="w-full max-w-md p-8 bg-background-secondary rounded-lg">
        <h1 class="text-2xl font-bold mb-6 text-center">VoiceChat</h1>
        
        <form onSubmit={handleLogin} class="space-y-4">
          <div>
            <label class="block text-sm text-text-secondary mb-1">Server URL</label>
            <input
              type="url"
              class="input-field"
              placeholder="https://chat.example.com"
              value={serverUrl()}
              onInput={(e) => setServerUrl(e.currentTarget.value)}
              required
            />
          </div>

          <div>
            <label class="block text-sm text-text-secondary mb-1">Username</label>
            <input
              type="text"
              class="input-field"
              placeholder="username"
              value={username()}
              onInput={(e) => setUsername(e.currentTarget.value)}
              required
            />
          </div>

          <div>
            <label class="block text-sm text-text-secondary mb-1">Password</label>
            <input
              type="password"
              class="input-field"
              placeholder="••••••••"
              value={password()}
              onInput={(e) => setPassword(e.currentTarget.value)}
              required
            />
          </div>

          {error() && (
            <div class="text-danger text-sm">{error()}</div>
          )}

          <button type="submit" class="btn-primary w-full">
            Login
          </button>
        </form>
      </div>
    </div>
  );
};

export default Login;
