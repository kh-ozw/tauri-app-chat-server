import { useEffect, useState } from "react";
import { listen } from '@tauri-apps/api/event'
import "./App.css";
import { MessageInfo, MsgPanel } from "./components/MsgPanel";

function App() {
  const [messagesInfo, setMessagesInfo] = useState<MessageInfo[]>([]);

  useEffect(() => {
    let unlisten: any;
    async function f() {
      unlisten = await listen('emit_all_message_info', event => {
        let payload = event.payload as MessageInfo;
        setMessagesInfo(prevMessagesInfo => [
          ...prevMessagesInfo,
          {
            message: payload.message,
            user: payload.user,
            date: new Date()
          }
        ]);
      })
    }
    f();
    return () => {
      if (unlisten) {
        unlisten();
      }
    }
  }, [])

  return (
    <div className="container">
      {messagesInfo.length ? messagesInfo.slice().reverse().map((messageInfo, index) => (
        <MsgPanel
          index={index}
          messageinfo={messageInfo}
        />
      )) : <div className="rounded-border">{"No message"}</div>}
    </div>
  );
}

export default App;
