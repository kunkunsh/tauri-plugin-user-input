<script>
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import {
    ping,
    startListening,
    stopListening,
  } from "tauri-plugin-user-input-api";
</script>

<main class="container">
  <h1>Tauri Plugin User Input API</h1>
  <button
    on:click={() => {
      startListening().then(() => {
        console.log("Started listening");
      });
      listen("user-input", (event) => {
        // console.log(Object.keys(event.payload.event_type));

        // if (!Object.keys(event.payload.event_type).includes("MouseMove")) {
        console.log("Received event", event.payload);
        // }
      });
    }}
  >
    Start listening
  </button>
  <button
    on:click={() => {
      console.log("stopListening clicked");
      stopListening()
        .then(() => {
          console.log("Stopped listening");
        })
        .catch((err) => {
          console.error("Error stopping listening", err);
        });
    }}
  >
    Stop listening
  </button>
</main>
