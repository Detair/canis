import { Component } from "solid-js";

const Main: Component = () => {
  return (
    <div class="flex h-screen">
      {/* Server/Channel Sidebar */}
      <aside class="w-60 bg-background-secondary flex flex-col">
        <div class="p-4 border-b border-background-tertiary">
          <h2 class="font-semibold">Server Name</h2>
        </div>
        <nav class="flex-1 overflow-y-auto p-2">
          {/* Channel list goes here */}
          <div class="text-text-muted text-sm p-2">No channels</div>
        </nav>
        <div class="p-2 bg-background-tertiary">
          {/* User panel */}
          <div class="flex items-center gap-2 p-2">
            <div class="w-8 h-8 rounded-full bg-primary"></div>
            <span class="text-sm">Username</span>
          </div>
        </div>
      </aside>

      {/* Main Content */}
      <main class="flex-1 flex flex-col bg-background-primary">
        {/* Header */}
        <header class="h-12 px-4 flex items-center border-b border-background-tertiary">
          <span class="font-medium"># general</span>
        </header>

        {/* Messages */}
        <div class="flex-1 overflow-y-auto p-4">
          <div class="text-text-muted text-center py-8">
            Welcome to VoiceChat!
          </div>
        </div>

        {/* Message Input */}
        <div class="p-4">
          <input
            type="text"
            class="input-field"
            placeholder="Message #general"
          />
        </div>
      </main>
    </div>
  );
};

export default Main;
