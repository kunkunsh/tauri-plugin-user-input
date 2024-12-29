<script lang="ts">
  import "./app.css";
  import { platform } from "@tauri-apps/plugin-os";
  import {
    scroll,
    EventType,
    InputEvent,
    setEventTypes,
    startListening,
    stopListening,
    key,
    text,
    selectAll,
    button,
    moveMouse,
  } from "tauri-plugin-user-input-api";
  import * as v from "valibot";
  import { LoremIpsum } from "lorem-ipsum";

  const lorem = new LoremIpsum({
    sentencesPerParagraph: {
      max: 8,
      min: 4,
    },
    wordsPerSentence: {
      max: 16,
      min: 4,
    },
  });

  let eventTypes = $state({
    KeyPress: false,
    KeyRelease: false,
    ButtonPress: false,
    ButtonRelease: false,
    MouseMove: false,
    Wheel: false,
  });

  let textarea: HTMLTextAreaElement | null = null;
  $effect(() => {
    const selectedEventTypes: EventType[] = Object.keys(eventTypes).filter(
      (key) => eventTypes[key]
    ) as EventType[];
    setEventTypes(selectedEventTypes);
  });
</script>

<main class="container mx-auto prose py-10">
  <h1>Tauri Plugin User Input API</h1>
  <h2>Listen to Events</h2>
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
  <h2>Simulate User Input</h2>
  <div class="flex flex-col gap-2">
    <textarea bind:this={textarea} class="textarea textarea-bordered"
    ></textarea>
    <button
      class="btn btn-secondary"
      onclick={() => {
        textarea.focus();
        text(lorem.generateParagraphs(1));
      }}>Text (Output Random Text)</button
    >
    <button
      class="btn btn-secondary"
      onclick={async () => {
        textarea.focus();
        switch (platform()) {
          case "macos":
            await key("KeyPress", "MetaLeft");
            await key("KeyPress", "KeyA");
            await key("KeyRelease", "MetaLeft");
            break;
          case "windows":
            await key("KeyPress", "ControlLeft");
            await key("KeyPress", "KeyA");
            await key("KeyRelease", "ControlLeft");
            break;
        }
        await key("KeyRelease", "KeyA");
        await key("KeyClick", "Backspace");
      }}>Key (Select All and Delete)</button
    >
    <button
      class="btn btn-secondary"
      onclick={async () => {
        await moveMouse(100, 100, "Rel");
        await button("Clicked", "Right");
      }}
    >
      Mouse Move and Right Click
    </button>
    <button
      class="btn btn-secondary"
      onclick={async () => {
        for (let i = 0; i < 10; i++) {
          await new Promise((resolve) => setTimeout(resolve, 100));
          await scroll(i, "Vertical");
          await new Promise((resolve) => setTimeout(resolve, 100));
          await scroll(-i, "Vertical");
        }
      }}
    >
      Scroll (10 times back and forth)
    </button>
  </div>
</main>
