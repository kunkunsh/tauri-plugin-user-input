<script lang="ts">
  import "./app.css";
  // import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import {
    EventType,
    InputEvent,
    setEventTypes,
    startListening,
    stopListening,
  } from "tauri-plugin-user-input-api";
  import * as v from "valibot";

  let eventTypes = $state({
    KeyPress: false,
    KeyRelease: false,
    ButtonPress: false,
    ButtonRelease: false,
    MouseMove: false,
    Wheel: false,
  });
  $effect(() => {
    const selectedEventTypes: EventType[] = Object.keys(eventTypes).filter(
      (key) => eventTypes[key]
    ) as EventType[];
    setEventTypes(selectedEventTypes);
  });
</script>

<main class="container">
  <h1>Tauri Plugin User Input API</h1>
  <button
    class="btn btn-primary"
    onclick={() => {
      startListening((evt) => {
        console.log("Received event", evt);
        const parsed = v.safeParse(InputEvent, evt);
        if (parsed.success) {
          console.log("parsed.output", parsed.output);
        } else {
          console.log(v.flatten(parsed.issues));
        }
      }).then(() => {
        console.log("Started listening");
      });
      // listen("user-input", (event) => {
      //   // console.log(Object.keys(event.payload.event_type));

      //   // if (!Object.keys(event.payload.event_type).includes("MouseMove")) {
      //   console.log("Received event", event.payload);
      //   // }
      // });
    }}
  >
    Start listening
  </button>
  <button
    class="btn btn-primary"
    onclick={() => {
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
  <label for="cars">Choose multiple event types to listen to:</label>
  <pre>{JSON.stringify(eventTypes, null, 2)}</pre>
  <div class="w-1/2">
    {#each Object.keys(eventTypes) as eventType}
      <div class="form-control">
        <label class="label cursor-pointer">
          <span class="label-text">{eventType}</span>
          <input
            type="checkbox"
            bind:checked={eventTypes[eventType]}
            class="checkbox"
          />
        </label>
      </div>
    {/each}
  </div>
</main>
