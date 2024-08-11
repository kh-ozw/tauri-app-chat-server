import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from '@tauri-apps/api/event'
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [text, setText] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  function pushBtn1() {
    invoke("push_btn_1")
  }

  function commandWithMessage() {
    invoke('command_with_message', { message: 'some message' }).then(message => {
      console.log('command_with_message', message)
    });
  }

  function commandWithObject() {
    invoke('command_with_object', { message: { field_str: 'some message', field_u32: 15 } }).then(message => {
      console.log('command_with_opject', message)
    })
  }

  function commandWithError() {
    for (let arg of [1, 2]) {
      invoke('command_with_error', { arg }).then(message => {
        console.log('command_with_error', message)
      }).catch(message => {
        console.error('command_with_error', message)
      })
    }
  }

  function commandWithAsync() {
    invoke('command_with_async', { arg: 14 }).then(message => {
      console.log('command_with_async', message)
    })
  }

  useEffect(() => {
    let unlisten: any;
    async function f() {
      unlisten = await listen('back-to-front', event => {
        console.log(`${event.payload}`);
        setText(`${event.payload}`);
      })
    }
    f();

    let unlisten2: any;
    async function f2() {
      unlisten = await listen('emit_all_text', event => {
        console.log(`${event.payload}`);
        setText(`${event.payload}`);
      })
    }
    f();
    f2();

    return () => {
      if (unlisten) {
        unlisten();
      }
      if (unlisten2) {
        unlisten();
      }
    }
  }, [])

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet!</button>
      </form>

      <p>{greetMsg}</p>
      <p>{text}</p>
      <button onClick={pushBtn1}>btn 1</button>
      <button onClick={commandWithMessage}>cwm</button>
      <button onClick={commandWithObject}>cwo</button>
      <button onClick={commandWithError}>cwe</button>
      <button onClick={commandWithAsync}>cwa</button>
    </div>
  );
}

export default App;
