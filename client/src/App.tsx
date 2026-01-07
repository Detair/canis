import { Component, createSignal, onMount } from "solid-js";
import { Routes, Route } from "@solidjs/router";

// Views
import Login from "./views/Login";
import Main from "./views/Main";

const App: Component = () => {
  const [isAuthenticated, setIsAuthenticated] = createSignal(false);

  onMount(async () => {
    // Check if user is already authenticated
    // TODO: Check stored session
  });

  return (
    <div class="h-screen bg-background-tertiary text-text-primary">
      <Routes>
        <Route path="/login" component={Login} />
        <Route path="/*" component={Main} />
      </Routes>
    </div>
  );
};

export default App;
